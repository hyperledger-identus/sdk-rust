#!/usr/bin/env python3
"""Validate the SDK-Rust/SDK-TS parity and conformance milestone."""

from __future__ import annotations

import re
import sys
import tomllib
from collections import Counter
from pathlib import Path


MATRIX_PATH = Path("docs/roadmap/sdk-ts-parity-conformance.toml")
INVENTORY_PATH = Path("docs/architecture/identus-platform-ts-capabilities.toml")
SHA_PATTERN = re.compile(r"^[0-9a-f]{40}$")
RELATIONS = {
    "semantic-equivalent",
    "compatible-adapter",
    "platform-owned",
    "intentionally-different",
    "deferred",
    "retired",
}
STATES = {"delivered", "partial", "blocked", "excluded", "not-applicable"}
DIMENSIONS = {
    "normative-profile",
    "rust-contract",
    "adapter-mapping",
    "shared-vectors",
    "error-behavior",
    "resource-security",
    "target-runtime",
    "consumer-proof",
    "rollback",
}
TOP_LEVEL_KEYS = {
    "schema_version",
    "milestone_id",
    "milestone_number",
    "milestone_url",
    "title",
    "program_issue",
    "definition_issue",
    "status",
    "snapshot_date",
    "source",
    "delivery",
    "vocabulary",
    "capability",
}
CAPABILITY_KEYS = {
    "id",
    "relation",
    "state",
    "rationale",
    "required_dimensions",
    "known_deviations",
    "evidence",
    "owner_issue",
    "next_action",
    "consumer",
    "rollback",
    "non_claims",
}
DELIVERY = {
    "mode": "local-first-capability-trains",
    "target_changed_loc_min": 5000,
    "target_changed_loc_max": 10000,
    "target_is_merge_gate": False,
    "excluded_loc": ["generated", "vendored", "lockfiles", "evidence-artifacts"],
    "required_contents": ["implementation", "adapters", "vectors", "migration", "documentation", "tests"],
    "split_conditions": [
        "independent-ownership",
        "security-or-release-authority",
        "independent-rollback",
        "unsafe-review-size",
    ],
    "checkpoint_policy": "signed-reviewable-commits-with-targeted-local-gates",
    "candidate_policy": "complete-applicable-local-gate-before-push",
    "hosted_ci_policy": "mandatory-final-confirmation",
}


def text(value: object, field: str, errors: list[str]) -> str:
    if not isinstance(value, str) or not value.strip():
        errors.append(f"{field} must be non-empty text")
        return ""
    return value


def strings(value: object, field: str, errors: list[str], *, allow_empty: bool = False) -> list[str]:
    if not isinstance(value, list) or (not allow_empty and not value):
        errors.append(f"{field} must be {'a' if allow_empty else 'a non-empty'} string array")
        return []
    if not all(isinstance(item, str) and item.strip() for item in value):
        errors.append(f"{field} must contain only non-empty strings")
        return []
    return value


def load(path: Path, label: str, errors: list[str]) -> dict[str, object]:
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        errors.append(f"cannot load {label}: {error}")
        return {}


def main() -> int:
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path.cwd().resolve()
    errors: list[str] = []
    matrix = load(root / MATRIX_PATH, str(MATRIX_PATH), errors)
    inventory = load(root / INVENTORY_PATH, str(INVENTORY_PATH), errors)
    if errors:
        print("sdk-ts-parity-conformance: " + "; ".join(errors), file=sys.stderr)
        return 1

    if set(matrix) != TOP_LEVEL_KEYS:
        errors.append(
            "matrix top-level fields differ: "
            f"missing={sorted(TOP_LEVEL_KEYS - set(matrix))}, "
            f"unknown={sorted(set(matrix) - TOP_LEVEL_KEYS)}"
        )
    if matrix.get("schema_version") != 1:
        errors.append("schema_version must equal 1")
    if matrix.get("milestone_id") != "TS1" or matrix.get("program_issue") != 502:
        errors.append("TS1 must remain the issue #502 parity program")
    if matrix.get("milestone_number") != 7 or matrix.get("milestone_url") != "https://github.com/hyperledger-identus/sdk-rust/milestone/7":
        errors.append("TS1 must remain bound to GitHub milestone 7")
    if matrix.get("definition_issue") != 594 or matrix.get("status") != "baseline":
        errors.append("definition_issue/status must remain 594/baseline")

    source = matrix.get("source")
    inventory_source = inventory.get("source")
    if not isinstance(source, dict) or not isinstance(inventory_source, dict):
        errors.append("source tables must exist in matrix and inventory")
        source = {}
        inventory_source = {}
    if source.get("inventory") != str(INVENTORY_PATH):
        errors.append("source.inventory must name the canonical capability inventory")
    revision = source.get("revision")
    if not isinstance(revision, str) or not SHA_PATTERN.fullmatch(revision):
        errors.append("source.revision must be a full lowercase Git SHA")
    for field in ("repository", "revision", "package", "version"):
        if source.get(field) != inventory_source.get(field):
            errors.append(f"source.{field} must match the capability inventory")

    if matrix.get("delivery") != DELIVERY:
        errors.append("delivery must preserve the local-first capability-train contract")
    vocabulary = matrix.get("vocabulary")
    expected_vocabulary = {
        "relations": sorted(RELATIONS),
        "states": sorted(STATES),
        "dimensions": sorted(DIMENSIONS),
    }
    if not isinstance(vocabulary, dict):
        errors.append("vocabulary must be a table")
    else:
        for field, expected in expected_vocabulary.items():
            actual = vocabulary.get(field)
            if not isinstance(actual, list) or sorted(actual) != expected:
                errors.append(f"vocabulary.{field} must contain the closed {field} set")

    inventory_rows = inventory.get("capability")
    rows = matrix.get("capability")
    if not isinstance(inventory_rows, list) or not all(isinstance(item, dict) for item in inventory_rows):
        errors.append("inventory.capability must be an array of tables")
        inventory_rows = []
    if not isinstance(rows, list) or not all(isinstance(item, dict) for item in rows):
        errors.append("matrix capability must be an array of tables")
        rows = []

    inventory_by_id = {str(item.get("id")): item for item in inventory_rows}
    identifiers = [str(item.get("id")) for item in rows]
    duplicates = sorted(identifier for identifier, count in Counter(identifiers).items() if count > 1)
    if duplicates:
        errors.append(f"duplicate capability IDs: {duplicates}")
    if set(identifiers) != set(inventory_by_id):
        errors.append(
            "matrix capability coverage differs: "
            f"missing={sorted(set(inventory_by_id) - set(identifiers))}, "
            f"extra={sorted(set(identifiers) - set(inventory_by_id))}"
        )

    counts: Counter[str] = Counter()
    for index, row in enumerate(rows, start=1):
        identifier = str(row.get("id"))
        prefix = f"capability[{index}] {identifier}"
        if set(row) != CAPABILITY_KEYS:
            errors.append(
                f"{prefix} fields differ: missing={sorted(CAPABILITY_KEYS - set(row))}, "
                f"unknown={sorted(set(row) - CAPABILITY_KEYS)}"
            )
        relation = row.get("relation")
        state = row.get("state")
        if relation not in RELATIONS:
            errors.append(f"{prefix}: unknown relation {relation!r}")
        if state not in STATES:
            errors.append(f"{prefix}: unknown state {state!r}")
        counts[str(state)] += 1
        text(row.get("rationale"), f"{prefix}.rationale", errors)
        required = set(strings(row.get("required_dimensions"), f"{prefix}.required_dimensions", errors))
        if required - DIMENSIONS:
            errors.append(f"{prefix}: unknown dimensions {sorted(required - DIMENSIONS)}")
        deviations = strings(row.get("known_deviations"), f"{prefix}.known_deviations", errors)
        strings(row.get("evidence"), f"{prefix}.evidence", errors)
        strings(row.get("non_claims"), f"{prefix}.non_claims", errors)
        for field in ("next_action", "consumer", "rollback"):
            text(row.get(field), f"{prefix}.{field}", errors)
        if not isinstance(row.get("owner_issue"), int) or row["owner_issue"] <= 0:
            errors.append(f"{prefix}.owner_issue must be a positive issue number")

        inventory_row = inventory_by_id.get(identifier, {})
        disposition = inventory_row.get("disposition") if isinstance(inventory_row, dict) else None
        if relation == "platform-owned" and disposition != "retain-platform":
            errors.append(f"{prefix}: platform-owned requires retain-platform inventory ownership")
        if state == "not-applicable" and relation != "platform-owned":
            errors.append(f"{prefix}: not-applicable requires platform-owned relation")
        if relation == "platform-owned" and state != "not-applicable":
            errors.append(f"{prefix}: platform-owned must be explicitly not-applicable to semantic parity")
        if relation == "intentionally-different" and not deviations:
            errors.append(f"{prefix}: intentional difference requires an explicit deviation")
        if relation in {"deferred", "retired"} and state not in {"blocked", "excluded"}:
            errors.append(f"{prefix}: {relation} must remain blocked or excluded")
        if state == "delivered":
            missing_dimensions = DIMENSIONS - required
            if missing_dimensions:
                errors.append(f"{prefix}: delivered claim lacks dimensions {sorted(missing_dimensions)}")
            if "not activated" in str(row.get("next_action", "")).lower():
                errors.append(f"{prefix}: delivered claim contradicts its next action")

    if not rows or counts["partial"] + counts["blocked"] + counts["not-applicable"] != len(rows):
        errors.append("baseline must expose every capability as partial, blocked, or not-applicable")
    if counts["delivered"] or counts["excluded"]:
        errors.append("baseline must not claim delivered or excluded SDK-TS routes")

    if errors:
        for error in errors:
            print(f"sdk-ts-parity-conformance: {error}", file=sys.stderr)
        return 1
    print(
        "sdk-ts-parity-conformance: "
        f"{len(rows)} capabilities passed "
        f"(partial={counts['partial']}, blocked={counts['blocked']}, "
        f"not-applicable={counts['not-applicable']})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
