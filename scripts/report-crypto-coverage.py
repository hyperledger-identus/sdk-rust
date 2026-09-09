#!/usr/bin/env python3
"""Normalize and enforce the identus-crypto LLVM line-coverage contract."""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from decimal import Decimal, ROUND_HALF_UP
from pathlib import Path, PurePosixPath
from typing import Any

TOOL_VERSION = "cargo-llvm-cov 0.9.0"
RUST_VERSION_PREFIX = "rustc 1.98.1 "
THRESHOLD_BASIS_POINTS = 7482
PROFILES = [
    {"id": "default", "cargo_arguments": ["--package", "identus-crypto"]},
    {
        "id": "all-features",
        "cargo_arguments": ["--package", "identus-crypto", "--all-features"],
    },
    {
        "id": "no-default-features",
        "cargo_arguments": [
            "--package",
            "identus-crypto",
            "--no-default-features",
        ],
    },
    {
        "id": "kmp-compat",
        "cargo_arguments": [
            "--package",
            "identus-crypto",
            "--features",
            "kmp-compat",
        ],
    },
]
EXCLUSIONS = ["dependencies", "tests", "examples", "generated-code"]
SHA = re.compile(r"^[0-9a-f]{40}$")


class CoverageError(ValueError):
    """Coverage input does not satisfy the evidence contract."""


def require_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise CoverageError(f"{label} must be an object")
    return value


def require_line_counts(value: Any, label: str) -> tuple[int, int]:
    lines = require_dict(value, label)
    count = lines.get("count")
    covered = lines.get("covered")
    if (
        not isinstance(count, int)
        or isinstance(count, bool)
        or count < 0
        or not isinstance(covered, int)
        or isinstance(covered, bool)
        or covered < 0
        or covered > count
    ):
        raise CoverageError(f"{label} has invalid count/covered values")
    return count, covered


def percent(covered: int, count: int) -> str:
    value = (Decimal(covered) * Decimal(100) / Decimal(count)).quantize(
        Decimal("0.000001"), rounding=ROUND_HALF_UP
    )
    return format(value, "f")


def load_files(raw_path: Path, repository_root: Path) -> list[dict[str, Any]]:
    try:
        raw = json.loads(raw_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        raise CoverageError(f"cannot load LLVM JSON: {error}") from error
    if raw.get("type") != "llvm.coverage.json.export":
        raise CoverageError("input is not an LLVM coverage JSON export")
    data = raw.get("data")
    if not isinstance(data, list) or len(data) != 1:
        raise CoverageError("LLVM JSON must contain exactly one data entry")
    rows = require_dict(data[0], "LLVM data entry").get("files")
    if not isinstance(rows, list):
        raise CoverageError("LLVM data files must be an array")

    lexical_source_root = Path(os.path.abspath(repository_root / "crates/crypto/src"))
    source_root = (repository_root / "crates/crypto/src").resolve(strict=True)
    normalized: list[dict[str, Any]] = []
    seen: set[str] = set()
    for index, value in enumerate(rows):
        row = require_dict(value, f"LLVM file[{index}]")
        filename = row.get("filename")
        if not isinstance(filename, str) or not filename:
            raise CoverageError(f"LLVM file[{index}] filename must be text")
        candidate = Path(filename)
        if not candidate.is_absolute():
            candidate = repository_root / PurePosixPath(filename)
        lexical = Path(os.path.abspath(candidate))
        try:
            lexical.relative_to(lexical_source_root)
        except ValueError:
            continue
        try:
            resolved = candidate.resolve(strict=True)
            relative = resolved.relative_to(source_root)
        except (OSError, ValueError) as error:
            raise CoverageError(f"crypto source path escapes or is missing: {filename}") from error
        if not resolved.is_file() or resolved.suffix != ".rs":
            raise CoverageError(f"crypto source path is not a regular Rust file: {filename}")
        path = (PurePosixPath("crates/crypto/src") / PurePosixPath(relative)).as_posix()
        if path in seen:
            raise CoverageError(f"duplicate crypto source path: {path}")
        seen.add(path)
        summary = require_dict(row.get("summary"), f"{path} summary")
        count, covered = require_line_counts(summary.get("lines"), f"{path} lines")
        if count == 0:
            continue
        normalized.append(
            {
                "path": path,
                "lines": count,
                "covered": covered,
                "uncovered": count - covered,
                "percent": percent(covered, count),
            }
        )
    normalized.sort(key=lambda item: item["path"])
    if not normalized:
        raise CoverageError("LLVM JSON contains no executable identus-crypto source files")
    return normalized


def normalize(
    raw_path: Path,
    repository_root: Path,
    revision: str,
    rust_version: str,
    tool_version: str,
) -> dict[str, Any]:
    if not SHA.fullmatch(revision):
        raise CoverageError("revision must be a full lowercase Git SHA")
    if not rust_version.startswith(RUST_VERSION_PREFIX):
        raise CoverageError("rust version must be the pinned Rust 1.98.1 compiler")
    if tool_version != TOOL_VERSION:
        raise CoverageError(f"coverage tool must be exactly {TOOL_VERSION}")
    files = load_files(raw_path, repository_root)
    total = sum(item["lines"] for item in files)
    covered = sum(item["covered"] for item in files)
    if covered * 10_000 < total * THRESHOLD_BASIS_POINTS:
        raise CoverageError(
            f"line coverage {percent(covered, total)}% is below the 74.82% threshold"
        )
    return {
        "schema_version": 1,
        "status": "threshold-met",
        "revision": revision,
        "rust_version": rust_version,
        "coverage_tool": tool_version,
        "scope": "crates/crypto/src/**/*.rs",
        "profiles": PROFILES,
        "exclusions": EXCLUSIONS,
        "threshold_percent": "74.82",
        "totals": {
            "lines": total,
            "covered": covered,
            "uncovered": total - covered,
            "percent": percent(covered, total),
        },
        "files": files,
        "limitations": (
            "Line coverage only; no branch, path, mutation, side-channel, runtime, "
            "production-input, certification, or release claim."
        ),
    }


def markdown(summary: dict[str, Any]) -> str:
    totals = summary["totals"]
    lines = [
        "# identus-crypto line coverage",
        "",
        f"- Revision: `{summary['revision']}`",
        f"- Rust: `{summary['rust_version']}`",
        f"- Tool: `{summary['coverage_tool']}`",
        f"- Scope: `{summary['scope']}`",
        f"- Result: **{totals['percent']}%** ({totals['covered']} / {totals['lines']})",
        f"- Threshold: **{summary['threshold_percent']}%**",
        "- Profiles: " + ", ".join(f"`{row['id']}`" for row in summary["profiles"]),
        "- Exclusions: " + ", ".join(summary["exclusions"]),
        "",
        "| First-party source | Covered | Lines | Percent |",
        "| --- | ---: | ---: | ---: |",
    ]
    for row in summary["files"]:
        lines.append(
            f"| `{row['path']}` | {row['covered']} | {row['lines']} | {row['percent']}% |"
        )
    lines.extend(["", f"Limitation: {summary['limitations']}", ""])
    return "\n".join(lines)


def write_atomic(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_name(f".{path.name}.tmp-{os.getpid()}")
    temporary.write_text(content, encoding="utf-8")
    temporary.replace(path)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("raw_json", type=Path)
    parser.add_argument("output_directory", type=Path)
    parser.add_argument("--repository-root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--revision", required=True)
    parser.add_argument("--rust-version", required=True)
    parser.add_argument("--tool-version", required=True)
    args = parser.parse_args()
    try:
        summary = normalize(
            args.raw_json,
            Path(os.path.abspath(args.repository_root)),
            args.revision,
            args.rust_version,
            args.tool_version,
        )
        write_atomic(
            args.output_directory / "summary.json",
            json.dumps(summary, indent=2, sort_keys=True) + "\n",
        )
        write_atomic(args.output_directory / "summary.md", markdown(summary))
    except CoverageError as error:
        print(f"crypto-coverage: {error}", file=sys.stderr)
        return 1
    totals = summary["totals"]
    print(
        f"crypto-coverage: {totals['covered']}/{totals['lines']} lines "
        f"({totals['percent']}%) meets 74.82%"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
