#!/usr/bin/env python3
"""Validate and render canonical Rust-to-language adapter mappings."""

from __future__ import annotations

import argparse
import re
import stat
import sys
import tomllib
from pathlib import Path


REGISTRY = Path("docs/architecture/language-adapter-mappings.toml")
RENDERED = Path("docs/architecture/language-adapter-mappings.md")
VECTOR_CATALOG = Path("docs/conformance/cross-language-vector-catalog.toml")
MAX_EVIDENCE_BYTES = 2 * 1024 * 1024
TOP_KEYS = {"schema_version", "registry_version", "status", "owner_issue", "canonical_owner", "mappings"}
MAPPING_KEYS = {
    "id", "capability", "owner_issue", "state", "kind", "canonical_crate", "canonical_rust_path",
    "canonical_version_origin", "canonical_revision", "canonical_description",
    "canonical_bound_path", "canonical_bound_symbol",
    "language", "language_package", "language_target", "language_version", "language_revision",
    "language_path", "language_license", "language_symbol", "language_shape_state", "direction",
    "compatibility_class", "fidelity", "version_window", "deprecation_phase", "version_negotiation",
    "consumers", "vector_ids", "rust_selectors", "language_selectors", "max_input_bytes",
    "redaction", "sensitive_fields", "async_ownership", "cancellation_ownership", "migration_action",
    "replacement", "observability", "fallback", "rollback", "removal_gate", "fields", "errors",
    "losses", "unsupported"
}
FIELD_KEYS = {"rust", "language", "transform", "direction", "required"}
ERROR_KEYS = {"rust_code", "language_class", "language_message_stable", "preserve_rust_code", "redact_input"}
LOSS_KEYS = {"id", "distinction", "consequence", "mitigation"}
UNSUPPORTED_KEYS = {"id", "reason", "behavior", "stable_error"}
REQUIRED_IDS = {
    "did.value.typescript.legacy-v1",
    "did-url.value.typescript.legacy-v1",
    "did.error.invalid-did.typescript.legacy-v1",
    "did.error.invalid-did-url.typescript.legacy-v1",
}
ID_PATTERN = re.compile(r"^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)+$")
SHA_PATTERN = re.compile(r"^[0-9a-f]{40}$")
SEMVER_COMPONENT = r"(?:0|[1-9][0-9]*)"
SEMVER_PATTERN = re.compile(
    rf"^{SEMVER_COMPONENT}\.{SEMVER_COMPONENT}\.{SEMVER_COMPONENT}$"
)
VERSION_WINDOW_PATTERN = re.compile(
    rf"^>=({SEMVER_COMPONENT}\.{SEMVER_COMPONENT}\.{SEMVER_COMPONENT}),"
    rf"<({SEMVER_COMPONENT}\.{SEMVER_COMPONENT}\.{SEMVER_COMPONENT})$"
)
STABLE_CODE_PATTERN = re.compile(r"^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9_-]*)+$")
SELECTOR_PATTERN = re.compile(r"^[A-Za-z0-9_@./:<>?=-]+$")
CONST_PATTERN = re.compile(r"^[A-Z][A-Z0-9_]*$")
RUST_PATH_PATTERN = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)+$")
STATES = {"planned", "active", "deprecated", "removed"}
KINDS = {"value", "error"}
LANGUAGES = {"typescript", "swift", "kotlin"}
DIRECTIONS = {"bidirectional", "rust-to-language", "language-to-rust"}
FIELD_DIRECTIONS = {"both", "rust-to-language", "language-to-rust"}
COMPATIBILITY_CLASSES = {"additive", "fixed", "behavioral", "deprecated", "breaking"}
FIDELITIES = {"lossless", "lossy"}
DEPRECATION_PHASES = {"none", "transitional", "announced", "removal-ready", "removed"}
OWNERSHIP = {"not-applicable", "rust", "language", "adapter"}
DIRECTION_MATRIX = {
    "bidirectional": FIELD_DIRECTIONS,
    "rust-to-language": {"rust-to-language"},
    "language-to-rust": {"language-to-rust"},
}
LIFECYCLE_MATRIX = {
    "planned": {"none"},
    "active": {"none", "transitional", "announced", "removal-ready"},
    "deprecated": {"announced", "removal-ready"},
    "removed": {"removed"},
}


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


def version_tuple(value: str) -> tuple[int, int, int]:
    major, minor, patch = value.split(".")
    return int(major), int(minor), int(patch)


def load_vector_catalog(
    root: Path, errors: list[str]
) -> dict[str, list[tuple[object, object, object, object]]]:
    path = root
    try:
        for part in VECTOR_CATALOG.parts:
            path /= part
            if path.is_symlink():
                errors.append("vector catalog must not contain symlink components")
                return {}
        resolved_root = root.resolve(strict=True)
        resolved_path = path.resolve(strict=True)
        resolved_path.relative_to(resolved_root)
        metadata = resolved_path.stat()
        if not stat.S_ISREG(metadata.st_mode) or metadata.st_size > MAX_EVIDENCE_BYTES:
            errors.append("vector catalog must be a bounded regular file")
            return {}
        document = tomllib.loads(resolved_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, ValueError, tomllib.TOMLDecodeError):
        errors.append("vector catalog cannot be read as bounded TOML")
        return {}
    if not isinstance(document, dict) or not isinstance(document.get("vectors"), list):
        errors.append("vector catalog must contain a vectors array")
        return {}
    found: dict[str, list[tuple[object, object, object, object]]] = {}
    for index, vector in enumerate(document["vectors"], start=1):
        if not isinstance(vector, dict):
            errors.append(f"vector catalog entry {index} must be a table")
            continue
        identifier = vector.get("id")
        if not isinstance(identifier, str) or not ID_PATTERN.fullmatch(identifier):
            errors.append(f"vector catalog entry {index} has an invalid id")
            continue
        found.setdefault(identifier, []).append(
            (
                vector.get("capability"),
                vector.get("targets"),
                vector.get("expected"),
                vector.get("source_locator"),
            )
        )
    return found


def canonical_bound(
    root: Path, mapping: dict[str, object], identifier: str, errors: list[str]
) -> int | None:
    path_value = mapping.get("canonical_bound_path")
    symbol_value = mapping.get("canonical_bound_symbol")
    if not isinstance(path_value, str) or not path_value.strip():
        errors.append(f"{identifier}.canonical_bound_path must be a safe repository-relative path")
        return None
    relative = Path(path_value)
    if (
        relative.is_absolute()
        or not relative.parts
        or any(part in {"", ".", ".."} for part in relative.parts)
        or "\\" in path_value
    ):
        errors.append(f"{identifier}.canonical_bound_path must be a safe repository-relative path")
        return None
    canonical_crate = mapping.get("canonical_crate")
    if isinstance(canonical_crate, str) and canonical_crate.startswith("identus-"):
        crate_source = Path("crates") / canonical_crate.removeprefix("identus-") / "src"
        try:
            relative.relative_to(crate_source)
        except ValueError:
            errors.append(
                f"{identifier}.canonical_bound_path must remain under declared Cargo package source {crate_source}"
            )
            return None
    if not isinstance(symbol_value, str) or not CONST_PATTERN.fullmatch(symbol_value):
        errors.append(f"{identifier}.canonical_bound_symbol must be an uppercase Rust constant")
        return None

    source = root
    try:
        for part in relative.parts:
            source /= part
            if source.is_symlink():
                errors.append(
                    f"{identifier}.canonical_bound_path must not contain symlink components"
                )
                return None
        resolved_root = root.resolve(strict=True)
        resolved_source = source.resolve(strict=True)
        resolved_source.relative_to(resolved_root)
        metadata = resolved_source.stat()
        if not stat.S_ISREG(metadata.st_mode):
            errors.append(f"{identifier}.canonical_bound_path must name a regular file")
            return None
        if metadata.st_size > MAX_EVIDENCE_BYTES:
            errors.append(
                f"{identifier}.canonical_bound_path exceeds {MAX_EVIDENCE_BYTES} bytes"
            )
            return None
        content = resolved_source.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        errors.append(f"{identifier}.canonical_bound_path cannot be read safely")
        return None
    except ValueError:
        errors.append(f"{identifier}.canonical_bound_path resolves outside the repository")
        return None
    except OSError:
        errors.append(f"{identifier}.canonical_bound_path cannot be read safely")
        return None
    pattern = re.compile(
        rf"(?m)^pub const {re.escape(symbol_value)}\s*:\s*usize\s*=\s*([0-9][0-9_]*)\s*;"
    )
    matches = pattern.findall(content)
    if len(matches) != 1:
        errors.append(
            f"{identifier}.canonical_bound_symbol must resolve exactly once to a literal public usize constant"
        )
        return None
    return int(matches[0].replace("_", ""))


def validate_registry(document: object, root: Path) -> tuple[list[dict[str, object]], list[str]]:
    errors: list[str] = []
    if not exact_keys(document, TOP_KEYS, "registry", errors):
        return [], errors
    assert isinstance(document, dict)
    if type(document.get("schema_version")) is not int or document.get("schema_version") != 1:
        errors.append("schema_version must equal 1")
    if not isinstance(document.get("registry_version"), str) or not SEMVER_PATTERN.fullmatch(document["registry_version"]):
        errors.append("registry_version must be semantic x.y.z")
    if (
        document.get("status") != "active"
        or type(document.get("owner_issue")) is not int
        or document.get("owner_issue") != 505
    ):
        errors.append("registry must be active and owned by issue 505")
    if document.get("canonical_owner") != "sdk-rust":
        errors.append("canonical_owner must remain sdk-rust")

    mappings = document.get("mappings")
    if not isinstance(mappings, list) or not mappings:
        errors.append("mappings must be a non-empty array of tables")
        return [], errors
    vector_catalog = load_vector_catalog(root, errors)
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
        if type(mapping.get("owner_issue")) is not int or mapping.get("owner_issue") != 505:
            errors.append(f"{identifier}.owner_issue must equal 505")
        if mapping.get("state") not in STATES:
            errors.append(f"{identifier}.state is invalid")
        kind = mapping.get("kind")
        if kind not in KINDS:
            errors.append(f"{identifier}.kind is invalid")

        canonical_crate = mapping.get("canonical_crate")
        if not isinstance(canonical_crate, str) or not canonical_crate.startswith("identus-"):
            errors.append(f"{identifier}.canonical_crate must be an identus-* crate")
        canonical_rust_path = mapping.get("canonical_rust_path")
        if not isinstance(canonical_rust_path, str) or not RUST_PATH_PATTERN.fullmatch(
            canonical_rust_path
        ):
            errors.append(f"{identifier}.canonical_rust_path must be an exact Rust API path")
        elif isinstance(canonical_crate, str):
            path_parts = canonical_rust_path.split("::")
            expected_prefix = canonical_crate.replace("-", "_")
            if path_parts[0] != expected_prefix:
                errors.append(
                    f"{identifier}.canonical_rust_path must begin with {expected_prefix}"
                )
            elif len(path_parts) > 1 and path_parts[1] == expected_prefix:
                errors.append(
                    f"{identifier}.canonical_rust_path must not duplicate its crate prefix"
                )
        for field in (
            "canonical_version_origin", "canonical_description",
            "language_package", "language_target", "language_symbol", "language_shape_state",
            "version_window", "version_negotiation", "migration_action", "replacement", "observability",
            "fallback", "rollback", "removal_gate",
        ):
            text(mapping.get(field), f"{identifier}.{field}", errors)
        if not SHA_PATTERN.fullmatch(str(mapping.get("canonical_revision", ""))):
            errors.append(f"{identifier}.canonical_revision must be a full lowercase Git SHA")
        if mapping.get("language") not in LANGUAGES:
            errors.append(f"{identifier}.language is invalid")
        language_version = str(mapping.get("language_version", ""))
        if not SEMVER_PATTERN.fullmatch(language_version):
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
        version_window = mapping.get("version_window")
        window_match = (
            VERSION_WINDOW_PATTERN.fullmatch(version_window)
            if isinstance(version_window, str)
            else None
        )
        if window_match is None:
            errors.append(
                f"{identifier}.version_window must use canonical >=x.y.z,<x.y.z syntax"
            )
        elif SEMVER_PATTERN.fullmatch(language_version):
            lower = version_tuple(window_match.group(1))
            upper = version_tuple(window_match.group(2))
            selected = version_tuple(language_version)
            exact_upper = (selected[0], selected[1], selected[2] + 1)
            if lower != selected or upper != exact_upper:
                errors.append(
                    f"{identifier}.version_window must equal the pinned language_version patch interval"
                )
        if mapping.get("deprecation_phase") not in DEPRECATION_PHASES:
            errors.append(f"{identifier}.deprecation_phase is invalid")
        consumers = strings(mapping.get("consumers"), f"{identifier}.consumers", errors)
        if mapping.get("language") == "typescript" and "sdk-ts" not in consumers:
            errors.append(f"{identifier}: TypeScript seed must name sdk-ts as a consumer")
        vectors = strings(mapping.get("vector_ids"), f"{identifier}.vector_ids", errors)
        if any(not ID_PATTERN.fullmatch(vector) for vector in vectors):
            errors.append(f"{identifier}.vector_ids contains an invalid stable ID")
        bound_has_vector_evidence = False
        for vector in vectors:
            if not ID_PATTERN.fullmatch(vector):
                continue
            matches = vector_catalog.get(vector, [])
            if len(matches) != 1:
                errors.append(
                    f"{identifier}.vector_ids entry {vector} must resolve exactly once in the canonical catalog"
                )
                continue
            vector_capability, vector_targets, vector_expected, vector_locator = matches[0]
            if vector_capability != mapping.get("capability"):
                errors.append(
                    f"{identifier}.vector_ids entry {vector} has the wrong capability"
                )
            if not isinstance(vector_targets, list) or mapping.get("language") not in vector_targets:
                errors.append(
                    f"{identifier}.vector_ids entry {vector} does not target {mapping.get('language')}"
                )
            declared_errors = mapping.get("errors")
            expected_outcomes = (
                {"success"}
                if kind == "value"
                else {
                    item.get("rust_code")
                    for item in declared_errors
                    if isinstance(item, dict)
                }
                if isinstance(declared_errors, list)
                else set()
            )
            if vector_expected not in expected_outcomes:
                errors.append(
                    f"{identifier}.vector_ids entry {vector} has an outcome incompatible with {kind} mapping evidence"
                )
            bound_symbol = mapping.get("canonical_bound_symbol")
            if isinstance(bound_symbol, str) and isinstance(vector_locator, str):
                if re.search(
                    rf"(?<![A-Z0-9_]){re.escape(bound_symbol)}(?![A-Z0-9_])",
                    vector_locator,
                ):
                    bound_has_vector_evidence = True
        if not bound_has_vector_evidence:
            errors.append(
                f"{identifier}.canonical_bound_symbol must be named by referenced vector evidence"
            )
        for selector_field in ("rust_selectors", "language_selectors"):
            selectors = strings(mapping.get(selector_field), f"{identifier}.{selector_field}", errors)
            if any(not SELECTOR_PATTERN.fullmatch(selector) for selector in selectors):
                errors.append(f"{identifier}.{selector_field} contains an invalid selector")
        limit = mapping.get("max_input_bytes")
        if not isinstance(limit, int) or isinstance(limit, bool) or limit <= 0:
            errors.append(f"{identifier}.max_input_bytes must be a positive integer")
        rust_limit = canonical_bound(root, mapping, str(identifier), errors)
        if (
            isinstance(limit, int)
            and not isinstance(limit, bool)
            and rust_limit is not None
            and limit > rust_limit
        ):
            errors.append(
                f"{identifier}.max_input_bytes cannot exceed canonical Rust bound {rust_limit}"
            )
        if mapping.get("redaction") != "caller-input":
            errors.append(f"{identifier}.redaction must preserve caller-input redaction")
        strings(mapping.get("sensitive_fields"), f"{identifier}.sensitive_fields", errors, allow_empty=True)
        if mapping.get("async_ownership") not in OWNERSHIP or mapping.get("cancellation_ownership") not in OWNERSHIP:
            errors.append(f"{identifier}: async/cancellation ownership is invalid")

        fields = mapping.get("fields")
        errors_map = mapping.get("errors")
        losses = mapping.get("losses")
        unsupported = mapping.get("unsupported")
        if not all(isinstance(value, list) for value in (fields, errors_map, losses, unsupported)):
            errors.append(f"{identifier}: fields/errors/losses/unsupported must be arrays")
            continue
        assert isinstance(fields, list)
        assert isinstance(errors_map, list)
        assert isinstance(losses, list)
        assert isinstance(unsupported, list)
        rust_fields: set[str] = set()
        language_fields: set[str] = set()
        for item_index, item in enumerate(fields, start=1):
            item_label = f"{identifier}.fields[{item_index}]"
            if not exact_keys(item, FIELD_KEYS, item_label, errors):
                continue
            for field in ("rust", "language", "transform"):
                text(item.get(field), f"{item_label}.{field}", errors)
            rust_field = item.get("rust")
            language_field = item.get("language")
            if isinstance(rust_field, str) and rust_field:
                if rust_field in rust_fields:
                    errors.append(f"{identifier}: duplicate Rust field {rust_field}")
                rust_fields.add(rust_field)
            if isinstance(language_field, str) and language_field:
                if language_field in language_fields:
                    errors.append(f"{identifier}: duplicate language field {language_field}")
                language_fields.add(language_field)
            field_direction = item.get("direction")
            if field_direction not in FIELD_DIRECTIONS or not isinstance(item.get("required"), bool):
                errors.append(f"{item_label}: direction/required is invalid")
            elif field_direction not in DIRECTION_MATRIX.get(str(mapping.get("direction")), set()):
                errors.append(
                    f"{item_label}.direction exceeds enclosing mapping direction"
                )
        error_codes: set[str] = set()
        for item_index, item in enumerate(errors_map, start=1):
            item_label = f"{identifier}.errors[{item_index}]"
            if not exact_keys(item, ERROR_KEYS, item_label, errors):
                continue
            rust_code = item.get("rust_code")
            if not STABLE_CODE_PATTERN.fullmatch(str(rust_code or "")):
                errors.append(f"{item_label}.rust_code is invalid")
            elif isinstance(rust_code, str):
                if rust_code in error_codes:
                    errors.append(f"{identifier}: duplicate Rust error code {rust_code}")
                error_codes.add(rust_code)
            text(item.get("language_class"), f"{item_label}.language_class", errors)
            if item.get("language_message_stable") is not False:
                errors.append(f"{item_label}.language_message_stable must be false")
            if item.get("preserve_rust_code") is not True or item.get("redact_input") is not True:
                errors.append(f"{item_label} must preserve Rust code and redact input")
        loss_ids: set[str] = set()
        for item_index, item in enumerate(losses, start=1):
            item_label = f"{identifier}.losses[{item_index}]"
            if not exact_keys(item, LOSS_KEYS, item_label, errors):
                continue
            loss_id = item.get("id")
            if not isinstance(loss_id, str) or not ID_PATTERN.fullmatch(loss_id):
                errors.append(f"{item_label}.id is invalid")
            elif loss_id in loss_ids:
                errors.append(f"{identifier}: duplicate loss id {loss_id}")
            else:
                loss_ids.add(loss_id)
            for field in ("distinction", "consequence", "mitigation"):
                text(item.get(field), f"{item_label}.{field}", errors)
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
                if unsupported_id not in loss_ids:
                    errors.append(
                        f"{item_label}.id must reference a declared loss"
                    )
            text(item.get("reason"), f"{item_label}.reason", errors)
            text(item.get("behavior"), f"{item_label}.behavior", errors)
            if not STABLE_CODE_PATTERN.fullmatch(str(item.get("stable_error", ""))):
                errors.append(f"{item_label}.stable_error is invalid")

        if kind == "value" and (not fields or errors_map):
            errors.append(f"{identifier}: value mapping requires fields and forbids error mappings")
        if kind == "error" and (fields or not errors_map):
            errors.append(f"{identifier}: error mapping requires errors and forbids field mappings")
        if fidelity == "lossless" and (losses or unsupported):
            errors.append(f"{identifier}: lossless mapping cannot declare losses or unsupported cases")
        if fidelity == "lossy" and not losses:
            errors.append(f"{identifier}: lossy mapping requires at least one loss record")
        state = mapping.get("state")
        phase = mapping.get("deprecation_phase")
        if state in LIFECYCLE_MATRIX and phase not in LIFECYCLE_MATRIX[state]:
            errors.append(f"{identifier}: state and deprecation phase are incoherent")

    missing = REQUIRED_IDS - seen
    if missing:
        errors.append(f"required seed mapping IDs are missing: {sorted(missing)}")
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
        f"Schema version: `{document['schema_version']}`<br>",
        f"Status: `{document['status']}`<br>",
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
                f"- Record: capability `{mapping['capability']}`, owner [#{mapping['owner_issue']}](https://github.com/hyperledger-identus/sdk-rust/issues/{mapping['owner_issue']}), state `{mapping['state']}`, kind `{mapping['kind']}`.",
                f"- Cargo package: `{mapping['canonical_crate']}`.",
                f"- Rust API: `{mapping['canonical_rust_path']}` at `{mapping['canonical_revision']}` ({mapping['canonical_version_origin']}).",
                f"- Canonical semantics: {mapping['canonical_description']}",
                f"- Canonical bound: `{mapping['canonical_bound_symbol']}` in `{mapping['canonical_bound_path']}`.",
                f"- Language source: `{mapping['language']}` package `{mapping['language_package']}`, target `{mapping['language_target']}`, symbol `{mapping['language_symbol']}`, version `{mapping['language_version']}` at `{mapping['language_revision']}`, path `{mapping['language_path']}`.",
                f"- Language evidence: license `{mapping['language_license']}`; shape state `{mapping['language_shape_state']}`.",
                f"- Compatibility: {mapping['compatibility_class']}, {mapping['direction']}, {mapping['fidelity']}; window `{mapping['version_window']}`; phase `{mapping['deprecation_phase']}`.",
                f"- Version negotiation: {mapping['version_negotiation']}",
                f"- Consumers: {', '.join(f'`{item}`' for item in mapping['consumers'])}.",
                f"- Resource policy: at most {mapping['max_input_bytes']} bytes; redaction `{mapping['redaction']}`; sensitive fields: {', '.join(f'`{item}`' for item in mapping['sensitive_fields']) or 'none'}.",
                f"- Ownership: async `{mapping['async_ownership']}`; cancellation `{mapping['cancellation_ownership']}`.",
                f"- Migration: {mapping['migration_action']}",
                f"- Replacement: {mapping['replacement']}",
                f"- Observability: {mapping['observability']}",
                f"- Fallback: {mapping['fallback']}",
                f"- Rollback: {mapping['rollback']}",
                f"- Removal gate: {mapping['removal_gate']}",
                f"- Vectors: {', '.join(f'`{item}`' for item in mapping['vector_ids'])}.",
                f"- Rust selectors: {', '.join(f'`{item}`' for item in mapping['rust_selectors'])}.",
                f"- Language selectors: {', '.join(f'`{item}`' for item in mapping['language_selectors'])}.",
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
        losses = mapping["losses"]
        if losses:
            lines.extend(["### Losses", "", "| ID | Lost distinction | Consequence | Mitigation |", "| --- | --- | --- | --- |"])
            for item in losses:
                lines.append(
                    f"| `{item['id']}` | {escape(item['distinction'])} | "
                    f"{escape(item['consequence'])} | {escape(item['mitigation'])} |"
                )
            lines.append("")
        unsupported = mapping["unsupported"]
        if unsupported:
            lines.extend(["### Unsupported legacy round trips", "", "| Case | Reason | Required behavior | Stable adapter error |", "| --- | --- | --- | --- |"])
            for item in unsupported:
                lines.append(f"| `{item['id']}` | {escape(item['reason'])} | {escape(item['behavior'])} | `{item['stable_error']}` |")
            lines.append("")
    compacted: list[str] = []
    for line in lines:
        if line or not compacted or compacted[-1]:
            compacted.append(line)
    return "\n".join(compacted).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    parser.add_argument("--render", action="store_true")
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    if args.render and args.write:
        parser.error("--render and --write are mutually exclusive")
    root = Path(args.root).resolve()
    try:
        document = tomllib.loads((root / REGISTRY).read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as error:
        print(f"language-adapter-mappings: cannot load {REGISTRY}: {error}", file=sys.stderr)
        return 1
    mappings, errors = validate_registry(document, root)
    if errors:
        for error in errors:
            print(f"language-adapter-mappings: {error}", file=sys.stderr)
        print(f"language-adapter-mappings: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    rendered = render(document, mappings)
    if args.render:
        print(rendered, end="")
        return 0
    if args.write:
        try:
            (root / RENDERED).write_text(rendered, encoding="utf-8")
        except OSError as error:
            print(f"language-adapter-mappings: cannot write {RENDERED}: {error}", file=sys.stderr)
            return 1
        print(f"language-adapter-mappings: wrote {RENDERED}")
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
