#!/usr/bin/env python3

import json
import os
import subprocess
import sys
import tempfile
import unittest
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-weekly-slow-live.py"
REPOSITORY = "hyperledger-identus/sdk-rust"


class WeeklySlowLiveTests(unittest.TestCase):
    def snapshot(self, **overrides: object) -> dict:
        document = {
            "schemaVersion": 1,
            "repository": REPOSITORY,
            "checkedAt": "2026-09-15T12:00:00Z",
            "defaultBranch": "develop",
            "runs": [
                {
                    "databaseId": 123,
                    "attempt": 1,
                    "event": "schedule",
                    "status": "completed",
                    "conclusion": "success",
                    "headBranch": "develop",
                    "headSha": "a" * 40,
                    "createdAt": "2026-09-14T02:23:00Z",
                    "url": f"https://github.com/{REPOSITORY}/actions/runs/123",
                    "workflowName": "slow",
                }
            ],
        }
        document.update(overrides)
        return document

    def run_snapshot(self, document: object) -> subprocess.CompletedProcess[str]:
        with tempfile.NamedTemporaryFile(mode="w", encoding="utf-8") as target:
            json.dump(document, target)
            target.flush()
            return subprocess.run(
                [sys.executable, str(CHECKER), "--snapshot", target.name],
                check=False,
                capture_output=True,
                text=True,
            )

    def test_recent_success_passes(self) -> None:
        result = self.run_snapshot(self.snapshot())
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("123 passed", result.stdout)

    def test_default_branch_must_be_develop(self) -> None:
        result = self.run_snapshot(self.snapshot(defaultBranch="main"))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("expected protected develop", result.stderr)

    def test_scheduled_run_must_come_from_develop(self) -> None:
        document = self.snapshot()
        document["runs"][0]["headBranch"] = "temporary-default"
        result = self.run_snapshot(document)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("run 123 headBranch must be protected develop", result.stderr)

    def test_missing_run_fails(self) -> None:
        result = self.run_snapshot(self.snapshot(runs=[]))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no native scheduled slow run", result.stderr)

    def test_failed_and_running_runs_fail_with_identity(self) -> None:
        for status, conclusion in (("completed", "failure"), ("in_progress", None)):
            with self.subTest(status=status):
                document = self.snapshot()
                document["runs"][0]["status"] = status
                document["runs"][0]["conclusion"] = conclusion
                result = self.run_snapshot(document)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn("run 123", result.stderr)

    def test_manual_rerun_cannot_be_natural_evidence(self) -> None:
        document = self.snapshot()
        document["runs"][0]["attempt"] = 2
        result = self.run_snapshot(document)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("rerun attempt 2", result.stderr)
        self.assertIn("not natural schedule evidence", result.stderr)

    def test_stale_run_fails(self) -> None:
        result = self.run_snapshot(
            self.snapshot(checkedAt="2026-09-24T12:00:00Z")
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("is stale", result.stderr)

    def test_evidence_is_stale_before_retention_expires(self) -> None:
        result = self.run_snapshot(
            self.snapshot(checkedAt="2026-09-20T19:24:00Z")
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("is stale (161h)", result.stderr)

    def test_schema_and_repository_are_strict(self) -> None:
        extra = self.snapshot(unexpected=True)
        result = self.run_snapshot(extra)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("exactly the documented fields", result.stderr)
        result = self.run_snapshot(self.snapshot(repository="example/wrong"))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("does not match", result.stderr)

    def test_github_failure_is_redacted(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            gh = Path(directory) / "gh"
            gh.write_text(
                "#!/bin/sh\necho sensitive-detail >&2\nexit 23\n", encoding="utf-8"
            )
            gh.chmod(0o700)
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
        self.assertIn("GitHub repository lookup failed", result.stderr)
        self.assertNotIn("sensitive-detail", result.stderr)

    def test_live_lookup_includes_disabled_workflow_history(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            gh = Path(directory) / "gh"
            gh.write_text(
                """#!/bin/sh
case "$1 $2" in
  "repo view")
    printf '%s\n' '{"defaultBranchRef":{"name":"develop"},"nameWithOwner":"hyperledger-identus/sdk-rust"}'
    ;;
  "run list")
    case " $* " in
      *" --all "*) ;;
      *) exit 42 ;;
    esac
    printf '[{"attempt":1,"conclusion":"success","createdAt":"%s","databaseId":123,"event":"schedule","headBranch":"develop","headSha":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","status":"completed","url":"https://github.com/hyperledger-identus/sdk-rust/actions/runs/123","workflowName":"slow"}]\n' "$FAKE_CREATED_AT"
    ;;
  *) exit 43 ;;
esac
""",
                encoding="utf-8",
            )
            gh.chmod(0o700)
            environment = os.environ.copy()
            environment["PATH"] = f"{directory}:/usr/bin:/bin"
            environment["FAKE_CREATED_AT"] = datetime.now(timezone.utc).strftime(
                "%Y-%m-%dT%H:%M:%SZ"
            )
            result = subprocess.run(
                [sys.executable, str(CHECKER)],
                check=False,
                capture_output=True,
                text=True,
                env=environment,
            )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("123 passed", result.stdout)


if __name__ == "__main__":
    unittest.main()
