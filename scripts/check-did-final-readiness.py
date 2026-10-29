#!/usr/bin/env python3
"""Validate the bounded DID candidate final-readiness review packet."""

from __future__ import annotations

import hashlib
import sys
import tomllib
from pathlib import Path
from typing import Any


MANIFEST = Path("docs/release/did-candidate-readiness.toml")
PAGE = Path("site/src/did-candidate-readiness.md")
EXPECTED_PAGE_SHA256 = "61d47fd322c077c5885990819e348ce643c08e2f47a82517ac6f8d1d2c458919"
EXPECTED_TOP_LEVEL_KEYS = {
    "schema_version",
    "candidate",
    "version",
    "lifecycle",
    "status",
    "coordinator",
    "delivery_issue",
    "packages",
    "limitations",
    "identity",
    "evidence",
    "historical_failed_slow",
    "downstream",
}
EXPECTED_PACKAGES = ["identus-did", "identus-did-resolver-http"]
EXPECTED_LIMITATIONS = [
    "No crates.io publication or registry availability claim.",
    "No stable API, certification, vulnerability-free, or production-readiness claim.",
    "Portable targets are compile-checked only for identus-did; identus-did-resolver-http remains host-only.",
    "NeoPRISM evidence is source-pinned downstream compatibility evidence, not SDK-owned Cardano or runtime support.",
    "Independent approval cannot be self-issued by the implementing agent.",
]
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
    "resolver_sbom_sha256": "fbaa3985eacf2c89649799c1a33effeb2b521aa21c5ec92ff70444a6afb126d6",
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
PAGE_IDENTITY_ROWS = (
    f"| Protected evidence base | `{EXPECTED_IDENTITY['protected_base_revision']}` |",
    f"| Staged lock SHA-256 | `{EXPECTED_IDENTITY['staged_lock_sha256']}` |",
    f"| `identus-did` API SHA-256 | `{EXPECTED_IDENTITY['did_api_sha256']}` |",
    f"| Resolver API SHA-256 | `{EXPECTED_IDENTITY['resolver_api_sha256']}` |",
    "| Candidate archive SHA-256 | "
    f"`{EXPECTED_IDENTITY['did_archive_sha256']}` / "
    f"`{EXPECTED_IDENTITY['resolver_archive_sha256']}` |",
    "| CycloneDX SHA-256 | "
    f"`{EXPECTED_IDENTITY['did_sbom_sha256']}` / "
    f"`{EXPECTED_IDENTITY['resolver_sbom_sha256']}` |",
    f"| Reserved future tag | `{EXPECTED_IDENTITY['reserved_tag']}` |",
)
PAGE_EVIDENCE_ROWS = {
    "train-identity": "| Independent train identity and deterministic archives | Complete | [#382](https://github.com/hyperledger-identus/sdk-rust/issues/382), ADR 0153 |",
    "api-and-sbom": "| Public API origins and normalized CycloneDX | Complete | [PR #584](https://github.com/hyperledger-identus/sdk-rust/pull/584), [factory receipt](https://github.com/hyperledger-identus/sdk-rust/pull/584#issuecomment-6493993716), ADR 0154 |",
    "documentation": "| Engineer handbook and source-adoption guidance | Complete | [#386](https://github.com/hyperledger-identus/sdk-rust/issues/386) |",
    "compiler-target-matrix": "| Linux/macOS primary + MSRV and package-specific portable targets | Complete | [#387](https://github.com/hyperledger-identus/sdk-rust/issues/387), ADR 0155 |",
    "exact-revision-slow": "| Exact protected-revision slow evidence | Pending | [#388](https://github.com/hyperledger-identus/sdk-rust/issues/388) |",
    "independent-approval": "| Independent exact-receipt approval | Pending | [#388](https://github.com/hyperledger-identus/sdk-rust/issues/388) |",
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
    if set(manifest) != EXPECTED_TOP_LEVEL_KEYS:
        failures.append(f"{MANIFEST}: top-level fields must equal the closed readiness schema")
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

    slow = manifest.get("historical_failed_slow", {})
    if slow != EXPECTED_SLOW:
        failures.append(f"{MANIFEST}: natural slow evidence must equal the exact failed receipt")

    downstream = manifest.get("downstream", {})
    if downstream != EXPECTED_DOWNSTREAM:
        failures.append(f"{MANIFEST}: downstream evidence must equal the exact bounded receipt")

    limitations = manifest.get("limitations", [])
    if limitations != EXPECTED_LIMITATIONS:
        failures.append(f"{MANIFEST}: limitations must equal the five bounded non-claims")

    try:
        page = (root / PAGE).read_text(encoding="utf-8")
    except OSError as error:
        failures.append(f"{PAGE}: unable to read: {error}")
        page = ""
    page_sha256 = hashlib.sha256(page.encode("utf-8")).hexdigest()
    if page_sha256 != EXPECTED_PAGE_SHA256:
        failures.append(f"{PAGE}: complete bounded page digest drifted")
    for phrase in REQUIRED_PAGE_TEXT:
        if phrase not in page:
            failures.append(f"{PAGE}: missing required bounded claim: {phrase}")
    for row in PAGE_IDENTITY_ROWS:
        if row not in page:
            failures.append(f"{PAGE}: displayed identity row drifted from the manifest: {row}")
    for evidence_id, row in PAGE_EVIDENCE_ROWS.items():
        if row not in page:
            failures.append(f"{PAGE}: displayed evidence row {evidence_id} drifted from the manifest")
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
