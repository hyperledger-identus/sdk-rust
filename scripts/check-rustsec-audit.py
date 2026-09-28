#!/usr/bin/env python3
"""Produce explicit RustSec advisory evidence without implying yank coverage."""

from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


SCHEMA_VERSION = 1
YANK_UNAVAILABLE_REASON = "registry-index-not-provided"


def audit_command(executable: str, database: Path, lockfile: Path) -> list[str]:
    return [
        executable,
        "audit",
        "--no-fetch",
        "--db",
        str(database),
        "--file",
        str(lockfile),
        "--no-yanked",
        "--format",
        "json",
    ]


def load_report(raw: bytes) -> dict[str, Any] | None:
    try:
        value = json.loads(raw)
    except (UnicodeDecodeError, json.JSONDecodeError):
        return None
    return value if isinstance(value, dict) else None


def vulnerability_ids(report: dict[str, Any]) -> list[str]:
    vulnerabilities = report.get("vulnerabilities")
    if not isinstance(vulnerabilities, dict):
        return []
    entries = vulnerabilities.get("list")
    if not isinstance(entries, list):
        return []
    ids: list[str] = []
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        advisory = entry.get("advisory")
        if isinstance(advisory, dict) and isinstance(advisory.get("id"), str):
            ids.append(advisory["id"])
    return sorted(set(ids))


def vulnerability_count(report: dict[str, Any]) -> int | None:
    vulnerabilities = report.get("vulnerabilities")
    if not isinstance(vulnerabilities, dict):
        return None
    count = vulnerabilities.get("count")
    if type(count) is int and count >= 0:
        return count
    return None


def report_is_compatible(report: dict[str, Any]) -> bool:
    database = report.get("database")
    vulnerabilities = report.get("vulnerabilities")
    warnings = report.get("warnings")
    if not isinstance(database, dict) or not isinstance(vulnerabilities, dict):
        return False
    advisory_count = database.get("advisory-count")
    count = vulnerability_count(report)
    found = vulnerabilities.get("found")
    entries = vulnerabilities.get("list")
    return (
        type(advisory_count) is int
        and advisory_count > 0
        and count is not None
        and isinstance(found, bool)
        and found == (count > 0)
        and isinstance(entries, list)
        and len(entries) == count
        and all(
            isinstance(entry, dict)
            and isinstance(entry.get("advisory"), dict)
            and isinstance(entry["advisory"].get("id"), str)
            and bool(entry["advisory"]["id"])
            for entry in entries
        )
        and isinstance(warnings, dict)
        and "yanked" not in warnings
    )


def tool_version(executable: str) -> str | None:
    result = subprocess.run(
        [executable, "audit", "--version"],
        check=False,
        capture_output=True,
    )
    if result.returncode != 0:
        return None
    words = result.stdout.decode("utf-8", errors="replace").strip().split()
    return words[-1] if words else None


def probe(args: argparse.Namespace) -> int:
    version = tool_version(args.cargo_audit)
    if version != args.expected_tool_version:
        print(
            "rustsec-audit: incompatible-tool: "
            f"expected cargo-audit {args.expected_tool_version}, got {version or 'unknown'}",
            file=sys.stderr,
        )
        return 1

    result = subprocess.run(
        audit_command(args.cargo_audit, args.advisory_db, args.lockfile),
        check=False,
        capture_output=True,
    )
    report = load_report(result.stdout)
    count = vulnerability_count(report) if report is not None else None
    if (
        result.returncode != 0
        or report is None
        or not report_is_compatible(report)
        or count != 0
    ):
        digest = hashlib.sha256(result.stderr).hexdigest()
        print(
            "rustsec-audit: incompatible-tool: CVSS 4 compatibility probe failed "
            f"(stderr-sha256={digest})",
            file=sys.stderr,
        )
        return 1

    print(
        "rustsec-audit: parser-compatible "
        f"cargo-audit={version} fixture=cvss-4.0"
    )
    return 0


def write_evidence(path: Path, evidence: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(evidence, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def classify(args: argparse.Namespace) -> int:
    raw = sys.stdin.buffer.read()
    report = load_report(raw)
    compatible = report is not None and report_is_compatible(report)
    count = vulnerability_count(report) if compatible else None
    ids = vulnerability_ids(report) if report is not None else []

    if not compatible or count is None:
        status = "incompatible-tool"
    elif count > 0:
        status = "vulnerability"
    elif args.command_exit != 0:
        status = "incompatible-tool"
    else:
        status = "success"

    evidence: dict[str, Any] = {
        "schemaVersion": SCHEMA_VERSION,
        "tool": {
            "name": "cargo-audit",
            "version": args.expected_tool_version,
        },
        "advisoryDatabase": {"revision": args.advisory_db_revision},
        "advisories": {
            "status": status,
            "vulnerabilityCount": count,
            "advisoryIds": ids,
        },
        "yanked": {
            "status": "unavailable",
            "reason": YANK_UNAVAILABLE_REASON,
        },
    }
    if status == "incompatible-tool":
        evidence["advisories"]["rawReportSha256"] = hashlib.sha256(raw).hexdigest()
        evidence["advisories"]["commandExit"] = args.command_exit

    write_evidence(args.output, evidence)
    print(
        "rustsec-audit: "
        f"advisory={status} vulnerabilities={count if count is not None else 'unknown'} "
        f"yanked=unavailable reason={YANK_UNAVAILABLE_REASON}"
    )
    return 0 if status == "success" else 1


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser()
    commands = root.add_subparsers(dest="command", required=True)

    probe_parser = commands.add_parser("probe")
    probe_parser.add_argument("--cargo-audit", required=True)
    probe_parser.add_argument("--expected-tool-version", required=True)
    probe_parser.add_argument("--advisory-db", required=True, type=Path)
    probe_parser.add_argument("--lockfile", required=True, type=Path)
    probe_parser.set_defaults(handler=probe)

    classify_parser = commands.add_parser("classify")
    classify_parser.add_argument("--command-exit", required=True, type=int)
    classify_parser.add_argument("--expected-tool-version", required=True)
    classify_parser.add_argument("--advisory-db-revision", required=True)
    classify_parser.add_argument("--output", required=True, type=Path)
    classify_parser.set_defaults(handler=classify)
    return root


def main() -> int:
    args = parser().parse_args()
    return args.handler(args)


if __name__ == "__main__":
    raise SystemExit(main())
