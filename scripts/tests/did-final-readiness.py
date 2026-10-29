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

    def test_failed_natural_run_cannot_be_relabelled(self) -> None:
        root = self.fixture()
        path = root / FILES[0]
        content = path.read_text(encoding="utf-8")
        path.write_text(content.replace('[last_natural_slow]\nstatus       = "failed"', '[last_natural_slow]\nstatus       = "success"'), encoding="utf-8")
        result = self.run_checker(root)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("must remain failed", result.stderr)

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
