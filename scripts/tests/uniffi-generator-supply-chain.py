#!/usr/bin/env python3

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-uniffi-generator-supply-chain.py"


class UniffiGeneratorSupplyChainTests(unittest.TestCase):
    def run_checker(
        self,
        *,
        as_of: str = "2026-10-28",
        mode: str = "pass",
    ) -> tuple[subprocess.CompletedProcess[str], str]:
        with tempfile.TemporaryDirectory() as directory:
            temporary = Path(directory)
            log = temporary / "cargo.log"
            cargo = temporary / "cargo"
            cargo.write_text(
                """#!/bin/sh
printf '%s\\n' "$*" >>"$FAKE_CARGO_LOG"
case "$1" in
  tree)
    if [ "$FAKE_CARGO_MODE" = graph-drift ]; then
      printf '%s\\n' 'basic-toml v0.1.11'
    else
      printf '%s\\n' \\
        'basic-toml v0.1.10' \\
        'askama_derive v0.16.1' \\
        'askama_macros v0.16.1 (proc-macro)' \\
        'askama v0.16.1' \\
        'uniffi_bindgen v0.32.0' \\
        'uniffi v0.32.0' \\
        "identus-uniffi-bindgen v0.0.0 ($FAKE_REPOSITORY_ROOT/tools/uniffi-bindgen)"
    fi
    ;;
  deny)
    [ "$FAKE_CARGO_MODE" != deny-failure ] || exit 41
    ;;
  audit)
    [ "$FAKE_CARGO_MODE" != audit-failure ] || exit 42
    ;;
  *) exit 43 ;;
esac
""",
                encoding="utf-8",
            )
            cargo.chmod(0o700)
            environment = os.environ.copy()
            environment.update(
                {
                    "PATH": f"{directory}:/usr/bin:/bin",
                    "FAKE_CARGO_LOG": str(log),
                    "FAKE_CARGO_MODE": mode,
                    "FAKE_REPOSITORY_ROOT": str(ROOT),
                }
            )
            result = subprocess.run(
                [sys.executable, str(CHECKER), "--as-of", as_of],
                check=False,
                capture_output=True,
                text=True,
                env=environment,
            )
            calls = log.read_text(encoding="utf-8") if log.exists() else ""
        return result, calls

    def test_exact_current_graph_passes_with_one_scoped_ignore(self) -> None:
        result, calls = self.run_checker()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("RUSTSEC-2026-0374", result.stdout)
        audit = next(line for line in calls.splitlines() if line.startswith("audit "))
        self.assertIn("--ignore RUSTSEC-2026-0374", audit)
        self.assertEqual(audit.count("--ignore"), 1)

    def test_expiry_and_malformed_date_fail_before_cargo(self) -> None:
        for as_of in ("2027-02-01", "2026-1-1", "not-a-date"):
            with self.subTest(as_of=as_of):
                result, calls = self.run_checker(as_of=as_of)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(calls, "")

    def test_graph_drift_fails_before_policy_and_audit(self) -> None:
        result, calls = self.run_checker(mode="graph-drift")
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(len(calls.splitlines()), 1)
        self.assertNotIn("deny ", calls)
        self.assertNotIn("audit ", calls)

    def test_deny_and_unexpected_audit_findings_remain_blocking(self) -> None:
        for mode, expected in (("deny-failure", "cargo deny"), ("audit-failure", "cargo audit")):
            with self.subTest(mode=mode):
                result, _ = self.run_checker(mode=mode)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(expected, result.stderr)


if __name__ == "__main__":
    unittest.main()
