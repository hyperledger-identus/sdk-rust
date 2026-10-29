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
    "train-identity": "complete",
    "api-and-sbom": "complete",
    "documentation": "complete",
    "compiler-target-matrix": "complete",
    "exact-revision-slow": "pending",
    "independent-approval": "pending",
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
    observed = {
        entry.get("id"): entry.get("status")
        for entry in evidence
        if isinstance(entry, dict)
    } if isinstance(evidence, list) else {}
    if observed != EXPECTED_EVIDENCE:
        failures.append(f"{MANIFEST}: evidence matrix must equal {EXPECTED_EVIDENCE!r}")

    slow = manifest.get("last_natural_slow", {})
    if not isinstance(slow, dict) or slow.get("status") != "failed":
        failures.append(f"{MANIFEST}: latest natural slow evidence must remain failed until replaced")
    elif slow.get("head_branch") != "develop" or slow.get("head_sha") != "5da04223c1c4e029cb368870877e56d699d40388":
        failures.append(f"{MANIFEST}: failed natural slow identity drifted")

    downstream = manifest.get("downstream", {})
    if not isinstance(downstream, dict) or downstream.get("semantic_result") != "pass":
        failures.append(f"{MANIFEST}: NeoPRISM semantic result must be explicit")
    elif downstream.get("qos_class") != "diagnostic" or downstream.get("network") != "preprod":
        failures.append(f"{MANIFEST}: downstream QoS/network scope must remain bounded")

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
