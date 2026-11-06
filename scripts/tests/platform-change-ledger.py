#!/usr/bin/env python3
"""Mutation tests for the closed platform change-ledger contract."""

from __future__ import annotations

from pathlib import Path
import os
import shutil
import subprocess
import sys
import tempfile


LEDGER = Path("docs/architecture/identus-platform-change-ledger.toml")
REPORT = Path("docs/architecture/identus-platform-change-ledger.md")
CHECKER = Path("scripts/check-platform-change-ledger.py")
FIXTURE = Path("scripts/tests/fixtures/platform-change-ledger/ledger.toml")
DEPENDENCIES = (
    Path("docs/conformance/cross-language-vector-catalog.toml"),
    Path("docs/architecture/language-adapter-mappings.toml"),
    Path("docs/architecture/quality-evidence-declarations.toml"),
)
IMPACT_FILES = (
    Path("openspec/changes/enforce-consumer-change-ledger/preimplementation.json"),
    Path("openspec/changes/enforce-consumer-change-ledger/compatibility-impact.toml"),
)
ARCHIVED_CHANGE = Path("openspec/changes/archive/2026-10-25-enforce-consumer-change-ledger")


def copy(
    source_root: Path,
    fixture_root: Path,
    relative: Path,
    source_relative: Path | None = None,
) -> None:
    destination = fixture_root / relative
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source_root / (source_relative or relative), destination)


def build_fixture(source_root: Path, fixture_root: Path) -> None:
    destination = fixture_root / LEDGER
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source_root / FIXTURE, destination)
    for relative in DEPENDENCIES:
        copy(source_root, fixture_root, relative)
    for relative in IMPACT_FILES:
        copy(source_root, fixture_root, relative, ARCHIVED_CHANGE / relative.name)
    synthetic_root = fixture_root / "openspec/changes/synthetic-did-migration"
    synthetic_root.mkdir(parents=True)
    (synthetic_root / "preimplementation.json").write_text(
        '{"issue":492,"contractHeadSha":"0000000000000000000000000000000000000001"}\n',
        encoding="utf-8",
    )
    (synthetic_root / "compatibility-impact.toml").write_text(
        """schema_version = 1
issue          = 492
disposition    = "consumer-visible"
scope          = "consumer-contract"
rationale      = "This synthetic declaration binds the noncanonical DID migration fixture to its exact ledger record for contract testing only."
ledger_ids     = [ "did.syntax.typescript-migration.v1" ]
""",
        encoding="utf-8",
    )


def run(source_root: Path, fixture_root: Path, *arguments: str) -> subprocess.CompletedProcess[str]:
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


def expect_failure(
    source_root: Path,
    fixture_root: Path,
    baseline: str,
    name: str,
    mutated: str,
    diagnostic: str,
) -> None:
    path = fixture_root / LEDGER
    path.write_text(mutated, encoding="utf-8")
    result = run(source_root, fixture_root)
    path.write_text(baseline, encoding="utf-8")
    if result.returncode == 0:
        raise AssertionError(f"{name}: checker unexpectedly accepted mutation")
    if diagnostic not in result.stderr:
        raise AssertionError(f"{name}: expected {diagnostic!r}; stderr was:\n{result.stderr}")


def main() -> int:
    source_root = Path(sys.argv[1] if len(sys.argv) > 1 else Path(__file__).parents[2]).resolve()
    with tempfile.TemporaryDirectory(prefix="platform-change-ledger-") as temporary:
        fixture_root = Path(temporary)
        build_fixture(source_root, fixture_root)
        baseline_path = fixture_root / LEDGER
        baseline = baseline_path.read_text(encoding="utf-8")

        result = run(source_root, fixture_root, "--write")
        if result.returncode != 0:
            raise AssertionError(f"synthetic fixture failed:\n{result.stderr}")
        result = run(source_root, fixture_root)
        if result.returncode != 0:
            raise AssertionError(f"rendered synthetic fixture failed:\n{result.stderr}")
        result = run(source_root, fixture_root, "--merge-ready")
        if result.returncode == 0 or "merge-ready records require" not in result.stderr:
            raise AssertionError(f"proposed fixture passed merge readiness:\n{result.stderr}")
        merge_ready = replace_once(
            replace_once(
                baseline,
                'lifecycle_state                   = "proposed"',
                'lifecycle_state                   = "active"',
            ),
            "pull_request                      = 0",
            "pull_request                      = 492",
        )
        baseline_path.write_text(merge_ready, encoding="utf-8")
        result = run(source_root, fixture_root, "--write", "--merge-ready", "--skip-impacts")
        if result.returncode != 0:
            raise AssertionError(f"active record failed merge readiness:\n{result.stderr}")
        baseline_path.write_text(baseline, encoding="utf-8")
        run(source_root, fixture_root, "--write")

        legacy_root = fixture_root / "openspec/changes/activate-first-crate-release-train"
        legacy_root.mkdir(parents=True)
        (legacy_root / "preimplementation.json").write_text(
            '{"issue":1,"contractHeadSha":"0000000000000000000000000000000000000001"}\n',
            encoding="utf-8",
        )
        grandfathered = replace_once(
            baseline,
            "grandfathered_changes             = [  ]",
            'grandfathered_changes             = [ "activate-first-crate-release-train" ]',
        )
        baseline_path.write_text(grandfathered, encoding="utf-8")
        result = run(source_root, fixture_root, "--write")
        if result.returncode != 0:
            raise AssertionError(f"explicit Git-less grandfathering failed:\n{result.stderr}")
        shutil.rmtree(legacy_root)
        activation_exempt = replace_once(
            baseline,
            "grandfathered_changes             = [  ]",
            'grandfathered_changes             = [ "enforce-consumer-change-ledger" ]',
        )
        baseline_path.write_text(activation_exempt, encoding="utf-8")
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "activation change cannot be grandfathered" not in result.stderr:
            raise AssertionError(f"activation change was grandfathered:\n{result.stderr}")
        baseline_path.write_text(baseline, encoding="utf-8")
        run(source_root, fixture_root, "--write")

        mutations = (
            ("missing field", 'issue                             = 492\n', "", "missing keys: issue"),
            ("unknown field", 'issue                             = 492\n', 'issue                             = 492\nfree_form = "forbidden"\n', "unknown keys: free_form"),
            ("class matrix", 'change_class                      = "breaking"', 'change_class                      = "additive"', "additive and fix records must be compatible"),
            ("merge-ready PR", 'lifecycle_state                   = "proposed"', 'lifecycle_state                   = "active"', "must be positive outside proposed state"),
            ("dangling vector", 'vector_ids                        = [ "did-url.syntax.invalid-percent", "did-url.syntax.valid-components", "did.syntax.invalid-prefix", "did.syntax.valid-basic" ]', 'vector_ids                        = [ "did-url.syntax.invalid-percent", "did-url.syntax.valid-components", "did.syntax.invalid-prefix", "did.syntax.unknown" ]', "vector ID is missing or inactive"),
            ("capability mismatch", 'capability                        = "did.syntax"', 'capability                        = "did.resolution"', "vector capability mismatch"),
            ("dangling quality", 'did.syntax.quality.v1" ]', 'did.syntax.quality.v2" ]', "quality declaration is missing or inactive"),
            ("dangling mapping", 'did.value.typescript.legacy-v1" ]', 'did.value.typescript.legacy-v2" ]', "mapping ID is missing or inactive"),
            ("mapping disposition", 'mapping_disposition               = "required"', 'mapping_disposition               = "not-applicable"', "requires no mapping IDs"),
            ("unsafe security policy", 'change_class                      = "breaking"', 'change_class                      = "security"', "security records must reject unsafe legacy behavior"),
            ("unsafe rollback", 'rollback_restores_unsafe_behavior = false', 'rollback_restores_unsafe_behavior = true', "security rollback must not restore unsafe behavior"),
            ("capability prefix", 'id                                = "did.syntax.typescript-migration.v1"', 'id                                = "wallet.typescript-migration.v1"', "must be prefixed by the capability ID"),
            ("malformed lifecycle", 'supersedes                        = [  ]', 'supersedes                        = 5', "must be an array"),
            ("whitespace prose", 'limitations                       = "This fixture proves contract connectivity only; it does not claim that the downstream canary or migration has run."', 'limitations                       = "                                                "', "must contain 24..1200 characters"),
            ("path traversal", 'docs/conformance/cross-language-vector-catalog.toml#did.syntax.valid-basic', '../outside#did.syntax.valid-basic', "safe repository-relative path"),
            ("unrelated selector", 'docs/conformance/cross-language-vector-catalog.toml#did.syntax.valid-basic', 'docs/conformance/cross-language-vector-catalog.toml#did.syntax', "selector needle must be one of the record's cited IDs"),
            ("missing replacement", 'replaced_by                       = ""', 'replaced_by                       = "did.syntax.next.v2"', "replacement is missing"),
        )
        security_baseline = replace_once(
            replace_once(baseline, 'change_class                      = "breaking"', 'change_class                      = "security"'),
            'legacy_bug_policy                 = "preserve-profile"',
            'legacy_bug_policy                 = "reject-as-unsafe"',
        )
        for name, before, after, diagnostic in mutations:
            mutation_base = security_baseline if name == "unsafe rollback" else baseline
            expect_failure(
                source_root,
                fixture_root,
                baseline,
                name,
                replace_once(mutation_base, before, after),
                diagnostic,
            )

        behavioral_break = replace_once(
            replace_once(
                baseline,
                'change_class                      = "breaking"',
                'change_class                      = "behavioral"',
            ),
            'migration_action                  = "Adopt the additive Rust-backed facade, then migrate callers before retiring legacy DTO constructors."',
            'migration_action                  = ""',
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "behavioral compatibility break",
            behavioral_break,
            "incompatible records require migration",
        )
        default_off = replace_once(
            replace_once(
                baseline,
                'deprecation_phase                 = "announced"',
                'deprecation_phase                 = "default-off"',
            ),
            'compatibility_window              = "Keep the legacy facade for one supported SDK-TS minor release after the Rust-backed facade becomes available."',
            'compatibility_window              = ""',
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "default-off window",
            default_off,
            "require replacement, migration, window, and removal gate",
        )
        simulated = replace_once(
            replace_once(
                baseline,
                'legacy_bug_policy                 = "preserve-profile"',
                'legacy_bug_policy                 = "simulate-temporarily"',
            ),
            'removal_gate                      = "All supported SDK-TS consumers use the Rust facade and the declared compatibility window has elapsed."',
            'removal_gate                      = ""',
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "bounded legacy simulation",
            simulated,
            "temporary legacy simulation must be bounded",
        )
        removed_phase = replace_once(
            baseline,
            'deprecation_phase                 = "announced"',
            'deprecation_phase                 = "removed"',
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "removed phase lifecycle",
            removed_phase,
            "removed deprecation phase requires removed lifecycle state",
        )
        missing_visibility = replace_once(
            baseline,
            'visibility                        = [ "error", "public-api" ]',
            'visibility                        = [ "error" ]',
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "compatibility visibility",
            missing_visibility,
            "source-break requires public-api visibility",
        )
        missing_replacement = replace_once(
            baseline,
            'replacement                       = "Rust-backed TypeScript DID and DID URL facade using the canonical serialized value and stable error code."',
            'replacement                       = ""',
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "replacement rationale",
            missing_replacement,
            "replacement or substantive no-replacement rationale",
        )

        record = baseline[baseline.index("[[changes]]") :]
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "duplicate ID",
            baseline + "\n" + record,
            "duplicate ID",
        )

        prefix = baseline[: baseline.index("[[changes]]")]
        first = replace_once(
            replace_once(
                replace_once(
                    replace_once(
                        record,
                        'lifecycle_state                   = "proposed"',
                        'lifecycle_state                   = "superseded"',
                    ),
                    "pull_request                      = 0",
                    "pull_request                      = 492",
                ),
                "active                            = true",
                "active                            = false",
            ),
            'replaced_by                       = ""',
            'replaced_by                       = "did.syntax.typescript-migration.v2"',
        )
        second = replace_once(
            replace_once(
                replace_once(
                    record,
                    'id                                = "did.syntax.typescript-migration.v1"',
                    'id                                = "did.syntax.typescript-migration.v2"',
                ),
                "pull_request                      = 0",
                "pull_request                      = 493",
            ),
            "supersedes                        = [  ]",
            'supersedes                        = [ "did.syntax.typescript-migration.v1" ]',
        )
        linked = prefix + first + "\n" + second
        synthetic_impact_path = (
            fixture_root
            / "openspec/changes/synthetic-did-migration/compatibility-impact.toml"
        )
        single_record_impact = synthetic_impact_path.read_text(encoding="utf-8")
        synthetic_impact_path.write_text(
            replace_once(
                single_record_impact,
                'ledger_ids     = [ "did.syntax.typescript-migration.v1" ]',
                'ledger_ids     = [ "did.syntax.typescript-migration.v1", "did.syntax.typescript-migration.v2" ]',
            ),
            encoding="utf-8",
        )
        baseline_path.write_text(linked, encoding="utf-8")
        result = run(source_root, fixture_root, "--write")
        if result.returncode != 0:
            raise AssertionError(f"valid reciprocal lifecycle failed:\n{result.stderr}")
        ordered_report = (fixture_root / REPORT).read_text(encoding="utf-8")
        baseline_path.write_text(prefix + second + "\n" + first, encoding="utf-8")
        result = run(source_root, fixture_root, "--write")
        if result.returncode != 0:
            raise AssertionError(f"reordered lifecycle failed:\n{result.stderr}")
        if (fixture_root / REPORT).read_text(encoding="utf-8") != ordered_report:
            raise AssertionError("record reordering changed deterministic output")
        nonreciprocal = replace_once(
            linked,
            'supersedes                        = [ "did.syntax.typescript-migration.v1" ]',
            "supersedes                        = [  ]",
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "nonreciprocal lifecycle",
            nonreciprocal,
            "replacement link is not reciprocal",
        )
        no_replacement = replace_once(
            linked,
            'replaced_by                       = "did.syntax.typescript-migration.v2"',
            'replaced_by                       = ""',
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "superseded without replacement",
            no_replacement,
            "superseded record requires replaced_by",
        )
        self_link = replace_once(
            linked,
            'supersedes                        = [ "did.syntax.typescript-migration.v1" ]',
            'supersedes                        = [ "did.syntax.typescript-migration.v2" ]',
        )
        expect_failure(
            source_root,
            fixture_root,
            baseline,
            "self supersession",
            self_link,
            "record cannot supersede itself",
        )
        baseline_path.write_text(baseline, encoding="utf-8")
        synthetic_impact_path.write_text(single_record_impact, encoding="utf-8")
        run(source_root, fixture_root, "--write")

        report_path = fixture_root / REPORT
        report_path.write_text(report_path.read_text(encoding="utf-8") + "drift\n", encoding="utf-8")
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "rendered report drift" not in result.stderr:
            raise AssertionError(f"render drift was not rejected:\n{result.stderr}")
        run(source_root, fixture_root, "--write")

        empty_ledger = replace_once(
            prefix,
            'status                            = "active"',
            'status                            = "active-empty"',
        ) + "changes = [  ]\n"
        baseline_path.write_text(empty_ledger, encoding="utf-8")
        result = run(source_root, fixture_root, "--write", "--skip-impacts")
        if result.returncode != 0:
            raise AssertionError(f"empty fixture failed:\n{result.stderr}")
        empty_report = (fixture_root / REPORT).read_text(encoding="utf-8")
        if empty_report.count("No consumer-visible changes are recorded.") != 5:
            raise AssertionError("canonical empty report must state the empty condition in all views")
        baseline_path.write_text(baseline, encoding="utf-8")
        run(source_root, fixture_root, "--write")

        report_path.unlink()
        os.symlink(baseline_path, report_path)
        result = run(source_root, fixture_root, "--write")
        if result.returncode == 0 or "regular non-symlink file" not in result.stderr:
            raise AssertionError(f"symlinked report was overwritten:\n{result.stderr}")
        report_path.unlink()
        run(source_root, fixture_root, "--write")

        impact_path = fixture_root / IMPACT_FILES[1]
        impact = impact_path.read_text(encoding="utf-8")
        impact_path.write_text(
            replace_once(impact, 'disposition    = "behavior-neutral"', 'disposition    = "consumer-visible"'),
            encoding="utf-8",
        )
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "must be consumer-contract" not in result.stderr:
            raise AssertionError(f"impact contradiction was not rejected:\n{result.stderr}")
        impact_path.write_text(
            replace_once(impact, 'scope          = "governance"', 'scope          = "consumer-contract"'),
            encoding="utf-8",
        )
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "is not a behavior-neutral scope" not in result.stderr:
            raise AssertionError(f"unsafe behavior-neutral scope was not rejected:\n{result.stderr}")
        impact_with_ids = replace_once(
            impact,
            "ledger_ids     = [  ]",
            'ledger_ids     = [ "did.syntax.typescript-migration.v1" ]',
        )
        impact_path.write_text(impact_with_ids, encoding="utf-8")
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "must be empty for behavior-neutral work" not in result.stderr:
            raise AssertionError(f"behavior-neutral ledger IDs were accepted:\n{result.stderr}")
        consumer_impact = replace_once(
            replace_once(
                impact_with_ids,
                'disposition    = "behavior-neutral"',
                'disposition    = "consumer-visible"',
            ),
            'scope          = "governance"',
            'scope          = "consumer-contract"',
        )
        impact_path.write_text(consumer_impact, encoding="utf-8")
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "ledger ID is owned by issue #492" not in result.stderr:
            raise AssertionError(f"cross-issue ledger ID was accepted:\n{result.stderr}")
        unknown_impact = replace_once(
            consumer_impact,
            "did.syntax.typescript-migration.v1",
            "did.syntax.unknown.v9",
        )
        impact_path.write_text(unknown_impact, encoding="utf-8")
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "unknown ledger ID" not in result.stderr:
            raise AssertionError(f"unknown consumer-visible ID was accepted:\n{result.stderr}")
        issue_mismatch = replace_once(impact, "issue          = 422", "issue          = 999")
        impact_path.write_text(issue_mismatch, encoding="utf-8")
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "must match preimplementation issue" not in result.stderr:
            raise AssertionError(f"impact issue mismatch was accepted:\n{result.stderr}")
        impact_path.write_text(impact, encoding="utf-8")
        reverse_contradiction = replace_once(
            baseline,
            "issue                             = 492",
            "issue                             = 422",
        )
        baseline_path.write_text(reverse_contradiction, encoding="utf-8")
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "contradicts canonical ledger records" not in result.stderr:
            raise AssertionError(f"reverse impact contradiction was accepted:\n{result.stderr}")
        baseline_path.write_text(baseline, encoding="utf-8")
        run(source_root, fixture_root, "--write")

        synthetic_impact = (
            fixture_root
            / "openspec/changes/synthetic-did-migration/compatibility-impact.toml"
        )
        saved_synthetic_impact = synthetic_impact.read_text(encoding="utf-8")
        synthetic_impact.unlink()
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "issue #492 has no compatibility-impact declaration" not in result.stderr:
            raise AssertionError(f"ledger issue without declaration was accepted:\n{result.stderr}")
        synthetic_impact.write_text(saved_synthetic_impact, encoding="utf-8")

        linked_change = fixture_root / "openspec/changes/symlinked-change"
        os.symlink(fixture_root / "openspec/changes/enforce-consumer-change-ledger", linked_change)
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "regular non-symlink directory" not in result.stderr:
            raise AssertionError(f"symlinked change directory was accepted:\n{result.stderr}")
        linked_change.unlink()

        missing_archived_impact = (
            fixture_root / "openspec/changes/archive/2026-10-25-missing-impact"
        )
        missing_archived_impact.mkdir(parents=True)
        (missing_archived_impact / "preimplementation.json").write_text(
            '{"issue":999,"contractHeadSha":"0000000000000000000000000000000000000001"}\n',
            encoding="utf-8",
        )
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "file does not exist" not in result.stderr:
            raise AssertionError(f"missing archived impact declaration was accepted:\n{result.stderr}")
        shutil.rmtree(missing_archived_impact)

        impact_path.unlink()
        os.symlink(source_root / ARCHIVED_CHANGE / IMPACT_FILES[1].name, impact_path)
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "regular non-symlink file" not in result.stderr:
            raise AssertionError(f"symlinked impact declaration was accepted:\n{result.stderr}")
        impact_path.unlink()
        copy(
            source_root,
            fixture_root,
            IMPACT_FILES[1],
            ARCHIVED_CHANGE / IMPACT_FILES[1].name,
        )

        unknown_grandfather = replace_once(
            baseline,
            "grandfathered_changes             = [  ]",
            'grandfathered_changes             = [ "new-change" ]',
        )
        baseline_path.write_text(unknown_grandfather, encoding="utf-8")
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "frozen allowlist" not in result.stderr:
            raise AssertionError(f"new grandfather entry was accepted:\n{result.stderr}")
        baseline_path.write_text(baseline, encoding="utf-8")
        run(source_root, fixture_root, "--write")
        impact_path.unlink()
        result = run(source_root, fixture_root)
        if result.returncode == 0 or "file does not exist" not in result.stderr:
            raise AssertionError(f"missing impact declaration was not rejected:\n{result.stderr}")

    print("platform-change-ledger mutations: passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
