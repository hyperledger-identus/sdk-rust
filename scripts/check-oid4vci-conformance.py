#!/usr/bin/env python3
"""Validate the OID4VCI Final wallet-core coverage matrix offline."""

from __future__ import annotations

import csv
import re
import sys
from collections import Counter
from pathlib import Path


MATRIX_PATH = Path("docs/conformance/oid4vci-final-wallet-core.csv")
EXPECTED_FIELDS = (
    "id",
    "section",
    "relevance",
    "status",
    "implementation_paths",
    "spec_paths",
    "test_paths",
    "limitation",
    "provenance",
    "followup_issue",
)
REQUIRED_SECTIONS = {str(section) for section in range(4, 13)}
RELEVANCE = {"required", "adjacent", "out-of-scope"}
STATUSES = {"implemented", "partial", "unsupported", "missing"}
PROVENANCE = {"sdk-authored", "reference-only", "not-applicable"}
ID_PATTERN = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*\Z")
SECTION_PATTERN = re.compile(
    r"(?:[4-9]|1[0-2])(?:\.[0-9]+)*(?:-(?:[4-9]|1[0-2])(?:\.[0-9]+)*)?\Z"
)
ISSUE_PATTERN = re.compile(r"#[1-9][0-9]*\Z")


def parse_paths(value: str, context: str, failures: list[str]) -> list[Path]:
    if value == "none":
        return []
    raw_paths = value.split(";")
    if any(not item for item in raw_paths):
        failures.append(f"{context}: evidence path list contains an empty item")
        return []
    if len(raw_paths) != len(set(raw_paths)):
        failures.append(f"{context}: evidence path list contains duplicates")
    paths: list[Path] = []
    for raw_path in raw_paths:
        candidate = Path(raw_path)
        if candidate.is_absolute() or ".." in candidate.parts or raw_path != candidate.as_posix():
            failures.append(f"{context}: unsafe repository path {raw_path}")
            continue
        paths.append(candidate)
    return paths


def validate_path(root: Path, relative: Path, context: str) -> str | None:
    candidate = root / relative
    try:
        resolved = candidate.resolve(strict=True)
        repository = root.resolve(strict=True)
    except OSError:
        return f"{context}: missing evidence path {relative.as_posix()}"
    if repository not in resolved.parents:
        return f"{context}: evidence path escapes repository {relative.as_posix()}"
    if candidate.is_symlink() or not candidate.is_file():
        return f"{context}: evidence path is not a regular non-symlink file {relative.as_posix()}"
    return None


def section_roots(section: str) -> set[str]:
    return {part.split(".", 1)[0] for part in section.split("-")}


def validate(root: Path, matrix: Path | None = None) -> tuple[list[str], Counter[str]]:
    failures: list[str] = []
    counts: Counter[str] = Counter()
    matrix_path = matrix or root / MATRIX_PATH
    try:
        with matrix_path.open(encoding="utf-8", newline="") as source:
            reader = csv.DictReader(source, strict=True)
            if tuple(reader.fieldnames or ()) != EXPECTED_FIELDS:
                return (["schema mismatch: expected " + ",".join(EXPECTED_FIELDS)], counts)
            rows = list(reader)
    except (OSError, csv.Error) as error:
        return ([f"cannot read {matrix_path}: {error}"], counts)

    identifiers = [row.get("id") or "" for row in rows]
    for identifier, count in Counter(identifiers).items():
        if count > 1:
            failures.append(f"duplicate capability id: {identifier}")

    covered_sections: set[str] = set()
    for line_number, row in enumerate(rows, start=2):
        values = {field: row.get(field) or "" for field in EXPECTED_FIELDS}
        context = values["id"] or f"line-{line_number}"
        if None in row:
            failures.append(f"{context}: row has fields outside the canonical schema")
        for field, value in values.items():
            if not value:
                failures.append(f"{context}: {field} is empty")
            elif value != value.strip():
                failures.append(f"{context}: {field} has surrounding whitespace")

        if not ID_PATTERN.fullmatch(values["id"]):
            failures.append(f"{context}: id must be kebab-case")
        section = values["section"]
        if section != "profile" and not SECTION_PATTERN.fullmatch(section):
            failures.append(f"{context}: invalid Final section {section}")
        if section != "profile":
            covered_sections.update(section_roots(section))
        if values["relevance"] not in RELEVANCE:
            failures.append(f"{context}: unknown relevance {values['relevance']}")
        if values["status"] not in STATUSES:
            failures.append(f"{context}: unknown status {values['status']}")
        else:
            counts[values["status"]] += 1
        if values["provenance"] not in PROVENANCE:
            failures.append(f"{context}: unknown provenance {values['provenance']}")
        if values["followup_issue"] != "none" and not ISSUE_PATTERN.fullmatch(values["followup_issue"]):
            failures.append(f"{context}: followup_issue must be none or #N")
        if len(values["limitation"].encode("utf-8")) > 512:
            failures.append(f"{context}: limitation exceeds 512 bytes")

        evidence: dict[str, list[Path]] = {}
        for field in ("implementation_paths", "spec_paths", "test_paths"):
            paths = parse_paths(values[field], f"{context}.{field}", failures)
            evidence[field] = paths
            for relative in paths:
                failure = validate_path(root, relative, f"{context}.{field}")
                if failure:
                    failures.append(failure)

        status = values["status"]
        if status in {"implemented", "partial"}:
            for field, paths in evidence.items():
                if not paths:
                    failures.append(f"{context}: {status} row requires {field}")
            if values["provenance"] != "sdk-authored":
                failures.append(f"{context}: executable SDK evidence must be sdk-authored")
        if status == "implemented" and values["followup_issue"] != "none":
            failures.append(f"{context}: implemented row cannot retain a follow-up issue")
        if status == "missing" and any(evidence.values()):
            failures.append(f"{context}: missing row cannot claim repository evidence")
        if status in {"partial", "unsupported", "missing"} and len(values["limitation"]) < 20:
            failures.append(f"{context}: incomplete row requires a substantive limitation")
        if status == "missing" and values["relevance"] == "required" and not ISSUE_PATTERN.fullmatch(values["followup_issue"]):
            failures.append(f"{context}: missing required row needs a focused issue")
        if values["provenance"] == "reference-only":
            if status not in {"unsupported", "missing"} or any(evidence.values()):
                failures.append(f"{context}: reference-only row cannot claim SDK execution evidence")
            if not ISSUE_PATTERN.fullmatch(values["followup_issue"]):
                failures.append(f"{context}: reference-only row needs a focused issue")

    missing_sections = sorted(REQUIRED_SECTIONS - covered_sections, key=int)
    if missing_sections:
        failures.append(f"missing Final sections: {','.join(missing_sections)}")
    if "profile" not in {row.get("section") for row in rows}:
        failures.append("missing cross-consumer profile evidence row")
    return failures, counts


def main() -> int:
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path(__file__).resolve().parents[1]
    failures, counts = validate(root)
    if failures:
        for failure in failures:
            print(f"oid4vci-conformance: {failure}", file=sys.stderr)
        print(f"oid4vci-conformance: {len(failures)} failure(s)", file=sys.stderr)
        return 1
    print(
        "oid4vci-conformance: "
        f"{sum(counts.values())} rows passed "
        f"(implemented={counts['implemented']}, partial={counts['partial']}, "
        f"unsupported={counts['unsupported']}, missing={counts['missing']})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
