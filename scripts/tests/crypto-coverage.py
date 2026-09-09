#!/usr/bin/env python3
"""Mutation tests for normalized identus-crypto coverage evidence."""

from __future__ import annotations

import json
import os
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/report-crypto-coverage.py"


class CryptoCoverageContract(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.source = self.root / "crates/crypto/src/lib.rs"
        self.source.parent.mkdir(parents=True)
        self.source.write_text("pub fn covered() {}\n", encoding="utf-8")

    def tearDown(self) -> None:
        self.temp.cleanup()

    def raw(self, *, count: int = 100, covered: int = 75) -> dict:
        return {
            "type": "llvm.coverage.json.export",
            "version": "3.1.0",
            "data": [
                {
                    "files": [
                        {
                            "filename": str(self.source),
                            "summary": {"lines": {"count": count, "covered": covered}},
                        }
                    ]
                }
            ],
        }

    def run_checker(self, data: dict, **overrides: str) -> subprocess.CompletedProcess[str]:
        raw = self.root / "raw.json"
        raw.write_text(json.dumps(data), encoding="utf-8")
        output = self.root / "output"
        command = [
            str(CHECKER),
            str(raw),
            str(output),
            "--repository-root",
            str(self.root),
            "--revision",
            overrides.get("revision", "a" * 40),
            "--rust-version",
            overrides.get("rust_version", "rustc 1.98.1 (fixture)"),
            "--tool-version",
            overrides.get("tool_version", "cargo-llvm-cov 0.9.0"),
        ]
        return subprocess.run(command, check=False, capture_output=True, text=True)

    def test_canonical_artifact_passes_and_is_deterministic(self) -> None:
        first = self.run_checker(self.raw())
        self.assertEqual(first.returncode, 0, first.stderr)
        json_first = (self.root / "output/summary.json").read_bytes()
        markdown_first = (self.root / "output/summary.md").read_bytes()
        second = self.run_checker(self.raw())
        self.assertEqual(second.returncode, 0, second.stderr)
        self.assertEqual(json_first, (self.root / "output/summary.json").read_bytes())
        self.assertEqual(markdown_first, (self.root / "output/summary.md").read_bytes())

    def test_exact_threshold_passes(self) -> None:
        self.assertEqual(self.run_checker(self.raw(count=10_000, covered=7_482)).returncode, 0)

    def test_below_threshold_fails_without_summary(self) -> None:
        result = self.run_checker(self.raw(count=10_000, covered=7_481))
        self.assertIn("below the 74.82% threshold", result.stderr)
        self.assertFalse((self.root / "output/summary.json").exists())

    def test_invalid_revision_fails(self) -> None:
        self.assertIn("full lowercase Git SHA", self.run_checker(self.raw(), revision="develop").stderr)

    def test_wrong_rust_version_fails(self) -> None:
        self.assertIn("pinned Rust 1.98.1", self.run_checker(self.raw(), rust_version="rustc 1.99.0").stderr)

    def test_wrong_tool_version_fails(self) -> None:
        self.assertIn("exactly cargo-llvm-cov 0.9.0", self.run_checker(self.raw(), tool_version="cargo-llvm-cov 0.9.1").stderr)

    def test_duplicate_crypto_file_fails(self) -> None:
        data = self.raw()
        data["data"][0]["files"].append(data["data"][0]["files"][0].copy())
        self.assertIn("duplicate crypto source path", self.run_checker(data).stderr)

    def test_outside_files_are_excluded_from_denominator(self) -> None:
        outside = self.root / "crates/core/src/lib.rs"
        outside.parent.mkdir(parents=True)
        outside.write_text("pub fn uncovered() {}\n", encoding="utf-8")
        data = self.raw()
        data["data"][0]["files"].append(
            {"filename": str(outside), "summary": {"lines": {"count": 10_000, "covered": 0}}}
        )
        result = self.run_checker(data)
        self.assertEqual(result.returncode, 0, result.stderr)
        summary = json.loads((self.root / "output/summary.json").read_text(encoding="utf-8"))
        self.assertEqual(summary["totals"]["lines"], 100)

    def test_crypto_symlink_escape_fails(self) -> None:
        outside = self.root / "outside.rs"
        outside.write_text("pub fn escaped() {}\n", encoding="utf-8")
        self.source.unlink()
        os.symlink(outside, self.source)
        self.assertIn("escapes or is missing", self.run_checker(self.raw()).stderr)

    def test_missing_crypto_file_fails(self) -> None:
        self.source.unlink()
        self.assertIn("escapes or is missing", self.run_checker(self.raw()).stderr)

    def test_non_rust_file_fails(self) -> None:
        text = self.source.with_suffix(".txt")
        self.source.rename(text)
        data = self.raw()
        data["data"][0]["files"][0]["filename"] = str(text)
        self.assertIn("not a regular Rust file", self.run_checker(data).stderr)

    def test_empty_crypto_denominator_fails(self) -> None:
        data = self.raw()
        data["data"][0]["files"] = []
        self.assertIn("no executable identus-crypto source files", self.run_checker(data).stderr)

    def test_invalid_line_counts_fail(self) -> None:
        self.assertIn("invalid count/covered", self.run_checker(self.raw(count=1, covered=2)).stderr)

    def test_boolean_line_count_fails(self) -> None:
        data = self.raw()
        data["data"][0]["files"][0]["summary"]["lines"]["count"] = True
        self.assertIn("invalid count/covered", self.run_checker(data).stderr)

    def test_multiple_data_entries_fail(self) -> None:
        data = self.raw()
        data["data"].append(data["data"][0])
        self.assertIn("exactly one data entry", self.run_checker(data).stderr)

    def test_non_llvm_document_fails(self) -> None:
        data = self.raw()
        data["type"] = "other"
        self.assertIn("not an LLVM coverage JSON export", self.run_checker(data).stderr)


if __name__ == "__main__":
    unittest.main()
