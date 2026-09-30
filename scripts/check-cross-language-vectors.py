#!/usr/bin/env python3
"""Validate the offline cross-language vector catalog and its packets."""

from __future__ import annotations

import hashlib
import json
import re
import sys
import tomllib
from pathlib import Path


CATALOG = Path("docs/conformance/cross-language-vector-catalog.toml")
FIXTURE_ROOT = Path("docs/conformance/fixtures")
TOP_KEYS = {"schema_version", "catalog_version", "status", "owner_issue", "sources", "packets", "vectors"}
REPOSITORY_SOURCE_KEYS = {
    "id", "kind", "url", "revision", "path", "authorship", "license", "redistribution"
}
STANDARD_SOURCE_KEYS = {
    "id", "kind", "url", "version", "section", "authorship", "license", "redistribution"
}
PACKET_KEYS = {
    "id", "capability", "schema_version", "path", "sha256", "authorship", "license",
    "redistribution", "public_data", "targets"
}
VECTOR_KEYS = {
    "id", "packet", "case", "capability", "authority", "source_ids", "source_locator",
    "operation", "expected", "profile_scope", "boundary", "resource_class", "targets",
    "rust_selector", "language_selectors", "owner_issue", "active", "supersedes",
    "replaced_by", "external_records", "limitations"
}
PACKET_DOCUMENT_KEYS = {"schemaVersion", "packetId", "capability", "cases"}
CASE_KEYS = {"id", "operation", "input", "expected"}
EXPECTED_KEYS = {"kind", "errorCode", "redacted", "fields"}
GENERATED_KEYS = {"prefix", "repeat", "count", "suffix"}
FIELD_KEYS = {"serialized", "did", "method", "methodSpecificId", "path", "query", "fragment"}
AUTHORITIES = {
    "normative", "identus-contract", "consumer-regression", "implementation-regression", "exploratory"
}
TARGETS = {"rust", "typescript", "swift", "kotlin"}
OPERATIONS = {"did.parse", "did-url.parse"}
BOUNDARIES = {"positive", "negative", "boundary", "mutation"}
ID_PATTERN = re.compile(r"^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)+$")
SHA_PATTERN = re.compile(r"^[0-9a-f]{40}$")
DIGEST_PATTERN = re.compile(r"^[0-9a-f]{64}$")
SEMVER_PATTERN = re.compile(r"^[0-9]+\.[0-9]+\.[0-9]+$")
SELECTOR_PATTERN = re.compile(r"^[A-Za-z0-9_.:/?=@%+ -]+$")


def exact_keys(value: object, expected: set[str], label: str, errors: list[str]) -> bool:
    if not isinstance(value, dict):
        errors.append(f"{label} must be a table/object")
        return False
    keys = set(value)
    if keys != expected:
        errors.append(f"{label} fields differ: missing={sorted(expected - keys)}, unknown={sorted(keys - expected)}")
        return False
    return True


def nonempty_string(value: object, label: str, errors: list[str]) -> bool:
    if not isinstance(value, str) or not value.strip():
        errors.append(f"{label} must be non-empty text")
        return False
    return True


def string_array(value: object, label: str, errors: list[str], *, allow_empty: bool = False) -> list[str]:
    if not isinstance(value, list) or (not value and not allow_empty) or not all(
        isinstance(item, str) and item.strip() for item in value
    ):
        qualifier = "a string array" if allow_empty else "a non-empty string array"
        errors.append(f"{label} must be {qualifier}")
        return []
    if len(value) != len(set(value)):
        errors.append(f"{label} contains duplicates")
    return value


def safe_payload(root: Path, raw: object, label: str, errors: list[str]) -> Path | None:
    if not nonempty_string(raw, label, errors):
        return None
    relative = Path(str(raw))
    if relative.is_absolute() or ".." in relative.parts or not relative.parts:
        errors.append(f"{label} must be a safe repository-relative path")
        return None
    try:
        relative.relative_to(FIXTURE_ROOT)
    except ValueError:
        errors.append(f"{label} must remain below {FIXTURE_ROOT}")
        return None
    candidate = root / relative
    if not candidate.is_file():
        errors.append(f"{label} does not exist: {relative}")
        return None
    if any(part.is_symlink() for part in [candidate, *candidate.parents] if part != root.parent):
        errors.append(f"{label} must not resolve through a symlink")
        return None
    try:
        candidate.resolve().relative_to((root / FIXTURE_ROOT).resolve())
    except ValueError:
        errors.append(f"{label} resolves outside {FIXTURE_ROOT}")
        return None
    return candidate


def validate_input(value: object, label: str, errors: list[str]) -> str | None:
    if not isinstance(value, dict) or set(value) not in ({"literal"}, {"generated"}):
        errors.append(f"{label} must contain exactly literal or generated")
        return None
    if "literal" in value:
        literal = value["literal"]
        if not isinstance(literal, str):
            errors.append(f"{label}.literal must be text")
            return None
        return literal
    generated = value["generated"]
    if not exact_keys(generated, GENERATED_KEYS, f"{label}.generated", errors):
        return None
    prefix = generated.get("prefix")
    repeated = generated.get("repeat")
    count = generated.get("count")
    suffix = generated.get("suffix")
    if not all(isinstance(item, str) and item.isascii() for item in (prefix, repeated, suffix)):
        errors.append(f"{label}.generated strings must be ASCII")
        return None
    if len(repeated) != 1:
        errors.append(f"{label}.generated.repeat must be exactly one ASCII character")
        return None
    if not isinstance(count, int) or isinstance(count, bool) or not 0 <= count <= 10_000:
        errors.append(f"{label}.generated.count must be an integer from 0 through 10000")
        return None
    return prefix + repeated * count + suffix


def validate_packet(path: Path, packet: dict[str, object], errors: list[str]) -> dict[str, dict[str, object]]:
    label = f"packet {packet.get('id', '<unknown>')}"
    try:
        document = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, json.JSONDecodeError) as error:
        errors.append(f"{label} cannot load JSON: {error}")
        return {}
    if not exact_keys(document, PACKET_DOCUMENT_KEYS, label, errors):
        return {}
    if document.get("schemaVersion") != packet.get("schema_version"):
        errors.append(f"{label} schemaVersion does not match catalog")
    if document.get("packetId") != packet.get("id"):
        errors.append(f"{label} packetId does not match catalog")
    if document.get("capability") != packet.get("capability"):
        errors.append(f"{label} capability does not match catalog")
    cases = document.get("cases")
    if not isinstance(cases, list) or not cases:
        errors.append(f"{label}.cases must be a non-empty array")
        return {}
    found: dict[str, dict[str, object]] = {}
    for index, case in enumerate(cases, start=1):
        case_label = f"{label}.cases[{index}]"
        if not exact_keys(case, CASE_KEYS, case_label, errors):
            continue
        identifier = case.get("id")
        if not isinstance(identifier, str) or not ID_PATTERN.fullmatch(identifier):
            errors.append(f"{case_label}.id is invalid")
            continue
        if identifier in found:
            errors.append(f"duplicate packet case: {identifier}")
        operation = case.get("operation")
        if operation not in OPERATIONS:
            errors.append(f"{identifier}.operation is invalid")
        validate_input(case.get("input"), f"{identifier}.input", errors)
        expected = case.get("expected")
        if not exact_keys(expected, EXPECTED_KEYS, f"{identifier}.expected", errors):
            continue
        kind = expected.get("kind")
        fields = expected.get("fields")
        if not isinstance(fields, dict) or set(fields) - FIELD_KEYS:
            errors.append(f"{identifier}.expected.fields has unknown fields or is not an object")
        elif not all(value is None or isinstance(value, str) for value in fields.values()):
            errors.append(f"{identifier}.expected.fields values must be text or null")
        if kind == "success":
            if expected.get("errorCode") is not None or expected.get("redacted") is not None:
                errors.append(f"{identifier}: success must use null errorCode and redacted")
        elif kind == "error":
            required_code = "did.invalid_did" if operation == "did.parse" else "did.invalid_did_url"
            if expected.get("errorCode") != required_code or expected.get("redacted") is not True or fields:
                errors.append(f"{identifier}: error expectation must use {required_code}, redacted=true, and empty fields")
        else:
            errors.append(f"{identifier}.expected.kind must be success or error")
        found[identifier] = case
    return found


def main() -> int:
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path.cwd().resolve()
    errors: list[str] = []
    try:
        document = tomllib.loads((root / CATALOG).read_text(encoding="utf-8"))
    except (OSError, UnicodeDecodeError, tomllib.TOMLDecodeError) as error:
        print(f"cross-language-vectors: cannot load {CATALOG}: {error}", file=sys.stderr)
        return 1

    exact_keys(document, TOP_KEYS, "catalog", errors)
    if document.get("schema_version") != 1:
        errors.append("schema_version must equal 1")
    if not isinstance(document.get("catalog_version"), str) or not SEMVER_PATTERN.fullmatch(document["catalog_version"]):
        errors.append("catalog_version must be semantic x.y.z")
    if document.get("status") != "active" or document.get("owner_issue") != 420:
        errors.append("catalog must be active and owned by issue 420")

    sources = document.get("sources")
    if not isinstance(sources, list) or not sources:
        errors.append("sources must be a non-empty array of tables")
        sources = []
    source_ids: set[str] = set()
    for index, source in enumerate(sources, start=1):
        label = f"sources[{index}]"
        if not isinstance(source, dict):
            errors.append(f"{label} must be a table")
            continue
        kind = source.get("kind")
        expected_keys = REPOSITORY_SOURCE_KEYS if kind == "repository" else STANDARD_SOURCE_KEYS if kind == "standard" else set()
        if not expected_keys:
            errors.append(f"{label}.kind must be repository or standard")
            continue
        exact_keys(source, expected_keys, label, errors)
        identifier = source.get("id")
        if not isinstance(identifier, str) or not ID_PATTERN.fullmatch(identifier):
            errors.append(f"{label}.id is invalid")
        elif identifier in source_ids:
            errors.append(f"duplicate source id: {identifier}")
        else:
            source_ids.add(identifier)
        if not isinstance(source.get("url"), str) or not source["url"].startswith("https://"):
            errors.append(f"{label}.url must be HTTPS")
        for field in ("authorship", "license", "redistribution"):
            nonempty_string(source.get(field), f"{label}.{field}", errors)
        if source.get("redistribution") not in {"allowed-with-attribution", "derived-cases-allowed"}:
            errors.append(f"{label}.redistribution is not approved")
        if kind == "repository":
            if not SHA_PATTERN.fullmatch(str(source.get("revision", ""))):
                errors.append(f"{label}.revision must be a full lowercase Git SHA")
            raw_path = source.get("path")
            if not isinstance(raw_path, str) or not raw_path.strip() or Path(raw_path).is_absolute() or ".." in Path(raw_path).parts:
                errors.append(f"{label}.path must be repository-relative")
        else:
            nonempty_string(source.get("version"), f"{label}.version", errors)
            nonempty_string(source.get("section"), f"{label}.section", errors)

    packets = document.get("packets")
    if not isinstance(packets, list) or not packets:
        errors.append("packets must be a non-empty array of tables")
        packets = []
    packet_ids: set[str] = set()
    packet_cases: dict[str, dict[str, dict[str, object]]] = {}
    for index, packet in enumerate(packets, start=1):
        label = f"packets[{index}]"
        if not exact_keys(packet, PACKET_KEYS, label, errors):
            continue
        identifier = packet.get("id")
        if not isinstance(identifier, str) or not ID_PATTERN.fullmatch(identifier):
            errors.append(f"{label}.id is invalid")
            continue
        if identifier in packet_ids:
            errors.append(f"duplicate packet id: {identifier}")
        packet_ids.add(identifier)
        if packet.get("schema_version") != 1:
            errors.append(f"{identifier}.schema_version must equal 1")
        if not isinstance(packet.get("capability"), str) or not ID_PATTERN.fullmatch(packet["capability"]):
            errors.append(f"{identifier}.capability must be a stable dotted ID")
        if packet.get("public_data") is not True:
            errors.append(f"{identifier}.public_data must be true")
        for field in ("capability", "authorship", "license", "redistribution"):
            nonempty_string(packet.get(field), f"{identifier}.{field}", errors)
        if packet.get("license") != "Apache-2.0" or packet.get("redistribution") != "allowed-with-attribution":
            errors.append(f"{identifier} must be Apache-2.0 and redistributable with attribution")
        targets = string_array(packet.get("targets"), f"{identifier}.targets", errors)
        if set(targets) - TARGETS or "rust" not in targets:
            errors.append(f"{identifier}.targets contains unsupported values or omits rust")
        payload = safe_payload(root, packet.get("path"), f"{identifier}.path", errors)
        if not DIGEST_PATTERN.fullmatch(str(packet.get("sha256", ""))):
            errors.append(f"{identifier}.sha256 must be lowercase SHA-256")
        if payload is not None:
            digest = hashlib.sha256(payload.read_bytes()).hexdigest()
            if digest != packet.get("sha256"):
                errors.append(f"{identifier}.sha256 does not match payload")
            packet_cases[identifier] = validate_packet(payload, packet, errors)

    vectors = document.get("vectors")
    if not isinstance(vectors, list) or not vectors:
        errors.append("vectors must be a non-empty array of tables")
        vectors = []
    vector_ids: set[str] = set()
    vector_by_id: dict[str, dict[str, object]] = {}
    referenced_cases: set[tuple[str, str]] = set()
    for index, vector in enumerate(vectors, start=1):
        label = f"vectors[{index}]"
        if not exact_keys(vector, VECTOR_KEYS, label, errors):
            continue
        identifier = vector.get("id")
        if not isinstance(identifier, str) or not ID_PATTERN.fullmatch(identifier):
            errors.append(f"{label}.id is invalid")
            continue
        if identifier in vector_ids:
            errors.append(f"duplicate vector id: {identifier}")
        vector_ids.add(identifier)
        vector_by_id[identifier] = vector
        packet_id = vector.get("packet")
        case_id = vector.get("case")
        if packet_id not in packet_ids or case_id not in packet_cases.get(str(packet_id), {}):
            errors.append(f"{identifier}: packet/case reference does not resolve")
        else:
            key = (str(packet_id), str(case_id))
            if key in referenced_cases:
                errors.append(f"{identifier}: packet case is referenced more than once")
            referenced_cases.add(key)
            case = packet_cases[str(packet_id)][str(case_id)]
            expected = case["expected"]
            case_expected = "success" if expected["kind"] == "success" else expected["errorCode"]
            if vector.get("operation") != case.get("operation") or vector.get("expected") != case_expected:
                errors.append(f"{identifier}: operation/expected differs from packet case")
            packet_record = next(item for item in packets if item.get("id") == packet_id)
            if vector.get("capability") != packet_record.get("capability"):
                errors.append(f"{identifier}: capability differs from packet")
        if vector.get("authority") not in AUTHORITIES:
            errors.append(f"{identifier}.authority is invalid")
        selected_sources = string_array(vector.get("source_ids"), f"{identifier}.source_ids", errors)
        if set(selected_sources) - source_ids:
            errors.append(f"{identifier}.source_ids contains unknown sources")
        if vector.get("authority") == "consumer-regression" and not any(item.startswith("sdk-") for item in selected_sources):
            errors.append(f"{identifier}: consumer-regression requires a pinned consumer source")
        for field in ("capability", "source_locator", "operation", "expected", "profile_scope", "resource_class", "rust_selector", "limitations"):
            nonempty_string(vector.get(field), f"{identifier}.{field}", errors)
        if vector.get("operation") not in OPERATIONS:
            errors.append(f"{identifier}.operation is invalid")
        if vector.get("boundary") not in BOUNDARIES:
            errors.append(f"{identifier}.boundary is invalid")
        targets = string_array(vector.get("targets"), f"{identifier}.targets", errors)
        if set(targets) - TARGETS or "rust" not in targets:
            errors.append(f"{identifier}.targets contains unsupported values or omits rust")
        selectors = string_array(vector.get("language_selectors"), f"{identifier}.language_selectors", errors, allow_empty=True)
        rust_selector = vector.get("rust_selector")
        if not isinstance(rust_selector, str) or not SELECTOR_PATTERN.fullmatch(rust_selector):
            errors.append(f"{identifier}.rust_selector is invalid")
        if any(not SELECTOR_PATTERN.fullmatch(selector) for selector in selectors):
            errors.append(f"{identifier}.language_selectors contains an invalid selector")
        if vector.get("owner_issue") != 420 or not isinstance(vector.get("active"), bool):
            errors.append(f"{identifier}: owner_issue must be 420 and active must be boolean")
        string_array(vector.get("supersedes"), f"{identifier}.supersedes", errors, allow_empty=True)
        if not isinstance(vector.get("replaced_by"), str):
            errors.append(f"{identifier}.replaced_by must be text")
        external = string_array(vector.get("external_records"), f"{identifier}.external_records", errors, allow_empty=True)
        if any(not ID_PATTERN.fullmatch(item) for item in external):
            errors.append(f"{identifier}.external_records contains an invalid stable ID")

    all_cases = {(packet_id, case_id) for packet_id, cases in packet_cases.items() for case_id in cases}
    if all_cases != referenced_cases:
        errors.append(f"packet case coverage differs: unreferenced={sorted(all_cases - referenced_cases)}, extra={sorted(referenced_cases - all_cases)}")
    for identifier, vector in vector_by_id.items():
        for previous in vector.get("supersedes", []):
            if previous not in vector_ids or previous == identifier:
                errors.append(f"{identifier}: supersedes must reference another local vector")
        replacement = vector.get("replaced_by")
        if replacement:
            if replacement not in vector_ids or identifier not in vector_by_id.get(str(replacement), {}).get("supersedes", []):
                errors.append(f"{identifier}: replaced_by must resolve to a vector that supersedes it")
            if vector.get("active") is True:
                errors.append(f"{identifier}: replaced vector cannot remain active")

    def reaches(start: str, current: str, seen: set[str]) -> bool:
        if current in seen:
            return current == start
        return any(reaches(start, item, seen | {current}) for item in vector_by_id.get(current, {}).get("supersedes", []))

    if any(reaches(identifier, identifier, set()) for identifier in vector_ids):
        errors.append("supersession graph contains a cycle")

    if errors:
        for error in errors:
            print(f"cross-language-vectors: {error}", file=sys.stderr)
        print(f"cross-language-vectors: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    print(f"cross-language-vectors: {len(sources)} sources, {len(packets)} packet, and {len(vectors)} vectors passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
