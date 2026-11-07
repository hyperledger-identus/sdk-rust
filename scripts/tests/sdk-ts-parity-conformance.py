#!/usr/bin/env python3
"""Mutation tests for the SDK-TS parity/conformance checker."""

from __future__ import annotations

import re
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-sdk-ts-parity-conformance.py"
MATRIX = ROOT / "docs/roadmap/sdk-ts-parity-conformance.toml"
INVENTORY = ROOT / "docs/architecture/identus-platform-ts-capabilities.toml"
REPORT = ROOT / "docs/roadmap/sdk-ts-parity-conformance.md"


def replace_once(value: str, before: str, after: str) -> str:
    count = value.count(before)
    if count != 1:
        raise AssertionError(f"expected one mutation source {before!r}, found {count}")
    return value.replace(before, after, 1)


def replace_regex_once(value: str, pattern: str, replacement: str) -> str:
    result, count = re.subn(pattern, replacement, value, count=1, flags=re.MULTILINE)
    if count != 1:
        raise AssertionError(f"expected one regex mutation source {pattern!r}, found {count}")
    return result


def remove_table(value: str, table: str, identifier: str) -> str:
    pattern = rf"(?ms)^\[\[{re.escape(table)}\]\]\nid\s*=\s*\"{re.escape(identifier)}\"\n.*?(?=^\[\[|\Z)"
    return replace_regex_once(value, pattern, "")


def mutate_first_target_state(value: str, target: str, state: str) -> str:
    pattern = rf'(?m)^(target_states\s*=\s*\{{[^\n]*\b{re.escape(target)}\s*=\s*)"[^"]+"'
    return replace_regex_once(value, pattern, rf'\1"{state}"')


def run(matrix: str, inventory: str, report: str, should_pass: bool) -> None:
    with tempfile.TemporaryDirectory(prefix="sdk-ts-parity-") as temporary:
        root = Path(temporary)
        matrix_path = root / "docs/roadmap/sdk-ts-parity-conformance.toml"
        inventory_path = root / "docs/architecture/identus-platform-ts-capabilities.toml"
        report_path = root / "docs/roadmap/sdk-ts-parity-conformance.md"
        matrix_path.parent.mkdir(parents=True)
        inventory_path.parent.mkdir(parents=True)
        matrix_path.write_text(matrix, encoding="utf-8")
        inventory_path.write_text(inventory, encoding="utf-8")
        report_path.write_text(report, encoding="utf-8")
        result = subprocess.run([str(CHECKER), str(root)], capture_output=True, text=True, check=False)
        if (result.returncode == 0) != should_pass:
            raise AssertionError(
                f"expected pass={should_pass}, got {result.returncode}\n"
                f"stdout={result.stdout}\nstderr={result.stderr}"
            )


def main() -> None:
    matrix = MATRIX.read_text(encoding="utf-8")
    inventory = INVENTORY.read_text(encoding="utf-8")
    report = REPORT.read_text(encoding="utf-8")
    run(matrix, inventory, report, True)
    run(replace_regex_once(matrix, r"^schema_version\s*=\s*2$", "schema_version = 1"), inventory, report, False)
    run(replace_once(matrix, 'id = "crypto.key-operations"', 'id = "crypto.unknown"'), inventory, report, False)
    run(
        replace_once(
            matrix,
            'id = "crypto.key-operations"\nrelation = "compatible-adapter"',
            'id = "crypto.key-operations"\nrelation = "same-api"',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_once(
            matrix,
            'id = "crypto.key-operations"\nrelation = "compatible-adapter"\nstate = "partial"',
            'id = "crypto.key-operations"\nrelation = "compatible-adapter"\nstate = "delivered"',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_once(
            matrix,
            'id = "wallet.storage-browser"\nrelation = "platform-owned"',
            'id = "wallet.storage-browser"\nrelation = "semantic-equivalent"',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_once(matrix, "target_changed_loc_min = 5000", "target_changed_loc_min = 500"),
        inventory,
        report,
        False,
    )
    run(remove_table(matrix, "target", "browser"), inventory, report, False)
    run(
        replace_once(matrix, 'protected_status = "fast"', 'protected_status = "typescript-bindings"'),
        inventory,
        report,
        False,
    )
    run(
        replace_once(matrix, 'unknown_impact = "select-all-target-jobs"', 'unknown_impact = "select-no-target-jobs"'),
        inventory,
        report,
        False,
    )
    run(
        replace_once(
            matrix,
            'legacy_public_names = [ "apollo", "castor", "mercury", "pluto", "pollux", "edge-agent" ]',
            'legacy_public_names = [ "apollo", "castor", "mercury", "pluto", "pollux" ]',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_regex_once(
            matrix,
            r'^(binding\s*=\s*)"ADR 0163 New Architecture/TurboModule package over a qualified native Rust binding"$',
            r'\1"browser WASM reused in React Native"',
        ),
        inventory,
        report,
        False,
    )
    run(remove_table(matrix, "success_criterion", "SC-09"), inventory, report, False)
    run(
        replace_once(
            matrix,
            'title = "Rust-core-first isolated CI"',
            'title = ""',
        ),
        inventory,
        report,
        False,
    )
    run(mutate_first_target_state(matrix, "browser", "not-applicable"), inventory, report, False)
    run(mutate_first_target_state(matrix, "browser", "unknown"), inventory, report, False)
    run(
        replace_regex_once(
            matrix,
            r'(?m)^(target_states\s*=\s*\{[^\n]*), react-native\s*=\s*"[^"]+"(\s*\})$',
            r"\1\2",
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_regex_once(
            matrix,
            r'(?m)^(missing_dimensions\s*=\s*\[\s*)"[^"]+"',
            r'\1"unknown-dimension"',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_once(
            matrix,
            'known_deviations = [ "dc+sd-jwt is canonical; vc+sd-jwt is only a named-consumer-gated compatibility adapter." ]',
            "known_deviations = []",
        ),
        inventory,
        report,
        False,
    )
    run(
        matrix,
        replace_once(
            inventory,
            'revision   = "4bf86ebf69d5e96616a148e4c973f831f95fa38e"',
            'revision   = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"',
        ),
        report,
        False,
    )
    active = replace_once(matrix, 'status           = "baseline"', 'status           = "active"')
    run(active, inventory, report, True)
    run(replace_once(matrix, 'status           = "baseline"', 'status           = "complete"'), inventory, report, False)
    feature_start = "<!-- feature-comparison:start -->"
    feature_end = "<!-- feature-comparison:end -->"
    feature_section = report.split(feature_start, 1)[1].split(feature_end, 1)[0]
    feature_lines = [line for line in feature_section.splitlines() if line.startswith("|")]
    data_lines = [
        line
        for line in feature_lines
        if "Capability" not in line and not re.fullmatch(r"\|[\s:|-]+\|", line)
    ]
    if not data_lines:
        raise AssertionError("feature-comparison table contains no data rows")
    run(matrix, inventory, replace_once(report, data_lines[0] + "\n", ""), False)
    cells = [cell.strip() for cell in data_lines[0][1:-1].split("|")]
    if len(cells) != 6:
        raise AssertionError("feature-comparison table data row must contain six cells")
    drifted = data_lines[0].replace(cells[1], cells[1] + " (drift)", 1)
    run(matrix, inventory, replace_once(report, data_lines[0], drifted), False)
    print("sdk-ts-parity-conformance-tests: mutation suite passed")


if __name__ == "__main__":
    main()
