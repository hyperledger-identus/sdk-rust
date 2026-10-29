#!/usr/bin/env python3
"""Validate the bounded DID candidate final-readiness review packet."""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path
from typing import Any


MANIFEST = Path("docs/release/did-candidate-readiness.toml")
PAGE = Path("site/src/did-candidate-readiness.md")
EXPECTED_PACKAGES = ["identus-did", "identus-did-resolver-http"]
EXPECTED_IDENTITY = {
    "descriptor": "docs/release/did-candidate.toml",
    "train_registry": "docs/release/release-trains.toml",
    "staged_lock": "docs/release/did-candidate.lock",
    "staged_lock_sha256": "f7535a6819c9b9e4474c95b427b54646d46ccac51703c680086ef75c790fba4f",
    "reserved_tag": "identus-did-v0.1.0-rc.1",
    "protected_base_revision": "34420235103fe9dc599aa061047525c459d422cf",
    "merge_revision": "pending-protected-merge",
    "did_api_sha256": "5c985cd2d3f26c07230ace1086d7a178662a4f583d0086574ac9baa6a00be639",
    "resolver_api_sha256": "9c432f0d0c3d5db92da3aa88aed05164aa2adf483c11be84b9239c9973179df6",
    "did_archive_sha256": "7736e445ffce327fae19a55c7b8b7adf38887f93f56aba4a9a804d18d765fb68",
    "resolver_archive_sha256": "90366988dd00a4730d398431531d0066eb109ff48a1fcc8f0aed4d9673678480",
    "did_sbom_sha256": "d0e0976fa6d96918d59202cd173c4162459f19b253120c41fce3058e5c59d109",
    "resolver_sbom_sha256": "fbaa3985eacf2c89649799c1a33effeb2b521aa21c5ec92ff7044a6afb126d6",
    "api_origin_receipt": "docs/release/did-api-origin-rebaseline.toml",
}
EXPECTED_EVIDENCE = {
    "train-identity": {
        "status": "complete",
        "uri": "https://github.com/hyperledger-identus/sdk-rust/issues/382",
    },
    "api-and-sbom": {
        "status": "complete",
        "uri": "https://github.com/hyperledger-identus/sdk-rust/pull/584#issuecomment-6493993716",
    },
    "documentation": {
        "status": "complete",
        "uri": "https://github.com/hyperledger-identus/sdk-rust/issues/386",
    },
    "compiler-target-matrix": {
        "status": "complete",
        "uri": "https://github.com/hyperledger-identus/sdk-rust/issues/387",
    },
    "exact-revision-slow": {
        "status": "pending",
        "uri": "https://github.com/hyperledger-identus/sdk-rust/issues/388",
    },
    "independent-approval": {
        "status": "pending",
        "uri": "https://github.com/hyperledger-identus/sdk-rust/issues/388",
    },
}
EXPECTED_SLOW = {
    "status": "failed",
    "head_branch": "develop",
    "head_sha": "5da04223c1c4e029cb368870877e56d699d40388",
    "run_uri": "https://github.com/hyperledger-identus/sdk-rust/actions/runs/40260649503",
    "failing_lane": "slow (macos-latest)",
    "cause": "RUSTSEC-2026-0374 classified build-time basic-toml 0.1.10 as unmaintained",
    "resolution": "Issue #564 and ADR 0191 added a narrow exact-path acceptance with machine-checked expiry; a fresh complete run is still required.",
}
EXPECTED_DOWNSTREAM = {
    "consumer": "hyperledger-identus/neoprism",
    "adoption_pr": "https://github.com/hyperledger-identus/neoprism/pull/324",
    "parity_issue": "https://github.com/hyperledger-identus/neoprism/issues/325",
    "parity_harness_pr": "https://github.com/hyperledger-identus/neoprism/pull/327",
    "final_candidate_summary": "https://github.com/hyperledger-identus/neoprism/issues/325#issuecomment-6441653199",
    "latest_composition_receipt": "https://github.com/hyperledger-identus/neoprism/issues/325#issuecomment-6487638362",
    "network": "preprod",
    "comparison_slot": 10723321,
    "semantic_result": "pass",
    "qos_class": "diagnostic",
}
PAGE_IDENTITY_FIELDS = (
    "staged_lock_sha256",
    "reserved_tag",
    "protected_base_revision",
    "did_api_sha256",
    "resolver_api_sha256",
    "did_archive_sha256",
    "resolver_archive_sha256",
    "did_sbom_sha256",
    "resolver_sbom_sha256",
)
PAGE_EVIDENCE_LABELS = {
    "train-identity": "Independent train identity and deterministic archives",
    "api-and-sbom": "Public API origins and normalized CycloneDX",
    "documentation": "Engineer handbook and source-adoption guidance",
    "compiler-target-matrix": "Linux/macOS primary + MSRV and package-specific portable targets",
    "exact-revision-slow": "Exact protected-revision slow evidence",
    "independent-approval": "Independent exact-receipt approval",
}
REQUIRED_PAGE_TEXT = (
    "pending an exact-protected-revision slow run and independent",
    "https://github.com/hyperledger-identus/sdk-rust/actions/runs/40260649503",
    "https://github.com/hyperledger-identus/sdk-rust/pull/584#issuecomment-6493993716",
    "https://github.com/hyperledger-identus/neoprism/pull/324",
    "https://github.com/hyperledger-identus/neoprism/pull/327",
    "https://github.com/hyperledger-identus/neoprism/issues/325#issuecomment-6487638362",
    "diagnostic duration, cursor-rate, CPU, RAM, and process",
    "does not claim crates.io publication",
)


def table(path: Path, failures: list[str]) -> dict[str, Any]:
    try:
        value = tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        failures.append(f"{path}: unable to read TOML: {error}")
        return {}
    if not isinstance(value, dict):
        failures.append(f"{path}: root must be a table")
        return {}
    return value


def check(root: Path) -> list[str]:
    failures: list[str] = []
    manifest = table(root / MANIFEST, failures)
    expected = {
        "schema_version": 1,
        "candidate": "identus-did-0.1.0-rc.1",
        "version": "0.1.0-rc.1",
        "lifecycle": "candidate-only",
        "status": "pending-exact-slow-and-independent-approval",
        "packages": EXPECTED_PACKAGES,
    }
    for key, value in expected.items():
        if manifest.get(key) != value:
            failures.append(f"{MANIFEST}: {key} must be {value!r}")

    identity = manifest.get("identity", {})
    if not isinstance(identity, dict):
        failures.append(f"{MANIFEST}: identity must be a table")
    elif identity != EXPECTED_IDENTITY:
        failures.append(f"{MANIFEST}: identity must equal the exact protected candidate receipt")

    evidence = manifest.get("evidence", [])
    observed: dict[str, dict[str, Any]] = {}
    if not isinstance(evidence, list):
        failures.append(f"{MANIFEST}: evidence must be an array of tables")
    else:
        for entry in evidence:
            if not isinstance(entry, dict) or set(entry) != {"id", "status", "uri"}:
                failures.append(f"{MANIFEST}: each evidence entry must contain only id, status, and uri")
                continue
            evidence_id = entry.get("id")
            if not isinstance(evidence_id, str):
                failures.append(f"{MANIFEST}: evidence id must be a string")
                continue
            if evidence_id in observed:
                failures.append(f"{MANIFEST}: duplicate evidence id: {evidence_id}")
                continue
            observed[evidence_id] = {
                "status": entry.get("status"),
                "uri": entry.get("uri"),
            }
    if observed != EXPECTED_EVIDENCE:
        failures.append(f"{MANIFEST}: evidence matrix must equal {EXPECTED_EVIDENCE!r}")

    slow = manifest.get("last_natural_slow", {})
    if slow != EXPECTED_SLOW:
        failures.append(f"{MANIFEST}: natural slow evidence must equal the exact failed receipt")

    downstream = manifest.get("downstream", {})
    if downstream != EXPECTED_DOWNSTREAM:
        failures.append(f"{MANIFEST}: downstream evidence must equal the exact bounded receipt")

    limitations = manifest.get("limitations", [])
    if not isinstance(limitations, list) or len(limitations) != 5:
        failures.append(f"{MANIFEST}: exactly five explicit limitations are required")

    try:
        page = (root / PAGE).read_text(encoding="utf-8")
    except OSError as error:
        failures.append(f"{PAGE}: unable to read: {error}")
        page = ""
    for phrase in REQUIRED_PAGE_TEXT:
        if phrase not in page:
            failures.append(f"{PAGE}: missing required bounded claim: {phrase}")
    for field in PAGE_IDENTITY_FIELDS:
        value = identity.get(field) if isinstance(identity, dict) else None
        if not isinstance(value, str) or value not in page:
            failures.append(f"{PAGE}: displayed identity {field} drifted from the manifest")
    for evidence_id, expected_entry in EXPECTED_EVIDENCE.items():
        label = PAGE_EVIDENCE_LABELS[evidence_id]
        status = expected_entry["status"].title()
        if f"| {label} | {status} |" not in page:
            failures.append(f"{PAGE}: displayed evidence status {evidence_id} drifted from the manifest")
        if expected_entry["uri"] not in page:
            failures.append(f"{PAGE}: displayed evidence URI {evidence_id} drifted from the manifest")
    forbidden = ("is production ready", "is published on crates.io", "all advisories are waived")
    for phrase in forbidden:
        if phrase in page.lower():
            failures.append(f"{PAGE}: forbidden overclaim: {phrase}")
    return failures


def main() -> int:
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path.cwd()
    failures = check(root)
    for failure in failures:
        print(f"did-final-readiness: {failure}", file=sys.stderr)
    if failures:
        print(f"did-final-readiness: {len(failures)} failure(s)", file=sys.stderr)
        return 1
    print("did-final-readiness: pending packet contract passed (6 evidence classes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
