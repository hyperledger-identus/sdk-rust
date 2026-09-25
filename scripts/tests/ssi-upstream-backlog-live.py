#!/usr/bin/env python3

import csv
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
CHECKER = REPOSITORY_ROOT / "scripts/check-ssi-upstream-backlog-live.py"
BACKLOG = REPOSITORY_ROOT / "docs/roadmap/ssi-upstream-dependency-backlog.csv"
CONFORMANCE_MATRIX = REPOSITORY_ROOT / "docs/conformance/oid4vci-final-wallet-core.csv"
REPOSITORY = "hyperledger-identus/sdk-rust"


class LiveBacklogContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        with BACKLOG.open(encoding="utf-8", newline="") as source:
            cls.rows = list(csv.DictReader(source))
        with CONFORMANCE_MATRIX.open(encoding="utf-8", newline="") as source:
            cls.conformance_rows = list(csv.DictReader(source))
        cls.issue_numbers = sorted(
            {int(row["issue"].removeprefix("#")) for row in cls.rows}
            | {
                int(row["followup_issue"].removeprefix("#"))
                for row in cls.conformance_rows
                if row["followup_issue"] != "none"
            }
        )

    def snapshot(self, overrides: dict[int, str] | None = None) -> dict:
        overrides = overrides or {}
        return {
            "schemaVersion": 1,
            "repository": REPOSITORY,
            "issues": [
                {
                    "number": number,
                    "state": overrides.get(number, "OPEN"),
                    "url": f"https://github.com/{REPOSITORY}/issues/{number}",
                }
                for number in self.issue_numbers
            ],
        }

    def run_snapshot(
        self, snapshot: object, extra_arguments: list[str] | None = None
    ) -> subprocess.CompletedProcess[str]:
        with tempfile.NamedTemporaryFile(
            mode="w", encoding="utf-8", suffix=".json"
        ) as target:
            json.dump(snapshot, target)
            target.flush()
            return subprocess.run(
                [
                    sys.executable,
                    str(CHECKER),
                    *(extra_arguments or []),
                    "--snapshot",
                    target.name,
                ],
                check=False,
                capture_output=True,
                text=True,
            )

    def test_current_backlog_passes_with_open_active_owners(self) -> None:
        result = self.run_snapshot(self.snapshot())
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("30 backlog rows", result.stdout)
        self.assertIn("24 conformance rows", result.stdout)
        self.assertIn("passed (snapshot)", result.stdout)

    def test_alternate_backlog_cannot_replace_canonical_ledger(self) -> None:
        result = self.run_snapshot(self.snapshot(), [str(BACKLOG)])
        self.assertEqual(result.returncode, 2)
        self.assertIn("unrecognized arguments", result.stderr)

    def test_closed_in_progress_owner_fails(self) -> None:
        active_issue = int(
            next(
                row["issue"]
                for row in self.rows
                if row["delivery_status"] == "in_progress"
            ).removeprefix("#")
        )
        result = self.run_snapshot(self.snapshot({active_issue: "CLOSED"}))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(f"in_progress owner #{active_issue} is CLOSED", result.stderr)

    def test_closed_specified_and_delivered_evidence_passes(self) -> None:
        active = {
            int(row["issue"].removeprefix("#"))
            for row in self.rows
            if row["delivery_status"] == "in_progress"
        }
        closed = {
            int(row["issue"].removeprefix("#")): "CLOSED"
            for row in self.rows
            if row["delivery_status"] in {"specified", "delivered"}
            and int(row["issue"].removeprefix("#")) not in active
        }
        result = self.run_snapshot(self.snapshot(closed))
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_closed_conformance_gap_owner_fails_when_one_exists(self) -> None:
        owned_rows = [
            row
            for row in self.conformance_rows
            if row["followup_issue"] != "none"
        ]
        if not owned_rows:
            result = self.run_snapshot(self.snapshot())
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertNotIn("conformance owner", result.stderr)
            return

        row = owned_rows[0]
        owner = int(row["followup_issue"].removeprefix("#"))
        result = self.run_snapshot(self.snapshot({owner: "CLOSED"}))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            f"{row['id']}: conformance owner #{owner} is CLOSED",
            result.stderr,
        )

    def test_missing_referenced_issue_fails(self) -> None:
        snapshot = self.snapshot()
        missing = snapshot["issues"].pop()
        result = self.run_snapshot(snapshot)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(f"missing referenced issues: #{missing['number']}", result.stderr)

    def test_duplicate_and_malformed_snapshots_fail(self) -> None:
        duplicate = self.snapshot()
        duplicate["issues"][-1] = duplicate["issues"][0]
        result = self.run_snapshot(duplicate)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("duplicate issue", result.stderr)

        malformed = self.snapshot()
        malformed["unexpected"] = True
        result = self.run_snapshot(malformed)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("must contain exactly", result.stderr)

        wrong_url = self.snapshot()
        wrong_url["issues"][0]["url"] = "https://github.com/example/wrong/issues/1"
        result = self.run_snapshot(wrong_url)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("url does not match repository", result.stderr)

    def test_github_command_failure_is_redacted_and_fails_closed(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            fake_gh = Path(directory) / "gh"
            fake_gh.write_text(
                "#!/bin/sh\necho sensitive-detail >&2\nexit 23\n", encoding="utf-8"
            )
            fake_gh.chmod(0o700)
            environment = os.environ.copy()
            environment["PATH"] = directory
            result = subprocess.run(
                [sys.executable, str(CHECKER)],
                check=False,
                capture_output=True,
                text=True,
                env=environment,
            )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("GitHub could not resolve issue", result.stderr)
        self.assertNotIn("sensitive-detail", result.stderr)


if __name__ == "__main__":
    unittest.main()
