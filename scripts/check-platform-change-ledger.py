#!/usr/bin/env python3
"""Validate and deterministically render the consumer change ledger."""

from __future__ import annotations

import argparse
from datetime import date
import json
from pathlib import Path
import re
import stat
import sys
import tomllib
from typing import Any


LEDGER = Path("docs/architecture/identus-platform-change-ledger.toml")
REPORT = Path("docs/architecture/identus-platform-change-ledger.md")
VECTORS = Path("docs/conformance/cross-language-vector-catalog.toml")
MAPPINGS = Path("docs/architecture/language-adapter-mappings.toml")
QUALITY = Path("docs/architecture/quality-evidence-declarations.toml")
IMPACT = "compatibility-impact.toml"

TOP_KEYS = {
    "schema_version",
    "registry_version",
    "program_issue",
    "owner_issue",
    "status",
    "impact_policy_activation_issue",
    "impact_policy_activation_date",
    "impact_policy_activation_revision",
    "grandfathered_changes",
    "allowed_change_classes",
    "allowed_release_note_classes",
    "allowed_compatibility",
    "allowed_visibility",
    "allowed_lifecycle_states",
    "allowed_deprecation_phases",
    "allowed_legacy_bug_policies",
    "allowed_mapping_dispositions",
    "changes",
}
CHANGE_KEYS = {
    "id",
    "capability",
    "lifecycle_state",
    "issue",
    "pull_request",
    "change_class",
    "release_note_class",
    "visibility",
    "affected_packages",
    "affected_consumers",
    "source_version",
    "target_version",
    "compatibility",
    "old_behavior",
    "new_behavior",
    "migration_action",
    "replacement",
    "no_replacement_rationale",
    "first_compatible_version",
    "last_compatible_version",
    "compatibility_window",
    "deprecation_phase",
    "legacy_bug_policy",
    "mapping_disposition",
    "mapping_ids",
    "mapping_rationale",
    "vector_ids",
    "quality_declaration_ids",
    "observability",
    "fallback",
    "rollback",
    "rollback_restores_unsafe_behavior",
    "removal_gate",
    "evidence_selectors",
    "evidence_artifacts",
    "limitations",
    "active",
    "supersedes",
    "replaced_by",
}
IMPACT_KEYS = {
    "schema_version",
    "issue",
    "disposition",
    "scope",
    "rationale",
    "ledger_ids",
}

EXPECTED_VOCABULARIES = {
    "allowed_change_classes": [
        "additive",
        "fix",
        "behavioral",
        "deprecated",
        "breaking",
        "security",
    ],
    "allowed_release_note_classes": [
        "added",
        "changed",
        "deprecated",
        "removed",
        "fixed",
        "security",
    ],
    "allowed_compatibility": [
        "compatible",
        "source-break",
        "abi-break",
        "wire-break",
        "persistence-break",
        "behavior-break",
        "runtime-requirement-break",
    ],
    "allowed_visibility": [
        "public-api",
        "wire",
        "error",
        "persistence",
        "abi",
        "target-support",
        "runtime-requirement",
        "behavior",
        "security",
    ],
    "allowed_lifecycle_states": ["proposed", "active", "superseded", "removed"],
    "allowed_deprecation_phases": [
        "not-applicable",
        "proposed",
        "announced",
        "available-replacement",
        "default-off",
        "removed",
    ],
    "allowed_legacy_bug_policies": [
        "not-applicable",
        "fix-and-document",
        "preserve-profile",
        "simulate-temporarily",
        "reject-as-unsafe",
    ],
    "allowed_mapping_dispositions": ["required", "not-applicable"],
}

ID_RE = re.compile(r"^[a-z][a-z0-9]*(?:[.-][a-z0-9]+)*$")
SEMVER_RE = re.compile(r"^(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)(?:-[0-9A-Za-z.-]+)?$")
SHA_RE = re.compile(r"^[0-9a-f]{40}$")
SAFE_SCOPES = {"governance", "documentation", "test-only", "internal-refactor", "tooling"}
ALLOWED_GRANDFATHERED_CHANGES = {"activate-first-crate-release-train"}


def fail(errors: list[str], context: str, message: str) -> None:
    errors.append(f"{context}: {message}")


def exact_keys(value: Any, expected: set[str], context: str, errors: list[str]) -> bool:
    if not isinstance(value, dict):
        fail(errors, context, "must be a table")
        return False
    missing = sorted(expected - set(value))
    unknown = sorted(set(value) - expected)
    if missing:
        fail(errors, context, f"missing keys: {', '.join(missing)}")
    if unknown:
        fail(errors, context, f"unknown keys: {', '.join(unknown)}")
    return not missing and not unknown


def text(
    value: Any,
    context: str,
    errors: list[str],
    *,
    minimum: int = 1,
    maximum: int = 1024,
) -> str:
    if not isinstance(value, str):
        fail(errors, context, "must be a string")
        return ""
    if not minimum <= len(value.strip()) or len(value) > maximum:
        fail(errors, context, f"must contain {minimum}..{maximum} characters")
    if any(ord(character) < 32 for character in value):
        fail(errors, context, "must not contain control characters or newlines")
    return value


def integer(value: Any, context: str, errors: list[str], *, zero: bool = False) -> int:
    minimum = 0 if zero else 1
    if isinstance(value, bool) or not isinstance(value, int) or value < minimum:
        fail(errors, context, f"must be an integer >= {minimum}")
        return 0
    return value


def strings(
    value: Any,
    context: str,
    errors: list[str],
    *,
    empty: bool = False,
    maximum: int = 64,
    ordered: bool = True,
) -> list[str]:
    if not isinstance(value, list):
        fail(errors, context, "must be an array")
        return []
    if not empty and not value:
        fail(errors, context, "must not be empty")
    if len(value) > maximum:
        fail(errors, context, f"must contain at most {maximum} entries")
    result = [text(item, f"{context}[{index}]", errors, maximum=512) for index, item in enumerate(value)]
    if len(set(result)) != len(result):
        fail(errors, context, "must not contain duplicates")
    if ordered and result != sorted(result):
        fail(errors, context, "must be sorted")
    return result


def regular_file(root: Path, relative: Path, context: str, errors: list[str]) -> Path | None:
    if relative.is_absolute() or ".." in relative.parts or relative.as_posix() in {"", "."}:
        fail(errors, context, "must be a safe repository-relative path")
        return None
    candidate = root / relative
    try:
        metadata = candidate.lstat()
    except FileNotFoundError:
        fail(errors, context, f"file does not exist: {relative.as_posix()}")
        return None
    if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISREG(metadata.st_mode):
        fail(errors, context, "must be a regular non-symlink file")
        return None
    if metadata.st_size > 1_000_000:
        fail(errors, context, "file exceeds 1000000 bytes")
        return None
    try:
        candidate.resolve().relative_to(root.resolve())
    except ValueError:
        fail(errors, context, "file escapes repository root")
        return None
    return candidate


def load_toml(root: Path, relative: Path, context: str, errors: list[str]) -> dict[str, Any]:
    path = regular_file(root, relative, context, errors)
    if path is None:
        return {}
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, tomllib.TOMLDecodeError) as error:
        fail(errors, context, f"cannot parse TOML: {error}")
        return {}


def validate_selector(
    root: Path,
    selector: str,
    cited_ids: set[str],
    context: str,
    errors: list[str],
) -> None:
    if "#" not in selector:
        fail(errors, context, "must use path#needle syntax")
        return
    raw_path, needle = selector.rsplit("#", 1)
    if not ID_RE.fullmatch(needle) or len(needle) > 160:
        fail(errors, context, "must contain a bounded stable-ID selector needle")
        return
    if needle not in cited_ids:
        fail(errors, context, "selector needle must be one of the record's cited IDs")
        return
    path = regular_file(root, Path(raw_path), context, errors)
    if path is None:
        return
    try:
        content = path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        fail(errors, context, f"cannot read selector source: {error}")
        return
    if needle not in content:
        fail(errors, context, f"selector needle not found in {raw_path}: {needle}")


def active_index(records: Any, kind: str, errors: list[str]) -> dict[str, dict[str, Any]]:
    if not isinstance(records, list):
        fail(errors, kind, "must be an array")
        return {}
    result: dict[str, dict[str, Any]] = {}
    for index, record in enumerate(records):
        context = f"{kind}[{index}]"
        if not isinstance(record, dict):
            fail(errors, context, "must be a table")
            continue
        identifier = record.get("id")
        if not isinstance(identifier, str):
            fail(errors, f"{context}.id", "must be a string")
            continue
        if identifier in result:
            fail(errors, f"{context}.id", f"duplicate ID: {identifier}")
        result[identifier] = record
    return result


def validate_change(
    root: Path,
    change: Any,
    index: int,
    vocabularies: dict[str, list[str]],
    vectors: dict[str, dict[str, Any]],
    mappings: dict[str, dict[str, Any]],
    qualities: dict[str, dict[str, Any]],
    merge_ready: bool,
    errors: list[str],
) -> dict[str, Any] | None:
    context = f"changes[{index}]"
    initial_error_count = len(errors)
    if not exact_keys(change, CHANGE_KEYS, context, errors):
        return None
    identifier = text(change["id"], f"{context}.id", errors, maximum=120)
    capability = text(change["capability"], f"{context}.capability", errors, maximum=120)
    if not ID_RE.fullmatch(identifier):
        fail(errors, f"{context}.id", "must be a stable lowercase dotted ID")
    elif not identifier.startswith(f"{capability}."):
        fail(errors, f"{context}.id", "must be prefixed by the capability ID")
    if not ID_RE.fullmatch(capability):
        fail(errors, f"{context}.capability", "must be a lowercase dotted capability")
    lifecycle = text(change["lifecycle_state"], f"{context}.lifecycle_state", errors, maximum=32)
    issue = integer(change["issue"], f"{context}.issue", errors)
    pull_request = integer(change["pull_request"], f"{context}.pull_request", errors, zero=True)
    change_class = text(change["change_class"], f"{context}.change_class", errors, maximum=32)
    note_class = text(change["release_note_class"], f"{context}.release_note_class", errors, maximum=32)
    compatibility = text(change["compatibility"], f"{context}.compatibility", errors, maximum=48)
    visibility = strings(change["visibility"], f"{context}.visibility", errors)
    packages = strings(change["affected_packages"], f"{context}.affected_packages", errors)
    consumers = strings(change["affected_consumers"], f"{context}.affected_consumers", errors)
    vector_ids = strings(change["vector_ids"], f"{context}.vector_ids", errors)
    quality_ids = strings(
        change["quality_declaration_ids"], f"{context}.quality_declaration_ids", errors
    )
    mapping_ids = strings(change["mapping_ids"], f"{context}.mapping_ids", errors, empty=True)
    supersedes = strings(change["supersedes"], f"{context}.supersedes", errors, empty=True)
    evidence_selectors = strings(
        change["evidence_selectors"], f"{context}.evidence_selectors", errors
    )
    evidence_artifacts = strings(
        change["evidence_artifacts"], f"{context}.evidence_artifacts", errors, empty=True
    )
    for field in (
        "old_behavior",
        "new_behavior",
        "observability",
        "fallback",
        "rollback",
        "limitations",
    ):
        text(change[field], f"{context}.{field}", errors, minimum=24, maximum=1200)
    for field in (
        "migration_action",
        "replacement",
        "no_replacement_rationale",
        "compatibility_window",
        "removal_gate",
        "mapping_rationale",
        "replaced_by",
    ):
        text(change[field], f"{context}.{field}", errors, minimum=0, maximum=1200)
    for field in (
        "source_version",
        "target_version",
        "first_compatible_version",
        "last_compatible_version",
    ):
        version = text(change[field], f"{context}.{field}", errors, maximum=64)
        if version != "unreleased" and not SEMVER_RE.fullmatch(version):
            fail(errors, f"{context}.{field}", "must be SemVer or unreleased")
    if lifecycle not in vocabularies["allowed_lifecycle_states"]:
        fail(errors, f"{context}.lifecycle_state", "is outside the closed vocabulary")
    if change_class not in vocabularies["allowed_change_classes"]:
        fail(errors, f"{context}.change_class", "is outside the closed vocabulary")
    if note_class not in vocabularies["allowed_release_note_classes"]:
        fail(errors, f"{context}.release_note_class", "is outside the closed vocabulary")
    if compatibility not in vocabularies["allowed_compatibility"]:
        fail(errors, f"{context}.compatibility", "is outside the closed vocabulary")
    if any(item not in vocabularies["allowed_visibility"] for item in visibility):
        fail(errors, f"{context}.visibility", "contains a value outside the closed vocabulary")
    active = change["active"]
    if not isinstance(active, bool):
        fail(errors, f"{context}.active", "must be a boolean")
    elif active != (lifecycle in {"proposed", "active"}):
        fail(errors, f"{context}.active", "must agree with lifecycle state")
    if lifecycle == "proposed" and pull_request < 0:
        fail(errors, f"{context}.pull_request", "must be zero or a positive PR number")
    if lifecycle != "proposed" and pull_request == 0:
        fail(errors, f"{context}.pull_request", "must be positive outside proposed state")
    if merge_ready and (lifecycle == "proposed" or pull_request == 0):
        fail(errors, context, "merge-ready records require active lifecycle and actual PR number")
    expected_note_classes = {
        "additive": {"added"},
        "fix": {"fixed"},
        "behavioral": {"changed"},
        "deprecated": {"deprecated"},
        "breaking": {"changed", "removed"},
        "security": {"security"},
    }
    if note_class not in expected_note_classes.get(change_class, set()):
        fail(errors, context, "change and release-note classes are inconsistent")
    if change_class in {"additive", "fix"} and compatibility != "compatible":
        fail(errors, context, "additive and fix records must be compatible")
    if change_class == "breaking" and compatibility == "compatible":
        fail(errors, context, "breaking records must name a breaking compatibility dimension")
    if compatibility != "compatible":
        required = (
            "migration_action",
            "compatibility_window",
            "observability",
            "fallback",
            "rollback",
        )
        if any(not str(change[field]).strip() for field in required):
            fail(errors, context, "incompatible records require migration, window, observability, fallback, and rollback")
        if not str(change["replacement"]).strip() and len(str(change["no_replacement_rationale"]).strip()) < 48:
            fail(errors, context, "incompatible records require a replacement or substantive no-replacement rationale")
        required_visibility = {
            "source-break": "public-api",
            "abi-break": "abi",
            "wire-break": "wire",
            "persistence-break": "persistence",
            "behavior-break": "behavior",
            "runtime-requirement-break": "runtime-requirement",
        }.get(compatibility)
        if required_visibility and required_visibility not in visibility:
            fail(errors, context, f"{compatibility} requires {required_visibility} visibility")
    deprecation = text(
        change["deprecation_phase"], f"{context}.deprecation_phase", errors, maximum=32
    )
    legacy = text(change["legacy_bug_policy"], f"{context}.legacy_bug_policy", errors, maximum=32)
    if deprecation not in vocabularies["allowed_deprecation_phases"]:
        fail(errors, f"{context}.deprecation_phase", "is outside the closed vocabulary")
    if legacy not in vocabularies["allowed_legacy_bug_policies"]:
        fail(errors, f"{context}.legacy_bug_policy", "is outside the closed vocabulary")
    if change_class == "deprecated" and deprecation == "not-applicable":
        fail(errors, context, "deprecated records require a deprecation phase")
    if deprecation in {"default-off", "removed"}:
        required = ("replacement", "migration_action", "compatibility_window", "removal_gate")
        if any(not str(change[field]).strip() for field in required):
            fail(errors, context, "default-off and removed phases require replacement, migration, window, and removal gate")
    if deprecation == "removed" and lifecycle != "removed":
        fail(errors, context, "removed deprecation phase requires removed lifecycle state")
    rollback_unsafe = change["rollback_restores_unsafe_behavior"]
    if not isinstance(rollback_unsafe, bool):
        fail(errors, f"{context}.rollback_restores_unsafe_behavior", "must be a boolean")
    if change_class == "security":
        if legacy != "reject-as-unsafe":
            fail(errors, context, "security records must reject unsafe legacy behavior")
        if rollback_unsafe is not False:
            fail(errors, context, "security rollback must not restore unsafe behavior")
        if "security" not in visibility:
            fail(errors, context, "security records must include security visibility")
    if legacy == "simulate-temporarily" and (
        not str(change["migration_action"]).strip()
        or not str(change["compatibility_window"]).strip()
        or not str(change["removal_gate"]).strip()
        or rollback_unsafe is not False
    ):
        fail(errors, context, "temporary legacy simulation must be bounded and safely removable")
    if lifecycle == "removed" and (
        deprecation != "removed"
        or not str(change["replacement"]).strip()
        or not str(change["removal_gate"]).strip()
    ):
        fail(errors, context, "removed records require removed phase, replacement, and removal gate")
    mapping_disposition = text(
        change["mapping_disposition"], f"{context}.mapping_disposition", errors, maximum=32
    )
    if mapping_disposition not in vocabularies["allowed_mapping_dispositions"]:
        fail(errors, f"{context}.mapping_disposition", "is outside the closed vocabulary")
    if mapping_disposition == "required" and not mapping_ids:
        fail(errors, context, "required mapping disposition needs mapping IDs")
    if mapping_disposition == "not-applicable":
        if mapping_ids:
            fail(errors, context, "not-applicable mapping disposition requires no mapping IDs")
        if len(str(change["mapping_rationale"]).strip()) < 48:
            fail(errors, context, "not-applicable mapping disposition needs substantive rationale")
    cited_ids = set(vector_ids) | set(mapping_ids) | set(quality_ids)
    for selector_index, selector in enumerate(evidence_selectors):
        validate_selector(
            root,
            selector,
            cited_ids,
            f"{context}.evidence_selectors[{selector_index}]",
            errors,
        )
    for artifact_index, artifact in enumerate(evidence_artifacts):
        regular_file(root, Path(artifact), f"{context}.evidence_artifacts[{artifact_index}]", errors)
    for vector_id in vector_ids:
        vector = vectors.get(vector_id)
        if vector is None or vector.get("active") is not True:
            fail(errors, context, f"vector ID is missing or inactive: {vector_id}")
        elif vector.get("capability") != capability:
            fail(errors, context, f"vector capability mismatch: {vector_id}")
    for quality_id in quality_ids:
        quality = qualities.get(quality_id)
        if quality is None or quality.get("active") is not True:
            fail(errors, context, f"quality declaration is missing or inactive: {quality_id}")
            continue
        if quality.get("capability") != capability:
            fail(errors, context, f"quality capability mismatch: {quality_id}")
    covered_vectors = {
        vector_id
        for quality_id in quality_ids
        for vector_id in qualities.get(quality_id, {}).get("vector_ids", [])
    }
    if not set(vector_ids) <= covered_vectors:
        fail(errors, context, "every cited vector must be covered by a cited quality declaration")
    for mapping_id in mapping_ids:
        mapping = mappings.get(mapping_id)
        if mapping is None or mapping.get("state") != "active":
            fail(errors, context, f"mapping ID is missing or inactive: {mapping_id}")
            continue
        if mapping.get("capability") != capability:
            fail(errors, context, f"mapping capability mismatch: {mapping_id}")
        if not set(mapping.get("vector_ids", [])) & set(vector_ids):
            fail(errors, context, f"mapping shares no cited vector: {mapping_id}")
    _ = packages, consumers, issue, supersedes
    return change if len(errors) == initial_error_count else None


def validate_lifecycle(changes: list[dict[str, Any]], errors: list[str]) -> None:
    by_id: dict[str, dict[str, Any]] = {}
    for change in changes:
        identifier = change["id"]
        if identifier in by_id:
            fail(errors, "changes", f"duplicate ID: {identifier}")
        by_id[identifier] = change
    for identifier, change in by_id.items():
        for previous in change["supersedes"]:
            if previous == identifier:
                fail(errors, identifier, "record cannot supersede itself")
                continue
            if previous not in by_id:
                fail(errors, identifier, f"supersedes missing record: {previous}")
            elif by_id[previous]["replaced_by"] != identifier:
                fail(errors, identifier, f"supersession is not reciprocal: {previous}")
        replacement = change["replaced_by"]
        if change["lifecycle_state"] == "superseded" and not replacement:
            fail(errors, identifier, "superseded record requires replaced_by")
        if replacement:
            if replacement == identifier:
                fail(errors, identifier, "record cannot replace itself")
                continue
            if replacement not in by_id:
                fail(errors, identifier, f"replacement is missing: {replacement}")
            elif identifier not in by_id[replacement]["supersedes"]:
                fail(errors, identifier, f"replacement link is not reciprocal: {replacement}")
            elif by_id[replacement]["lifecycle_state"] not in {"proposed", "active"}:
                fail(errors, identifier, f"replacement must be proposed or active: {replacement}")
            if change["lifecycle_state"] not in {"superseded", "removed"}:
                fail(errors, identifier, "only superseded or removed records may have replaced_by")
    for start in by_id:
        seen: set[str] = set()
        current = start
        while current in by_id and by_id[current]["replaced_by"]:
            if current in seen:
                fail(errors, start, "replacement lifecycle contains a cycle")
                break
            seen.add(current)
            current = by_id[current]["replaced_by"]


def validate_impacts(
    root: Path,
    activation_issue: int,
    activation_date: date,
    grandfathered_changes: set[str],
    ledger_issues: dict[str, int],
    errors: list[str],
) -> None:
    changes_root = root / "openspec/changes"
    if not changes_root.is_dir() or changes_root.is_symlink():
        fail(errors, "openspec/changes", "must be a regular directory")
        return
    seen_grandfathered: set[str] = set()
    governed_change_roots: list[Path] = []
    for entry in sorted(changes_root.iterdir()):
        if entry.name == "archive":
            if entry.is_symlink() or not entry.is_dir():
                fail(errors, "openspec/changes/archive", "must be a regular directory")
            continue
        if entry.is_symlink() or not entry.is_dir():
            fail(errors, f"openspec/changes/{entry.name}", "must be a regular non-symlink directory")
            continue
        governed_change_roots.append(entry)
    archive_root = changes_root / "archive"
    if archive_root.is_dir() and not archive_root.is_symlink():
        for entry in sorted(archive_root.iterdir()):
            if entry.is_symlink() or not entry.is_dir():
                fail(
                    errors,
                    f"openspec/changes/archive/{entry.name}",
                    "must be a regular non-symlink directory",
                )
                continue
            try:
                archive_date = date.fromisoformat(entry.name[:10])
            except ValueError:
                fail(errors, entry.name, "archive name must begin with an ISO calendar date")
                continue
            archived_impact = entry / IMPACT
            if archive_date >= activation_date or archived_impact.exists() or archived_impact.is_symlink():
                governed_change_roots.append(entry)
    declared_issues: set[int] = set()
    for change_root in governed_change_roots:
        receipt_path = change_root / "preimplementation.json"
        grandfathered = change_root.name in grandfathered_changes
        if grandfathered:
            seen_grandfathered.add(change_root.name)
            receipt_file = regular_file(
                root,
                receipt_path.relative_to(root),
                f"{change_root.name}.preimplementation.json",
                errors,
            )
            if receipt_file is None:
                fail(errors, change_root.name, "grandfathered change requires a durable receipt")
            else:
                try:
                    receipt = json.loads(receipt_file.read_text(encoding="utf-8"))
                except (OSError, UnicodeError, json.JSONDecodeError) as error:
                    fail(errors, change_root.name, f"cannot parse preimplementation receipt: {error}")
                    receipt = {}
                if not isinstance(receipt, dict) or receipt.get("issue") == activation_issue:
                    fail(errors, change_root.name, "activation change cannot be grandfathered")
            continue
        issue: Any = None
        if receipt_path.exists() or receipt_path.is_symlink():
            receipt_file = regular_file(
                root,
                receipt_path.relative_to(root),
                f"{change_root.name}.preimplementation.json",
                errors,
            )
            if receipt_file is not None:
                try:
                    receipt = json.loads(receipt_file.read_text(encoding="utf-8"))
                except (OSError, UnicodeError, json.JSONDecodeError) as error:
                    fail(errors, change_root.name, f"cannot parse preimplementation receipt: {error}")
                    receipt = {}
                issue = receipt.get("issue") if isinstance(receipt, dict) else None
        impact_path = change_root / IMPACT
        relative = impact_path.relative_to(root)
        impact = load_toml(root, relative, f"{change_root.name}.{IMPACT}", errors)
        if not exact_keys(impact, IMPACT_KEYS, f"{change_root.name}.{IMPACT}", errors):
            continue
        if impact["schema_version"] != 1:
            fail(errors, f"{change_root.name}.{IMPACT}.schema_version", "must be 1")
        impact_issue = integer(impact["issue"], f"{change_root.name}.{IMPACT}.issue", errors)
        declared_issues.add(impact_issue)
        if issue is not None and impact_issue != issue:
            fail(errors, f"{change_root.name}.{IMPACT}.issue", "must match preimplementation issue")
        disposition = text(
            impact["disposition"], f"{change_root.name}.{IMPACT}.disposition", errors, maximum=32
        )
        scope = text(impact["scope"], f"{change_root.name}.{IMPACT}.scope", errors, maximum=32)
        rationale = text(
            impact["rationale"], f"{change_root.name}.{IMPACT}.rationale", errors, minimum=48, maximum=1200
        )
        referenced = strings(
            impact["ledger_ids"], f"{change_root.name}.{IMPACT}.ledger_ids", errors, empty=True
        )
        owned = {identifier for identifier, owner in ledger_issues.items() if owner == impact_issue}
        if disposition == "behavior-neutral":
            if scope not in SAFE_SCOPES:
                fail(errors, f"{change_root.name}.{IMPACT}.scope", "is not a behavior-neutral scope")
            if referenced:
                fail(errors, f"{change_root.name}.{IMPACT}.ledger_ids", "must be empty for behavior-neutral work")
            if owned:
                fail(
                    errors,
                    f"{change_root.name}.{IMPACT}.disposition",
                    "contradicts canonical ledger records owned by this issue",
                )
        elif disposition == "consumer-visible":
            if scope != "consumer-contract":
                fail(errors, f"{change_root.name}.{IMPACT}.scope", "must be consumer-contract")
            if not referenced:
                fail(errors, f"{change_root.name}.{IMPACT}.ledger_ids", "must not be empty for consumer-visible work")
            for identifier in referenced:
                if identifier not in ledger_issues:
                    fail(errors, f"{change_root.name}.{IMPACT}.ledger_ids", f"unknown ledger ID: {identifier}")
                elif ledger_issues[identifier] != impact_issue:
                    fail(
                        errors,
                        f"{change_root.name}.{IMPACT}.ledger_ids",
                        f"ledger ID is owned by issue #{ledger_issues[identifier]}: {identifier}",
                    )
            if set(referenced) != owned:
                fail(
                    errors,
                    f"{change_root.name}.{IMPACT}.ledger_ids",
                    "must exactly match canonical ledger records owned by this issue",
                )
        else:
            fail(errors, f"{change_root.name}.{IMPACT}.disposition", "must be behavior-neutral or consumer-visible")
        _ = rationale
    for missing in sorted(grandfathered_changes - seen_grandfathered):
        fail(errors, "ledger.grandfathered_changes", f"active change does not exist: {missing}")
    for missing_issue in sorted(set(ledger_issues.values()) - declared_issues):
        fail(errors, "ledger.changes", f"issue #{missing_issue} has no compatibility-impact declaration")


def render(ledger: dict[str, Any], changes: list[dict[str, Any]]) -> str:
    lines = [
        "# Identus platform change ledger",
        "",
        "This document is generated from",
        "`docs/architecture/identus-platform-change-ledger.toml`. Do not edit it by",
        "hand.",
        "",
        f"Registry version: `{ledger['registry_version']}`<br>",
        f"Status: `{ledger['status']}`",
        "",
    ]
    sections = (
        (
            "Release notes",
            lambda item: (
                f"State: `{item['lifecycle_state']}`. Class: `{item['change_class']}` / "
                f"`{item['release_note_class']}`. Target: `{item['target_version']}`. "
                f"{item['new_behavior']}"
            ),
        ),
        (
            "Migration actions",
            lambda item: (
                f"`{item['source_version']}` → `{item['target_version']}` for "
                f"{', '.join(item['affected_consumers'])}. "
                f"{item['migration_action'] or 'No migration action required.'}"
            ),
        ),
        (
            "Compatibility windows",
            lambda item: (
                f"Compatibility: `{item['compatibility']}`. First compatible: "
                f"`{item['first_compatible_version']}`. Last compatible: "
                f"`{item['last_compatible_version']}`. "
                f"{item['compatibility_window'] or 'No compatibility window required.'}"
            ),
        ),
        (
            "Rollback and removal",
            lambda item: (
                f"Fallback: {item['fallback']} Rollback: {item['rollback']} "
                f"Removal gate: {item['removal_gate'] or 'Not applicable.'}"
            ),
        ),
        (
            "Limitations",
            lambda item: (
                f"Visibility: {', '.join(item['visibility'])}. {item['limitations']}"
            ),
        ),
    )
    for heading, formatter in sections:
        lines.extend([f"## {heading}", ""])
        if not changes:
            lines.extend(["No consumer-visible changes are recorded.", ""])
            continue
        for change in changes:
            lines.extend([f"### `{change['id']}`", "", formatter(change), ""])
    return "\n".join(lines).rstrip() + "\n"


def validate(
    root: Path, *, check_impacts: bool, merge_ready: bool
) -> tuple[dict[str, Any], list[dict[str, Any]], list[str]]:
    errors: list[str] = []
    ledger = load_toml(root, LEDGER, "ledger", errors)
    if not exact_keys(ledger, TOP_KEYS, "ledger", errors):
        return ledger, [], errors
    if ledger["schema_version"] != 2:
        fail(errors, "ledger.schema_version", "must be 2")
    text(ledger["registry_version"], "ledger.registry_version", errors, maximum=32)
    integer(ledger["program_issue"], "ledger.program_issue", errors)
    owner_issue = integer(ledger["owner_issue"], "ledger.owner_issue", errors)
    status = text(ledger["status"], "ledger.status", errors, maximum=32)
    activation_issue = integer(
        ledger["impact_policy_activation_issue"], "ledger.impact_policy_activation_issue", errors
    )
    activation_date_raw = text(
        ledger["impact_policy_activation_date"],
        "ledger.impact_policy_activation_date",
        errors,
        minimum=10,
        maximum=10,
    )
    try:
        activation_date = date.fromisoformat(activation_date_raw)
    except ValueError:
        fail(errors, "ledger.impact_policy_activation_date", "must be an ISO calendar date")
        activation_date = date.max
    activation_revision = text(
        ledger["impact_policy_activation_revision"],
        "ledger.impact_policy_activation_revision",
        errors,
        minimum=40,
        maximum=40,
    )
    if owner_issue != activation_issue:
        fail(errors, "ledger", "owner and impact-policy activation issues must match")
    if not SHA_RE.fullmatch(activation_revision):
        fail(errors, "ledger.impact_policy_activation_revision", "must be a full lowercase Git SHA")
    grandfathered = strings(
        ledger["grandfathered_changes"],
        "ledger.grandfathered_changes",
        errors,
        empty=True,
    )
    for name in grandfathered:
        if not ID_RE.fullmatch(name):
            fail(errors, "ledger.grandfathered_changes", f"invalid change name: {name}")
    unexpected_grandfathering = set(grandfathered) - ALLOWED_GRANDFATHERED_CHANGES
    if unexpected_grandfathering:
        fail(
            errors,
            "ledger.grandfathered_changes",
            f"entries are not in the frozen allowlist: {', '.join(sorted(unexpected_grandfathering))}",
        )
    vocabularies: dict[str, list[str]] = {}
    for key, expected in EXPECTED_VOCABULARIES.items():
        actual = strings(ledger[key], f"ledger.{key}", errors, ordered=False)
        if actual != expected:
            fail(errors, f"ledger.{key}", "must match the closed schema-v2 vocabulary")
        vocabularies[key] = actual
    vector_data = load_toml(root, VECTORS, "vector catalog", errors)
    mapping_data = load_toml(root, MAPPINGS, "mapping registry", errors)
    quality_data = load_toml(root, QUALITY, "quality registry", errors)
    vectors = active_index(vector_data.get("vectors"), "vectors", errors)
    mappings = active_index(mapping_data.get("mappings"), "mappings", errors)
    qualities = active_index(quality_data.get("declarations"), "quality declarations", errors)
    raw_changes = ledger["changes"]
    if not isinstance(raw_changes, list):
        fail(errors, "ledger.changes", "must be an array")
        raw_changes = []
    changes = [
        validated
        for index, change in enumerate(raw_changes)
        if (validated := validate_change(
            root,
            change,
            index,
            vocabularies,
            vectors,
            mappings,
            qualities,
            merge_ready,
            errors,
        ))
        is not None
    ]
    validate_lifecycle(changes, errors)
    changes.sort(key=lambda change: change["id"])
    expected_status = "active-empty" if not changes else "active"
    if status != expected_status:
        fail(errors, "ledger.status", f"must be {expected_status}")
    if check_impacts:
        validate_impacts(
            root,
            activation_issue,
            activation_date,
            set(grandfathered),
            {change["id"]: change["issue"] for change in changes},
            errors,
        )
    return ledger, changes, errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=Path(__file__).parents[1])
    parser.add_argument("--write", action="store_true", help="write deterministic Markdown")
    parser.add_argument(
        "--merge-ready",
        action="store_true",
        help="require active records to carry the actual pull-request number",
    )
    parser.add_argument("--skip-impacts", action="store_true", help=argparse.SUPPRESS)
    arguments = parser.parse_args()
    root = Path(arguments.root).resolve()
    ledger, changes, errors = validate(
        root,
        check_impacts=not arguments.skip_impacts,
        merge_ready=arguments.merge_ready,
    )
    if errors:
        for error in errors:
            print(f"platform-change-ledger: {error}", file=sys.stderr)
        return 1
    rendered = render(ledger, changes)
    report_path = root / REPORT
    if arguments.write:
        if report_path.exists() or report_path.is_symlink():
            report_file = regular_file(root, REPORT, "rendered report", errors)
            if report_file is None:
                for error in errors:
                    print(f"platform-change-ledger: {error}", file=sys.stderr)
                return 1
        report_path.write_text(rendered, encoding="utf-8")
    else:
        report_file = regular_file(root, REPORT, "rendered report", errors)
        if report_file is None:
            for error in errors:
                print(f"platform-change-ledger: {error}", file=sys.stderr)
            return 1
        try:
            existing = report_file.read_text(encoding="utf-8")
        except (OSError, UnicodeError) as error:
            print(f"platform-change-ledger: cannot read rendered report: {error}", file=sys.stderr)
            return 1
        if existing != rendered:
            print("platform-change-ledger: rendered report drift; run checker with --write", file=sys.stderr)
            return 1
    print(f"platform-change-ledger: {len(changes)} canonical change record(s) passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
