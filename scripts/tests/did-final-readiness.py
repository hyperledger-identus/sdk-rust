#!/usr/bin/env python3
"""Mutation tests for the DID final-readiness packet checker."""

from __future__ import annotations

import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-did-final-readiness.py"
FILES = (
    Path("docs/release/did-candidate-readiness.toml"),
    Path("site/src/did-candidate-readiness.md"),
)


class ReadinessContractTests(unittest.TestCase):
    def fixture(self) -> Path:
        directory = Path(self.enterContext(tempfile.TemporaryDirectory()))
        for relative in FILES:
            destination = directory / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)
        return directory

    def run_checker(self, root: Path) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [str(CHECKER), str(root)],
            check=False,
            capture_output=True,
            text=True,
        )

    def test_repository_packet_passes(self) -> None:
        result = self.run_checker(ROOT)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_premature_completion_fails(self) -> None:
        root = self.fixture()
        path = root / FILES[0]
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                'status         = "pending-exact-slow-and-independent-approval"',
                'status         = "complete"',
            ),
            encoding="utf-8",
        )
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("status must be", result.stderr)

    def test_historical_failed_run_cannot_be_relabelled(self) -> None:
        root = self.fixture()
        path = root / FILES[0]
        content = path.read_text(encoding="utf-8")
        path.write_text(content.replace('[historical_failed_slow]\nstatus       = "failed"', '[historical_failed_slow]\nstatus       = "success"'), encoding="utf-8")
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("exact failed receipt", result.stderr)

    def test_publication_non_claim_cannot_be_reversed(self) -> None:
        root = self.fixture()
        path = root / FILES[0]
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "No crates.io publication or registry availability claim.",
                "Crates.io publication and registry availability are confirmed.",
            ),
            encoding="utf-8",
        )
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("five bounded non-claims", result.stderr)

    def test_candidate_archive_identity_cannot_drift(self) -> None:
        root = self.fixture()
        path = root / FILES[0]
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "7736e445ffce327fae19a55c7b8b7adf38887f93f56aba4a9a804d18d765fb68",
                "0" * 64,
            ),
            encoding="utf-8",
        )
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("exact protected candidate receipt", result.stderr)

    def test_manifest_evidence_uri_cannot_drift(self) -> None:
        root = self.fixture()
        path = root / FILES[0]
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "https://github.com/hyperledger-identus/sdk-rust/pull/584#issuecomment-6493993716",
                "https://example.invalid/mutable",
            ),
            encoding="utf-8",
        )
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("evidence matrix must equal", result.stderr)

    def test_duplicate_evidence_id_cannot_be_overwritten(self) -> None:
        root = self.fixture()
        path = root / FILES[0]
        duplicate = (
            '[[evidence]]\n'
            'id = "train-identity"\n'
            'status = "failed"\n'
            'uri = "https://example.invalid/contradictory"\n\n'
        )
        path.write_text(
            path.read_text(encoding="utf-8").replace("[[evidence]]", duplicate + "[[evidence]]", 1),
            encoding="utf-8",
        )
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("duplicate evidence id", result.stderr)

    def test_natural_slow_receipt_uri_cannot_drift(self) -> None:
        root = self.fixture()
        path = root / FILES[0]
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "https://github.com/hyperledger-identus/sdk-rust/actions/runs/40260649503",
                "https://example.invalid/slow",
            ),
            encoding="utf-8",
        )
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("exact failed receipt", result.stderr)

    def test_downstream_receipt_uri_cannot_drift(self) -> None:
        root = self.fixture()
        path = root / FILES[0]
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "https://github.com/hyperledger-identus/neoprism/issues/325#issuecomment-6487638362",
                "https://example.invalid/e2e",
            ),
            encoding="utf-8",
        )
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("exact bounded receipt", result.stderr)

    def test_displayed_identity_cannot_drift_from_manifest(self) -> None:
        root = self.fixture()
        path = root / FILES[1]
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "5c985cd2d3f26c07230ace1086d7a178662a4f583d0086574ac9baa6a00be639",
                "0" * 64,
            ),
            encoding="utf-8",
        )
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("displayed identity row drifted", result.stderr)

    def test_displayed_api_hashes_cannot_swap_labels(self) -> None:
        root = self.fixture()
        path = root / FILES[1]
        did_hash = "5c985cd2d3f26c07230ace1086d7a178662a4f583d0086574ac9baa6a00be639"
        resolver_hash = "9c432f0d0c3d5db92da3aa88aed05164aa2adf483c11be84b9239c9973179df6"
        content = path.read_text(encoding="utf-8")
        path.write_text(
            content.replace(did_hash, "SWAPPED_HASH").replace(resolver_hash, did_hash).replace("SWAPPED_HASH", resolver_hash),
            encoding="utf-8",
        )
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("displayed identity row drifted", result.stderr)

    def test_missing_e2e_receipt_fails(self) -> None:
        root = self.fixture()
        path = root / FILES[1]
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                "https://github.com/hyperledger-identus/neoprism/issues/325#issuecomment-6487638362",
                "https://example.invalid/missing",
            ),
            encoding="utf-8",
        )
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing required bounded claim", result.stderr)


if __name__ == "__main__":
    unittest.main()
