#!/usr/bin/env python3
"""Fail-closed supply-chain gate for the separately locked UniFFI generator."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from datetime import date, datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "tools/uniffi-bindgen/Cargo.toml"
LOCK = ROOT / "tools/uniffi-bindgen/Cargo.lock"
ADVISORY = "RUSTSEC-2026-0374"
EXPIRY = date(2027, 1, 31)
EXPECTED_TREE = (
    "basic-toml v0.1.10",
    "askama_derive v0.16.1",
    "askama_macros v0.16.1 (proc-macro)",
    "askama v0.16.1",
    "uniffi_bindgen v0.32.0",
    "uniffi v0.32.0",
    "identus-uniffi-bindgen v0.0.0 (workspace)",
)
WORKSPACE_ROOT = re.compile(
    r"^identus-uniffi-bindgen v0\.0\.0 \((?P<path>.+)/tools/uniffi-bindgen\)$"
)


class GateFailure(RuntimeError):
    """A redacted fail-closed gate failure."""


def parse_as_of(value: str | None) -> date:
    if value is None:
        return datetime.now(timezone.utc).date()
    try:
        parsed = date.fromisoformat(value)
    except ValueError as error:
        raise GateFailure("--as-of must be an ISO YYYY-MM-DD date") from error
    if parsed.isoformat() != value:
        raise GateFailure("--as-of must be an ISO YYYY-MM-DD date")
    return parsed


def validate_clock(as_of: date) -> None:
    if as_of > EXPIRY:
        raise GateFailure(
            f"{ADVISORY} acceptance expired after {EXPIRY.isoformat()}; "
            "remove the dependency or approve a new issue-owned decision"
        )


def normalize_tree(output: str) -> tuple[str, ...]:
    lines = tuple(line.rstrip() for line in output.splitlines() if line.strip())
    if not lines:
        raise GateFailure("locked inverse dependency tree is empty")
    match = WORKSPACE_ROOT.fullmatch(lines[-1])
    if match is None or Path(match.group("path")).resolve() != ROOT.resolve():
        raise GateFailure("locked inverse dependency tree has an unexpected tool root")
    return (*lines[:-1], "identus-uniffi-bindgen v0.0.0 (workspace)")


def run(command: list[str], *, capture: bool = False) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(
        command,
        cwd=ROOT,
        check=False,
        capture_output=capture,
        text=True,
        timeout=600,
    )
    if result.returncode != 0:
        label = " ".join(command[:2])
        raise GateFailure(f"{label} failed with exit code {result.returncode}")
    return result


def validate_files() -> None:
    for path in (MANIFEST, LOCK):
        if path.is_symlink() or not path.is_file():
            raise GateFailure(f"required regular file is unavailable: {path.relative_to(ROOT)}")


def check(as_of: date) -> None:
    validate_clock(as_of)
    validate_files()
    tree = run(
        [
            "cargo",
            "tree",
            "--manifest-path",
            str(MANIFEST),
            "--locked",
            "-i",
            "basic-toml@0.1.10",
            "--prefix",
            "none",
        ],
        capture=True,
    )
    if normalize_tree(tree.stdout) != EXPECTED_TREE:
        raise GateFailure(
            "locked inverse dependency tree differs from ADR 0191; "
            "do not extend the acceptance implicitly"
        )

    run(
        [
            "cargo",
            "deny",
            "--manifest-path",
            str(MANIFEST),
            "--locked",
            "check",
            "bans",
            "licenses",
            "sources",
        ]
    )
    run(
        [
            "cargo",
            "audit",
            "--file",
            str(LOCK),
            "--deny",
            "warnings",
            "--ignore",
            ADVISORY,
        ]
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--as-of",
        help="UTC date override for deterministic tests; production omits this flag",
    )
    arguments = parser.parse_args()
    try:
        as_of = parse_as_of(arguments.as_of)
        check(as_of)
    except (GateFailure, subprocess.TimeoutExpired) as error:
        print(f"uniffi-generator-supply-chain: {error}", file=sys.stderr)
        return 1
    print(
        "uniffi-generator-supply-chain: exact build-only acceptance passed "
        f"for {ADVISORY} through {EXPIRY.isoformat()}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
