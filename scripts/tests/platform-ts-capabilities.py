#!/usr/bin/env python3
"""Mutation tests for the SDK-TS capability inventory checker."""

from __future__ import annotations

import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-platform-ts-capabilities.py"
SOURCE = ROOT / "docs/architecture/identus-platform-ts-capabilities.toml"
ROADMAP_SOURCE = ROOT / "docs/roadmap/platform-core-adoption-milestones.toml"


def run(root: Path, should_pass: bool) -> None:
    result = subprocess.run([str(CHECKER), str(root)], capture_output=True, text=True, check=False)
    if (result.returncode == 0) != should_pass:
        raise AssertionError(
            f"expected pass={should_pass}, got {result.returncode}\nstdout={result.stdout}\nstderr={result.stderr}"
        )


def run_text(inventory_text: str, roadmap_text: str, should_pass: bool) -> None:
    with tempfile.TemporaryDirectory(prefix="platform-ts-capabilities-") as temporary:
        root = Path(temporary)
        target = root / "docs/architecture/identus-platform-ts-capabilities.toml"
        target.parent.mkdir(parents=True)
        target.write_text(inventory_text, encoding="utf-8")
        roadmap_target = root / "docs/roadmap/platform-core-adoption-milestones.toml"
        roadmap_target.parent.mkdir(parents=True)
        roadmap_target.write_text(roadmap_text, encoding="utf-8")
        run(root, should_pass)


def replace_once(text: str, before: str, after: str) -> str:
    count = text.count(before)
    if count != 1:
        raise AssertionError(f"mutation source must occur exactly once: {before!r}; found {count}")
    return text.replace(before, after, 1)


def main() -> None:
    original = SOURCE.read_text(encoding="utf-8")
    roadmap = ROADMAP_SOURCE.read_text(encoding="utf-8")
    run_text(original, roadmap, True)
    run_text(replace_once(original, '"did.syntax"', '"castor.syntax"'), roadmap, False)
    run_text(replace_once(original, '"standard-or-security",', '"donor-precedent",'), roadmap, False)
    run_text(
        replace_once(
            original,
            '"4bf86ebf69d5e96616a148e4c973f831f95fa38e"',
            '"main"',
        ),
        roadmap,
        False,
    )
    run_text(
        replace_once(original, 'canonical_contract       = "sdk-rust"', 'canonical_contract       = "sdk-ts"'),
        roadmap,
        False,
    )
    run_text(
        replace_once(original, 'id                 = "did.method.peer"', 'id                 = "did.method.peer-legacy"'),
        roadmap,
        False,
    )
    run_text(
        original,
        replace_once(
            roadmap,
            'title = "portable DID platform"\nstate = "planned"\nowner_issue = 493\nsupporting_issues = [ 420, 492, 501 ]\ndepends_on = [ "A1" ]',
            'title = "portable DID platform"\nstate = "planned"\nowner_issue = 493\nsupporting_issues = [ 420, 492, 501 ]\ndepends_on = [ "A9" ]',
        ),
        False,
    )
    run_text(
        original,
        replace_once(
            roadmap,
            'quality_classes = [ "property", "fuzz", "benchmark", "differential" ]',
            'quality_classes = [ "property", "fuzz", "differential" ]',
        ),
        False,
    )
    print("platform-ts-capabilities-tests: mutation suite passed")


if __name__ == "__main__":
    main()
