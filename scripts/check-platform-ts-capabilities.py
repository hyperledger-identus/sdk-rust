#!/usr/bin/env python3
"""Validate the immutable SDK-TS capability inventory contract."""

from __future__ import annotations

import re
import sys
import tomllib
from collections import Counter
from pathlib import Path


RELATIVE_PATH = Path("docs/architecture/identus-platform-ts-capabilities.toml")
TOP_LEVEL_KEYS = {
    "schema_version",
    "program_issue",
    "snapshot_date",
    "status",
    "source",
    "precedence",
    "vocabulary",
    "capability",
}
CAPABILITY_KEYS = {
    "id",
    "outcome",
    "source_aliases",
    "source_paths",
    "public_surface",
    "dependency_basis",
    "standard_basis",
    "test_authority",
    "test_paths",
    "known_consumers",
    "compatibility_risk",
    "rust_state",
    "target_owner",
    "typescript_owner",
    "disposition",
    "decision_basis",
    "follow_up_issue",
}
REQUIRED_IDS = {
    "crypto.key-operations",
    "crypto.hd-derivation",
    "did.syntax",
    "did.document",
    "credentials.jwt-vc",
    "credentials.sd-jwt",
    "credentials.anoncreds-v1",
    "openid4vc.issuance-wallet",
    "messaging.didcomm-v2",
    "wallet.storage-contracts",
    "agent.runtime-orchestration",
    "platform.typescript-plugin-host",
    "platform.npm-packaging",
    "bindings.wasm-did-values",
}
LEGACY_TARGET_NAMES = ("apollo", "castor", "pollux", "mercury", "pluto", "edgeagent", "edge-agent")
ID_PATTERN = re.compile(r"^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)+$")
SHA_PATTERN = re.compile(r"^[0-9a-f]{40}$")


def strings(value: object, field: str, errors: list[str]) -> list[str]:
    if not isinstance(value, list) or not value or not all(isinstance(item, str) and item.strip() for item in value):
        errors.append(f"{field} must be a non-empty string array")
        return []
    return value


def main() -> int:
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path.cwd().resolve()
    source_path = root / RELATIVE_PATH
    errors: list[str] = []

    try:
        document = tomllib.loads(source_path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        print(f"platform-ts-capabilities: cannot load {RELATIVE_PATH}: {error}", file=sys.stderr)
        return 1

    unknown = set(document) - TOP_LEVEL_KEYS
    missing = TOP_LEVEL_KEYS - set(document)
    if unknown:
        errors.append(f"unknown top-level fields: {sorted(unknown)}")
    if missing:
        errors.append(f"missing top-level fields: {sorted(missing)}")
    if document.get("schema_version") != 1:
        errors.append("schema_version must equal 1")
    if document.get("program_issue") != 417:
        errors.append("program_issue must equal 417")
    if document.get("status") not in {"preliminary", "accepted", "superseded"}:
        errors.append("status must be preliminary, accepted, or superseded")

    source = document.get("source", {})
    if not isinstance(source, dict):
        errors.append("source must be a table")
        source = {}
    if not SHA_PATTERN.fullmatch(str(source.get("revision", ""))):
        errors.append("source.revision must be a full lowercase Git SHA")
    if source.get("package") != "@hyperledger/identus-sdk" or source.get("version") != "8.1.4":
        errors.append("source package/version must remain pinned to @hyperledger/identus-sdk 8.1.4")

    precedence = document.get("precedence", {})
    expected_precedence = [
        "standard-or-security",
        "identus-profile",
        "maintained-rust-upstream",
        "identus-contract-or-consumer",
        "cross-sdk-interoperability",
        "donor-precedent",
    ]
    if not isinstance(precedence, dict) or precedence.get("order") != expected_precedence:
        errors.append("precedence.order must preserve ADR 0169 authority order")
    if isinstance(precedence, dict) and precedence.get("decision_adr") != 169:
        errors.append("precedence.decision_adr must equal 169")

    vocabulary = document.get("vocabulary", {})
    if not isinstance(vocabulary, dict):
        errors.append("vocabulary must be a table")
        vocabulary = {}
    dispositions = set(strings(vocabulary.get("dispositions"), "vocabulary.dispositions", errors))
    authorities = set(strings(vocabulary.get("test_authorities"), "vocabulary.test_authorities", errors))
    risks = set(strings(vocabulary.get("compatibility_risks"), "vocabulary.compatibility_risks", errors))
    states = set(strings(vocabulary.get("rust_states"), "vocabulary.rust_states", errors))

    capabilities = document.get("capability")
    if not isinstance(capabilities, list) or len(capabilities) < 20:
        errors.append("capability must contain at least 20 normalized records")
        capabilities = []

    seen: set[str] = set()
    disposition_counts: Counter[str] = Counter()
    for index, item in enumerate(capabilities, start=1):
        prefix = f"capability[{index}]"
        if not isinstance(item, dict):
            errors.append(f"{prefix} must be a table")
            continue
        item_keys = set(item)
        if item_keys != CAPABILITY_KEYS:
            errors.append(
                f"{prefix} fields differ: missing={sorted(CAPABILITY_KEYS - item_keys)}, "
                f"unknown={sorted(item_keys - CAPABILITY_KEYS)}"
            )

        identifier = item.get("id")
        if not isinstance(identifier, str) or not ID_PATTERN.fullmatch(identifier):
            errors.append(f"{prefix}.id must be a dotted responsibility identifier")
            continue
        if identifier in seen:
            errors.append(f"duplicate capability id: {identifier}")
        seen.add(identifier)

        target = str(item.get("target_owner", "")).lower().replace(" ", "")
        if any(name in identifier or name in target for name in LEGACY_TARGET_NAMES):
            errors.append(f"{identifier}: legacy donor name appears in target identity/owner")

        for field in (
            "source_aliases",
            "source_paths",
            "public_surface",
            "dependency_basis",
            "standard_basis",
            "test_paths",
            "known_consumers",
            "decision_basis",
        ):
            strings(item.get(field), f"{identifier}.{field}", errors)
        if not all(path.startswith(("packages/", "integration-tests/", ".github/")) for path in item.get("source_paths", [])):
            errors.append(f"{identifier}: source_paths must be repository-relative donor paths")

        disposition = item.get("disposition")
        if disposition not in dispositions:
            errors.append(f"{identifier}: invalid disposition {disposition!r}")
        else:
            disposition_counts[disposition] += 1
        if item.get("test_authority") not in authorities:
            errors.append(f"{identifier}: invalid test_authority")
        if item.get("compatibility_risk") not in risks:
            errors.append(f"{identifier}: invalid compatibility_risk")
        if item.get("rust_state") not in states:
            errors.append(f"{identifier}: invalid rust_state")
        if not isinstance(item.get("follow_up_issue"), int) or item["follow_up_issue"] <= 0:
            errors.append(f"{identifier}: follow_up_issue must be a positive issue number")
        for field in ("outcome", "target_owner", "typescript_owner"):
            if not isinstance(item.get(field), str) or not item[field].strip():
                errors.append(f"{identifier}.{field} must be non-empty text")

    missing_ids = REQUIRED_IDS - seen
    if missing_ids:
        errors.append(f"missing required normalized capabilities: {sorted(missing_ids)}")
    for required_disposition in ("move-to-rust", "retain-platform", "replace-upstream", "defer"):
        if disposition_counts[required_disposition] == 0:
            errors.append(f"inventory must exercise disposition {required_disposition}")

    if errors:
        for error in errors:
            print(f"platform-ts-capabilities: {error}", file=sys.stderr)
        return 1

    counts = ", ".join(f"{key}={disposition_counts[key]}" for key in sorted(disposition_counts))
    print(f"platform-ts-capabilities: {len(capabilities)} capabilities passed ({counts})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
