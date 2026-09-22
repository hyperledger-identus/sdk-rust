#!/usr/bin/env python3
"""Mutation tests for protected release-train policy."""

from __future__ import annotations

import importlib.util
import shutil
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-release-train.py"


def load_checker():
    spec = importlib.util.spec_from_file_location("release_train_checker", CHECKER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load release-train checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def copy_fixture(destination: Path) -> None:
    files = [
        "Cargo.toml",
        "RELEASING.md",
        ".github/workflows/publish-crates.yml",
        "docs/release/crypto-candidate.toml",
        "docs/adr/0134-activate-protected-crates-io-release-trains.md",
        "scripts/publish-release-train.py",
    ]
    files.extend(
        path.relative_to(ROOT).as_posix()
        for path in sorted((ROOT / "crates").glob("*/Cargo.toml"))
    )
    for relative in files:
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / relative, target)


def replace(path: Path, old: str, new: str) -> None:
    source = path.read_text(encoding="utf-8")
    if old not in source:
        raise AssertionError(f"fixture text not found: {old}")
    path.write_text(source.replace(old, new, 1), encoding="utf-8")


def require_rejection(checker, root: Path, mutation, expected: str) -> None:
    mutation(root)
    errors = checker.validate(root)
    if not any(expected in error for error in errors):
        raise AssertionError(f"mutation was accepted; expected {expected!r}: {errors!r}")


def main() -> int:
    checker = load_checker()
    with tempfile.TemporaryDirectory(prefix="release-train-policy-") as temporary:
        fixture = Path(temporary) / "valid"
        copy_fixture(fixture)
        if errors := checker.validate(fixture):
            raise AssertionError(f"valid fixture failed: {errors!r}")
        cases = (
            (
                lambda root: replace(
                    root / "crates/did/Cargo.toml",
                    "version.workspace      = true",
                    'version                = "0.1.0-rc.1"',
                ),
                "unrelated package overrides workspace version",
            ),
            (
                lambda root: replace(
                    root / "crates/crypto/Cargo.toml",
                    'publish                = [ "crates-io" ]',
                    "publish                = false",
                ),
                "selected package registry permission differs",
            ),
            (
                lambda root: replace(
                    root / ".github/workflows/publish-crates.yml",
                    "workflow_dispatch:",
                    "push:",
                ),
                "forbidden automatic trigger",
            ),
            (
                lambda root: replace(
                    root / ".github/workflows/publish-crates.yml",
                    "environment: crates-io",
                    "environment: production",
                ),
                "environment: crates-io",
            ),
            (
                lambda root: replace(
                    root / ".github/workflows/publish-crates.yml",
                    "'.can_admins_bypass'",
                    "'.name'",
                ),
                "can_admins_bypass",
            ),
            (
                lambda root: replace(
                    root / ".github/workflows/publish-crates.yml",
                    "@c6f97d42243bad5fab37ca0427f495c86d5b1a18",
                    "@v1",
                ),
                "release workflow action is not immutable",
            ),
            (
                lambda root: replace(
                    root / "scripts/publish-release-train.py",
                    '("identus-derive", "identus-core", "identus-crypto")',
                    '("identus-crypto", "identus-core", "identus-derive")',
                ),
                "publisher is missing fail-closed contract",
            ),
            (
                lambda root: replace(
                    root / "scripts/publish-release-train.py",
                    '"--registry",\n                "crates-io",',
                    '"--no-verify",\n                "--registry",\n                "crates-io",',
                ),
                "live cargo publish must not use --no-verify",
            ),
        )
        for index, (mutation, expected) in enumerate(cases):
            case = Path(temporary) / f"case-{index}"
            shutil.copytree(fixture, case)
            require_rejection(checker, case, mutation, expected)
    print("release-train test: mutation cases passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
