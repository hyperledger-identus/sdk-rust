#!/usr/bin/env python3
"""Mutation tests for the crypto benchmark artifact contract."""

from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-crypto-benchmark.py"
OPERATION_NAMES = [
    "sha-256", "sha-512", "ed25519-sign", "ed25519-verify",
    "x25519-agreement", "secp256k1-sign", "secp256k1-verify",
    "p256-sign", "p256-verify", "bip39-seed",
    "secp256k1-bip32-hardened-child", "cardano-v2-private-child",
    "cardano-v2-public-child",
]


def canonical() -> dict:
    return {
        "schema_version": 1,
        "status": "measurement-only",
        "apollo_comparison": "unavailable",
        "revision": "a" * 40,
        "compiler": "rustc 1.98.0",
        "os": "linux",
        "arch": "x86_64",
        "cpu": "test cpu",
        "environment": "fixture",
        "feature_profile": "default",
        "samples": 20,
        "warmup_batches": 1,
        "operations": [
            {
                "name": name,
                "iterations_per_sample": 10,
                "p50_ns": 2,
                "p95_ns": 3,
                "min_ns": 1,
                "max_ns": 4,
            }
            for name in OPERATION_NAMES
        ],
    }


class CryptoBenchmarkContract(unittest.TestCase):
    def run_checker(self, data: dict) -> subprocess.CompletedProcess[str]:
        with tempfile.TemporaryDirectory() as directory:
            artifact = Path(directory) / "artifact.json"
            artifact.write_text(json.dumps(data), encoding="utf-8")
            return subprocess.run(
                [str(CHECKER), str(artifact)], check=False, capture_output=True, text=True,
            )

    def test_canonical_artifact_passes(self) -> None:
        self.assertEqual(self.run_checker(canonical()).returncode, 0)

    def test_comparison_claim_fails(self) -> None:
        data = canonical()
        data["apollo_comparison"] = "faster"
        self.assertIn("must be unavailable", self.run_checker(data).stderr)

    def test_sample_floor_fails(self) -> None:
        data = canonical()
        data["samples"] = 19
        self.assertIn("from 20 through 10000", self.run_checker(data).stderr)

    def test_sample_ceiling_fails(self) -> None:
        data = canonical()
        data["samples"] = 10_001
        self.assertIn("from 20 through 10000", self.run_checker(data).stderr)

    def test_zero_batch_size_fails(self) -> None:
        data = canonical()
        data["operations"][0]["iterations_per_sample"] = 0
        self.assertIn("at least 1", self.run_checker(data).stderr)

    def test_operation_inventory_drift_fails(self) -> None:
        data = canonical()
        data["operations"].pop()
        self.assertIn("inventory differs", self.run_checker(data).stderr)

    def test_impossible_percentile_order_fails(self) -> None:
        data = canonical()
        data["operations"][0]["p95_ns"] = 5
        self.assertIn("timing order", self.run_checker(data).stderr)

    def test_secret_shaped_unknown_field_fails(self) -> None:
        data = canonical()
        data["private_key"] = "do-not-publish"
        self.assertIn("top-level fields differ", self.run_checker(data).stderr)


if __name__ == "__main__":
    unittest.main()
