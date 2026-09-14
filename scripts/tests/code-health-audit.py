#!/usr/bin/env python3
"""Tests for the code-health population parser."""

from __future__ import annotations

import importlib.util
import copy
import hashlib
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


SCRIPT = Path(__file__).resolve().parents[1] / "code-health-audit.py"
SPEC = importlib.util.spec_from_file_location("code_health_audit", SCRIPT)
assert SPEC and SPEC.loader
audit = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = audit
SPEC.loader.exec_module(audit)


class CfgEvaluationTests(unittest.TestCase):
    def test_test_is_false_for_production(self) -> None:
        self.assertIs(audit.cfg_value("cfg(test)"), False)
        self.assertIs(audit.cfg_value("cfg(all(unix, test))"), False)

    def test_unknown_alternative_stays_production(self) -> None:
        self.assertIsNone(audit.cfg_value('cfg(any(test, feature = "diagnostics"))'))
        self.assertIsNone(audit.cfg_value("cfg(unix)"))

    def test_not_test_is_true(self) -> None:
        self.assertIs(audit.cfg_value("cfg(not(test))"), True)


class RustSpanTests(unittest.TestCase):
    def test_balanced_inline_module_is_excluded(self) -> None:
        source = '''
const TEXT: &str = "#[cfg(test)] mod fake { }";
/* #[cfg(test)] mod commented { } */
#[cfg(test)]
#[allow(dead_code)]
mod tests {
    fn braces() { let _ = "}"; }
}
pub fn shipping() {}
'''
        spans = audit.test_only_spans(source)
        lines = audit.span_lines(source, spans)
        self.assertEqual(len(spans), 1)
        self.assertIn(6, lines)
        self.assertIn(7, lines)
        self.assertNotIn(9, lines)

    def test_all_false_and_any_unknown(self) -> None:
        source = '''
#[cfg(all(test, feature = "slow"))]
fn test_only() {}
#[cfg(any(test, feature = "diagnostics"))]
fn maybe_shipping() {}
'''
        lines = audit.span_lines(source, audit.test_only_spans(source))
        self.assertIn(3, lines)
        self.assertNotIn(5, lines)

    def test_lifetime_does_not_hide_item_boundary(self) -> None:
        source = "#[cfg(test)]\nfn borrowed<'a>(value: &'a str) -> &'a str { value }\n"
        spans = audit.test_only_spans(source)
        self.assertEqual(len(spans), 1)
        self.assertEqual(spans[0].end, len(source) - 1)

    def test_comma_terminated_fields_and_variants_do_not_consume_shipping_code(self) -> None:
        source = '''
struct Named {
    #[cfg(test)]
    hidden: Vec<(u8, u8)>,
    visible: u8,
}
struct Tuple(
    #[cfg(test)] Vec<(u8, u8)>,
    u8,
);
enum Choice {
    #[cfg(test)] Hidden { value: u8 },
    #[cfg(test)] Unit,
    Visible,
}
'''
        lines = audit.span_lines(source, audit.test_only_spans(source))
        self.assertIn(4, lines)
        self.assertNotIn(5, lines)
        self.assertIn(8, lines)
        self.assertNotIn(9, lines)
        self.assertIn(12, lines)
        self.assertIn(13, lines)
        self.assertNotIn(14, lines)

    def test_cfg_inherits_preceding_outer_attributes_and_nested_items(self) -> None:
        source = '''
#[allow(dead_code)]
#[cfg(test)]
enum Hidden {
    #[allow(dead_code)]
    Variant,
}
pub fn shipping() {}
'''
        spans = audit.test_only_spans(source)
        self.assertEqual(spans[0].start, source.index("#[allow"))
        lines = audit.span_lines(source, spans)
        self.assertIn(2, lines)
        self.assertIn(6, lines)
        self.assertNotIn(8, lines)

    def test_outer_doc_comments_are_part_of_test_only_item(self) -> None:
        source = '''
/// line documentation
/** block
 * documentation
 */
#[cfg(test)]
fn helper() {}
//! inner module documentation
pub fn shipping() {}
'''
        lines = audit.span_lines(source, audit.test_only_spans(source))
        self.assertTrue(set(range(2, 8)).issubset(lines))
        self.assertNotIn(8, lines)
        self.assertNotIn(9, lines)


class ModulePopulationTests(unittest.TestCase):
    def test_test_only_out_of_line_module_tree_is_inherited(self) -> None:
        sources = {
            Path("crates/demo/src/lib.rs"): "#[cfg(test)]\nmod guard;\nmod shipping;\n",
            Path("crates/demo/src/guard/mod.rs"): "mod nested;\nfn guarded() {}\n",
            Path("crates/demo/src/guard/nested.rs"): "fn nested() {}\n",
            Path("crates/demo/src/shipping.rs"): "pub fn shipping() {}\n",
        }
        inherited = audit.inherited_test_files(sources, sorted(sources))
        self.assertEqual(
            inherited,
            {
                Path("crates/demo/src/guard/mod.rs"),
                Path("crates/demo/src/guard/nested.rs"),
            },
        )

    def test_src_tests_tree_requires_test_only_module_reachability(self) -> None:
        lib = Path("crates/demo/src/lib.rs")
        tests = Path("crates/demo/src/tests.rs")
        child = Path("crates/demo/src/tests/helper.rs")
        sources = {
            lib: "#[cfg(test)]\nmod tests;\n",
            tests: "mod helper;\nfn test_root() {}\n",
            child: "fn helper() {}\n",
        }
        production, external, _ = audit.classify_sources(
            sources, {"generated_exclusions": []}
        )
        self.assertEqual(external, [])
        self.assertEqual(audit.inherited_test_files(sources, production), {tests, child})

        sources[lib] = "mod tests;\n"
        self.assertEqual(audit.inherited_test_files(sources, production), set())

    def test_nested_inline_module_resolves_children_in_its_own_directory(self) -> None:
        lib = Path("crates/demo/src/lib.rs")
        nested = Path("crates/demo/src/tests/helper.rs")
        shipping = Path("crates/demo/src/helper.rs")
        sources = {
            lib: "#[cfg(test)]\nmod tests { mod helper; }\n",
            nested: "fn nested_test_helper() {}\n",
            shipping: "pub fn shipping_helper() {}\n",
        }
        inherited = audit.inherited_test_files(sources, sorted(sources))
        self.assertEqual(inherited, {nested})
        self.assertNotIn(shipping, inherited)

    def test_nested_inline_module_never_falls_back_to_parent_directory(self) -> None:
        sources = {
            Path("crates/demo/src/lib.rs"): "#[cfg(test)]\nmod tests { mod helper; }\n",
            Path("crates/demo/src/helper.rs"): "pub fn shipping_helper() {}\n",
        }
        with self.assertRaises(audit.AuditError):
            audit.inherited_test_files(sources, sorted(sources))

    def test_macro_tokens_cannot_create_a_test_only_module_edge(self) -> None:
        helper = Path("crates/demo/src/helper.rs")
        sources = {
            Path("crates/demo/src/lib.rs"): "#[cfg(test)]\nfixture!(mod helper;);\n",
            helper: "pub fn shipping_helper() {}\n",
        }
        self.assertEqual(audit.inherited_test_files(sources, sorted(sources)), set())

    def test_path_attributed_test_module_fails_closed(self) -> None:
        sources = {
            Path("crates/demo/src/lib.rs"): (
                '#[cfg(test)]\n#[path = "actual.rs"]\nmod helper;\n'
            ),
            Path("crates/demo/src/helper.rs"): "pub fn shipping_helper() {}\n",
            Path("crates/demo/src/actual.rs"): "fn actual_test_helper() {}\n",
        }
        with self.assertRaises(audit.AuditError):
            audit.inherited_test_files(sources, sorted(sources))

    def test_generated_marker_requires_exact_path_allowlist(self) -> None:
        marked = Path("crates/demo/src/marked.rs")
        ordinary = Path("crates/demo/src/ordinary.rs")
        sources = {
            marked: "// do not edit\npub fn generated() {}\n",
            ordinary: "// do not edit this example\npub fn ordinary() {}\n",
        }
        production, _, generated = audit.classify_sources(
            sources, {"generated_exclusions": []}
        )
        self.assertEqual(production, [marked, ordinary])
        self.assertEqual(generated, [])

        production, _, generated = audit.classify_sources(
            sources,
            {
                "generated_exclusions": [
                    {"path": marked.as_posix(), "marker": "// do not edit"}
                ]
            },
        )
        self.assertEqual(production, [ordinary])
        self.assertEqual(generated, [marked])


class ReportBindingTests(unittest.TestCase):
    revision = "a" * 40
    fingerprint = "b" * 64

    def report(self) -> dict[str, object]:
        return {
            "analyzer": {
                "contract_version": "code-health-v1",
                "engine": "rust-code-analysis-cli",
                "engine_version": "0.0.25",
            },
            "attention": {
                "cognitive": 15,
                "cyclomatic": 15,
                "function_sloc": 100,
                "module_authored_nonblank_lines": 1000,
            },
            "generated_exclusions": [],
            "hotspots": [
                {
                    "id": "fixture",
                    "locations": ["crates/demo/src/lib.rs"],
                    "disposition": "document-exception",
                    "owner": "test",
                    "evidence": "test",
                }
            ],
            "populations": {
                "production": {
                    "authored_nonblank_lines": 1,
                    "files": 1,
                    "functions": 1,
                },
                "external_test": {
                    "authored_nonblank_lines": 0,
                    "files": 0,
                    "functions": 0,
                },
                "inline_test": {
                    "authored_nonblank_lines": 0,
                    "files": 0,
                    "functions": 0,
                },
            },
            "revision": self.revision,
            "schema_version": 1,
            "signals": {"functions": [], "modules": []},
            "source_fingerprint_sha256": self.fingerprint,
        }

    def write_fixture(self, root: Path, report: dict[str, object]) -> Path:
        rendered = audit.canonical_json(report)
        digest = hashlib.sha256(rendered.encode()).hexdigest()
        config = f'''\
schema_version = 1
contract_version = "code-health-v1"
engine = "rust-code-analysis-cli"
engine_version = "0.0.25"
baseline_revision = "{self.revision}"
baseline_source_fingerprint_sha256 = "{self.fingerprint}"
baseline_report_sha256 = "{digest}"
generated_exclusions = []

[attention]
function_sloc = 100
cognitive = 15
cyclomatic = 15
module_authored_nonblank_lines = 1000

[[hotspots]]
id = "fixture"
locations = ["crates/demo/src/lib.rs"]
disposition = "document-exception"
owner = "test"
evidence = "test"
'''
        architecture = root / "docs/architecture"
        architecture.mkdir(parents=True)
        (architecture / "code-health.toml").write_text(config, encoding="utf-8")
        path = architecture / "report.json"
        path.write_text(rendered, encoding="utf-8")
        return path

    def test_canonical_report_mutations_fail_closed(self) -> None:
        mutations = {
            "revision": lambda value: value.update(revision="c" * 40),
            "fingerprint": lambda value: value.update(source_fingerprint_sha256="d" * 64),
            "counts": lambda value: value["populations"]["production"].update(files=0),
            "signals": lambda value: value["signals"]["modules"].append(
                {"path": "crates/demo/src/lib.rs", "authored_nonblank_lines": 1001}
            ),
            "exclusions": lambda value: value["generated_exclusions"].append(
                "crates/demo/src/lib.rs"
            ),
            "missing_attention": lambda value: value.pop("attention"),
        }
        for name, mutate in mutations.items():
            with self.subTest(name=name), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                path = self.write_fixture(root, self.report())
                report = copy.deepcopy(self.report())
                mutate(report)
                path.write_text(audit.canonical_json(report), encoding="utf-8")
                with self.assertRaises(audit.AuditError):
                    audit.validate_report(root, path, policy_only=True)

    def test_fast_source_binding_rejects_coordinated_count_tampering(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            changed = self.report()
            changed["populations"]["production"]["files"] = 0
            root = Path(directory)
            path = self.write_fixture(root, changed)
            sources = {Path("crates/demo/src/lib.rs"): "pub fn shipping() {}\n"}
            with (
                mock.patch.object(audit, "git_tree_sources", return_value=sources),
                mock.patch.object(
                    audit, "source_fingerprint", return_value=self.fingerprint
                ),
            ):
                with self.assertRaises(audit.AuditError):
                    audit.validate_report(root, path)

    def test_slow_verification_compares_the_complete_report(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            expected = self.report()
            changed = copy.deepcopy(expected)
            changed["signals"]["modules"].append(
                {"path": "crates/demo/src/lib.rs", "authored_nonblank_lines": 1001}
            )
            root = Path(directory)
            path = self.write_fixture(root, changed)
            with (
                mock.patch.object(audit, "validate_report", return_value=changed),
                mock.patch.object(audit, "regenerate_baseline", return_value=expected),
            ):
                with self.assertRaises(audit.AuditError):
                    audit.verify_baseline(root, path)

    def test_schema_and_analyzer_primitive_types_fail_closed(self) -> None:
        mutations = {
            "boolean_report_schema": lambda value: value.update(schema_version=True),
            "numeric_report_engine": lambda value: value["analyzer"].update(engine=1),
        }
        for name, mutate in mutations.items():
            with self.subTest(name=name), tempfile.TemporaryDirectory() as directory:
                changed = self.report()
                mutate(changed)
                root = Path(directory)
                path = self.write_fixture(root, changed)
                with self.assertRaises(audit.AuditError):
                    audit.validate_report(root, path, policy_only=True)

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = self.write_fixture(root, self.report())
            config_path = root / "docs/architecture/code-health.toml"
            config_path.write_text(
                config_path.read_text(encoding="utf-8").replace(
                    "schema_version = 1", "schema_version = true"
                ),
                encoding="utf-8",
            )
            with self.assertRaises(audit.AuditError):
                audit.validate_report(root, path, policy_only=True)

        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = self.write_fixture(root, self.report())
            config_path = root / "docs/architecture/code-health.toml"
            config_path.write_text(
                config_path.read_text(encoding="utf-8").replace(
                    'contract_version = "code-health-v1"', "contract_version = 1"
                ),
                encoding="utf-8",
            )
            with self.assertRaises(audit.AuditError):
                audit.validate_report(root, path, policy_only=True)


if __name__ == "__main__":
    unittest.main()
