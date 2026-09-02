#!/usr/bin/env python3

import csv
import re
import sys
from collections import Counter
from pathlib import Path


EXPECTED_FIELDS = (
    "id",
    "owner",
    "priority",
    "target_gate",
    "component",
    "outcome",
    "acceptance_evidence",
    "consumer_dependency",
    "commitment",
    "delivery_status",
    "issue",
    "owner_surface",
    "predecessors",
    "normative_source",
    "source_repositories",
)
EXPECTED_IDS = (
    "IDR-001",
    "IDR-002",
    "IDR-003",
    "IDR-004",
    "IDR-005",
    "IDR-006",
    "IDR-007",
    "IDR-008",
    "IDR-009",
    "IDR-010",
    "IDR-011",
    "IDR-020",
    "IDR-021",
    "IDR-022",
    "IDR-023",
    "IDR-024",
    "IDR-025",
    "IDR-026",
    "IDR-027",
    "IDR-028",
    "IDR-029",
    "IDR-030",
    "IDR-031",
    "IDR-040",
    "IDR-041",
    "IDR-042",
    "IDR-043",
    "IDR-044",
    "IDR-050",
    "IDR-051",
)
PRIORITIES = {"P0", "P1", "P2", "P3"}
TARGET_GATES = {"R0", "R2", "R3", "R5"}
COMMITMENTS = {"Foundation", "Committed", "Planned", "Conditional"}
DELIVERY_STATUSES = {"delivered", "in_progress", "specified", "queued", "conditional"}
SOURCE_REPOSITORIES = {
    "sdk-rust",
    "apollo",
    "neoprism",
    "midnight-identity",
    "lace-id-portal",
    "oxid",
}
ID_PATTERN = re.compile(r"IDR-[0-9]{3}\Z")
ISSUE_PATTERN = re.compile(r"#[1-9][0-9]*\Z")
SURFACE_PATTERN = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*\Z")


def default_backlog() -> Path:
    return Path(__file__).resolve().parents[1] / "docs/roadmap/ssi-upstream-dependency-backlog.csv"


def validate(path: Path) -> list[str]:
    failures: list[str] = []
    try:
        with path.open(encoding="utf-8", newline="") as source:
            reader = csv.DictReader(source, strict=True)
            if tuple(reader.fieldnames or ()) != EXPECTED_FIELDS:
                return [
                    "schema mismatch: expected "
                    + ",".join(EXPECTED_FIELDS)
                    + "; received "
                    + ",".join(reader.fieldnames or ())
                ]
            rows = list(reader)
    except (OSError, csv.Error) as error:
        return [f"cannot read {path}: {error}"]

    identifiers = [row.get("id") or "" for row in rows]
    if tuple(identifiers) != EXPECTED_IDS:
        missing = sorted(set(EXPECTED_IDS) - set(identifiers))
        unexpected = sorted(set(identifiers) - set(EXPECTED_IDS))
        duplicates = sorted(identifier for identifier, count in Counter(identifiers).items() if count > 1)
        if missing:
            failures.append(f"missing identifiers: {','.join(missing)}")
        if unexpected:
            failures.append(f"unexpected identifiers: {','.join(unexpected)}")
        if duplicates:
            failures.append(f"duplicate identifiers: {','.join(duplicates)}")
        if not (missing or unexpected or duplicates):
            failures.append("identifiers are not in canonical order")

    known_identifiers = set(identifiers)
    identifier_positions = {identifier: position for position, identifier in enumerate(EXPECTED_IDS)}
    for line_number, row in enumerate(rows, start=2):
        values = {field: row.get(field) or "" for field in EXPECTED_FIELDS}
        identifier = values["id"] or f"line-{line_number}"
        if None in row:
            failures.append(f"{identifier}: row has fields outside the canonical schema")
        for field in EXPECTED_FIELDS:
            value = values[field]
            if not value.strip():
                failures.append(f"{identifier}: {field} is empty")
            elif value != value.strip():
                failures.append(f"{identifier}: {field} has surrounding whitespace")

        if not ID_PATTERN.fullmatch(values["id"]):
            failures.append(f"{identifier}: id must match IDR-NNN")
        if values["owner"] != "Identus SDK-Rust":
            failures.append(f"{identifier}: owner must be Identus SDK-Rust")
        if values["priority"] not in PRIORITIES:
            failures.append(f"{identifier}: unknown priority {values['priority']}")
        if values["target_gate"] not in TARGET_GATES:
            failures.append(f"{identifier}: unknown target_gate {values['target_gate']}")
        if values["commitment"] not in COMMITMENTS:
            failures.append(f"{identifier}: unknown commitment {values['commitment']}")
        if values["delivery_status"] not in DELIVERY_STATUSES:
            failures.append(f"{identifier}: unknown delivery_status {values['delivery_status']}")
        if not ISSUE_PATTERN.fullmatch(values["issue"]):
            failures.append(f"{identifier}: issue must match #N")
        if not SURFACE_PATTERN.fullmatch(values["owner_surface"]):
            failures.append(f"{identifier}: owner_surface must be kebab-case")

        if values["issue"] == "#20" and values["delivery_status"] not in {"queued", "conditional"}:
            failures.append(f"{identifier}: program issue #20 cannot own active component delivery")
        if values["commitment"] == "Conditional" and values["delivery_status"] != "conditional":
            failures.append(f"{identifier}: Conditional commitment must have conditional delivery_status")
        if values["delivery_status"] == "conditional" and values["commitment"] != "Conditional":
            failures.append(f"{identifier}: conditional delivery_status requires Conditional commitment")

        predecessors = values["predecessors"].split(";")
        if predecessors != ["none"]:
            if len(predecessors) != len(set(predecessors)):
                failures.append(f"{identifier}: predecessors contain duplicates")
            for predecessor in predecessors:
                if predecessor == identifier:
                    failures.append(f"{identifier}: row cannot depend on itself")
                elif predecessor not in known_identifiers:
                    failures.append(f"{identifier}: unknown predecessor {predecessor}")
                elif predecessor in identifier_positions and identifier in identifier_positions and (
                    identifier_positions[predecessor] >= identifier_positions[identifier]
                ):
                    failures.append(f"{identifier}: predecessor {predecessor} must appear earlier")

        sources = values["source_repositories"].split(";")
        if len(sources) != len(set(sources)):
            failures.append(f"{identifier}: source_repositories contain duplicates")
        for source in sources:
            if source not in SOURCE_REPOSITORIES:
                failures.append(f"{identifier}: unknown source repository {source}")

    return failures


def main() -> int:
    path = Path(sys.argv[1]) if len(sys.argv) > 1 else default_backlog()
    failures = validate(path)
    if failures:
        for failure in failures:
            print(f"ssi-upstream-backlog: {failure}", file=sys.stderr)
        return 1
    print(f"ssi-upstream-backlog: {len(EXPECTED_IDS)} rows passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
