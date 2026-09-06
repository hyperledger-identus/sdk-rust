#!/usr/bin/env python3

from __future__ import annotations

import hashlib
import shutil
import subprocess
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
CHECKER = REPOSITORY_ROOT / "scripts/check-openspec-archive.py"
CAPABILITY = "example-capability"
REQUIREMENT = "Existing boundary"
CANONICAL_BLOCK = textwrap.dedent(
    """\
    ### Requirement: Existing boundary

    The system SHALL preserve the first normative paragraph.

    The system SHALL preserve the unrelated second paragraph.

    #### Scenario: First behavior remains

    - **WHEN** the first behavior is exercised
    - **THEN** the first result remains observable

    #### Scenario: Unrelated behavior remains

    - **WHEN** the unrelated behavior is exercised
    - **THEN** the unrelated result remains observable
    """
).rstrip()


def canonical_spec(block: str = CANONICAL_BLOCK) -> str:
    return (
        "# Example Specification\n\n"
        "## Purpose\n\n"
        "Exercise archive preservation.\n\n"
        "## Requirements\n\n"
        f"{block}\n"
    )


def normalized_hash(block: str) -> str:
    lines = block.replace("\r\n", "\n").replace("\r", "\n").split("\n")
    lines = [line.rstrip() for line in lines]
    while lines and not lines[0]:
        lines.pop(0)
    while lines and not lines[-1]:
        lines.pop()
    return hashlib.sha256("\n".join(lines).encode("utf-8")).hexdigest()


class OpenSpecArchivePreservationTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        self.canonical_path = self.root / "openspec/specs" / CAPABILITY / "spec.md"
        self.delta_path = (
            self.root / "openspec/changes/example-change/specs" / CAPABILITY / "spec.md"
        )
        self.canonical_path.parent.mkdir(parents=True)
        self.delta_path.parent.mkdir(parents=True)
        (self.root / "openspec/changes/archive").mkdir(parents=True)
        self.canonical_path.write_text(canonical_spec(), encoding="utf-8")

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def write_delta(self, body: str, capability: str = CAPABILITY) -> None:
        target = self.root / "openspec/changes/example-change/specs" / capability / "spec.md"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(textwrap.dedent(body), encoding="utf-8")

    def write_intent(self, digest: str, reason: str = "Replace obsolete behavior") -> None:
        path = self.root / "openspec/changes/example-change/archive-intent.toml"
        path.write_text(
            textwrap.dedent(
                f'''\
                [[modified_requirement]]
                capability = "{CAPABILITY}"
                requirement = "{REQUIREMENT}"
                canonical_sha256 = "{digest}"
                reason = "{reason}"
                '''
            ),
            encoding="utf-8",
        )

    def run_checker(self) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            [sys.executable, str(CHECKER), str(self.root)],
            check=False,
            capture_output=True,
            text=True,
        )

    def partial_delta(self) -> str:
        return """\
        ## MODIFIED Requirements

        ### Requirement: Existing boundary

        The system SHALL expose only the first behavior.

        #### Scenario: First behavior remains

        - **WHEN** the first behavior is exercised
        - **THEN** the first result remains observable
        """

    def test_issue_102_partial_replacement_fails(self) -> None:
        self.write_delta(self.partial_delta())
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "lossy MODIFIED requirement lacks exact intent: "
            f"{CAPABILITY} / {REQUIREMENT}",
            result.stderr,
        )
        self.assertNotIn("unrelated second paragraph", result.stderr)

    def test_complete_additive_replacement_passes(self) -> None:
        self.write_delta(
            f"""\
            ## MODIFIED Requirements

            {CANONICAL_BLOCK}

            #### Scenario: Added behavior

            - **WHEN** added behavior is exercised
            - **THEN** its result is observable
            """
        )
        result = self.run_checker()
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_exact_reasoned_intent_permits_rewrite(self) -> None:
        self.write_delta(self.partial_delta())
        self.write_intent(normalized_hash(CANONICAL_BLOCK))
        result = self.run_checker()
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_stale_intent_hash_fails(self) -> None:
        self.write_delta(self.partial_delta())
        self.write_intent("0" * 64)
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(f"stale canonical hash for {CAPABILITY} / {REQUIREMENT}", result.stderr)

    def test_empty_intent_reason_fails(self) -> None:
        self.write_delta(self.partial_delta())
        self.write_intent(normalized_hash(CANONICAL_BLOCK), reason="")
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("capability, requirement and reason must be nonempty", result.stderr)

    def test_duplicate_intent_fails(self) -> None:
        self.write_delta(self.partial_delta())
        path = self.root / "openspec/changes/example-change/archive-intent.toml"
        entry = textwrap.dedent(
            f'''\
            [[modified_requirement]]
            capability = "{CAPABILITY}"
            requirement = "{REQUIREMENT}"
            canonical_sha256 = "{normalized_hash(CANONICAL_BLOCK)}"
            reason = "Intentional replacement"
            '''
        )
        path.write_text(entry + "\n" + entry, encoding="utf-8")
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(f"duplicate archive intent: {CAPABILITY} / {REQUIREMENT}", result.stderr)

    def test_unused_intent_fails_for_safe_addition(self) -> None:
        self.write_delta(f"## MODIFIED Requirements\n\n{CANONICAL_BLOCK}\n")
        self.write_intent(normalized_hash(CANONICAL_BLOCK))
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(f"unused archive intent: {CAPABILITY} / {REQUIREMENT}", result.stderr)

    def test_rename_then_additive_modification_passes(self) -> None:
        renamed_body = "\n".join(CANONICAL_BLOCK.splitlines()[1:])
        self.write_delta(
            f"""\
            ## RENAMED Requirements

            - FROM: `### Requirement: {REQUIREMENT}`
            - TO: `### Requirement: Renamed boundary`

            ## MODIFIED Requirements

            ### Requirement: Renamed boundary
            {renamed_body}

            #### Scenario: Added after rename

            - **WHEN** the renamed requirement is extended
            - **THEN** the canonical source content remains
            """
        )
        result = self.run_checker()
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_new_capability_with_added_requirement_passes(self) -> None:
        self.delta_path.unlink(missing_ok=True)
        self.write_delta(
            """\
            ## ADDED Requirements

            ### Requirement: New capability remains independent

            The system SHALL permit a new capability.

            #### Scenario: New behavior

            - **WHEN** the new capability is archived
            - **THEN** no canonical modified block is required
            """,
            capability="new-capability",
        )
        result = self.run_checker()
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_symlinked_capability_input_fails_closed(self) -> None:
        shutil.rmtree(self.delta_path.parent)
        self.delta_path.parent.symlink_to(self.canonical_path.parent, target_is_directory=True)
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("symlinked capability input is forbidden", result.stderr)

    def test_symlinked_canonical_parent_fails_closed(self) -> None:
        self.write_delta(f"## MODIFIED Requirements\n\n{CANONICAL_BLOCK}\n")
        source = self.root / "canonical-source"
        self.canonical_path.parent.rename(source)
        self.canonical_path.parent.symlink_to(source, target_is_directory=True)
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("symlinked OpenSpec input is forbidden", result.stderr)


if __name__ == "__main__":
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(
        OpenSpecArchivePreservationTests
    )
    outcome = unittest.TextTestRunner(verbosity=2).run(suite)
    if outcome.wasSuccessful():
        print("openspec archive preservation tests: passed")
    raise SystemExit(not outcome.wasSuccessful())
