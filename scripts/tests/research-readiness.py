#!/usr/bin/env python3
"""Contract tests for check-research-readiness.py."""

from __future__ import annotations

import subprocess
import tempfile
import unittest
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
CHECKER = REPOSITORY_ROOT / "scripts" / "check-research-readiness.py"

VALID = """# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-09-07
Source retrieval date: 2026-09-07
Research blockers: none

## Problem and existing implementation

The current implementation and two consumer call shapes are inspected.

## Normative sources

Primary source: https://example.com/spec at revision 1234567. License: MIT.
Provenance is recorded from the authoritative repository.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| focused | 1.2.3 | `adopt` | cohesive | Removal of required support. |

## Compatibility and dependency evidence

Exact version and minimal feature selection, MSRV, target matrix, direct and
resolved dependency cone, public API, wire compatibility, facade boundary and
rollback were checked.

## Security, privacy and maintenance evidence

Unsafe and native code, supply-chain advisories, maintenance, release and
security posture, and protocol/draft currency were inspected.

## Rejected or deferred candidates

The broad alternative is `not-adopt` because of coupling.

## Open questions and blockers

None.

## Evidence commands

The exact command passed. No unrun checks remain.
"""


class ResearchReadinessTests(unittest.TestCase):
    def run_checker(
        self, text: str | None, *extra: str
    ) -> subprocess.CompletedProcess[str]:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            change = root / "openspec" / "changes" / "example"
            change.mkdir(parents=True)
            if text is not None:
                (change / "research.md").write_text(text, encoding="utf-8")
            return subprocess.run(
                [str(CHECKER), str(root), *extra],
                capture_output=True,
                check=False,
                text=True,
            )

    def test_ready_full_record_passes(self) -> None:
        result = self.run_checker(VALID, "--require-ready")
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_missing_record_fails(self) -> None:
        result = self.run_checker(None)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing research.md", result.stderr)

    def test_draft_is_structural_but_not_ready(self) -> None:
        draft = VALID.replace("Research status: ready", "Research status: draft")
        draft = draft.replace("Research blockers: none", "Research blockers: blocked - source")
        self.assertEqual(self.run_checker(draft).returncode, 0)
        result = self.run_checker(draft, "--require-ready")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("research is not ready", result.stderr)

    def test_ready_record_cannot_have_blocker(self) -> None:
        blocked = VALID.replace(
            "Research blockers: none", "Research blockers: blocked - source"
        )
        result = self.run_checker(blocked)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("cannot be ready", result.stderr)

    def test_full_record_requires_dependency_evidence(self) -> None:
        incomplete = VALID.replace("dependency cone", "resolved packages")
        result = self.run_checker(incomplete)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("dependency cone", result.stderr)

    def test_full_record_requires_security_and_compatibility_evidence(self) -> None:
        incomplete = VALID.replace("Unsafe and native code", "Implementation code")
        incomplete = incomplete.replace("wire compatibility", "serialization")
        result = self.run_checker(incomplete)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("unsafe and native-code evidence", result.stderr)
        self.assertIn("public and wire compatibility", result.stderr)

    def test_full_record_rejects_empty_section(self) -> None:
        incomplete = VALID.replace(
            "The current implementation and two consumer call shapes are inspected.",
            "...",
        )
        result = self.run_checker(incomplete)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("section is empty", result.stderr)

    def test_routine_record_may_use_not_applicable(self) -> None:
        routine = VALID.replace("Research class: protocol", "Research class: routine")
        routine = routine.replace("`adopt`", "`not-applicable`")
        result = self.run_checker(routine, "--require-ready")
        self.assertEqual(result.returncode, 0, result.stderr)


if __name__ == "__main__":
    unittest.main()
