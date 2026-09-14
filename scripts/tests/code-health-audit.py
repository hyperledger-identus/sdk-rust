#!/usr/bin/env python3
"""Tests for the code-health population parser."""

from __future__ import annotations

import importlib.util
import sys
import unittest
from pathlib import Path


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


if __name__ == "__main__":
    unittest.main()
