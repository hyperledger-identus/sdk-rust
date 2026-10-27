#!/usr/bin/env python3

import csv
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
CHECKER = REPOSITORY_ROOT / "scripts/check-oid4vp-conformance.py"
FIELDS = (
    "id",
    "section",
    "relevance",
    "status",
    "implementation_paths",
    "spec_paths",
    "test_paths",
    "limitation",
    "provenance",
    "followup_issue",
)


class Oid4vpConformanceContractTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        for relative in (
            "crates/oid4vp/src/evidence.rs",
            "openspec/specs/evidence/spec.md",
            "crates/oid4vp/tests/evidence.rs",
        ):
            target = self.root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text("evidence\n", encoding="utf-8")
        (self.root / "docs/conformance").mkdir(parents=True)
        (self.root / "docs/roadmap").mkdir(parents=True)
        (self.root / "docs/roadmap/ssi-upstream-dependency-backlog.csv").write_text(
            "id,delivery_status\nIDR-024,in_progress\n", encoding="utf-8"
        )
        required = {
            "authorization-request-core": "5.1-5.7",
            "request-object-audience-security": "5.8",
            "client-identifier-schemes": "5.9",
            "request-uri-post": "5.10",
            "dcql": "6",
            "claims-path-pointer": "7",
            "vp-token-response": "8.1",
            "direct-post": "8.2",
            "redirect-response-modes": "8.1-8.2",
            "encrypted-response": "8.3-8.3.1",
            "transaction-data": "8.4",
            "authorization-error-response": "8.5",
            "vp-token-validation": "8.6",
            "wallet-invocation": "9",
            "verifier-metadata": "11",
            "security-considerations": "14",
            "reference-harness": "profile",
        }
        self.rows = [self.row(section, identifier=identifier) for identifier, section in required.items()]
        adjacent = {
            "verifier-info": "5.11",
            "wallet-metadata": "10",
            "verifier-attestation": "12",
            "implementation-considerations": "13",
            "privacy-considerations": "15",
        }
        self.rows.extend(
            self.row(section, identifier=identifier, relevance="adjacent")
            for identifier, section in adjacent.items()
        )
        separate = {
            "digital-credentials-api": "A",
            "credential-format-profiles": "B",
            "siopv2-combination": "C",
        }
        self.rows.extend(
            self.row(
                section,
                identifier=identifier,
                relevance="separate",
                status="not-applicable",
                implementation_paths="none",
                spec_paths="none",
                test_paths="none",
                provenance="not-applicable",
            )
            for identifier, section in separate.items()
        )

    def tearDown(self) -> None:
        self.temporary.cleanup()

    @staticmethod
    def row(section: str, **overrides: str) -> dict[str, str]:
        row = {
            "id": f"section-{section.lower()}",
            "section": section,
            "relevance": "required",
            "status": "implemented",
            "implementation_paths": "crates/oid4vp/src/evidence.rs",
            "spec_paths": "openspec/specs/evidence/spec.md",
            "test_paths": "crates/oid4vp/tests/evidence.rs",
            "limitation": "Bounded synthetic evidence for the checker fixture only.",
            "provenance": "sdk-authored",
            "followup_issue": "none",
        }
        if "identifier" in overrides:
            row["id"] = overrides.pop("identifier")
        row.update(overrides)
        return row

    def write(self) -> None:
        target = self.root / "docs/conformance/oid4vp-final-wallet-core.csv"
        with target.open("w", encoding="utf-8", newline="") as destination:
            writer = csv.DictWriter(destination, fieldnames=FIELDS)
            writer.writeheader()
            writer.writerows(self.rows)

    def run_checker(self) -> subprocess.CompletedProcess[str]:
        self.write()
        return subprocess.run(
            [sys.executable, str(CHECKER), str(self.root)],
            check=False,
            capture_output=True,
            text=True,
        )

    def test_complete_matrix_passes(self) -> None:
        result = self.run_checker()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("25 rows passed", result.stdout)

    def test_missing_section_fails(self) -> None:
        self.rows = [row for row in self.rows if not row["section"].startswith("8")]
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing Final sections: 8", result.stderr)

    def test_partial_required_row_needs_evidence_and_issue(self) -> None:
        self.rows[0].update(status="partial", test_paths="none", followup_issue="none")
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("partial row requires test_paths", result.stderr)
        self.assertIn("incomplete required row needs a focused issue", result.stderr)

    def test_unsupported_row_cannot_claim_execution_evidence(self) -> None:
        self.rows[0].update(status="unsupported", followup_issue="#99")
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("unsupported row cannot claim executable evidence", result.stderr)

    def test_missing_path_and_duplicate_id_fail(self) -> None:
        self.rows[0]["implementation_paths"] = "crates/oid4vp/src/missing.rs"
        self.rows[1]["id"] = self.rows[0]["id"]
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("duplicate capability id", result.stderr)
        self.assertIn("missing evidence path crates/oid4vp/src/missing.rs", result.stderr)

    def test_delivered_backlog_rejects_required_gap(self) -> None:
        self.rows[0].update(status="partial", followup_issue="#99")
        (self.root / "docs/roadmap/ssi-upstream-dependency-backlog.csv").write_text(
            "id,delivery_status\nIDR-024,delivered\n", encoding="utf-8"
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("IDR-024 cannot be delivered with 1 incomplete required row", result.stderr)

    def test_unsafe_and_noncanonical_paths_fail(self) -> None:
        cases = {
            "/tmp/evidence.rs": "unsafe repository path",
            "../evidence.rs": "unsafe repository path",
            "crates//oid4vp/tests/evidence.rs": "unsafe repository path",
        }
        for path, message in cases.items():
            with self.subTest(path=path):
                original = self.rows[0]["test_paths"]
                self.rows[0]["test_paths"] = path
                result = self.run_checker()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(message, result.stderr)
                self.rows[0]["test_paths"] = original

    def test_symlinked_evidence_fails(self) -> None:
        link = self.root / "crates/oid4vp/tests/link.rs"
        link.symlink_to(self.root / "crates/oid4vp/tests/evidence.rs")
        self.rows[0]["test_paths"] = "crates/oid4vp/tests/link.rs"
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("not a regular non-symlink file", result.stderr)

    def test_evidence_kinds_are_path_scoped(self) -> None:
        cases = (
            ("implementation_paths", "crates/oid4vp/README.md", "not a crate implementation path"),
            ("spec_paths", "crates/oid4vp/README.md", "not a canonical OpenSpec path"),
            ("test_paths", "crates/oid4vp/src/evidence.rs", "not an executable crate test path"),
        )
        readme = self.root / "crates/oid4vp/README.md"
        readme.write_text("evidence\n", encoding="utf-8")
        for field, path, message in cases:
            with self.subTest(field=field):
                original = self.rows[0][field]
                self.rows[0][field] = path
                result = self.run_checker()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(message, result.stderr)
                self.rows[0][field] = original

    def test_required_capability_cannot_be_downgraded_or_moved(self) -> None:
        self.rows[0]["relevance"] = "adjacent"
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("capability contract changed", result.stderr)
        self.rows[0]["relevance"] = "required"
        self.rows[0]["section"] = "6"
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("section-or-relevance-mismatch=authorization-request-core", result.stderr)

    def test_closed_inventory_rejects_missing_adjacent_row(self) -> None:
        self.rows = [row for row in self.rows if row["id"] != "verifier-info"]
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing=verifier-info", result.stderr)

    def test_schema_enums_provenance_and_followup_rules_fail_closed(self) -> None:
        mutations = (
            ("relevance", "unknown", "unknown relevance"),
            ("status", "unknown", "unknown status"),
            ("provenance", "unknown", "unknown provenance"),
            ("followup_issue", "#99", "implemented row cannot retain a follow-up issue"),
        )
        for field, value, message in mutations:
            with self.subTest(field=field):
                original = self.rows[0][field]
                self.rows[0][field] = value
                result = self.run_checker()
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(message, result.stderr)
                self.rows[0][field] = original

    def test_separate_profile_and_limitation_rules_fail_closed(self) -> None:
        separate = next(row for row in self.rows if row["section"] == "A")
        separate.update(status="partial", provenance="sdk-authored", followup_issue="#99",
                        implementation_paths="crates/oid4vp/src/evidence.rs",
                        spec_paths="openspec/specs/evidence/spec.md",
                        test_paths="crates/oid4vp/tests/evidence.rs")
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("separate capability must be not-applicable", result.stderr)

        self.setUp_rows_after_mutation()
        profile = next(row for row in self.rows if row["id"] == "reference-harness")
        self.rows.remove(profile)
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing reference-harness profile evidence row", result.stderr)

        self.setUp_rows_after_mutation()
        self.rows[0]["limitation"] = "x" * 513
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("limitation exceeds 512 bytes", result.stderr)

    def setUp_rows_after_mutation(self) -> None:
        self.tearDown()
        self.setUp()


if __name__ == "__main__":
    unittest.main()
