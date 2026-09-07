#!/usr/bin/env python3
"""Focused tests for the SDK constraint-governance checker."""

from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "check_constraints", ROOT / "scripts/check-constraints.py"
)
assert SPEC is not None and SPEC.loader is not None
CHECKER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKER)


ENTRY_TEMPLATE = """
[[entries]]
id = "{entry_id}"
kind = "{kind}"
category = "compatibility"
state = "{state}"
summary = "A meaningful summary."
scope = "Every supported surface."
canonical_source = "docs/source.md"
authority = "Accepted decision."
rationale = "A measured reason."
consumer_impact = "Consumers can observe the consequence."
enforcement = ["A deterministic gate."]
owner = "SDK maintainers"
review_triggers = ["The source changes."]
activation = "{activation}"
rollback = "Restore the previous state through a focused decision."
value = "{value}"
{value_source}
"""


VALID_CHANGE = """# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/166
Constraint blockers: none

## Existing entries affected

No effective entry changes.

## Introduced or changed constraints

The governance constraint is explicit.

## Introduced or changed limitations

No product limitation is introduced.

## Consumer and product impact

Consumers receive clearer evidence.

## Activation and rollback

Activation occurs on merge and rollback reverts it.

## Evidence

The focused test is deterministic evidence.
"""


class ConstraintGovernanceTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        (self.root / "docs/governance").mkdir(parents=True)
        (self.root / "docs/architecture").mkdir(parents=True)
        (self.root / "docs/adr").mkdir(parents=True)
        (self.root / "docs/source.md").write_text("# Source\n", encoding="utf-8")
        (self.root / "docs/adr/0063.md").write_text("# ADR\n", encoding="utf-8")
        (self.root / "docs/architecture/sdk-support-policy.toml").write_text(
            '[toolchains]\nmsrv = "1.85.0"\n', encoding="utf-8"
        )
        self.change = self.root / "openspec/changes/example-change"
        self.change.mkdir(parents=True)
        (self.change / "constraints.md").write_text(VALID_CHANGE, encoding="utf-8")
        self.write_index()

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def write_index(
        self,
        *,
        effective_state: str = "effective",
        effective_value: str = "1.85.0",
        target_state: str = "target",
        target_activation: str = "Issue #154 records the focused decision.",
    ) -> None:
        header = (
            'schema_version = 1\npolicy_revision = "2026-09-07"\n'
            'governing_adr = "docs/adr/0063.md"\n'
        )
        effective = ENTRY_TEMPLATE.format(
            entry_id="SDK-COMPAT-002",
            kind="budget",
            state=effective_state,
            activation="Already effective through support policy.",
            value=effective_value,
            value_source='value_source = "toolchains.msrv"',
        )
        target = ENTRY_TEMPLATE.format(
            entry_id="SDK-COMPAT-003",
            kind="budget",
            state=target_state,
            activation=target_activation,
            value="1.95.0",
            value_source="",
        )
        (self.root / "docs/governance/sdk-constraints.toml").write_text(
            header + effective + target, encoding="utf-8"
        )

    def failures(self, require_ready: bool = False) -> list[str]:
        return CHECKER.validate(self.root, None, require_ready)

    def test_valid_index_and_directed_material_change_pass(self) -> None:
        self.assertEqual([], self.failures(require_ready=True))

    def test_missing_required_entry_field_fails(self) -> None:
        path = self.root / "docs/governance/sdk-constraints.toml"
        path.write_text(
            path.read_text(encoding="utf-8").replace(
                'consumer_impact = "Consumers can observe the consequence."\n',
                "",
                1,
            ),
            encoding="utf-8",
        )
        self.assertTrue(any("consumer_impact" in item for item in self.failures()))

    def test_duplicate_identifier_fails(self) -> None:
        path = self.root / "docs/governance/sdk-constraints.toml"
        text = path.read_text(encoding="utf-8").replace(
            "SDK-COMPAT-003", "SDK-COMPAT-002"
        )
        path.write_text(text, encoding="utf-8")
        self.assertTrue(any("duplicate" in item for item in self.failures()))

    def test_missing_canonical_source_fails(self) -> None:
        path = self.root / "docs/source.md"
        path.unlink()
        self.assertTrue(any("does not exist" in item for item in self.failures()))

    def test_effective_msrv_drift_fails(self) -> None:
        self.write_index(effective_value="1.86.0")
        self.assertTrue(any("support-policy" in item for item in self.failures()))

    def test_target_cannot_be_marked_effective(self) -> None:
        self.write_index(target_state="effective")
        self.assertTrue(any("must remain a target" in item for item in self.failures()))

    def test_target_cannot_claim_existing_activation(self) -> None:
        self.write_index(target_activation="Already effective through an ADR.")
        self.assertTrue(
            any("claims it is already effective" in item for item in self.failures())
        )

    def test_missing_change_record_fails(self) -> None:
        (self.change / "constraints.md").unlink()
        self.assertTrue(
            any("missing constraints.md" in item for item in self.failures())
        )

    def test_proposed_material_change_plans_but_is_not_ready(self) -> None:
        path = self.change / "constraints.md"
        path.write_text(
            VALID_CHANGE.replace(
                "Decision status: directed", "Decision status: proposed"
            ),
            encoding="utf-8",
        )
        self.assertEqual([], self.failures(require_ready=False))
        self.assertTrue(
            any(
                "requires directed" in item
                for item in self.failures(require_ready=True)
            )
        )

    def test_effective_constraint_id_is_a_valid_material_reference(self) -> None:
        path = self.change / "constraints.md"
        path.write_text(
            VALID_CHANGE.replace(
                "https://github.com/hyperledger-identus/sdk-rust/issues/166",
                "SDK-COMPAT-002",
            ),
            encoding="utf-8",
        )
        self.assertEqual([], self.failures(require_ready=True))

    def test_target_constraint_id_is_not_material_authority(self) -> None:
        path = self.change / "constraints.md"
        path.write_text(
            VALID_CHANGE.replace(
                "https://github.com/hyperledger-identus/sdk-rust/issues/166",
                "SDK-COMPAT-003",
            ),
            encoding="utf-8",
        )
        self.assertTrue(
            any("effective constraint ID" in item for item in self.failures(True))
        )

    def test_empty_change_section_fails(self) -> None:
        path = self.change / "constraints.md"
        path.write_text(
            VALID_CHANGE.replace(
                "## Evidence\n\nThe focused test is deterministic evidence.",
                "## Evidence\n\n...",
            ),
            encoding="utf-8",
        )
        self.assertTrue(
            any("empty section: Evidence" in item for item in self.failures())
        )


if __name__ == "__main__":
    unittest.main(verbosity=2)
