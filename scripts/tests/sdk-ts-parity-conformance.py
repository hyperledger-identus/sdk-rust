#!/usr/bin/env python3
"""Mutation tests for the SDK-TS parity/conformance checker."""

from __future__ import annotations

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
    print("sdk-ts-parity-conformance-tests: mutation suite passed")


if __name__ == "__main__":
    main()
