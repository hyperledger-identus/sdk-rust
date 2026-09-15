#!/usr/bin/env python3
"""Mutation tests for the unpublished crypto-candidate policy."""

from __future__ import annotations

import importlib.util
import shutil
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-crypto-candidate.py"
RUNNER = ROOT / "scripts/prepare-crypto-candidate.py"


def load_checker():
    spec = importlib.util.spec_from_file_location("crypto_candidate_checker", CHECKER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load crypto-candidate checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def load_runner():
    spec = importlib.util.spec_from_file_location("crypto_candidate_runner", RUNNER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load crypto-candidate runner")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def copy_fixture(destination: Path) -> None:
    files = (
        "Cargo.toml",
        "docs/release/crypto-candidate.toml",
        "docs/release/identus-crypto-0.1.0-rc.1.api.txt",
        "docs/adr/0113-prepare-isolated-unpublished-crypto-candidate.md",
        "scripts/prepare-crypto-candidate.py",
        "crates/derive/Cargo.toml",
        "crates/derive/README.md",
        "crates/core/Cargo.toml",
        "crates/core/README.md",
        "crates/crypto/Cargo.toml",
        "crates/crypto/README.md",
    )
    for relative in files:
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / relative, target)


def require_rejection(checker, fixture: Path, mutate, expected: str) -> None:
    mutate(fixture)
    errors = checker.validate(fixture)
    if not any(expected in error for error in errors):
        raise AssertionError(f"mutation was accepted; expected {expected!r}, got {errors!r}")


def replace(path: Path, old: str, new: str) -> None:
    source = path.read_text(encoding="utf-8")
    if old not in source:
        raise AssertionError(f"fixture text not found: {old}")
    path.write_text(source.replace(old, new, 1), encoding="utf-8")


def main() -> int:
    checker = load_checker()
    runner = load_runner()
    with tempfile.TemporaryDirectory(prefix="crypto-candidate-policy-") as temporary:
        test_root = Path(temporary)
        repository = test_root / "repository"
        nested_scratch = repository / "artifacts/build"
        nested_scratch.mkdir(parents=True)
        try:
            runner.require_external_build_scratch(repository, nested_scratch)
        except runner.CandidateError:
            pass
        else:
            raise AssertionError("repository-contained build scratch was accepted")
        external_scratch = test_root / "external-build"
        external_scratch.mkdir()
        runner.require_external_build_scratch(repository, external_scratch)

        fixture = test_root / "valid"
        copy_fixture(fixture)
        if errors := checker.validate(fixture):
            raise AssertionError(f"valid fixture failed: {errors!r}")

        cases = (
            (
                lambda root: replace(root / "Cargo.toml", "publish      = false", "publish      = true"),
                "canonical workspace publish must remain false",
            ),
            (
                lambda root: replace(root / "docs/release/crypto-candidate.toml", 'publication       = "prohibited"', 'publication       = "allowed"'),
                "descriptor publication",
            ),
            (
                lambda root: replace(root / "docs/release/crypto-candidate.toml", 'cargo_cyclonedx     = "0.5.9"', 'cargo_cyclonedx     = "0.6.0"'),
                "descriptor tools",
            ),
            (
                lambda root: (root / "scripts/prepare-crypto-candidate.py").write_text('run(["cargo", "publish"])\n', encoding="utf-8"),
                "prohibited remote mutation",
            ),
            (
                lambda root: replace(
                    root / "scripts/prepare-crypto-candidate.py",
                    'TemporaryDirectory(prefix=".identus-crypto-candidate-build-")',
                    'TemporaryDirectory(prefix=".identus-crypto-candidate-build-", dir=output.parent)',
                ),
                "build scratch must not be rooted in the output destination",
            ),
            (
                lambda root: replace(
                    root / "scripts/prepare-crypto-candidate.py",
                    "require_external_build_scratch(root, scratch)",
                    "# removed repository boundary check",
                ),
                "missing staging boundary",
            ),
        )
        for index, (mutation, expected) in enumerate(cases):
            case = test_root / f"case-{index}"
            shutil.copytree(fixture, case)
            require_rejection(checker, case, mutation, expected)
    print("crypto-candidate test: mutation cases passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
