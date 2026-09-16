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


class ClassifierProtocolTests(unittest.TestCase):
    def test_exact_classifier_response_is_accepted(self) -> None:
        sources = {Path("crates/demo/src/lib.rs"): "#[cfg(test)]\nfn helper() {}\n"}
        response = {
            "classifier": "syn-ast-v1",
            "files": [
                {
                    "inline_test_lines": [1, 2],
                    "path": "crates/demo/src/lib.rs",
                }
            ],
            "inherited_inline_paths": [],
            "protocol_version": 1,
        }
        completed = mock.Mock(returncode=0, stdout=audit.canonical_json(response), stderr="")
        with mock.patch.object(audit.subprocess, "run", return_value=completed) as run:
            lines, inherited = audit.rust_classifier_population(Path("/repo"), sources)
        self.assertEqual(lines, {Path("crates/demo/src/lib.rs"): {1, 2}})
        self.assertEqual(inherited, set())
        self.assertEqual(run.call_args.args[0], audit.CLASSIFIER_COMMAND)
        self.assertEqual(run.call_args.kwargs["cwd"], Path("/repo"))
        request = __import__("json").loads(run.call_args.kwargs["input"])
        self.assertEqual(request["protocol_version"], 1)
        self.assertEqual(request["sources"][0]["path"], "crates/demo/src/lib.rs")

    def test_classifier_failure_is_actionable(self) -> None:
        completed = mock.Mock(returncode=1, stdout="", stderr="bounded diagnostic")
        with mock.patch.object(audit.subprocess, "run", return_value=completed):
            with self.assertRaisesRegex(audit.AuditError, "bounded diagnostic"):
                audit.rust_classifier_population(Path("/repo"), {})

    def test_classifier_protocol_primitives_fail_closed(self) -> None:
        sources = {Path("crates/demo/src/lib.rs"): "pub fn shipping() {}\n"}
        base = {
            "classifier": "syn-ast-v1",
            "files": [
                {
                    "inline_test_lines": [],
                    "path": "crates/demo/src/lib.rs",
                }
            ],
            "inherited_inline_paths": [],
            "protocol_version": 1,
        }
        mutations = {
            "boolean_protocol": lambda value: value.update(protocol_version=True),
            "numeric_path": lambda value: value["files"][0].update(path=1),
            "wrong_identity": lambda value: value.update(classifier="other"),
            "extra_field": lambda value: value.update(extra=True),
        }
        for name, mutate in mutations.items():
            with self.subTest(name=name):
                response = copy.deepcopy(base)
                mutate(response)
                completed = mock.Mock(
                    returncode=0,
                    stdout=audit.canonical_json(response),
                    stderr="",
                )
                with mock.patch.object(audit.subprocess, "run", return_value=completed):
                    with self.assertRaises(audit.AuditError):
                        audit.rust_classifier_population(Path("/repo"), sources)

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

    def test_generated_sources_remain_visible_to_module_resolution(self) -> None:
        root = Path("/repo")
        library = Path("crates/demo/src/lib.rs")
        generated = Path("crates/demo/src/generated.rs")
        sources = {
            library: "mod generated;\npub fn shipping() {}\n",
            generated: "// generated\npub fn generated() {}\n",
        }
        config = {
            "classifier_command": list(audit.CLASSIFIER_COMMAND),
            "generated_exclusions": [
                {"path": generated.as_posix(), "marker": "// generated"}
            ],
        }
        classifier_result = ({library: set(), generated: set()}, set())
        with mock.patch.object(
            audit, "rust_classifier_population", return_value=classifier_result
        ) as classifier:
            production, external, excluded, lines, counts = (
                audit.source_population_evidence(root, sources, config)
            )
        self.assertEqual(set(classifier.call_args.args[1]), {library, generated})
        self.assertEqual(production, [library])
        self.assertEqual(external, [])
        self.assertEqual(excluded, [generated])
        self.assertEqual(lines, {library: set()})
        self.assertEqual(counts["production"]["authored_nonblank_lines"], 2)


class ReportBindingTests(unittest.TestCase):
    revision = "a" * 40
    fingerprint = "b" * 64
    projection = "c" * 64

    def report(self) -> dict[str, object]:
        return {
            "analyzer": {
                "classifier": "syn-ast-v1",
                "classifier_command": list(audit.CLASSIFIER_COMMAND),
                "classifier_protocol_version": 1,
                "contract_version": "code-health-v2",
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
            "population_projection_sha256": self.projection,
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
            "schema_version": 2,
            "signals": {"functions": [], "modules": []},
            "source_fingerprint_sha256": self.fingerprint,
        }

    def write_fixture(self, root: Path, report: dict[str, object]) -> Path:
        rendered = audit.canonical_json(report)
        digest = hashlib.sha256(rendered.encode()).hexdigest()
        config = f'''\
schema_version = 2
contract_version = "code-health-v2"
engine = "rust-code-analysis-cli"
engine_version = "0.0.25"
classifier = "syn-ast-v1"
classifier_protocol_version = 1
classifier_command = ["cargo", "run", "--quiet", "--locked", "-p", "identus-conformance", "--bin", "code-health-classifier", "--"]
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
            "projection": lambda value: value.update(
                population_projection_sha256="d" * 64
            ),
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
                mock.patch.object(
                    audit,
                    "rust_classifier_population",
                    return_value=({Path("crates/demo/src/lib.rs"): set()}, set()),
                ),
                mock.patch.object(
                    audit,
                    "population_projection_sha256",
                    return_value=self.projection,
                ),
            ):
                with self.assertRaises(audit.AuditError):
                    audit.validate_report(root, path)

    def test_fast_source_binding_rejects_per_file_projection_drift(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = self.write_fixture(root, self.report())
            sources = {Path("crates/demo/src/lib.rs"): "pub fn shipping() {}\n"}
            with (
                mock.patch.object(audit, "git_tree_sources", return_value=sources),
                mock.patch.object(
                    audit, "source_fingerprint", return_value=self.fingerprint
                ),
                mock.patch.object(
                    audit,
                    "rust_classifier_population",
                    return_value=({Path("crates/demo/src/lib.rs"): set()}, set()),
                ),
                mock.patch.object(
                    audit,
                    "population_projection_sha256",
                    return_value="d" * 64,
                ),
            ):
                with self.assertRaisesRegex(audit.AuditError, "population projection"):
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
            "boolean_report_classifier_protocol": lambda value: value["analyzer"].update(
                classifier_protocol_version=True
            ),
            "wrong_report_classifier": lambda value: value["analyzer"].update(
                classifier="other"
            ),
            "wrong_report_classifier_command": lambda value: value["analyzer"].update(
                classifier_command=["cargo", "metadata"]
            ),
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
                    "schema_version = 2", "schema_version = true"
                ),
                encoding="utf-8",
            )
            with self.assertRaises(audit.AuditError):
                audit.validate_report(root, path, policy_only=True)

        config_mutations = {
            "boolean_config_protocol": (
                "classifier_protocol_version = 1",
                "classifier_protocol_version = true",
            ),
            "wrong_config_classifier": (
                'classifier = "syn-ast-v1"',
                'classifier = "other"',
            ),
            "wrong_config_command": (
                'classifier_command = ["cargo", "run", "--quiet", "--locked", "-p", "identus-conformance", "--bin", "code-health-classifier", "--"]',
                'classifier_command = ["cargo", "metadata"]',
            ),
        }
        for name, (before, after) in config_mutations.items():
            with self.subTest(name=name), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                path = self.write_fixture(root, self.report())
                config_path = root / "docs/architecture/code-health.toml"
                config_path.write_text(
                    config_path.read_text(encoding="utf-8").replace(before, after),
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
                    'contract_version = "code-health-v2"', "contract_version = 1"
                ),
                encoding="utf-8",
            )
            with self.assertRaises(audit.AuditError):
                audit.validate_report(root, path, policy_only=True)


if __name__ == "__main__":
    unittest.main()
