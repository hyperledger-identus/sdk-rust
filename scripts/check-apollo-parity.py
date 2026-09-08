#!/usr/bin/env python3
"""Validate and render the executable Apollo cryptography parity manifest."""

from __future__ import annotations

import argparse
import re
import sys
import tomllib
from collections import Counter
from pathlib import Path, PurePosixPath
from typing import Any

MANIFEST = Path("docs/architecture/apollo-crypto-parity.toml")
SUPPORT_POLICY = Path("docs/architecture/sdk-support-policy.toml")
SHA = re.compile(r"^[0-9a-f]{40}$")
ID = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
DISPOSITIONS = {"parity", "sdk-exceeds", "accepted-difference", "gap"}
APOLLO_REPOSITORY = "hyperledger-identus/apollo"
APOLLO_REVISION = "ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c"
SDK_REPOSITORY = "hyperledger-identus/sdk-rust"
CAPABILITY_IDS = {
    "entropy-csprng", "encoding-hex", "encoding-base64url-nopad",
    "encoding-base64-other-profiles", "sha2-digests", "hmac-sha512",
    "pbkdf2-generic", "bip39-english", "bip39-alternate-wordlists",
    "derivation-path", "secp256k1-bip32-hardened",
    "secp256k1-bip32-non-hardened", "cardano-v2-private",
    "cardano-v2-public", "slip0010-ed25519", "secp256k1-lifecycle",
    "secp256k1-private-tweak", "ed25519-lifecycle",
    "ed25519-x25519-conversion", "x25519-key-generation",
    "x25519-diffie-hellman", "p256-lifecycle", "public-jwk",
    "public-cose-key", "platform-distribution", "platform-scaffolding",
    "bip340-schnorr-comment",
}
VECTOR_IDS = {
    "apollo-secure-random", "apollo-rfc4648-base16", "apollo-base64url",
    "apollo-bip39-kmp", "sdk-bip39-published", "sdk-derivation-path",
    "apollo-secp256k1-hd", "sdk-bip32-published",
    "apollo-cardano-v2-private", "apollo-cardano-v2-public",
    "sdk-slip0010-published", "apollo-secp256k1-lifecycle",
    "apollo-secp256k1-prism-50", "apollo-ed25519-lifecycle",
    "sdk-ed25519-x25519-conversion", "apollo-x25519-lifecycle",
    "sdk-x25519-dh", "sdk-p256-lifecycle", "sdk-rfc8037-jwk",
    "sdk-cose-key", "sdk-sha2-known-vectors",
    "sdk-hmac-sha512-rfc4231",
}
DEPENDENCY_IDS = {
    "secp256k1", "ed25519", "x25519", "cardano-v2", "bip39",
    "sha-hmac", "p256", "encodings", "public-serialization",
    "secret-handling", "platform-plumbing",
}
TARGET_IDS = {
    "linux-host", "wasm32-unknown-unknown", "aarch64-apple-ios",
    "aarch64-linux-android", "language-bindings",
}
PORTABLE_TARGET_IDS = {
    "wasm32-unknown-unknown", "aarch64-apple-ios",
    "aarch64-linux-android",
}
CAPABILITY_KEYS = {
    "id", "category", "name", "disposition", "apollo_surface",
    "apollo_source_uri", "sdk_surface", "sdk_test_uri", "feature",
    "vector_ids", "decision_receipt", "ci_receipt", "rationale",
    "consumer_impact", "reopen_trigger", "tracking_issue",
}
VECTOR_KEYS = {
    "id", "family", "kind", "origin_repository", "origin_revision",
    "origin_path", "origin_uri", "license", "transformation",
    "expected_result", "sdk_test_path", "sdk_test_selectors",
    "sdk_test_uri",
}


class Validation:
    def __init__(self, root: Path) -> None:
        self.root = root
        self.errors: list[str] = []

    def fail(self, message: str) -> None:
        self.errors.append(message)

    def exact_keys(self, value: dict[str, Any], expected: set[str], label: str) -> None:
        missing = expected - value.keys()
        extra = value.keys() - expected
        if missing:
            self.fail(f"{label}: missing fields: {', '.join(sorted(missing))}")
        if extra:
            self.fail(f"{label}: unknown fields: {', '.join(sorted(extra))}")

    def identifiers(self, rows: list[dict[str, Any]], label: str) -> list[str]:
        ids: list[str] = []
        for index, row in enumerate(rows):
            value = row.get("id")
            if not isinstance(value, str) or not ID.fullmatch(value):
                self.fail(f"{label}[{index}]: invalid id")
                continue
            ids.append(value)
        duplicates = sorted(key for key, count in Counter(ids).items() if count > 1)
        if duplicates:
            self.fail(f"{label}: duplicate ids: {', '.join(duplicates)}")
        return ids

    def text_fields(self, row: dict[str, Any], fields: set[str], label: str) -> None:
        for field in fields:
            if not isinstance(row.get(field), str) or not row[field].strip():
                self.fail(f"{label}: {field} must be a non-empty string")

    def safe_path(self, value: Any, label: str) -> Path | None:
        if not isinstance(value, str) or not value:
            self.fail(f"{label}: path must be a non-empty string")
            return None
        path = PurePosixPath(value)
        if path.is_absolute() or ".." in path.parts or str(path) != value:
            self.fail(f"{label}: unsafe repository-relative path: {value}")
            return None
        candidate = self.root / value
        try:
            candidate.resolve(strict=False).relative_to(self.root.resolve())
        except ValueError:
            self.fail(f"{label}: path resolves outside the repository: {value}")
            return None
        return candidate


def load(root: Path) -> dict[str, Any]:
    return tomllib.loads((root / MANIFEST).read_text(encoding="utf-8"))


def immutable_blob(uri: str, repository: str, revision: str, path: str) -> bool:
    expected = f"https://github.com/{repository}/blob/{revision}/{path}"
    return uri == expected


def validate(root: Path, data: dict[str, Any]) -> list[str]:
    check = Validation(root)
    check.exact_keys(data, {
        "schema_version", "title", "assessment_date", "parent_issue",
        "delivery_issue", "report_discussion", "baselines", "summary",
        "performance", "target_evidence", "capabilities", "vectors",
        "dependencies", "targets",
    }, "manifest")
    if data.get("schema_version") != 1:
        check.fail("manifest: schema_version must be 1")
    check.text_fields(data, {
        "title", "assessment_date", "parent_issue", "delivery_issue",
        "report_discussion",
    }, "manifest")

    baselines = data.get("baselines", {})
    if not isinstance(baselines, dict):
        check.fail("baselines: expected table")
        baselines = {}
    baseline_keys = {
        "apollo_repository", "apollo_revision", "apollo_license",
        "apollo_tree_uri", "sdk_repository", "sdk_revision", "sdk_license",
        "sdk_tree_uri", "sdk_fast_ci_uri", "sdk_fast_ci_revision",
        "sdk_fast_ci_conclusion",
    }
    check.exact_keys(baselines, baseline_keys, "baselines")
    check.text_fields(baselines, baseline_keys, "baselines")
    for key in ("apollo_revision", "sdk_revision", "sdk_fast_ci_revision"):
        if not SHA.fullmatch(str(baselines.get(key, ""))):
            check.fail(f"baselines: {key} must be a full lowercase Git SHA")
    if baselines.get("apollo_repository") != APOLLO_REPOSITORY:
        check.fail("baselines: Apollo repository differs from the audited source")
    if baselines.get("apollo_revision") != APOLLO_REVISION:
        check.fail("baselines: Apollo revision differs from the audited source")
    if baselines.get("sdk_repository") != SDK_REPOSITORY:
        check.fail("baselines: SDK repository differs from this project")
    for project in ("apollo", "sdk"):
        repository = baselines.get(f"{project}_repository", "")
        revision = baselines.get(f"{project}_revision", "")
        expected = f"https://github.com/{repository}/tree/{revision}"
        if baselines.get(f"{project}_tree_uri") != expected:
            check.fail(f"baselines: {project}_tree_uri does not bind the baseline")
    if baselines.get("sdk_fast_ci_revision") != baselines.get("sdk_revision"):
        check.fail("baselines: fast CI revision must equal SDK baseline")
    if baselines.get("sdk_fast_ci_conclusion") != "success":
        check.fail("baselines: fast CI conclusion must be success")

    performance = data.get("performance", {})
    performance_keys = {
        "status", "apollo_comparison", "apollo_evidence_uri",
        "sdk_harness_path", "sdk_runner_path", "sdk_harness_uri",
        "sample_minimum", "statistics", "threshold_policy", "workflow_path",
        "artifact_receipt", "tracking_issue", "limitations",
    }
    if not isinstance(performance, dict):
        check.fail("performance: expected table")
        performance = {}
    check.exact_keys(performance, performance_keys, "performance")
    check.text_fields(
        performance,
        performance_keys - {"sample_minimum", "statistics"},
        "performance",
    )
    if performance.get("status") != "sdk-baseline-only":
        check.fail("performance: status must be sdk-baseline-only")
    if performance.get("apollo_comparison") != "unavailable":
        check.fail("performance: Apollo comparison must be unavailable")
    if performance.get("apollo_evidence_uri") != baselines.get("apollo_tree_uri"):
        check.fail("performance: Apollo evidence must bind the audited tree")
    if performance.get("sample_minimum") != 20:
        check.fail("performance: sample_minimum must be 20")
    if performance.get("statistics") != ["p50", "p95", "min", "max"]:
        check.fail("performance: statistics must be p50, p95, min and max")
    if performance.get("threshold_policy") != "measurement-only":
        check.fail("performance: threshold policy must remain measurement-only")
    if performance.get("tracking_issue") != "https://github.com/hyperledger-identus/sdk-rust/issues/214":
        check.fail("performance: tracking issue must be #214")
    if not re.fullmatch(
        r"https://github\.com/hyperledger-identus/sdk-rust/actions/runs/[0-9]+",
        str(performance.get("artifact_receipt", "")),
    ):
        check.fail("performance: artifact receipt must be a GitHub Actions run")
    for field in ("sdk_harness_path", "sdk_runner_path", "workflow_path"):
        path = check.safe_path(performance.get(field), f"performance {field}")
        if path is not None and not path.is_file():
            check.fail(f"performance: file does not exist: {performance.get(field)}")
    harness_path = str(performance.get("sdk_harness_path", ""))
    harness_uri = str(performance.get("sdk_harness_uri", ""))
    expected_prefix = f"https://github.com/{SDK_REPOSITORY}/blob/"
    if not harness_uri.startswith(expected_prefix) or not harness_uri.endswith(f"/{harness_path}"):
        check.fail("performance: harness URI does not bind its repository path")
    else:
        harness_revision = harness_uri[len(expected_prefix):].split("/", 1)[0]
        if not SHA.fullmatch(harness_revision):
            check.fail("performance: harness URI is not commit-pinned")

    target_evidence = data.get("target_evidence", {})
    target_evidence_keys = {
        "status", "sdk_revision", "toolchain", "support_policy_path",
        "workflow_path", "ci_run_uri", "ci_conclusion", "targets",
        "packages", "features", "no_default_features", "limitations",
        "tracking_issue", "discussion_receipt",
    }
    if not isinstance(target_evidence, dict):
        check.fail("target_evidence: expected table")
        target_evidence = {}
    check.exact_keys(target_evidence, target_evidence_keys, "target_evidence")
    check.text_fields(
        target_evidence,
        target_evidence_keys
        - {"targets", "packages", "features", "no_default_features"},
        "target_evidence",
    )
    if target_evidence.get("status") != "complete":
        check.fail("target_evidence: status must be complete")
    if not SHA.fullmatch(str(target_evidence.get("sdk_revision", ""))):
        check.fail("target_evidence: sdk_revision must be a full lowercase Git SHA")
    if target_evidence.get("ci_conclusion") != "success":
        check.fail("target_evidence: CI conclusion must be success")
    if not re.fullmatch(
        r"https://github\.com/hyperledger-identus/sdk-rust/actions/runs/[0-9]+",
        str(target_evidence.get("ci_run_uri", "")),
    ):
        check.fail("target_evidence: CI receipt must be a GitHub Actions run")
    if target_evidence.get("workflow_path") != ".github/workflows/nix-checks.yml":
        check.fail("target_evidence: workflow must be the slow workflow")
    if target_evidence.get("tracking_issue") != "https://github.com/hyperledger-identus/sdk-rust/issues/213":
        check.fail("target_evidence: tracking issue must be #213")
    if not re.fullmatch(
        r"https://github\.com/hyperledger-identus/sdk-rust/discussions/178#discussioncomment-[0-9]+",
        str(target_evidence.get("discussion_receipt", "")),
    ):
        check.fail("target_evidence: discussion receipt must be a Discussion #178 comment")
    evidence_arrays: dict[str, list[str]] = {}
    for field in ("targets", "packages", "features"):
        values = target_evidence.get(field)
        if not isinstance(values, list) or not all(isinstance(item, str) and item for item in values):
            check.fail(f"target_evidence: {field} must be a non-empty string array")
            evidence_arrays[field] = []
        else:
            evidence_arrays[field] = values
    if len(evidence_arrays["targets"]) != len(PORTABLE_TARGET_IDS) or set(evidence_arrays["targets"]) != PORTABLE_TARGET_IDS:
        check.fail("target_evidence: targets must be exactly the portable compile targets")
    if target_evidence.get("no_default_features") is not False:
        check.fail("target_evidence: no_default_features must be false")

    policy_path = check.safe_path(
        target_evidence.get("support_policy_path"),
        "target_evidence support_policy_path",
    )
    if target_evidence.get("support_policy_path") != str(SUPPORT_POLICY):
        check.fail("target_evidence: support policy path must name the canonical policy")
    workflow_path = check.safe_path(
        target_evidence.get("workflow_path"),
        "target_evidence workflow_path",
    )
    if workflow_path is not None and not workflow_path.is_file():
        check.fail("target_evidence: workflow file does not exist")
    policy: dict[str, Any] = {}
    if policy_path is not None:
        if not policy_path.is_file():
            check.fail("target_evidence: support policy file does not exist")
        else:
            try:
                policy = tomllib.loads(policy_path.read_text(encoding="utf-8"))
            except (OSError, tomllib.TOMLDecodeError) as error:
                check.fail(f"target_evidence: cannot load support policy: {error}")
    raw_policy_targets = policy.get("targets", [])
    if not isinstance(raw_policy_targets, list):
        check.fail("target_evidence: support policy targets must be an array")
        raw_policy_targets = []
    policy_targets = {
        row.get("triple"): row
        for row in raw_policy_targets
        if isinstance(row, dict) and row.get("triple") in PORTABLE_TARGET_IDS
    }
    if set(policy_targets) != PORTABLE_TARGET_IDS:
        check.fail("target_evidence: support policy lacks the exact portable targets")
    toolchains = policy.get("toolchains", {})
    if not isinstance(toolchains, dict) or target_evidence.get("toolchain") != toolchains.get("primary"):
        check.fail("target_evidence: toolchain differs from support-policy primary")
    portable_shapes = {
        (
            tuple(row.get("packages", [])),
            tuple(row.get("features", [])),
            row.get("no_default_features"),
        )
        for row in policy_targets.values()
    }
    if len(portable_shapes) != 1:
        check.fail("target_evidence: portable support-policy package/feature shapes differ")
    elif portable_shapes:
        packages, features, no_default_features = next(iter(portable_shapes))
        if target_evidence.get("packages") != list(packages):
            check.fail("target_evidence: packages differ from support policy")
        if target_evidence.get("features") != list(features):
            check.fail("target_evidence: features differ from support policy")
        if target_evidence.get("no_default_features") != no_default_features:
            check.fail("target_evidence: default-feature mode differs from support policy")

    capabilities = data.get("capabilities", [])
    vectors = data.get("vectors", [])
    dependencies = data.get("dependencies", [])
    targets = data.get("targets", [])
    for name, rows in (
        ("capabilities", capabilities),
        ("vectors", vectors),
        ("dependencies", dependencies),
        ("targets", targets),
    ):
        if not isinstance(rows, list) or not all(isinstance(row, dict) for row in rows):
            check.fail(f"{name}: expected array of tables")
    if check.errors:
        return check.errors

    capability_ids = check.identifiers(capabilities, "capabilities")
    vector_ids = check.identifiers(vectors, "vectors")
    dependency_ids = check.identifiers(dependencies, "dependencies")
    target_ids = check.identifiers(targets, "targets")
    for actual, expected, label in (
        (set(capability_ids), CAPABILITY_IDS, "capabilities"),
        (set(vector_ids), VECTOR_IDS, "vectors"),
        (set(dependency_ids), DEPENDENCY_IDS, "dependencies"),
        (set(target_ids), TARGET_IDS, "targets"),
    ):
        if actual != expected:
            check.fail(f"{label}: inventory differs; missing={sorted(expected-actual)}, extra={sorted(actual-expected)}")

    known_vectors = set(vector_ids)
    referenced_vectors: set[str] = set()
    counts = Counter()
    for row in capabilities:
        label = f"capability {row.get('id', '?')}"
        check.exact_keys(row, CAPABILITY_KEYS, label)
        check.text_fields(row, CAPABILITY_KEYS - {"vector_ids"}, label)
        disposition = row.get("disposition")
        if disposition not in DISPOSITIONS:
            check.fail(f"{label}: unsupported disposition: {disposition}")
        else:
            counts[disposition] += 1
        refs = row.get("vector_ids")
        if not isinstance(refs, list) or not all(isinstance(item, str) for item in refs):
            check.fail(f"{label}: vector_ids must be a string array")
            refs = []
        missing = set(refs) - known_vectors
        if missing:
            check.fail(f"{label}: unknown vector ids: {', '.join(sorted(missing))}")
        referenced_vectors.update(refs)
        if disposition in {"parity", "sdk-exceeds"} and not refs:
            check.fail(f"{label}: parity claims require vector evidence")
        apollo_uri = str(row.get("apollo_source_uri", ""))
        if apollo_uri == "not-applicable":
            if disposition != "sdk-exceeds":
                check.fail(f"{label}: Apollo evidence may be absent only for sdk-exceeds")
        elif baselines.get("apollo_revision", "") not in apollo_uri:
            check.fail(f"{label}: Apollo evidence is not baseline-pinned")
        sdk_uri = str(row.get("sdk_test_uri", ""))
        if disposition in {"parity", "sdk-exceeds"}:
            pattern = rf"^https://github.com/{re.escape(str(baselines.get('sdk_repository', '')))}/blob/[0-9a-f]{{40}}/"
            if not re.match(pattern, sdk_uri):
                check.fail(f"{label}: SDK evidence is not commit-pinned")
            if row.get("ci_receipt") != baselines.get("sdk_fast_ci_uri"):
                check.fail(f"{label}: CI receipt differs from the green baseline")

    summary = data.get("summary", {})
    expected_summary = {
        "total": len(capabilities), "parity": counts["parity"],
        "sdk_exceeds": counts["sdk-exceeds"],
        "accepted_difference": counts["accepted-difference"],
        "gap": counts["gap"],
    }
    if summary != expected_summary:
        check.fail(f"summary: expected {expected_summary}, got {summary}")
    unused = known_vectors - referenced_vectors
    if unused:
        check.fail(f"vectors: unreferenced evidence: {', '.join(sorted(unused))}")

    allowed_kinds = {
        "apollo-owned", "normative-capture", "sdk-negative",
        "sdk-known-answer", "sdk-behavior",
    }
    for row in vectors:
        label = f"vector {row.get('id', '?')}"
        check.exact_keys(row, VECTOR_KEYS, label)
        check.text_fields(row, VECTOR_KEYS - {"sdk_test_selectors"}, label)
        revision = str(row.get("origin_revision", ""))
        if not SHA.fullmatch(revision):
            check.fail(f"{label}: origin_revision must be a full lowercase Git SHA")
        if row.get("kind") not in allowed_kinds:
            check.fail(f"{label}: unsupported kind: {row.get('kind')}")
        if row.get("license") != "Apache-2.0":
            check.fail(f"{label}: license must be Apache-2.0")
        origin_path = str(row.get("origin_path", ""))
        check.safe_path(origin_path, f"{label} origin")
        if not immutable_blob(str(row.get("origin_uri", "")), str(row.get("origin_repository", "")), revision, origin_path):
            check.fail(f"{label}: origin_uri does not bind repository, revision and path")
        test_path_value = row.get("sdk_test_path")
        test_path = check.safe_path(test_path_value, f"{label} SDK test")
        selectors = row.get("sdk_test_selectors")
        if not isinstance(selectors, list) or not selectors or not all(isinstance(item, str) and item for item in selectors):
            check.fail(f"{label}: sdk_test_selectors must be a non-empty string array")
            selectors = []
        if test_path is not None:
            if not test_path.is_file():
                check.fail(f"{label}: SDK test file does not exist: {test_path_value}")
            else:
                content = test_path.read_text(encoding="utf-8")
                for selector in selectors:
                    if selector not in content:
                        check.fail(f"{label}: selector not found in {test_path_value}: {selector}")
        sdk_uri = str(row.get("sdk_test_uri", ""))
        expected_prefix = f"https://github.com/{baselines.get('sdk_repository', '')}/blob/"
        expected_suffix = f"/{test_path_value}"
        if not sdk_uri.startswith(expected_prefix) or not sdk_uri.endswith(expected_suffix):
            check.fail(f"{label}: sdk_test_uri does not bind the SDK test path")
        else:
            linked_revision = sdk_uri[len(expected_prefix):].split("/", 1)[0]
            if not SHA.fullmatch(linked_revision):
                check.fail(f"{label}: sdk_test_uri is not commit-pinned")

    dependency_keys = {"id", "concern", "apollo", "sdk", "disposition", "evidence_uri"}
    target_keys = {"id", "apollo_surface", "sdk_tier", "sdk_gate", "evidence_revision", "evidence_uri", "limitation", "closing_issue"}
    for row in dependencies:
        label = f"dependency {row.get('id', '?')}"
        check.exact_keys(row, dependency_keys, label)
        check.text_fields(row, dependency_keys, label)
    for row in targets:
        label = f"target {row.get('id', '?')}"
        check.exact_keys(row, target_keys, label)
        check.text_fields(row, target_keys, label)
        if not SHA.fullmatch(str(row.get("evidence_revision", ""))):
            check.fail(f"{label}: evidence_revision must be a full lowercase Git SHA")
        if row.get("sdk_tier") not in {"host-tested", "compile-checked", "not-supported"}:
            check.fail(f"{label}: unsupported sdk_tier: {row.get('sdk_tier')}")
        target_id = row.get("id")
        if target_id in PORTABLE_TARGET_IDS:
            policy_target = policy_targets.get(target_id, {})
            if row.get("sdk_tier") != policy_target.get("tier"):
                check.fail(f"{label}: tier differs from support policy")
            if row.get("sdk_gate") != policy_target.get("gate"):
                check.fail(f"{label}: gate differs from support policy")
            if row.get("limitation") != policy_target.get("limitation"):
                check.fail(f"{label}: limitation differs from support policy")
            if row.get("evidence_revision") != target_evidence.get("sdk_revision"):
                check.fail(f"{label}: evidence revision differs from the closing receipt")
            if row.get("evidence_uri") != target_evidence.get("ci_run_uri"):
                check.fail(f"{label}: evidence URI differs from the closing receipt")
        elif target_id == "linux-host":
            if row.get("sdk_gate") != "fast":
                check.fail(f"{label}: host gate must be fast")
            if row.get("evidence_revision") != baselines.get("sdk_fast_ci_revision"):
                check.fail(f"{label}: host revision differs from the fast baseline")
            if row.get("evidence_uri") != baselines.get("sdk_fast_ci_uri"):
                check.fail(f"{label}: host evidence differs from the fast baseline")
    return check.errors


def cell(value: Any) -> str:
    return str(value).replace("|", "\\|").replace("\n", " ")


def render(data: dict[str, Any]) -> str:
    baselines = data["baselines"]
    summary = data["summary"]
    lines = [
        "# Apollo cryptography parity report", "",
        "> Generated from `docs/architecture/apollo-crypto-parity.toml`; do not edit by hand.", "",
        f"Apollo baseline: `{baselines['apollo_revision']}`  ",
        f"SDK-Rust baseline: `{baselines['sdk_revision']}`  ",
        f"Fast CI: [{baselines['sdk_fast_ci_conclusion']}]({baselines['sdk_fast_ci_uri']})  ",
        f"Mapped vector/evidence suites: **{len(data['vectors'])}**", "",
        "## Summary", "", "| Total | Parity | SDK exceeds | Accepted difference | Gap |",
        "| ---: | ---: | ---: | ---: | ---: |",
        f"| {summary['total']} | {summary['parity']} | {summary['sdk_exceeds']} | {summary['accepted_difference']} | {summary['gap']} |", "",
        "## Performance evidence", "",
        f"Status: **{cell(data['performance']['status'])}**  ",
        f"Apollo comparison: **{cell(data['performance']['apollo_comparison'])}**  ",
        f"Sample minimum: **{data['performance']['sample_minimum']}**  ",
        f"Statistics: **{', '.join(data['performance']['statistics'])}**  ",
        f"Artifact receipt: [slow workflow run]({data['performance']['artifact_receipt']})  ",
        f"Limitations: {cell(data['performance']['limitations'])}", "",
        "## Capabilities", "",
        "| ID | Capability | Disposition | Apollo | SDK evidence | Vectors | Decision | Rationale |",
        "| --- | --- | --- | --- | --- | --- | --- | --- |",
    ]
    for row in data["capabilities"]:
        sdk_evidence = "n/a" if row["sdk_test_uri"] == "not-applicable" else f"[test]({row['sdk_test_uri']})"
        apollo_evidence = "n/a" if row["apollo_source_uri"] == "not-applicable" else f"[source]({row['apollo_source_uri']})"
        vectors = ", ".join(f"`{cell(item)}`" for item in row["vector_ids"]) or "n/a"
        lines.append(
            f"| `{cell(row['id'])}` | {cell(row['name'])} | "
            f"{cell(row['disposition'])} | {apollo_evidence} | "
            f"{sdk_evidence} | {vectors} | "
            f"[receipt]({row['decision_receipt']}) | "
            f"{cell(row['rationale'])} |"
        )
    lines += ["", "## Vector evidence", "", "| ID | Family | Kind | Test selectors |", "| --- | --- | --- | --- |"]
    for row in data["vectors"]:
        selectors = ", ".join(f"`{cell(item)}`" for item in row["sdk_test_selectors"])
        lines.append(f"| `{cell(row['id'])}` | {cell(row['family'])} | {cell(row['kind'])} | {selectors} |")
    lines += ["", "## Dependency decisions", "", "| Concern | Apollo | SDK-Rust | Disposition |", "| --- | --- | --- | --- |"]
    for row in data["dependencies"]:
        lines.append(f"| {cell(row['concern'])} | {cell(row['apollo'])} | {cell(row['sdk'])} | {cell(row['disposition'])} |")
    target_evidence = data["target_evidence"]
    packages = ", ".join(f"`{cell(item)}`" for item in target_evidence["packages"])
    features = ", ".join(f"`{cell(item)}`" for item in target_evidence["features"])
    lines += [
        "", "## Portable-target closing receipt", "",
        f"Revision: `{target_evidence['sdk_revision']}`  ",
        f"Rust toolchain: `{target_evidence['toolchain']}`  ",
        f"Packages: {packages}  ",
        f"Features: {features}  ",
        f"Slow CI: [{target_evidence['ci_conclusion']}]({target_evidence['ci_run_uri']})  ",
        f"Discussion receipt: [comment]({target_evidence['discussion_receipt']})  ",
        f"Limitations: {cell(target_evidence['limitations'])}", "",
        "## Target evidence", "",
        "| Target | Tier | Gate | Revision | Evidence | Limitation |",
        "| --- | --- | --- | --- | --- | --- |",
    ]
    for row in data["targets"]:
        lines.append(
            f"| `{cell(row['id'])}` | {cell(row['sdk_tier'])} | "
            f"`{cell(row['sdk_gate'])}` | `{cell(row['evidence_revision'])}` | "
            f"[receipt]({row['evidence_uri']}) | {cell(row['limitation'])} |"
        )
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", type=Path, default=Path.cwd())
    parser.add_argument("--render-markdown", action="store_true")
    args = parser.parse_args()
    try:
        data = load(args.root.resolve())
    except (OSError, tomllib.TOMLDecodeError) as error:
        print(f"apollo-parity: cannot load manifest: {error}", file=sys.stderr)
        return 1
    errors = validate(args.root.resolve(), data)
    if errors:
        for error in errors:
            print(f"apollo-parity: {error}", file=sys.stderr)
        print(f"apollo-parity: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    if args.render_markdown:
        print(render(data), end="")
    else:
        print(f"apollo-parity: {len(data['capabilities'])} capabilities and {len(data['vectors'])} vectors passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
