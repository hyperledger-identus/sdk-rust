#!/usr/bin/env python3

import csv
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
CHECKER = REPOSITORY_ROOT / "scripts/check-ssi-upstream-backlog.py"
BACKLOG = REPOSITORY_ROOT / "docs/roadmap/ssi-upstream-dependency-backlog.csv"


class BacklogContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        with BACKLOG.open(encoding="utf-8", newline="") as source:
            reader = csv.DictReader(source)
            cls.fields = reader.fieldnames
            cls.rows = list(reader)

    def run_checker(
        self, rows: list[dict[str, str]]
    ) -> subprocess.CompletedProcess[str]:
        with tempfile.NamedTemporaryFile(
            mode="w", encoding="utf-8", newline="", suffix=".csv"
        ) as target:
            writer = csv.DictWriter(target, fieldnames=self.fields)
            writer.writeheader()
            writer.writerows(rows)
            target.flush()
            return subprocess.run(
                [sys.executable, str(CHECKER), target.name],
                check=False,
                capture_output=True,
                text=True,
            )

    def test_canonical_backlog_passes(self) -> None:
        result = self.run_checker(self.rows)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("30 rows passed", result.stdout)

    def test_duplicate_identifier_fails(self) -> None:
        result = self.run_checker([*self.rows, self.rows[0].copy()])
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("duplicate identifiers: IDR-001", result.stderr)

    def test_midnight_identifier_fails(self) -> None:
        rows = [row.copy() for row in self.rows]
        rows[0]["id"] = "MID-001"
        result = self.run_checker(rows)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("unexpected identifiers: MID-001", result.stderr)
        self.assertIn("id must match IDR-NNN", result.stderr)

    def test_unknown_source_fails(self) -> None:
        rows = [row.copy() for row in self.rows]
        rows[0]["source_repositories"] = "unknown-wallet"
        result = self.run_checker(rows)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("unknown source repository unknown-wallet", result.stderr)

    def test_program_issue_cannot_claim_active_delivery(self) -> None:
        rows = [row.copy() for row in self.rows]
        program_row = next(row for row in rows if row["issue"] == "#20")
        program_row["delivery_status"] = "specified"
        result = self.run_checker(rows)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("program issue #20 cannot own active component delivery", result.stderr)

    def test_empty_required_field_fails_without_crashing(self) -> None:
        rows = [row.copy() for row in self.rows]
        rows[0]["issue"] = ""
        result = self.run_checker(rows)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("IDR-001: issue is empty", result.stderr)
        self.assertIn("IDR-001: issue must match #N", result.stderr)


if __name__ == "__main__":
    unittest.main()
