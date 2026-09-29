#!/usr/bin/env python3
"""Validate the A1 compatibility-foundation delivery graph offline."""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path
from typing import Any


PLAN_PATH = Path("docs/roadmap/a1-shared-compatibility-foundation.toml")
ROADMAP_PATH = Path("docs/roadmap/platform-core-adoption-milestones.toml")
QUALITY_CLASSES = ["property", "fuzz", "benchmark", "differential"]
CAPABILITY_ISSUES = [420, 505, 501, 422]
EXPECTED_CAPABILITIES = {
    420: ("vector-catalog", []),
    505: ("adapter-mappings", []),
    501: ("quality-routing", [420]),
    422: ("change-ledger", [420, 505]),
}
TOP_LEVEL_KEYS = {
    "schema_version",
    "status",
    "program_issue",
    "parent_issue",
    "github_milestone",
    "title",
    "ordering",
    "planning_adr",
    "openspec_change",
    "quality_classes",
    "root_issues",
    "closure_order",
    "stop_before",
    "capability",
    "closeout",
    "downstream",
}
CAPABILITY_KEYS = {
    "id",
    "issue",
    "title",
    "state",
    "blocked_by",
    "completion_needs",
    "planned_paths",
    "deliverables",
    "acceptance",
    "non_claims",
}


def load(path: Path, label: str, errors: list[str]) -> dict[str, Any]:
    try:
        with path.open("rb") as handle:
            value = tomllib.load(handle)
    except (OSError, tomllib.TOMLDecodeError) as error:
        errors.append(f"cannot load {label} {path}: {error}")
        return {}
    return value


def string_list(value: object, label: str, errors: list[str]) -> list[str]:
    if not isinstance(value, list) or not value or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        errors.append(f"{label} must be a non-empty string array")
        return []
    return value


def int_list(value: object, label: str, errors: list[str]) -> list[int]:
    if not isinstance(value, list) or not all(
        isinstance(item, int) and item > 0 for item in value
    ):
        errors.append(f"{label} must be an array of positive issue numbers")
        return []
    return value


def validate_plan(root: Path, errors: list[str]) -> None:
    plan = load(root / PLAN_PATH, "A1 plan", errors)
    if not plan:
        return
    if set(plan) != TOP_LEVEL_KEYS:
        errors.append(
            "A1 top-level fields differ: "
            f"missing={sorted(TOP_LEVEL_KEYS - set(plan))}, "
            f"unknown={sorted(set(plan) - TOP_LEVEL_KEYS)}"
        )

    expected_scalars = {
        "schema_version": 1,
        "status": "implementation-ready",
        "program_issue": 497,
        "parent_issue": 504,
        "github_milestone": 5,
        "ordering": "dependency",
        "planning_adr": 173,
        "openspec_change": "specify-a1-compatibility-foundation",
    }
    for key, expected in expected_scalars.items():
        if plan.get(key) != expected:
            errors.append(f"A1 {key} must equal {expected!r}")
    if plan.get("quality_classes") != QUALITY_CLASSES:
        errors.append(f"A1 quality_classes must equal {QUALITY_CLASSES}")
    if plan.get("root_issues") != [420, 505]:
        errors.append("A1 root_issues must equal [420, 505]")
    if plan.get("closure_order") != [420, 505, 501, 422, 504]:
        errors.append("A1 closure_order must equal [420, 505, 501, 422, 504]")
    stop_before = string_list(plan.get("stop_before"), "A1 stop_before", errors)
    for required in ("sdk-ts mutation", "peer DID implementation", "agent runtime implementation"):
        if required not in stop_before:
            errors.append(f"A1 stop_before is missing {required!r}")

    capabilities = plan.get("capability")
    if not isinstance(capabilities, list):
        errors.append("A1 capability must be an array of tables")
        return
    issues = [item.get("issue") for item in capabilities if isinstance(item, dict)]
    if issues != CAPABILITY_ISSUES:
        errors.append(f"A1 capability order must equal {CAPABILITY_ISSUES}")

    seen: set[int] = set()
    for position, capability in enumerate(capabilities, start=1):
        label = f"A1 capability[{position}]"
        if not isinstance(capability, dict):
            errors.append(f"{label} must be a table")
            continue
        if set(capability) != CAPABILITY_KEYS:
            errors.append(
                f"{label} fields differ: missing={sorted(CAPABILITY_KEYS - set(capability))}, "
                f"unknown={sorted(set(capability) - CAPABILITY_KEYS)}"
            )
        issue = capability.get("issue")
        if issue not in EXPECTED_CAPABILITIES:
            errors.append(f"{label} has unknown issue {issue!r}")
            continue
        expected_id, expected_blockers = EXPECTED_CAPABILITIES[issue]
        if capability.get("id") != expected_id:
            errors.append(f"issue #{issue} id must equal {expected_id!r}")
        if capability.get("state") != "planned":
            errors.append(f"issue #{issue} state must equal 'planned'")
        blockers = int_list(capability.get("blocked_by"), f"issue #{issue}.blocked_by", errors)
        if blockers != expected_blockers:
            errors.append(f"issue #{issue} blocked_by must equal {expected_blockers}")
        if capability.get("completion_needs") != expected_blockers:
            errors.append(f"issue #{issue} completion_needs must equal {expected_blockers}")
        if any(blocker not in seen for blocker in blockers):
            errors.append(f"issue #{issue} blockers must precede it in the plan")
        for field in ("planned_paths", "deliverables", "acceptance", "non_claims"):
            string_list(capability.get(field), f"issue #{issue}.{field}", errors)
        seen.add(issue)

    closeout = plan.get("closeout")
    if not isinstance(closeout, dict):
        errors.append("A1 closeout must be a table")
    else:
        if set(closeout) != {"issue", "blocked_by", "deliverables", "acceptance", "non_claims"}:
            errors.append("A1 closeout fields differ")
        if closeout.get("issue") != 504:
            errors.append("A1 closeout issue must equal 504")
        if closeout.get("blocked_by") != [420, 422, 501, 505]:
            errors.append("A1 closeout blockers must equal [420, 422, 501, 505]")
        for field in ("deliverables", "acceptance", "non_claims"):
            string_list(closeout.get(field), f"A1 closeout.{field}", errors)

    downstream = plan.get("downstream")
    if not isinstance(downstream, dict):
        errors.append("A1 downstream must be a table")
    else:
        if set(downstream) != {"issue", "milestone", "blocked_by", "outcome", "stop_reason"}:
            errors.append("A1 downstream fields differ")
        if downstream.get("issue") != 492 or downstream.get("milestone") != "A2":
            errors.append("A1 downstream must be issue #492 in A2")
        if downstream.get("blocked_by") != [420, 422, 501, 505]:
            errors.append("A1 downstream blockers must equal [420, 422, 501, 505]")
        for field in ("outcome", "stop_reason"):
            if not isinstance(downstream.get(field), str) or not downstream[field].strip():
                errors.append(f"A1 downstream.{field} must be non-empty")


def validate_main_roadmap(root: Path, errors: list[str]) -> None:
    roadmap = load(root / ROADMAP_PATH, "platform-core roadmap", errors)
    milestones = roadmap.get("milestone") if roadmap else None
    if not isinstance(milestones, list):
        errors.append("platform-core roadmap milestone must be an array")
        return
    a1 = next((item for item in milestones if isinstance(item, dict) and item.get("id") == "A1"), None)
    if a1 is None:
        errors.append("platform-core roadmap is missing A1")
        return
    if a1.get("owner_issue") != 504:
        errors.append("platform-core A1 owner_issue must equal 504")
    if a1.get("supporting_issues") != [420, 422, 501, 505]:
        errors.append("platform-core A1 supporting_issues must equal [420, 422, 501, 505]")
    if a1.get("depends_on") != ["A0"]:
        errors.append("platform-core A1 must depend only on A0")
    if "consumer-ready DID packet" not in str(a1.get("adoption_proof", "")):
        errors.append("platform-core A1 adoption proof must remain infrastructure-only")


def main() -> int:
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path.cwd().resolve()
    errors: list[str] = []
    validate_plan(root, errors)
    validate_main_roadmap(root, errors)
    if errors:
        for error in errors:
            print(f"a1-compatibility-plan: {error}", file=sys.stderr)
        print(f"a1-compatibility-plan: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    print("a1-compatibility-plan: dependency graph passed (4 capabilities, 1 downstream)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
