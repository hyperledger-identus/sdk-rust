#!/usr/bin/env python3
"""Validate SDK constraints and active-change impact records offline."""

from __future__ import annotations

import argparse
import re
import sys
import tomllib
from pathlib import Path
from typing import Any


INDEX_PATH = Path("docs/governance/sdk-constraints.toml")
SUPPORT_POLICY_PATH = Path("docs/architecture/sdk-support-policy.toml")
ALLOWED_KINDS = {"hard", "guardrail", "budget", "limitation"}
ALLOWED_CATEGORIES = {
    "architecture",
    "compatibility",
    "delivery",
    "platform",
    "product",
    "release",
    "repository",
    "security",
}
ALLOWED_STATES = {"effective", "target", "deferred", "prohibited"}
REQUIRED_STRING_FIELDS = (
    "id",
    "kind",
    "category",
    "state",
    "summary",
    "scope",
    "canonical_source",
    "authority",
    "rationale",
    "consumer_impact",
    "owner",
    "activation",
    "rollback",
)
REQUIRED_LIST_FIELDS = ("enforcement", "review_triggers")
REQUIRED_CHANGE_HEADINGS = (
    "Existing entries affected",
    "Introduced or changed constraints",
    "Introduced or changed limitations",
    "Consumer and product impact",
    "Activation and rollback",
    "Evidence",
)


def load_toml(path: Path, context: str) -> tuple[dict[str, Any] | None, list[str]]:
    try:
        with path.open("rb") as handle:
            return tomllib.load(handle), []
    except FileNotFoundError:
        return None, [f"missing {context}: {path}"]
    except tomllib.TOMLDecodeError as error:
        return None, [f"invalid TOML in {context} {path}: {error}"]


def nonempty_string(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def valid_string_list(value: Any) -> bool:
    return (
        isinstance(value, list)
        and bool(value)
        and all(nonempty_string(item) for item in value)
    )


def source_path(root: Path, source: str) -> Path | None:
    relative = source.split("#", 1)[0]
    candidate = Path(relative)
    if candidate.is_absolute() or ".." in candidate.parts:
        return None
    return root / candidate


def nested_value(document: dict[str, Any], dotted_key: str) -> Any:
    value: Any = document
    for part in dotted_key.split("."):
        if not isinstance(value, dict) or part not in value:
            return None
        value = value[part]
    return value


def validate_index(root: Path) -> list[str]:
    document, failures = load_toml(root / INDEX_PATH, "constraint index")
    if document is None:
        return failures

    if document.get("schema_version") != 1:
        failures.append("constraint index schema_version must be 1")
    if not nonempty_string(document.get("policy_revision")):
        failures.append("constraint index policy_revision must be non-empty")

    governing_adr = document.get("governing_adr")
    if not nonempty_string(governing_adr):
        failures.append("constraint index governing_adr must be non-empty")
    else:
        governing_path = source_path(root, governing_adr)
        if governing_path is None or not governing_path.is_file():
            failures.append(
                f"constraint index governing_adr does not exist: {governing_adr}"
            )

    entries = document.get("entries")
    if not isinstance(entries, list) or not entries:
        failures.append("constraint index must contain at least one [[entries]] record")
        return failures

    seen: set[str] = set()
    by_id: dict[str, dict[str, Any]] = {}
    for position, entry in enumerate(entries, start=1):
        context = f"constraint entry {position}"
        if not isinstance(entry, dict):
            failures.append(f"{context} must be a TOML table")
            continue

        entry_id = entry.get("id")
        if nonempty_string(entry_id):
            context = f"constraint {entry_id}"
            if re.fullmatch(r"SDK-[A-Z]+-[0-9]{3}", entry_id) is None:
                failures.append(f"{context} has an invalid stable identifier")
            if entry_id in seen:
                failures.append(f"duplicate constraint identifier: {entry_id}")
            seen.add(entry_id)
            by_id[entry_id] = entry

        for field_name in REQUIRED_STRING_FIELDS:
            if not nonempty_string(entry.get(field_name)):
                failures.append(f"{context} missing non-empty {field_name}")
        for field_name in REQUIRED_LIST_FIELDS:
            if not valid_string_list(entry.get(field_name)):
                failures.append(f"{context} missing non-empty {field_name} list")

        kind = entry.get("kind")
        if kind not in ALLOWED_KINDS:
            failures.append(f"{context} kind must be one of {sorted(ALLOWED_KINDS)}")
        category = entry.get("category")
        if category not in ALLOWED_CATEGORIES:
            failures.append(
                f"{context} category must be one of {sorted(ALLOWED_CATEGORIES)}"
            )
        state = entry.get("state")
        if state not in ALLOWED_STATES:
            failures.append(f"{context} state must be one of {sorted(ALLOWED_STATES)}")

        canonical_source = entry.get("canonical_source")
        if nonempty_string(canonical_source):
            canonical_path = source_path(root, canonical_source)
            if canonical_path is None:
                failures.append(
                    f"{context} canonical_source must be repository-relative"
                )
            elif not canonical_path.is_file():
                failures.append(
                    f"{context} canonical_source does not exist: {canonical_source}"
                )

        activation = entry.get("activation")
        if state in {"target", "deferred"} and nonempty_string(activation):
            if "already effective" in activation.lower():
                failures.append(
                    f"{context} {state} activation claims it is already effective"
                )
            if not re.search(r"(?i)(issue|decision|adr|focused)", activation):
                failures.append(
                    f"{context} {state} activation must name a decision path"
                )

        value_source = entry.get("value_source")
        if value_source is not None and not nonempty_string(entry.get("value")):
            failures.append(f"{context} value_source requires a non-empty value")

    effective_msrv = by_id.get("SDK-COMPAT-002")
    target_msrv = by_id.get("SDK-COMPAT-003")
    if effective_msrv is None:
        failures.append("constraint index missing SDK-COMPAT-002 effective MSRV")
    elif effective_msrv.get("state") != "effective":
        failures.append("SDK-COMPAT-002 must remain the effective MSRV entry")
    if target_msrv is None:
        failures.append("constraint index missing SDK-COMPAT-003 target MSRV")
    elif target_msrv.get("state") != "target":
        failures.append(
            "SDK-COMPAT-003 must remain a target until separately activated"
        )

    support_policy, support_failures = load_toml(
        root / SUPPORT_POLICY_PATH, "SDK support policy"
    )
    failures.extend(support_failures)
    if effective_msrv is not None and support_policy is not None:
        value_source = effective_msrv.get("value_source")
        if not nonempty_string(value_source):
            failures.append("SDK-COMPAT-002 must declare value_source")
        else:
            canonical_value = nested_value(support_policy, value_source)
            if effective_msrv.get("value") != canonical_value:
                failures.append(
                    "SDK-COMPAT-002 value must equal support-policy "
                    f"{value_source}: {canonical_value!r}"
                )
    if effective_msrv is not None and target_msrv is not None:
        if effective_msrv.get("value") == target_msrv.get("value"):
            failures.append("effective and target MSRV values must remain distinct")

    return failures


def field(text: str, name: str) -> str | None:
    match = re.search(rf"(?m)^{re.escape(name)}:\s*(\S.*)$", text)
    return match.group(1).strip() if match else None


def section_body(text: str, heading: str) -> str | None:
    match = re.search(rf"(?ms)^## {re.escape(heading)}\s*$\n(.*?)(?=^## |\Z)", text)
    return match.group(1).strip() if match else None


def effective_constraint_ids(root: Path) -> set[str]:
    document, _ = load_toml(root / INDEX_PATH, "constraint index")
    if document is None or not isinstance(document.get("entries"), list):
        return set()
    return {
        entry["id"]
        for entry in document["entries"]
        if isinstance(entry, dict)
        and entry.get("state") == "effective"
        and nonempty_string(entry.get("id"))
    }


def valid_decision_reference(root: Path, reference: str | None) -> bool:
    if reference is None:
        return False
    if re.fullmatch(
        r"https://github\.com/[^/]+/[^/]+/(issues|discussions)/[0-9]+",
        reference,
    ):
        return True
    return reference in effective_constraint_ids(root)


def validate_change_record(root: Path, path: Path, require_ready: bool) -> list[str]:
    context = f"active change {path.parent.name}"
    if not path.is_file():
        return [f"{context} is missing constraints.md"]
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return [f"{context} constraints.md is not valid UTF-8"]

    failures: list[str] = []
    impact = field(text, "Impact class")
    decision = field(text, "Decision status")
    reference = field(text, "Decision reference")
    blockers = field(text, "Constraint blockers")

    if impact not in {"none", "routine", "material"}:
        failures.append(f"{context} Impact class must be none, routine or material")
    if decision not in {"not-required", "proposed", "directed"}:
        failures.append(
            f"{context} Decision status must be not-required, proposed or directed"
        )
    if not reference:
        failures.append(f"{context} must declare Decision reference")
    if not blockers:
        failures.append(f"{context} must declare Constraint blockers")

    for heading in REQUIRED_CHANGE_HEADINGS:
        body = section_body(text, heading)
        if body is None:
            failures.append(f"{context} is missing heading: {heading}")
        elif body.strip().lower().rstrip(".") in {"", "...", "n/a"}:
            failures.append(f"{context} has an empty section: {heading}")

    if impact in {"none", "routine"} and decision == "proposed":
        failures.append(f"{context} non-material impact cannot have proposed status")
    if decision == "not-required" and reference not in {None, "not-required"}:
        failures.append(
            f"{context} not-required decision must use reference not-required"
        )
    if decision in {"proposed", "directed"} and reference == "not-required":
        failures.append(f"{context} {decision} decision requires a durable reference")

    if require_ready:
        if blockers != "none":
            failures.append(
                f"{context} constraints cannot be ready with blockers {blockers!r}"
            )
        if impact == "material" and decision != "directed":
            failures.append(
                f"{context} material impact requires directed decision status"
            )
        if decision == "directed" and not valid_decision_reference(root, reference):
            failures.append(
                f"{context} directed decision requires an exact GitHub issue, "
                "discussion URL or effective constraint ID"
            )

    return failures


def active_changes(root: Path, selected: str | None) -> list[Path]:
    changes_root = root / "openspec" / "changes"
    if selected:
        return [changes_root / selected]
    if not changes_root.is_dir():
        return []
    return sorted(
        path
        for path in changes_root.iterdir()
        if path.is_dir() and path.name != "archive"
    )


def validate(root: Path, selected: str | None, require_ready: bool) -> list[str]:
    failures = validate_index(root)
    changes = active_changes(root, selected)
    if selected and not changes[0].is_dir():
        failures.append(f"active change not found: {selected}")
        return failures
    for change in changes:
        failures.extend(
            validate_change_record(root, change / "constraints.md", require_ready)
        )
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    parser.add_argument("--change")
    parser.add_argument("--require-ready", action="store_true")
    args = parser.parse_args()

    root = Path(args.root).resolve()
    failures = validate(root, args.change, args.require_ready)
    if failures:
        for failure in failures:
            print(f"constraint-governance: {failure}", file=sys.stderr)
        print(f"constraint-governance: {len(failures)} failure(s)", file=sys.stderr)
        return 1

    changes = active_changes(root, args.change)
    qualifier = "ready" if args.require_ready else "structure"
    print(f"constraint-governance: {qualifier} passed for {len(changes)} change(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
