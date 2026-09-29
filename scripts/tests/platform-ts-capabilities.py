#!/usr/bin/env python3
"""Mutation tests for the SDK-TS capability inventory checker."""

from __future__ import annotations

import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-platform-ts-capabilities.py"
SOURCE = ROOT / "docs/architecture/identus-platform-ts-capabilities.toml"


def run(root: Path, should_pass: bool) -> None:
    result = subprocess.run([str(CHECKER), str(root)], capture_output=True, text=True, check=False)
    if (result.returncode == 0) != should_pass:
        raise AssertionError(
            f"expected pass={should_pass}, got {result.returncode}\nstdout={result.stdout}\nstderr={result.stderr}"
        )


def run_text(text: str, should_pass: bool) -> None:
    with tempfile.TemporaryDirectory(prefix="platform-ts-capabilities-") as temporary:
        root = Path(temporary)
        target = root / "docs/architecture/identus-platform-ts-capabilities.toml"
        target.parent.mkdir(parents=True)
        target.write_text(text, encoding="utf-8")
        run(root, should_pass)


def main() -> None:
    original = SOURCE.read_text(encoding="utf-8")
    run_text(original, True)
    run_text(original.replace('id = "did.syntax"', 'id = "castor.syntax"', 1), False)
    run_text(original.replace('"standard-or-security",', '"donor-precedent",', 1), False)
    run_text(
        original.replace(
            'revision = "4bf86ebf69d5e96616a148e4c973f831f95fa38e"',
            'revision = "main"',
            1,
        ),
        False,
    )
    print("platform-ts-capabilities-tests: mutation suite passed")


if __name__ == "__main__":
    main()
