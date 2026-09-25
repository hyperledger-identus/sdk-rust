#!/usr/bin/env python3

import csv
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[2]
CHECKER = REPOSITORY_ROOT / "scripts/check-oid4vci-conformance.py"
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


class Oid4vciConformanceContractTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        for relative in ("src/evidence.rs", "specs/evidence.md", "tests/evidence.rs"):
            target = self.root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text("evidence\n", encoding="utf-8")
        (self.root / "docs/conformance").mkdir(parents=True)
        self.rows = [self.row(str(section)) for section in range(4, 10)]
        self.rows.extend(
            [
                self.row(
                    "10",
                    identifier="encrypted-credential-exchange",
                    relevance="out-of-scope",
                    status="unsupported",
                    implementation_paths="none",
                    spec_paths="none",
                    test_paths="none",
                    provenance="not-applicable",
                ),
                self.row(
                    "11",
                    identifier="notification-endpoint",
                    relevance="out-of-scope",
                    status="unsupported",
                    implementation_paths="none",
                    spec_paths="none",
                    test_paths="none",
                    provenance="not-applicable",
                ),
                self.row("12"),
            ]
        )
        self.rows.append(
            self.row(
                "profile",
                identifier="cross-consumer-profile",
                status="missing",
                implementation_paths="none",
                spec_paths="none",
                test_paths="none",
                provenance="reference-only",
                followup_issue="#99",
            )
        )

    def tearDown(self) -> None:
        self.temporary.cleanup()

    @staticmethod
    def row(section: str, **overrides: str) -> dict[str, str]:
        row = {
            "id": f"section-{section.replace('.', '-')}",
            "section": section,
            "relevance": "required",
            "status": "implemented",
            "implementation_paths": "src/evidence.rs",
            "spec_paths": "specs/evidence.md",
            "test_paths": "tests/evidence.rs",
            "limitation": "Bounded synthetic evidence for the checker fixture only.",
            "provenance": "sdk-authored",
            "followup_issue": "none",
        }
        if "identifier" in overrides:
            row["id"] = overrides.pop("identifier")
        row.update(overrides)
        return row

    def write(self) -> None:
        target = self.root / "docs/conformance/oid4vci-final-wallet-core.csv"
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
        self.assertIn("10 rows passed", result.stdout)

    def test_missing_section_fails(self) -> None:
        self.rows = [row for row in self.rows if row["section"] != "8"]
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing Final sections: 8", result.stderr)

    def test_missing_required_row_needs_issue(self) -> None:
        self.rows[0].update(
            status="missing",
            implementation_paths="none",
            spec_paths="none",
            test_paths="none",
            provenance="not-applicable",
            followup_issue="none",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing required row needs a focused issue", result.stderr)

    def test_implemented_row_needs_complete_evidence(self) -> None:
        self.rows[0]["test_paths"] = "none"
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("implemented row requires test_paths", result.stderr)

    def test_missing_path_fails(self) -> None:
        self.rows[0]["implementation_paths"] = "src/missing.rs"
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing evidence path src/missing.rs", result.stderr)

    def test_reference_only_cannot_claim_execution_evidence(self) -> None:
        profile = self.rows[-1]
        profile["implementation_paths"] = "src/evidence.rs"
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("reference-only row cannot claim SDK execution evidence", result.stderr)

    def test_duplicate_id_fails(self) -> None:
        self.rows[1]["id"] = self.rows[0]["id"]
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("duplicate capability id", result.stderr)

    def test_required_unsupported_dispositions_are_dedicated(self) -> None:
        self.rows = [
            row for row in self.rows if row["id"] != "notification-endpoint"
        ]
        encryption = next(
            row
            for row in self.rows
            if row["id"] == "encrypted-credential-exchange"
        )
        encryption.update(
            section="10-11",
            relevance="required",
            status="partial",
            implementation_paths="src/evidence.rs",
            spec_paths="specs/evidence.md",
            test_paths="tests/evidence.rs",
            provenance="sdk-authored",
        )
        result = self.run_checker()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(
            "encrypted-credential-exchange: disposition must remain",
            result.stderr,
        )
        self.assertIn(
            "missing required disposition row: notification-endpoint",
            result.stderr,
        )


if __name__ == "__main__":
    unittest.main()
