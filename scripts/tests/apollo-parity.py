#!/usr/bin/env python3
"""Mutation tests for the Apollo parity manifest contract."""

from __future__ import annotations

import shutil
import subprocess
import tempfile
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-apollo-parity.py"
MANIFEST = Path("docs/architecture/apollo-crypto-parity.toml")


class ApolloParityContract(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        source = ROOT / MANIFEST
        target = self.root / MANIFEST
        target.parent.mkdir(parents=True)
        shutil.copy2(source, target)
        policy_source = ROOT / "docs/architecture/sdk-support-policy.toml"
        policy_target = self.root / "docs/architecture/sdk-support-policy.toml"
        shutil.copy2(policy_source, policy_target)
        data = tomllib.loads(source.read_text(encoding="utf-8"))
        for field in ("sdk_harness_path", "sdk_runner_path", "workflow_path"):
            relative = Path(data["performance"][field])
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)
        for field in ("runner_path", "normalizer_path", "workflow_path"):
            relative = Path(data["coverage"][field])
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            if not destination.exists():
                shutil.copy2(ROOT / relative, destination)
        for relative in (
            Path(".github/workflows/factory-contract.yml"),
            Path("nix/rust-toolchain.nix"),
            Path("nix/devshells/default.nix"),
        ):
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)
        evidence_paths = {
            Path(row["path"]) for row in data["coverage"]["source_mappings"]
        } | {
            Path(row["path"]) for row in data["coverage"]["critical_tests"]
        }
        for relative in evidence_paths:
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)
        for vector in data["vectors"]:
            relative = Path(vector["sdk_test_path"])
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def run_checker(self, *arguments: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [str(CHECKER), str(self.root), *arguments],
            check=False, capture_output=True, text=True,
        )

    def replace(self, old: str, new: str, count: int = 1) -> None:
        path = self.root / MANIFEST
        content = path.read_text(encoding="utf-8")
        self.assertIn(old, content)
        path.write_text(content.replace(old, new, count), encoding="utf-8")

    def test_canonical_manifest_passes(self) -> None:
        self.assertEqual(self.run_checker().returncode, 0)

    def test_unknown_disposition_fails(self) -> None:
        self.replace('disposition       = "parity"', 'disposition       = "maybe"')
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("unsupported disposition", result.stderr)

    def test_duplicate_capability_fails(self) -> None:
        self.replace('id                = "encoding-hex"', 'id                = "entropy-csprng"')
        self.assertIn("duplicate ids", self.run_checker().stderr)

    def test_unknown_vector_reference_fails(self) -> None:
        self.replace(
            'vector_ids        = [ "apollo-secure-random" ]',
            'vector_ids        = [ "absent-vector" ]',
        )
        self.assertIn("unknown vector ids", self.run_checker().stderr)

    def test_vector_inventory_drift_fails(self) -> None:
        self.replace(
            'id                 = "sdk-hmac-sha512-rfc4231"',
            'id                 = "new-vector"',
        )
        self.assertIn("vectors: inventory differs", self.run_checker().stderr)

    def test_summary_drift_fails(self) -> None:
        self.replace("total               = 27", "total               = 26")
        self.assertIn("summary: expected", self.run_checker().stderr)

    def test_performance_comparison_claim_fails(self) -> None:
        self.replace('apollo_comparison   = "unavailable"', 'apollo_comparison   = "faster"')
        self.assertIn("comparison must be unavailable", self.run_checker().stderr)

    def test_performance_harness_must_be_commit_pinned(self) -> None:
        self.replace(
            "blob/0f1074b766f3cbf1ffc905b4388220b27b1c4556/crates/crypto/examples/crypto_baseline.rs",
            "blob/develop/crates/crypto/examples/crypto_baseline.rs",
        )
        self.assertIn("not commit-pinned", self.run_checker().stderr)

    def test_coverage_threshold_reduction_fails(self) -> None:
        self.replace("threshold_basis_points = 7482", "threshold_basis_points = 7000")
        self.assertIn("threshold must remain 74.82 percent", self.run_checker().stderr)

    def test_coverage_denominator_drift_fails(self) -> None:
        self.replace("sdk_lines              = 1722", "sdk_lines              = 2000")
        self.assertIn("percentage differs from covered/line counts", self.run_checker().stderr)

    def test_coverage_below_threshold_evidence_fails(self) -> None:
        self.replace("sdk_covered            = 1481", "sdk_covered            = 1")
        result = self.run_checker()
        self.assertIn("falls below the Apollo threshold", result.stderr)

    def test_coverage_profile_omission_fails(self) -> None:
        self.replace(
            'profiles               = [ "default", "all-features", "no-default-features", "kmp-compat" ]',
            'profiles               = [ "all-features" ]',
        )
        self.assertIn("four isolated surfaces", self.run_checker().stderr)

    def test_coverage_unknown_capability_mapping_fails(self) -> None:
        self.replace(
            'capability_ids = [ "encoding-base64url-nopad", "public-jwk" ]',
            'capability_ids = [ "unknown-capability" ]',
        )
        self.assertIn("unknown capability ids", self.run_checker().stderr)

    def test_coverage_unknown_vector_mapping_fails(self) -> None:
        self.replace(
            'vector_ids     = [ "apollo-base64url", "sdk-rfc8037-jwk" ]',
            'vector_ids     = [ "unknown-vector" ]',
        )
        self.assertIn("unknown vector ids", self.run_checker().stderr)

    def test_coverage_source_inventory_drift_fails(self) -> None:
        source = self.root / "crates/crypto/src/new.rs"
        source.write_text("pub fn new_source() {}\n", encoding="utf-8")
        self.assertIn("source mapping inventory differs", self.run_checker().stderr)

    def test_coverage_missing_critical_selector_fails(self) -> None:
        self.replace(
            '"one_over_limit_wins_before_malformed_base64url_and_is_redacted"',
            '"missing_critical_selector"',
        )
        self.assertIn("selector not found", self.run_checker().stderr)

    def test_coverage_fast_lane_contamination_fails(self) -> None:
        fast = self.root / ".github/workflows/factory-contract.yml"
        fast.write_text(
            fast.read_text(encoding="utf-8") + "\n# coverage-crypto\n",
            encoding="utf-8",
        )
        self.assertIn("fast workflow must not execute coverage", self.run_checker().stderr)

    def test_stale_apollo_link_fails(self) -> None:
        self.replace("ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c/apollo/src", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/apollo/src")
        self.assertIn("not baseline-pinned", self.run_checker().stderr)

    def test_stale_ci_receipt_fails(self) -> None:
        self.replace(
            'ci_receipt        = "https://github.com/hyperledger-identus/sdk-rust/actions/runs/34262060203"',
            'ci_receipt        = "https://github.com/hyperledger-identus/sdk-rust/actions/runs/1"',
        )
        self.assertIn("CI receipt differs", self.run_checker().stderr)

    def test_portable_host_gate_substitution_fails(self) -> None:
        self.replace('sdk_gate          = "rust-build-wasm32"', 'sdk_gate          = "fast"')
        self.assertIn("gate differs from support policy", self.run_checker().stderr)

    def test_portable_stale_revision_fails(self) -> None:
        self.replace(
            'evidence_revision = "244ded689a29a5e6606eeb36aede14149d585071"',
            'evidence_revision = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"',
        )
        self.assertIn("evidence revision differs from the closing receipt", self.run_checker().stderr)

    def test_portable_non_actions_receipt_fails(self) -> None:
        self.replace(
            'ci_run_uri          = "https://github.com/hyperledger-identus/sdk-rust/actions/runs/34286965807"',
            'ci_run_uri          = "https://github.com/hyperledger-identus/sdk-rust/issues/213"',
        )
        self.assertIn("CI receipt must be a GitHub Actions run", self.run_checker().stderr)

    def test_portable_package_drift_fails(self) -> None:
        self.replace(
            'packages            = [ "identus-core", "identus-crypto", "identus-did", "identus-jose", "identus-oid4vci", "identus-adapters-entropy" ]',
            'packages            = [ "identus-crypto" ]',
        )
        self.assertIn("packages differ from support policy", self.run_checker().stderr)

    def test_portable_target_array_fails_closed(self) -> None:
        self.replace(
            'targets             = [ "wasm32-unknown-unknown", "aarch64-apple-ios", "aarch64-linux-android" ]',
            'targets             = [ { malformed = true } ]',
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("targets must be a non-empty string array", result.stderr)

    def test_portable_limitation_drift_fails(self) -> None:
        self.replace(
            'limitation        = "Compile-only with the getrandom browser backend; no browser runtime, bundler, CSP, storage, FFI, performance, or certification claim."',
            'limitation        = "Production browser support."',
        )
        self.assertIn("limitation differs from support policy", self.run_checker().stderr)

    def test_missing_selector_fails(self) -> None:
        self.replace('"sha2_empty_message_known_answers"', '"selector_does_not_exist"')
        self.assertIn("selector not found", self.run_checker().stderr)

    def test_unsafe_test_path_fails(self) -> None:
        self.replace(
            'sdk_test_path      = "crates/adapters-entropy/src/lib.rs"',
            'sdk_test_path      = "../outside.rs"',
        )
        self.assertIn("unsafe repository-relative path", self.run_checker().stderr)

    def test_symlink_escape_fails(self) -> None:
        test_path = self.root / "crates/adapters-entropy/src/lib.rs"
        outside = self.root.parent / f"{self.root.name}-outside.rs"
        outside.write_text("apollo_secure_random_contract", encoding="utf-8")
        self.addCleanup(outside.unlink, missing_ok=True)
        test_path.unlink()
        test_path.symlink_to(outside)
        self.assertIn("resolves outside", self.run_checker().stderr)

    def test_render_is_deterministic(self) -> None:
        first = self.run_checker("--render-markdown")
        second = self.run_checker("--render-markdown")
        self.assertEqual(first.returncode, 0)
        self.assertEqual(first.stdout, second.stdout)
        self.assertIn("| 27 | 14 | 6 | 7 | 0 |", first.stdout)
        self.assertIn("`cardano-v2-private`", first.stdout)
        self.assertIn("`244ded689a29a5e6606eeb36aede14149d585071`", first.stdout)
        self.assertIn("actions/runs/34286965807", first.stdout)
        self.assertIn("rust-build-ios-aarch64", first.stdout)
        self.assertIn("## Line coverage evidence", first.stdout)
        self.assertIn("74.82%", first.stdout)


if __name__ == "__main__":
    unittest.main()
