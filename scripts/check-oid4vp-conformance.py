#!/usr/bin/env python3
"""Validate the OpenID4VP Final wallet-core coverage matrix offline."""

from __future__ import annotations

import csv
import re
import sys
from collections import Counter
from pathlib import Path


MATRIX_PATH = Path("docs/conformance/oid4vp-final-wallet-core.csv")
REPORT_PATH = Path("docs/conformance/oid4vp-final-wallet-core.md")
BACKLOG_PATH = Path("docs/roadmap/ssi-upstream-dependency-backlog.csv")
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
REQUIRED_SECTIONS = {str(section) for section in range(5, 16)} | {"A", "B", "C"}
RELEVANCE = {"required", "adjacent", "separate"}
STATUSES = {"implemented", "partial", "unsupported", "not-applicable"}
PROVENANCE = {"sdk-authored", "not-applicable"}
ID_PATTERN = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*\Z")
SECTION_PATTERN = re.compile(
    r"(?:(?:[5-9]|1[0-5])(?:\.[0-9]+)*(?:-(?:[5-9]|1[0-5])(?:\.[0-9]+)*)?|[ABC]|profile)\Z"
)
ISSUE_PATTERN = re.compile(r"#[1-9][0-9]*\Z")
REPORT_SUMMARY_PATTERN = re.compile(
    r"<!-- oid4vp-summary: implemented=([0-9]+) partial=([0-9]+) "
    r"unsupported=([0-9]+) not-applicable=([0-9]+) required-gaps=([0-9]+) -->"
)
IMPLEMENTATION_PATH_PATTERN = re.compile(
    r"crates/[a-z0-9]+(?:-[a-z0-9]+)*/src/(?:[A-Za-z0-9_-]+/)*[A-Za-z0-9_-]+\.rs\Z"
)
SPEC_PATH_PATTERN = re.compile(r"openspec/specs/[a-z0-9]+(?:-[a-z0-9]+)*/spec\.md\Z")
TEST_PATH_PATTERN = re.compile(r"crates/[a-z0-9]+(?:-[a-z0-9]+)*/tests/[A-Za-z0-9_-]+\.rs\Z")
EXPECTED_CAPABILITIES = {
    "authorization-request-core": ("5.1-5.7", "required"),
    "request-object-audience-security": ("5.8", "required"),
    "client-identifier-schemes": ("5.9", "required"),
    "request-uri-post": ("5.10", "required"),
    "verifier-info": ("5.11", "adjacent"),
    "dcql": ("6", "required"),
    "claims-path-pointer": ("7", "required"),
    "vp-token-response": ("8.1", "required"),
    "direct-post": ("8.2", "required"),
    "redirect-response-modes": ("8.1-8.2", "required"),
    "encrypted-response": ("8.3-8.3.1", "required"),
    "transaction-data": ("8.4", "required"),
    "authorization-error-response": ("8.5", "required"),
    "vp-token-validation": ("8.6", "required"),
    "wallet-invocation": ("9", "required"),
    "wallet-metadata": ("10", "adjacent"),
    "verifier-metadata": ("11", "required"),
    "verifier-attestation": ("12", "adjacent"),
    "implementation-considerations": ("13", "adjacent"),
    "security-considerations": ("14", "required"),
    "privacy-considerations": ("15", "adjacent"),
    "digital-credentials-api": ("A", "separate"),
    "credential-format-profiles": ("B", "separate"),
    "siopv2-combination": ("C", "separate"),
    "reference-harness": ("profile", "required"),
}


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


def section_root(section: str) -> str:
    return section.split(".", 1)[0].split("-", 1)[0]


def backlog_status(root: Path, failures: list[str]) -> str | None:
    try:
        with (root / BACKLOG_PATH).open(encoding="utf-8", newline="") as source:
            rows = list(csv.DictReader(source, strict=True))
    except (OSError, csv.Error) as error:
        failures.append(f"cannot read {BACKLOG_PATH}: {error}")
        return None
    matches = [row for row in rows if row.get("id") == "IDR-024"]
    if len(matches) != 1:
        failures.append("backlog must contain exactly one IDR-024 row")
        return None
    return matches[0].get("delivery_status")


def validate_report_summary(
    root: Path, counts: Counter[str], required_gaps: int, failures: list[str]
) -> None:
    try:
        report = (root / REPORT_PATH).read_text(encoding="utf-8")
    except OSError as error:
        failures.append(f"cannot read {REPORT_PATH}: {error}")
        return
    matches = REPORT_SUMMARY_PATTERN.findall(report)
    if len(matches) != 1:
        failures.append("report must contain exactly one canonical oid4vp-summary marker")
        return
    actual = tuple(int(value) for value in matches[0])
    expected = (
        counts["implemented"],
        counts["partial"],
        counts["unsupported"],
        counts["not-applicable"],
        required_gaps,
    )
    if actual != expected:
        failures.append(f"report summary is stale: expected {expected}, found {actual}")


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
    required_gaps = 0
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
        if not SECTION_PATTERN.fullmatch(values["section"]):
            failures.append(f"{context}: invalid Final section {values['section']}")
        elif values["section"] != "profile":
            covered_sections.add(section_root(values["section"]))
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
                rendered = relative.as_posix()
                if field == "implementation_paths" and not IMPLEMENTATION_PATH_PATTERN.fullmatch(rendered):
                    failures.append(f"{context}.{field}: not a crate implementation path {rendered}")
                if field == "spec_paths" and not SPEC_PATH_PATTERN.fullmatch(rendered):
                    failures.append(f"{context}.{field}: not a canonical OpenSpec path {rendered}")
                if field == "test_paths" and not TEST_PATH_PATTERN.fullmatch(rendered):
                    failures.append(f"{context}.{field}: not an executable crate test path {rendered}")
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
        elif any(evidence.values()):
            failures.append(f"{context}: {status} row cannot claim executable evidence")
        if status == "implemented" and values["followup_issue"] != "none":
            failures.append(f"{context}: implemented row cannot retain a follow-up issue")
        if status != "implemented" and len(values["limitation"]) < 20:
            failures.append(f"{context}: incomplete row requires a substantive limitation")
        if values["relevance"] == "required" and status != "implemented":
            required_gaps += 1
            if not ISSUE_PATTERN.fullmatch(values["followup_issue"]):
                failures.append(f"{context}: incomplete required row needs a focused issue")
        if values["relevance"] == "separate" and status != "not-applicable":
            failures.append(f"{context}: separate capability must be not-applicable")
        if status == "not-applicable" and values["provenance"] != "not-applicable":
            failures.append(f"{context}: not-applicable row needs not-applicable provenance")

    missing_sections = sorted(REQUIRED_SECTIONS - covered_sections)
    if missing_sections:
        failures.append(f"missing Final sections: {','.join(missing_sections)}")
    if "profile" not in {row.get("section") for row in rows}:
        failures.append("missing reference-harness profile evidence row")

    observed_capabilities = {
        row.get("id", ""): (row.get("section", ""), row.get("relevance", ""))
        for row in rows
    }
    if observed_capabilities != EXPECTED_CAPABILITIES:
        missing = sorted(EXPECTED_CAPABILITIES.keys() - observed_capabilities.keys())
        extra = sorted(observed_capabilities.keys() - EXPECTED_CAPABILITIES.keys())
        moved = sorted(
            identifier
            for identifier in EXPECTED_CAPABILITIES.keys() & observed_capabilities.keys()
            if observed_capabilities[identifier] != EXPECTED_CAPABILITIES[identifier]
        )
        failures.append(
            "capability contract changed "
            f"(missing={','.join(missing) or 'none'}; extra={','.join(extra) or 'none'}; "
            f"section-or-relevance-mismatch={','.join(moved) or 'none'})"
        )

    status = backlog_status(root, failures)
    if required_gaps and status != "in_progress":
        failures.append(
            f"IDR-024 must remain in_progress with {required_gaps} incomplete required row(s)"
        )
    validate_report_summary(root, counts, required_gaps, failures)
    return failures, counts


def main() -> int:
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path(__file__).resolve().parents[1]
    failures, counts = validate(root)
    if failures:
        for failure in failures:
            print(f"oid4vp-conformance: {failure}", file=sys.stderr)
        print(f"oid4vp-conformance: {len(failures)} failure(s)", file=sys.stderr)
        return 1
    print(
        "oid4vp-conformance: "
        f"{sum(counts.values())} rows passed "
        f"(implemented={counts['implemented']}, partial={counts['partial']}, "
        f"unsupported={counts['unsupported']}, not-applicable={counts['not-applicable']})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
