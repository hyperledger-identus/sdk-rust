#!/usr/bin/env python3
"""Mutation tests for the isolated JWE qualification contract."""

from __future__ import annotations

import importlib.util
import shutil
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-jwe-engine-qualification.py"


def load_checker():
    spec = importlib.util.spec_from_file_location("jwe_engine_qualification", CHECKER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load JWE qualification checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def copy_fixture(destination: Path) -> None:
    for relative in (
        "scripts/qualify-jwe-engine.py",
        "docs/adr/0192-require-all-gates-before-adopting-a-jwe-engine.md",
        "docs/architecture/oid4vp-encrypted-response-boundary.md",
    ):
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / relative, target)


def replace(path: Path, old: str, new: str) -> None:
    source = path.read_text(encoding="utf-8")
    if old not in source:
        raise AssertionError(f"fixture text not found: {old}")
    path.write_text(source.replace(old, new, 1), encoding="utf-8")


def require_rejection(checker, fixture: Path, old: str, new: str, expected: str) -> None:
    copy = fixture.parent / f"mutation-{len(list(fixture.parent.glob('mutation-*')))}"
    shutil.copytree(fixture, copy)
    replace(copy / "scripts/qualify-jwe-engine.py", old, new)
    errors = checker.validate(copy)
    if not any(expected in error for error in errors):
        raise AssertionError(f"mutation was accepted: {errors!r}")


def main() -> int:
    checker = load_checker()
    with tempfile.TemporaryDirectory(prefix="jwe-qualification-policy-") as temporary:
        fixture = Path(temporary) / "valid"
        copy_fixture(fixture)
        if errors := checker.validate(fixture):
            raise AssertionError(f"valid fixture failed: {errors!r}")
        require_rejection(checker, fixture, 'VERSION = "0.10.0"', 'VERSION = "0.11.0"', "VERSION")
        require_rejection(
            checker,
            fixture,
            'BLOCKING_ADVISORY = "RUSTSEC-2023-0071"',
            'BLOCKING_ADVISORY = "ignored"',
            "BLOCKING_ADVISORY",
        )
        require_rejection(
            checker,
            fixture,
            'default-features = false',
            'default-features = true',
            "jose-rs",
        )
        require_rejection(
            checker,
            fixture,
            '"--deny",',
            '"--ignore", "RUSTSEC-2023-0071", "--deny",',
            "suppress",
        )
    print("jwe-engine-qualification tests: passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
