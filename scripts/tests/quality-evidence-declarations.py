#!/usr/bin/env python3
"""Mutation tests for the closed quality-evidence declaration contract."""

from __future__ import annotations

from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib


REGISTRY = Path("docs/architecture/quality-evidence-declarations.toml")
PLAN = Path("docs/architecture/quality-evidence-plan.md")
CATALOG = Path("docs/conformance/cross-language-vector-catalog.toml")
CHECKER = Path("scripts/check-quality-evidence-declarations.py")


def copy_file(source_root: Path, fixture_root: Path, relative: Path) -> None:
    destination = fixture_root / relative
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source_root / relative, destination)


def build_fixture(source_root: Path, fixture_root: Path) -> None:
    for relative in (REGISTRY, PLAN, CATALOG):
        copy_file(source_root, fixture_root, relative)
    registry = tomllib.loads((source_root / REGISTRY).read_text(encoding="utf-8"))
    paths: set[Path] = set()
    for declaration in registry["declarations"]:
        for obligation in declaration["obligations"]:
            for selector in obligation["selectors"]:
                paths.add(Path(selector.rsplit("#", 1)[0]))
            receipt = obligation["receipt"]
            if receipt.startswith("source:"):
                paths.add(Path(receipt[7:]))
    for relative in sorted(paths):
        copy_file(source_root, fixture_root, relative)


def run_checker(source_root: Path, fixture_root: Path, *arguments: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(source_root / CHECKER), str(fixture_root), *arguments],
        check=False,
        capture_output=True,
        text=True,
    )


def replace_once(content: str, before: str, after: str) -> str:
    if content.count(before) != 1:
        raise AssertionError(f"mutation anchor must occur exactly once: {before!r}")
    return content.replace(before, after, 1)


def obligation_block(content: str, class_name: str, disposition: str) -> str:
    marker = "[[declarations.obligations]]"
    needle = f'class = "{class_name}"\ndisposition = "{disposition}"'
    candidates = [marker + part for part in content.split(marker)[1:] if needle in part]
    if len(candidates) != 1:
        raise AssertionError(
            f"expected one {class_name}/{disposition} obligation, found {len(candidates)}"
        )
    return candidates[0]


def replace_field(block: str, key: str, value: str) -> str:
    updated, count = re.subn(
        rf"(?m)^{re.escape(key)}\s*=.*$", f"{key} = {value}", block, count=1
    )
    if count != 1:
        raise AssertionError(f"expected one field in obligation: {key}")
    return updated


def expect_failure(
    source_root: Path,
    fixture_root: Path,
    baseline: str,
    name: str,
    mutated: str,
    diagnostic: str,
) -> None:
    registry_path = fixture_root / REGISTRY
    registry_path.write_text(mutated, encoding="utf-8")
    result = run_checker(source_root, fixture_root, "--render")
    registry_path.write_text(baseline, encoding="utf-8")
    if result.returncode == 0:
        raise AssertionError(f"{name}: checker unexpectedly accepted the mutation")
    if diagnostic not in result.stderr:
        raise AssertionError(
            f"{name}: expected diagnostic {diagnostic!r}; stderr was:\n{result.stderr}"
        )


def main() -> int:
    source_root = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).parents[2]).resolve()
    with tempfile.TemporaryDirectory(prefix="quality-evidence-") as temporary:
        fixture_root = Path(temporary)
        build_fixture(source_root, fixture_root)
        baseline = (fixture_root / REGISTRY).read_text(encoding="utf-8")

        result = run_checker(source_root, fixture_root)
        if result.returncode != 0:
            raise AssertionError(f"baseline registry failed:\n{result.stderr}")

        omitted_budget = replace_once(
            baseline,
            'budget = "Exhaust ASCII bytes 0..127 in each grammar position and verify exact 131,072-byte DID and 139,264-byte DID URL limits."\n',
            "",
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "required-field omission",
            omitted_budget,
            "missing keys: budget",
        )

        absent_rationale = replace_once(
            baseline,
            'rationale         = "No A1 consumer has declared a DID syntax latency or throughput objective; add a benchmark only with a stable fixture, comparable environment, statistic, sample budget, and threshold."',
            'rationale         = ""',
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "not-applicable rationale",
            absent_rationale,
            "must explain the bounded omission",
        )

        satisfied_fuzz = obligation_block(baseline, "fuzz", "satisfied")
        stale_fuzz = replace_field(satisfied_fuzz, "recorded_at", '"2026-09-01"')
        stale_receipt = replace_once(baseline, satisfied_fuzz, stale_fuzz)
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "stale receipt",
            stale_receipt,
            "receipt is stale",
        )

        unowned_fuzz = replace_field(satisfied_fuzz, "disposition", '"required"')
        unowned_fuzz = replace_field(unowned_fuzz, "freshness", '"pending"')
        unowned_fuzz = replace_field(unowned_fuzz, "freshness_days", "0")
        unowned_fuzz = replace_field(unowned_fuzz, "receipt_kind", '"none"')
        unowned_fuzz = replace_field(unowned_fuzz, "receipt", '""')
        unowned_fuzz = replace_field(unowned_fuzz, "evidence_revision", '""')
        unowned_fuzz = replace_field(unowned_fuzz, "recorded_at", '""')
        unowned_fuzz = replace_field(unowned_fuzz, "debt_status", '"open"')
        unowned_fuzz = replace_field(unowned_fuzz, "debt_issue", "0")
        unowned_debt = replace_once(baseline, satisfied_fuzz, unowned_fuzz)
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "unowned required evidence debt",
            unowned_debt,
            "required evidence must have open debt with a positive debt issue",
        )

        missing_selector = replace_once(
            baseline,
            "crates/did/tests/did_syntax.rs#generic_did_ascii_grammar_is_exhaustive",
            "crates/did/tests/did_syntax.rs#missing_property_invariant",
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "unresolvable selector",
            missing_selector,
            "selector needle not found",
        )

        unknown_field = replace_once(
            baseline,
            'status           = "active"',
            'status           = "active"\nfree_form_policy = "not allowed"',
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "unknown free-form field",
            unknown_field,
            "unknown keys: free_form_policy",
        )

        declaration_offset = baseline.index("[[declarations]]")
        next_declaration_offset = baseline.index("[[declarations]]", declaration_offset + 1)
        additive_declaration = baseline[declaration_offset:next_declaration_offset]
        additive_declaration = replace_once(
            additive_declaration,
            'id = "did.syntax.quality.v1"',
            'id = "example.portable.quality.v1"',
        )
        additive_declaration = replace_once(
            additive_declaration,
            'capability = "did.syntax"',
            'capability = "example.portable"',
        )
        additive = baseline.rstrip() + "\n\n" + additive_declaration
        (fixture_root / REGISTRY).write_text(additive, encoding="utf-8")
        result = run_checker(source_root, fixture_root, "--render")
        (fixture_root / REGISTRY).write_text(baseline, encoding="utf-8")
        expected_count = baseline.count("[[declarations]]") + 1
        if result.returncode != 0 or f"Declaration count: `{expected_count}`" not in result.stdout:
            raise AssertionError(f"additive declaration was not accepted:\n{result.stderr}")

        drifted_plan = (fixture_root / PLAN).read_text(encoding="utf-8") + "\nmanual edit\n"
        (fixture_root / PLAN).write_text(drifted_plan, encoding="utf-8")
        result = run_checker(source_root, fixture_root)
        if result.returncode == 0 or "deterministic plan drift" not in result.stderr:
            raise AssertionError(f"plan drift was not rejected:\n{result.stderr}")

    print("quality-evidence mutations: 8 passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
