#!/usr/bin/env python3
"""Validate the immutable SDK-TS capability inventory contract."""

from __future__ import annotations

import re
import sys
import tomllib
from collections import Counter
from pathlib import Path


RELATIVE_PATH = Path("docs/architecture/identus-platform-ts-capabilities.toml")
ROADMAP_PATH = Path("docs/roadmap/platform-core-adoption-milestones.toml")
TOP_LEVEL_KEYS = {
    "schema_version",
    "program_issue",
    "correction_issue",
    "snapshot_date",
    "status",
    "source",
    "ownership",
    "quality",
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
    "did.method.peer",
    "messaging.didcomm-v2",
    "messaging.didcomm-application-protocols",
    "wallet.storage-contracts",
    "agent.runtime-orchestration",
    "platform.browser-node-networking",
    "platform.typescript-plugin-host",
    "platform.npm-packaging",
    "bindings.wasm-did",
}
REQUIRED_RUST_OWNED = {
    "did.method.peer",
    "messaging.didcomm-application-protocols",
    "agent.protocol-state",
    "agent.runtime-orchestration",
    "platform.browser-node-networking",
    "bindings.wasm-did",
}
QUALITY_CLASSES = ["property", "fuzz", "benchmark", "differential"]
MILESTONE_IDS = [f"A{index}" for index in range(10)]
MILESTONE_KEYS = {
    "id",
    "title",
    "state",
    "owner_issue",
    "supporting_issues",
    "depends_on",
    "deliverables",
    "exit_evidence",
    "adoption_proof",
    "quality",
    "non_claims",
}
LEGACY_TARGET_NAMES = ("apollo", "castor", "pollux", "mercury", "pluto", "edgeagent", "edge-agent")
ID_PATTERN = re.compile(r"^[a-z][a-z0-9-]*(?:\.[a-z][a-z0-9-]*)+$")
SHA_PATTERN = re.compile(r"^[0-9a-f]{40}$")


def strings(value: object, field: str, errors: list[str]) -> list[str]:
    if not isinstance(value, list) or not value or not all(isinstance(item, str) and item.strip() for item in value):
        errors.append(f"{field} must be a non-empty string array")
        return []
    return value


def validate_roadmap(root: Path, errors: list[str]) -> int:
    roadmap_path = root / ROADMAP_PATH
    try:
        roadmap = tomllib.loads(roadmap_path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        errors.append(f"cannot load {ROADMAP_PATH}: {error}")
        return 0

    expected_keys = {
        "schema_version",
        "program_issue",
        "status",
        "ordering",
        "quality_issue",
        "quality_classes",
        "milestone",
    }
    if set(roadmap) != expected_keys:
        errors.append(
            "roadmap top-level fields differ: "
            f"missing={sorted(expected_keys - set(roadmap))}, "
            f"unknown={sorted(set(roadmap) - expected_keys)}"
        )
    if roadmap.get("schema_version") != 1:
        errors.append("roadmap schema_version must equal 1")
    if roadmap.get("program_issue") != 497:
        errors.append("roadmap program_issue must equal 497")
    if roadmap.get("status") != "target" or roadmap.get("ordering") != "dependency":
        errors.append("roadmap must be a dependency-ordered target")
    if roadmap.get("quality_issue") != 501:
        errors.append("roadmap quality_issue must equal 501")
    if roadmap.get("quality_classes") != QUALITY_CLASSES:
        errors.append(f"roadmap quality_classes must equal {QUALITY_CLASSES}")

    milestones = roadmap.get("milestone")
    if not isinstance(milestones, list):
        errors.append("roadmap milestone must be an array of tables")
        return 0
    identifiers = [item.get("id") for item in milestones if isinstance(item, dict)]
    if identifiers != MILESTONE_IDS:
        errors.append(f"roadmap milestone order must equal {MILESTONE_IDS}")

    preceding: set[str] = set()
    for index, item in enumerate(milestones, start=1):
        prefix = f"roadmap.milestone[{index}]"
        if not isinstance(item, dict):
            errors.append(f"{prefix} must be a table")
            continue
        if set(item) != MILESTONE_KEYS:
            errors.append(
                f"{prefix} fields differ: missing={sorted(MILESTONE_KEYS - set(item))}, "
                f"unknown={sorted(set(item) - MILESTONE_KEYS)}"
            )
        identifier = item.get("id")
        if item.get("state") not in {"planned", "in-progress", "complete"}:
            errors.append(f"{identifier}: invalid milestone state")
        if not isinstance(item.get("owner_issue"), int) or item["owner_issue"] <= 0:
            errors.append(f"{identifier}: owner_issue must be positive")
        supporting = item.get("supporting_issues")
        if not isinstance(supporting, list) or not all(
            isinstance(issue, int) and issue > 0 for issue in supporting
        ):
            errors.append(f"{identifier}: supporting_issues must contain positive issue numbers")
        dependencies = item.get("depends_on")
        if not isinstance(dependencies, list) or not all(
            isinstance(dependency, str) for dependency in dependencies
        ):
            errors.append(f"{identifier}: depends_on must be a string array")
        elif any(dependency not in preceding for dependency in dependencies):
            errors.append(f"{identifier}: dependencies must reference preceding milestones only")
        for field in ("deliverables", "exit_evidence", "non_claims"):
            strings(item.get(field), f"{identifier}.{field}", errors)
        if item.get("quality") != QUALITY_CLASSES:
            errors.append(f"{identifier}: quality must include all required evidence classes")
        if not isinstance(item.get("adoption_proof"), str) or not item["adoption_proof"].strip():
            errors.append(f"{identifier}: adoption_proof must be non-empty text")
        if isinstance(identifier, str):
            preceding.add(identifier)
    return len(milestones)


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
    if document.get("schema_version") != 2:
        errors.append("schema_version must equal 2")
    if document.get("program_issue") != 417:
        errors.append("program_issue must equal 417")
    if document.get("correction_issue") != 497:
        errors.append("correction_issue must equal 497")
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

    expected_ownership = {
        "canonical_contract": "sdk-rust",
        "language_adapter_policy": "versioned-compatibility-only",
        "private_workspace_policy": "port-or-qualified-rust-dependency",
        "did_domain_owner": "sdk-rust",
    }
    if document.get("ownership") != expected_ownership:
        errors.append("ownership must preserve the corrected Rust-canonical contract")

    quality = document.get("quality", {})
    if not isinstance(quality, dict):
        errors.append("quality must be a table")
    elif quality != {
        "required_classes": QUALITY_CLASSES,
        "authority": "engineering-not-normative",
        "policy_issue": 501,
    }:
        errors.append("quality must require the four engineering evidence classes under issue 501")

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
    by_id = {
        item.get("id"): item
        for item in capabilities
        if isinstance(item, dict) and isinstance(item.get("id"), str)
    }
    for identifier in sorted(REQUIRED_RUST_OWNED):
        if by_id.get(identifier, {}).get("disposition") != "move-to-rust":
            errors.append(f"{identifier}: corrected disposition must be move-to-rust")

    milestone_count = validate_roadmap(root, errors)

    if errors:
        for error in errors:
            print(f"platform-ts-capabilities: {error}", file=sys.stderr)
        return 1

    counts = ", ".join(f"{key}={disposition_counts[key]}" for key in sorted(disposition_counts))
    print(
        f"platform-ts-capabilities: {len(capabilities)} capabilities and "
        f"{milestone_count} adoption milestones passed ({counts})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
