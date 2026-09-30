#!/usr/bin/env python3
"""Validate and render canonical Rust-to-language adapter mappings."""

from __future__ import annotations

import argparse
import re
import sys
import tomllib
from pathlib import Path


REGISTRY = Path("docs/architecture/language-adapter-mappings.toml")
RENDERED = Path("docs/architecture/language-adapter-mappings.md")
TOP_KEYS = {"schema_version", "registry_version", "status", "owner_issue", "canonical_owner", "mappings"}
MAPPING_KEYS = {
    "id", "capability", "owner_issue", "state", "kind", "canonical_crate", "canonical_module",
    "canonical_symbol", "canonical_version_origin", "canonical_revision", "canonical_description",
    "language", "language_package", "language_target", "language_version", "language_revision",
    "language_path", "language_license", "language_symbol", "language_shape_state", "direction",
    "compatibility_class", "fidelity", "version_window", "deprecation_phase", "version_negotiation",
    "consumers", "vector_ids", "rust_selectors", "language_selectors", "max_input_bytes",
    "redaction", "sensitive_fields", "async_ownership", "cancellation_ownership", "migration_action",
    "replacement", "observability", "fallback", "rollback", "removal_gate", "fields", "errors",
    "unsupported"
}
FIELD_KEYS = {"rust", "language", "transform", "direction", "required"}
ERROR_KEYS = {"rust_code", "language_class", "language_message_stable", "preserve_rust_code", "redact_input"}
UNSUPPORTED_KEYS = {"id", "reason", "behavior", "stable_error"}
REQUIRED_IDS = {
    "did.value.typescript.legacy-v1",
    "did-url.value.typescript.legacy-v1",
    "did.error.invalid-did.typescript.legacy-v1",
    "did.error.invalid-did-url.typescript.legacy-v1",
}
ID_PATTERN = re.compile(r"^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)+$")
SHA_PATTERN = re.compile(r"^[0-9a-f]{40}$")
SEMVER_PATTERN = re.compile(r"^[0-9]+\.[0-9]+\.[0-9]+$")
STABLE_CODE_PATTERN = re.compile(r"^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9_-]*)+$")
SELECTOR_PATTERN = re.compile(r"^[A-Za-z0-9_@./:<>?=-]+$")
STATES = {"planned", "active", "deprecated", "removed"}
KINDS = {"value", "error"}
LANGUAGES = {"typescript", "swift", "kotlin"}
DIRECTIONS = {"bidirectional", "rust-to-language", "language-to-rust"}
FIELD_DIRECTIONS = {"both", "rust-to-language", "language-to-rust"}
COMPATIBILITY_CLASSES = {"additive", "fixed", "behavioral", "deprecated", "breaking"}
FIDELITIES = {"lossless", "lossy"}
DEPRECATION_PHASES = {"none", "transitional", "announced", "removal-ready", "removed"}
OWNERSHIP = {"not-applicable", "rust", "language", "adapter"}


def exact_keys(value: object, expected: set[str], label: str, errors: list[str]) -> bool:
    if not isinstance(value, dict):
        errors.append(f"{label} must be a table")
        return False
    keys = set(value)
    if keys != expected:
        errors.append(f"{label} fields differ: missing={sorted(expected - keys)}, unknown={sorted(keys - expected)}")
        return False
    return True


def text(value: object, label: str, errors: list[str]) -> str:
    if not isinstance(value, str) or not value.strip():
        errors.append(f"{label} must be non-empty text")
        return ""
    return value


def strings(value: object, label: str, errors: list[str], *, allow_empty: bool = False) -> list[str]:
    if not isinstance(value, list) or (not allow_empty and not value) or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        qualifier = "a string array" if allow_empty else "a non-empty string array"
        errors.append(f"{label} must be {qualifier}")
        return []
    if len(value) != len(set(value)):
        errors.append(f"{label} contains duplicates")
    return value


def validate_registry(document: object) -> tuple[list[dict[str, object]], list[str]]:
    errors: list[str] = []
    if not exact_keys(document, TOP_KEYS, "registry", errors):
        return [], errors
    assert isinstance(document, dict)
    if document.get("schema_version") != 1:
        errors.append("schema_version must equal 1")
    if not isinstance(document.get("registry_version"), str) or not SEMVER_PATTERN.fullmatch(document["registry_version"]):
        errors.append("registry_version must be semantic x.y.z")
    if document.get("status") != "active" or document.get("owner_issue") != 505:
        errors.append("registry must be active and owned by issue 505")
    if document.get("canonical_owner") != "sdk-rust":
        errors.append("canonical_owner must remain sdk-rust")

    mappings = document.get("mappings")
    if not isinstance(mappings, list) or not mappings:
        errors.append("mappings must be a non-empty array of tables")
        return [], errors
    seen: set[str] = set()
    for index, mapping in enumerate(mappings, start=1):
        label = f"mappings[{index}]"
        if not exact_keys(mapping, MAPPING_KEYS, label, errors):
            continue
        identifier = mapping.get("id")
        if not isinstance(identifier, str) or not ID_PATTERN.fullmatch(identifier):
            errors.append(f"{label}.id must be a stable dotted ID")
            identifier = label
        elif identifier in seen:
            errors.append(f"duplicate mapping id: {identifier}")
        else:
            seen.add(identifier)
        if not isinstance(mapping.get("capability"), str) or not ID_PATTERN.fullmatch(mapping["capability"]):
            errors.append(f"{identifier}.capability must be a stable dotted ID")
        if mapping.get("owner_issue") != 505:
            errors.append(f"{identifier}.owner_issue must equal 505")
        if mapping.get("state") not in STATES:
            errors.append(f"{identifier}.state is invalid")
        kind = mapping.get("kind")
        if kind not in KINDS:
            errors.append(f"{identifier}.kind is invalid")

        if not str(mapping.get("canonical_crate", "")).startswith("identus-"):
            errors.append(f"{identifier}.canonical_crate must be an identus-* crate")
        for field in (
            "canonical_module", "canonical_symbol", "canonical_version_origin", "canonical_description",
            "language_package", "language_target", "language_symbol", "language_shape_state",
            "version_window", "version_negotiation", "migration_action", "replacement", "observability",
            "fallback", "rollback", "removal_gate",
        ):
            text(mapping.get(field), f"{identifier}.{field}", errors)
        if not SHA_PATTERN.fullmatch(str(mapping.get("canonical_revision", ""))):
            errors.append(f"{identifier}.canonical_revision must be a full lowercase Git SHA")
        if mapping.get("language") not in LANGUAGES:
            errors.append(f"{identifier}.language is invalid")
        if not SEMVER_PATTERN.fullmatch(str(mapping.get("language_version", ""))):
            errors.append(f"{identifier}.language_version must be semantic x.y.z")
        if not SHA_PATTERN.fullmatch(str(mapping.get("language_revision", ""))):
            errors.append(f"{identifier}.language_revision must be a full lowercase Git SHA")
        language_path = mapping.get("language_path")
        if not isinstance(language_path, str) or not language_path.strip() or Path(language_path).is_absolute() or ".." in Path(language_path).parts:
            errors.append(f"{identifier}.language_path must be repository-relative")
        if mapping.get("language_license") != "Apache-2.0":
            errors.append(f"{identifier}.language_license must equal Apache-2.0 for this seed")
        if mapping.get("direction") not in DIRECTIONS:
            errors.append(f"{identifier}.direction is invalid")
        if mapping.get("compatibility_class") not in COMPATIBILITY_CLASSES:
            errors.append(f"{identifier}.compatibility_class is invalid")
        fidelity = mapping.get("fidelity")
        if fidelity not in FIDELITIES:
            errors.append(f"{identifier}.fidelity is invalid")
        if mapping.get("deprecation_phase") not in DEPRECATION_PHASES:
            errors.append(f"{identifier}.deprecation_phase is invalid")
        consumers = strings(mapping.get("consumers"), f"{identifier}.consumers", errors)
        if mapping.get("language") == "typescript" and "sdk-ts" not in consumers:
            errors.append(f"{identifier}: TypeScript seed must name sdk-ts as a consumer")
        vectors = strings(mapping.get("vector_ids"), f"{identifier}.vector_ids", errors)
        if any(not ID_PATTERN.fullmatch(vector) for vector in vectors):
            errors.append(f"{identifier}.vector_ids contains an invalid stable ID")
        for selector_field in ("rust_selectors", "language_selectors"):
            selectors = strings(mapping.get(selector_field), f"{identifier}.{selector_field}", errors)
            if any(not SELECTOR_PATTERN.fullmatch(selector) for selector in selectors):
                errors.append(f"{identifier}.{selector_field} contains an invalid selector")
        limit = mapping.get("max_input_bytes")
        if not isinstance(limit, int) or isinstance(limit, bool) or limit <= 0:
            errors.append(f"{identifier}.max_input_bytes must be a positive integer")
        if mapping.get("redaction") != "caller-input":
            errors.append(f"{identifier}.redaction must preserve caller-input redaction")
        strings(mapping.get("sensitive_fields"), f"{identifier}.sensitive_fields", errors, allow_empty=True)
        if mapping.get("async_ownership") not in OWNERSHIP or mapping.get("cancellation_ownership") not in OWNERSHIP:
            errors.append(f"{identifier}: async/cancellation ownership is invalid")

        fields = mapping.get("fields")
        errors_map = mapping.get("errors")
        unsupported = mapping.get("unsupported")
        if not isinstance(fields, list) or not isinstance(errors_map, list) or not isinstance(unsupported, list):
            errors.append(f"{identifier}: fields/errors/unsupported must be arrays")
            continue
        for item_index, item in enumerate(fields, start=1):
            item_label = f"{identifier}.fields[{item_index}]"
            if not exact_keys(item, FIELD_KEYS, item_label, errors):
                continue
            for field in ("rust", "language", "transform"):
                text(item.get(field), f"{item_label}.{field}", errors)
            if item.get("direction") not in FIELD_DIRECTIONS or not isinstance(item.get("required"), bool):
                errors.append(f"{item_label}: direction/required is invalid")
        for item_index, item in enumerate(errors_map, start=1):
            item_label = f"{identifier}.errors[{item_index}]"
            if not exact_keys(item, ERROR_KEYS, item_label, errors):
                continue
            if not STABLE_CODE_PATTERN.fullmatch(str(item.get("rust_code", ""))):
                errors.append(f"{item_label}.rust_code is invalid")
            text(item.get("language_class"), f"{item_label}.language_class", errors)
            if item.get("language_message_stable") is not False:
                errors.append(f"{item_label}.language_message_stable must be false")
            if item.get("preserve_rust_code") is not True or item.get("redact_input") is not True:
                errors.append(f"{item_label} must preserve Rust code and redact input")
        unsupported_ids: set[str] = set()
        for item_index, item in enumerate(unsupported, start=1):
            item_label = f"{identifier}.unsupported[{item_index}]"
            if not exact_keys(item, UNSUPPORTED_KEYS, item_label, errors):
                continue
            unsupported_id = item.get("id")
            if not isinstance(unsupported_id, str) or not ID_PATTERN.fullmatch(unsupported_id):
                errors.append(f"{item_label}.id is invalid")
            elif unsupported_id in unsupported_ids:
                errors.append(f"{identifier}: duplicate unsupported id {unsupported_id}")
            else:
                unsupported_ids.add(unsupported_id)
            text(item.get("reason"), f"{item_label}.reason", errors)
            text(item.get("behavior"), f"{item_label}.behavior", errors)
            if not STABLE_CODE_PATTERN.fullmatch(str(item.get("stable_error", ""))):
                errors.append(f"{item_label}.stable_error is invalid")

        if kind == "value" and (not fields or errors_map):
            errors.append(f"{identifier}: value mapping requires fields and forbids error mappings")
        if kind == "error" and (fields or not errors_map):
            errors.append(f"{identifier}: error mapping requires errors and forbids field mappings")
        if fidelity == "lossless" and unsupported:
            errors.append(f"{identifier}: lossless mapping cannot declare unsupported cases")
        if fidelity == "lossy" and kind == "value" and not unsupported:
            errors.append(f"{identifier}: lossy value mapping requires unsupported behavior")
        if fidelity == "lossy" and kind == "error" and not all(item.get("preserve_rust_code") is True for item in errors_map):
            errors.append(f"{identifier}: lossy error mapping must preserve the canonical code")
        if mapping.get("state") == "active" and mapping.get("deprecation_phase") == "removed":
            errors.append(f"{identifier}: active mapping cannot be removed")

    if seen != REQUIRED_IDS:
        errors.append(f"seed mapping IDs differ: missing={sorted(REQUIRED_IDS - seen)}, extra={sorted(seen - REQUIRED_IDS)}")
    vector_owners: dict[str, str] = {}
    for mapping in mappings:
        for vector in mapping.get("vector_ids", []):
            previous = vector_owners.get(vector)
            if previous is not None:
                errors.append(f"vector {vector} is claimed by both {previous} and {mapping.get('id')}")
            vector_owners[vector] = str(mapping.get("id"))
    return mappings, errors


def escape(value: object) -> str:
    return str(value).replace("|", "\\|").replace("\n", " ")


def render(document: dict[str, object], mappings: list[dict[str, object]]) -> str:
    lines = [
        "# Language adapter mappings",
        "",
        "<!-- Generated by scripts/check-language-adapter-mappings.py; edit the TOML registry. -->",
        "",
        f"Registry version: `{document['registry_version']}`<br>",
        f"Canonical owner: `{document['canonical_owner']}`<br>",
        f"Owner issue: [#{document['owner_issue']}](https://github.com/hyperledger-identus/sdk-rust/issues/{document['owner_issue']})",
        "",
        "| Mapping | Kind | Language surface | Direction | Fidelity | Phase |",
        "| --- | --- | --- | --- | --- | --- |",
    ]
    for mapping in mappings:
        lines.append(
            f"| `{escape(mapping['id'])}` | {mapping['kind']} | `{escape(mapping['language_package'])}` "
            f"`{escape(mapping['language_symbol'])}` | {mapping['direction']} | {mapping['fidelity']} | "
            f"{mapping['deprecation_phase']} |"
        )
    for mapping in mappings:
        lines.extend(
            [
                "",
                f"## `{mapping['id']}`",
                "",
                f"- Canonical: `{mapping['canonical_crate']}::{mapping['canonical_symbol']}` at `{mapping['canonical_revision']}` ({mapping['canonical_version_origin']}).",
                f"- Language source: `{mapping['language_package']}` {mapping['language_version']} at `{mapping['language_revision']}`, `{mapping['language_path']}`.",
                f"- Compatibility: {mapping['compatibility_class']}, {mapping['direction']}, {mapping['fidelity']}; window `{mapping['version_window']}`.",
                f"- Resource policy: at most {mapping['max_input_bytes']} bytes; redaction `{mapping['redaction']}`.",
                f"- Migration: {mapping['migration_action']}",
                f"- Replacement: {mapping['replacement']}",
                f"- Observability: {mapping['observability']}",
                f"- Fallback: {mapping['fallback']}",
                f"- Rollback: {mapping['rollback']}",
                f"- Removal gate: {mapping['removal_gate']}",
                f"- Vectors: {', '.join(f'`{item}`' for item in mapping['vector_ids'])}.",
                "",
            ]
        )
        fields = mapping["fields"]
        if fields:
            lines.extend(["### Fields", "", "| Rust | Language | Transform | Direction | Required |", "| --- | --- | --- | --- | --- |"])
            for item in fields:
                lines.append(f"| `{escape(item['rust'])}` | `{escape(item['language'])}` | {escape(item['transform'])} | {item['direction']} | {str(item['required']).lower()} |")
            lines.append("")
        errors_map = mapping["errors"]
        if errors_map:
            lines.extend(["### Errors", "", "| Rust code | Legacy class | Stable message | Preserve code | Redact input |", "| --- | --- | --- | --- | --- |"])
            for item in errors_map:
                lines.append(f"| `{item['rust_code']}` | `{escape(item['language_class'])}` | {str(item['language_message_stable']).lower()} | {str(item['preserve_rust_code']).lower()} | {str(item['redact_input']).lower()} |")
            lines.append("")
        unsupported = mapping["unsupported"]
        if unsupported:
            lines.extend(["### Unsupported legacy round trips", "", "| Case | Reason | Required behavior | Stable adapter error |", "| --- | --- | --- | --- |"])
            for item in unsupported:
                lines.append(f"| `{item['id']}` | {escape(item['reason'])} | {escape(item['behavior'])} | `{item['stable_error']}` |")
            lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    parser.add_argument("--render", action="store_true")
    args = parser.parse_args()
    root = Path(args.root).resolve()
    try:
        document = tomllib.loads((root / REGISTRY).read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as error:
        print(f"language-adapter-mappings: cannot load {REGISTRY}: {error}", file=sys.stderr)
        return 1
    mappings, errors = validate_registry(document)
    if errors:
        for error in errors:
            print(f"language-adapter-mappings: {error}", file=sys.stderr)
        print(f"language-adapter-mappings: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    rendered = render(document, mappings)
    if args.render:
        print(rendered, end="")
        return 0
    try:
        checked_in = (root / RENDERED).read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError) as error:
        print(f"language-adapter-mappings: cannot load {RENDERED}: {error}", file=sys.stderr)
        return 1
    if checked_in != rendered:
        print("language-adapter-mappings: rendered Markdown is stale; regenerate from the TOML registry", file=sys.stderr)
        return 1
    print(f"language-adapter-mappings: {len(mappings)} canonical mappings passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
