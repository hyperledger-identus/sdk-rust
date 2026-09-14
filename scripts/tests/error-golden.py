#!/usr/bin/env python3
"""Mutation tests for the immutable credentials error-golden binding."""

from __future__ import annotations

import importlib.util
import shutil
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-error-golden.py"
STABLE = Path("crates/credentials/tests/fixtures/credentials-error-contract-v1.csv")
ACTIVE = Path(
    "openspec/changes/decompose-public-error-contracts/golden/credentials-error-contract-v1.csv"
)
ARCHIVE_FILE = Path(
    "openspec/changes/archive/2026-09-15-decompose-public-error-contracts/golden/credentials-error-contract-v1.csv"
)


def load_checker():
    spec = importlib.util.spec_from_file_location("error_golden_checker", CHECKER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load error-golden checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def write(path: Path, payload: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(payload)


def active_fixture(root: Path) -> None:
    payload = (ROOT / STABLE).read_bytes()
    write(root / STABLE, payload)
    write(root / ACTIVE, payload)


def archive_fixture(root: Path) -> None:
    (root / ACTIVE).unlink()
    write(root / ARCHIVE_FILE, (root / STABLE).read_bytes())


def require_error(checker, root: Path, expected: str) -> None:
    errors = checker.validate(root)
    if not any(expected in error for error in errors):
        raise AssertionError(f"mutation was accepted; expected {expected!r}, got {errors!r}")


def replace_both(root: Path, old: bytes, new: bytes) -> None:
    for relative in (STABLE, ACTIVE):
        path = root / relative
        payload = path.read_bytes()
        if old not in payload:
            raise AssertionError(f"fixture bytes not found: {old!r}")
        path.write_bytes(payload.replace(old, new, 1))


def main() -> int:
    checker = load_checker()
    with tempfile.TemporaryDirectory(prefix="error-golden-") as temporary:
        test_root = Path(temporary)

        active = test_root / "active"
        active_fixture(active)
        if errors := checker.validate(active):
            raise AssertionError(f"active fixture failed: {errors!r}")

        archived = test_root / "archived"
        shutil.copytree(active, archived)
        archive_fixture(archived)
        if errors := checker.validate(archived):
            raise AssertionError(f"archive fixture failed: {errors!r}")

        coordinated = test_root / "coordinated-row-drift"
        shutil.copytree(active, coordinated)
        replace_both(
            coordinated,
            b"credential format is invalid",
            b"credential format was invalid",
        )
        require_error(checker, coordinated, "SHA-256")

        single = test_root / "single-copy-drift"
        shutil.copytree(active, single)
        with (single / STABLE).open("ab") as destination:
            destination.write(b"\n")
        require_error(checker, single, "byte-for-byte")

        provenance = test_root / "provenance-drift"
        shutil.copytree(active, provenance)
        replace_both(
            provenance,
            b"# generated_at=2026-09-14",
            b"# generated_at=2026-09-15",
        )
        require_error(checker, provenance, "generated_at provenance")

        header = test_root / "header-drift"
        shutil.copytree(active, header)
        replace_both(header, b"error_type,variant", b"error_type,variant_name")
        require_error(checker, header, "CSV header")

        missing = test_root / "missing-archive"
        shutil.copytree(active, missing)
        (missing / ACTIVE).unlink()
        require_error(checker, missing, "found 0")

        ambiguous = test_root / "ambiguous-archive"
        shutil.copytree(archived, ambiguous)
        second = Path(
            "openspec/changes/archive/2026-09-16-decompose-public-error-contracts/golden/credentials-error-contract-v1.csv"
        )
        write(ambiguous / second, (ambiguous / STABLE).read_bytes())
        require_error(checker, ambiguous, "found 2")

        active_symlink = test_root / "active-symlink-plus-archive"
        shutil.copytree(archived, active_symlink)
        active_path = active_symlink / ACTIVE
        active_path.parent.mkdir(parents=True, exist_ok=True)
        active_path.symlink_to(active_symlink / STABLE)
        require_error(
            checker,
            active_symlink,
            "active planning golden must not contain symlinked path components",
        )

        active_and_archive = test_root / "active-plus-archive"
        shutil.copytree(archived, active_and_archive)
        write(active_and_archive / ACTIVE, (active_and_archive / STABLE).read_bytes())
        require_error(checker, active_and_archive, "found 2")

        archive_directory_symlink = test_root / "archive-directory-symlink"
        shutil.copytree(archived, archive_directory_symlink)
        linked_directory = (
            archive_directory_symlink
            / "openspec/changes/archive/2026-09-16-decompose-public-error-contracts"
        )
        linked_directory.symlink_to(
            archive_directory_symlink
            / "openspec/changes/archive/2026-09-15-decompose-public-error-contracts",
            target_is_directory=True,
        )
        require_error(
            checker,
            archive_directory_symlink,
            "matching archived change must be a regular directory",
        )

        archive_file_symlink = test_root / "archive-file-symlink"
        shutil.copytree(archived, archive_file_symlink)
        linked_file = (
            archive_file_symlink
            / "openspec/changes/archive/2026-09-16-decompose-public-error-contracts/golden"
            / "credentials-error-contract-v1.csv"
        )
        linked_file.parent.mkdir(parents=True, exist_ok=True)
        linked_file.symlink_to(archive_file_symlink / STABLE)
        require_error(
            checker,
            archive_file_symlink,
            "archived planning golden must not contain symlinked path components",
        )

        active_parent_symlink = test_root / "active-golden-parent-symlink"
        shutil.copytree(active, active_parent_symlink)
        active_golden_dir = (active_parent_symlink / ACTIVE).parent
        active_golden_target = active_parent_symlink / "active-golden-target"
        active_golden_dir.rename(active_golden_target)
        active_golden_dir.symlink_to(active_golden_target, target_is_directory=True)
        require_error(
            checker,
            active_parent_symlink,
            "active planning golden must not contain symlinked path components",
        )

        archive_parent_symlink = test_root / "archive-golden-parent-symlink"
        shutil.copytree(archived, archive_parent_symlink)
        archive_golden_dir = (archive_parent_symlink / ARCHIVE_FILE).parent
        archive_golden_target = archive_parent_symlink / "archive-golden-target"
        archive_golden_dir.rename(archive_golden_target)
        archive_golden_dir.symlink_to(archive_golden_target, target_is_directory=True)
        require_error(
            checker,
            archive_parent_symlink,
            "archived planning golden must not contain symlinked path components",
        )

        stable_parent_symlink = test_root / "stable-parent-symlink"
        shutil.copytree(active, stable_parent_symlink)
        stable_fixture_dir = (stable_parent_symlink / STABLE).parent
        stable_fixture_target = stable_parent_symlink / "stable-fixture-target"
        stable_fixture_dir.rename(stable_fixture_target)
        stable_fixture_dir.symlink_to(stable_fixture_target, target_is_directory=True)
        require_error(
            checker,
            stable_parent_symlink,
            "stable error fixture must not contain symlinked path components",
        )

    print("error-golden test: 15 active/archive and mutation cases passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
