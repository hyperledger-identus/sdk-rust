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
    else:
        if identity.get("staged_lock_sha256") != "1f1d4206e2ced5bd74675d654876684536dd8f82bf79cd6de4c2fa67db904447":
            failures.append(f"{MANIFEST}: staged lock identity drifted")
        if identity.get("merge_revision") != "pending-protected-merge":
            failures.append(f"{MANIFEST}: self-referential merge identity must remain explicitly pending")

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
