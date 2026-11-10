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


def complete_without_receipts(matrix: str, report: str) -> tuple[str, str]:
    prefix, capabilities = matrix.split("[[capability]]", 1)
    capabilities = re.sub(r'(?m)^state = "(?:partial|blocked)"$', 'state = "excluded"', capabilities)
    capabilities = re.sub(
        r'(?m)^target_states = \{.*\}$',
        lambda match: re.sub(r'"(?:partial|blocked)"', '"excluded"', match.group(0)),
        capabilities,
    )
    mutated_matrix = replace_once(prefix + "[[capability]]" + capabilities, 'status           = "baseline"', 'status           = "complete"')

    state_rows = {}
    for state in ("partial", "blocked"):
        match = re.search(rf'^\| `{state}` \| [0-9]+ \| (.+) \|$', report, flags=re.MULTILINE)
        if match is None:
            raise AssertionError(f"missing report state row {state}")
        state_rows[state] = match.group(0)
    members = re.findall(r'`([^`]+)`', state_rows["partial"] + state_rows["blocked"])
    members = [member for member in members if member not in {"partial", "blocked"}]
    mutated_report = replace_once(report, "matrix status = `baseline`", "matrix status = `complete`")
    mutated_report = replace_once(mutated_report, state_rows["partial"], "| `partial` | 0 | none |")
    mutated_report = replace_once(mutated_report, state_rows["blocked"], "| `blocked` | 0 | none |")
    mutated_report = replace_once(
        mutated_report,
        "| `excluded` | 0 | none |",
        f"| `excluded` | {len(members)} | " + ", ".join(f"`{member}`" for member in members) + " |",
    )
    start = "<!-- feature-comparison:start -->"
    end = "<!-- feature-comparison:end -->"
    before, remainder = mutated_report.split(start, 1)
    feature, after = remainder.split(end, 1)
    feature = re.sub(r'=(?:partial|blocked)', "=excluded", feature)
    return mutated_matrix, before + start + feature + end + after


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
    run(
        replace_regex_once(
            matrix,
            r'(?m)^id\s*=\s*"crypto\.key-operations"$',
            'id = "crypto.unknown"',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_regex_once(
            matrix,
            r'(?m)^(id\s*=\s*"crypto\.key-operations"\nrelation\s*=\s*)"compatible-adapter"$',
            r'\1"same-api"',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_regex_once(
            matrix,
            r'(?m)^(id\s*=\s*"crypto\.key-operations"\nrelation\s*=\s*"compatible-adapter"\nstate\s*=\s*)"partial"$',
            r'\1"delivered"',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_regex_once(
            matrix,
            r'(?m)^(id\s*=\s*"wallet\.storage-browser"\nrelation\s*=\s*)"platform-owned"$',
            r'\1"semantic-equivalent"',
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
        replace_regex_once(
            matrix,
            r'(?m)^protected_status\s*=\s*"fast"$',
            'protected_status = "typescript-bindings"',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_regex_once(
            matrix,
            r'(?m)^unknown_impact\s*=\s*"select-all-target-jobs"$',
            'unknown_impact = "select-no-target-jobs"',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_regex_once(
            matrix,
            r'(?m)^selected_terminal\s*=\s*"only-success-satisfies-selected-job; failure-cancelled-skipped-timed-out-or-missing-blocks"$',
            'selected_terminal = "skipped-selected-jobs-are-acceptable"',
        ),
        inventory,
        report,
        False,
    )
    run(
        replace_regex_once(
            matrix,
            r'(?m)^legacy_public_names\s*=\s*\[ "apollo", "castor", "mercury", "pluto", "pollux", "edge-agent" \]$',
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
        replace_regex_once(
            matrix,
            r'(?m)^title\s*=\s*"Rust-core-first isolated CI"$',
            'title = ""',
        ),
        inventory,
        report,
        False,
    )
    sc_contradiction = "Historical component names are allowed in new public exports."
    sc_matrix = replace_once(
        matrix,
        "New public exports, package paths, and primary documentation use capability or standards names and contain none of the closed legacy-public-name vocabulary.",
        sc_contradiction,
    )
    sc_report = replace_once(
        report,
        "New public exports, package paths, and primary documentation use capability or standards names and contain none of the closed legacy-public-name vocabulary.",
        sc_contradiction,
    )
    run(sc_matrix, inventory, sc_report, False)
    run(mutate_first_target_state(matrix, "browser", "not-applicable"), inventory, report, False)
    run(mutate_first_target_state(matrix, "browser", "unknown"), inventory, report, False)
    run(
        replace_regex_once(matrix, r'(?m)^missing_proof\s*=\s*"[^"]+"$', 'missing_proof = "none"'),
        inventory,
        report,
        False,
    )
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
        replace_regex_once(
            matrix,
            r'(?m)^known_deviations\s*=\s*\[ "dc\+sd-jwt is canonical; vc\+sd-jwt is only a named-consumer-gated compatibility adapter\." \]$',
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
    active_report = replace_once(report, "matrix status = `baseline`", "matrix status = `active`")
    run(active, inventory, active_report, True)
    complete = replace_once(matrix, 'status           = "baseline"', 'status           = "complete"')
    complete_report = replace_once(report, "matrix status = `baseline`", "matrix status = `complete`")
    run(complete, inventory, complete_report, False)
    receiptless_complete, receiptless_complete_report = complete_without_receipts(matrix, report)
    run(receiptless_complete, inventory, receiptless_complete_report, False)
    excluded = replace_regex_once(
        active,
        r'(?m)^(id\s*=\s*"crypto\.key-operations"\nrelation\s*=\s*"compatible-adapter"\nstate\s*=\s*)"partial"$',
        r'\1"excluded"',
    )
    excluded_report = replace_once(active_report, "| `partial` | 14 | `crypto.key-operations`, ", "| `partial` | 13 | ")
    excluded_report = replace_once(excluded_report, "| `excluded` | 0 | none |", "| `excluded` | 1 | `crypto.key-operations` |")
    run(excluded, inventory, excluded_report, False)

    run(
        matrix,
        inventory,
        replace_once(report, "browser ESM WASM package", "contradictory browser artifact"),
        False,
    )
    run(
        matrix,
        inventory,
        replace_once(report, "Capability-oriented TypeScript contract", "Legacy-shaped TypeScript contract"),
        False,
    )
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
    run(
        matrix,
        inventory,
        replace_once(report, "delivered capabilities = 0", "delivered capabilities = 1"),
        False,
    )
    cells = [cell.strip() for cell in data_lines[0][1:-1].split("|")]
    if len(cells) != 8:
        raise AssertionError("feature-comparison table data row must contain eight cells")
    drifted = data_lines[0].replace(cells[1], cells[1] + " (drift)", 1)
    run(matrix, inventory, replace_once(report, data_lines[0], drifted), False)
    print("sdk-ts-parity-conformance-tests: mutation suite passed")


if __name__ == "__main__":
    main()
