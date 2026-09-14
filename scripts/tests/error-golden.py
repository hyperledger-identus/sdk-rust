#!/usr/bin/env python3
"""Mutation tests for immutable SDK error-golden bindings."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import shutil
import subprocess
import tempfile
from dataclasses import replace
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-error-golden.py"


def load_checker():
    spec = importlib.util.spec_from_file_location("error_golden_checker", CHECKER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load error-golden checker")
    module = importlib.util.module_from_spec(spec)
    # Dataclasses resolve annotations through the defining module.
    import sys

    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def write(path: Path, payload: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(payload)


def archive_file(checker, binding, date: str) -> Path:
    return (
        checker.ARCHIVE
        / f"{date}-{binding.change_name}"
        / "golden"
        / binding.file_name
    )


def planning_payload(checker, binding) -> bytes:
    errors: list[str] = []
    path = checker.resolve_planning_golden(ROOT, binding, errors)
    if path is None or errors:
        raise AssertionError(f"cannot resolve {binding.label} planning golden: {errors!r}")
    return path.read_bytes()


def active_fixture(root: Path, binding, payload: bytes) -> None:
    write(root / binding.stable, payload)
    write(root / binding.active, payload)


def archive_fixture(checker, root: Path, binding, payload: bytes) -> None:
    shutil.rmtree(root / binding.active_change)
    write(root / archive_file(checker, binding, "2026-09-15"), payload)


def require_error(checker, root: Path, binding, expected: str) -> None:
    errors = checker.validate_binding(root, binding, planning_payload(checker, binding))
    if not any(expected in error for error in errors):
        raise AssertionError(f"mutation was accepted; expected {expected!r}, got {errors!r}")


def replace_both(root: Path, binding, old: bytes, new: bytes) -> None:
    for relative in (binding.stable, binding.active):
        path = root / relative
        payload = path.read_bytes()
        if old not in payload:
            raise AssertionError(f"fixture bytes not found: {old!r}")
        path.write_bytes(payload.replace(old, new, 1))


def run_binding_cases(checker, test_root: Path, binding, drift: tuple[bytes, bytes]) -> None:
    payload = planning_payload(checker, binding)
    prefix = binding.label

    active = test_root / f"{prefix}-active"
    active_fixture(active, binding, payload)
    if errors := checker.validate_binding(active, binding, payload):
        raise AssertionError(f"{prefix} active fixture failed: {errors!r}")

    archived = test_root / f"{prefix}-archived"
    shutil.copytree(active, archived)
    archive_fixture(checker, archived, binding, payload)
    if errors := checker.validate_binding(archived, binding, payload):
        raise AssertionError(f"{prefix} archive fixture failed: {errors!r}")

    coordinated = test_root / f"{prefix}-coordinated-row-drift"
    shutil.copytree(active, coordinated)
    replace_both(coordinated, binding, *drift)
    require_error(checker, coordinated, binding, "SHA-256")

    single = test_root / f"{prefix}-single-copy-drift"
    shutil.copytree(active, single)
    with (single / binding.stable).open("ab") as destination:
        destination.write(b"\n")
    require_error(checker, single, binding, "byte-for-byte")

    provenance = test_root / f"{prefix}-provenance-drift"
    shutil.copytree(active, provenance)
    replace_both(
        provenance,
        binding,
        f"# generated_at={binding.generated_at}".encode(),
        b"# generated_at=1900-01-01",
    )
    require_error(checker, provenance, binding, "generated_at provenance")

    repository_provenance = test_root / f"{prefix}-repository-provenance-drift"
    shutil.copytree(active, repository_provenance)
    replace_both(
        repository_provenance,
        binding,
        b"# source_repository=hyperledger-identus/sdk-rust",
        b"# source_repository=example/other",
    )
    require_error(
        checker,
        repository_provenance,
        binding,
        "source_repository provenance",
    )

    revision_provenance = test_root / f"{prefix}-revision-provenance-drift"
    shutil.copytree(active, revision_provenance)
    replace_both(
        revision_provenance,
        binding,
        f"# source_revision={binding.source_revision}".encode(),
        b"# source_revision=0000000000000000000000000000000000000000",
    )
    require_error(
        checker,
        revision_provenance,
        binding,
        "source_revision provenance",
    )

    header = test_root / f"{prefix}-header-drift"
    shutil.copytree(active, header)
    replace_both(header, binding, b"error_type,variant", b"error_type,variant_name")
    require_error(checker, header, binding, "CSV header")

    missing = test_root / f"{prefix}-missing-archive"
    shutil.copytree(active, missing)
    shutil.rmtree(missing / binding.active_change)
    require_error(checker, missing, binding, "found 0")

    incomplete_archive = test_root / f"{prefix}-incomplete-archive"
    shutil.copytree(archived, incomplete_archive)
    (incomplete_archive / archive_file(checker, binding, "2026-09-15")).unlink()
    require_error(checker, incomplete_archive, binding, "missing its planning golden")

    ambiguous = test_root / f"{prefix}-ambiguous-archive"
    shutil.copytree(archived, ambiguous)
    write(ambiguous / archive_file(checker, binding, "2026-09-16"), payload)
    require_error(checker, ambiguous, binding, "found 2")

    active_symlink = test_root / f"{prefix}-active-symlink-plus-archive"
    shutil.copytree(archived, active_symlink)
    active_path = active_symlink / binding.active
    active_path.parent.mkdir(parents=True, exist_ok=True)
    active_path.symlink_to(active_symlink / binding.stable)
    require_error(checker, active_symlink, binding, "active planning golden must not contain")

    active_and_archive = test_root / f"{prefix}-active-plus-archive"
    shutil.copytree(archived, active_and_archive)
    write(active_and_archive / binding.active, payload)
    require_error(checker, active_and_archive, binding, "found 2")

    archive_directory_symlink = test_root / f"{prefix}-archive-directory-symlink"
    shutil.copytree(archived, archive_directory_symlink)
    linked_directory = (
        archive_directory_symlink
        / checker.ARCHIVE
        / f"2026-09-16-{binding.change_name}"
    )
    linked_directory.symlink_to(
        archive_directory_symlink / archive_file(checker, binding, "2026-09-15").parents[1],
        target_is_directory=True,
    )
    require_error(
        checker,
        archive_directory_symlink,
        binding,
        "matching archived change must be a regular directory",
    )

    archive_file_symlink = test_root / f"{prefix}-archive-file-symlink"
    shutil.copytree(archived, archive_file_symlink)
    linked_file = archive_file_symlink / archive_file(checker, binding, "2026-09-16")
    linked_file.parent.mkdir(parents=True, exist_ok=True)
    linked_file.symlink_to(archive_file_symlink / binding.stable)
    require_error(
        checker,
        archive_file_symlink,
        binding,
        "archived planning golden must not contain symlinked path components",
    )

    active_parent_symlink = test_root / f"{prefix}-active-golden-parent-symlink"
    shutil.copytree(active, active_parent_symlink)
    active_golden_dir = (active_parent_symlink / binding.active).parent
    active_golden_target = active_parent_symlink / "active-golden-target"
    active_golden_dir.rename(active_golden_target)
    active_golden_dir.symlink_to(active_golden_target, target_is_directory=True)
    require_error(checker, active_parent_symlink, binding, "active planning golden must not contain")

    archive_parent_symlink = test_root / f"{prefix}-archive-golden-parent-symlink"
    shutil.copytree(archived, archive_parent_symlink)
    archived_path = archive_parent_symlink / archive_file(checker, binding, "2026-09-15")
    archive_golden_dir = archived_path.parent
    archive_golden_target = archive_parent_symlink / "archive-golden-target"
    archive_golden_dir.rename(archive_golden_target)
    archive_golden_dir.symlink_to(archive_golden_target, target_is_directory=True)
    require_error(
        checker,
        archive_parent_symlink,
        binding,
        "archived planning golden must not contain symlinked path components",
    )

    stable_parent_symlink = test_root / f"{prefix}-stable-parent-symlink"
    shutil.copytree(active, stable_parent_symlink)
    stable_fixture_dir = (stable_parent_symlink / binding.stable).parent
    stable_fixture_target = stable_parent_symlink / "stable-fixture-target"
    stable_fixture_dir.rename(stable_fixture_target)
    stable_fixture_dir.symlink_to(stable_fixture_target, target_is_directory=True)
    require_error(checker, stable_parent_symlink, binding, "stable error fixture must not contain")

    stable_file_symlink = test_root / f"{prefix}-stable-file-symlink"
    shutil.copytree(active, stable_file_symlink)
    stable_path = stable_file_symlink / binding.stable
    stable_target = stable_file_symlink / "stable-fixture-target.csv"
    stable_path.rename(stable_target)
    stable_path.symlink_to(stable_target)
    require_error(checker, stable_file_symlink, binding, "stable error fixture must not contain")

    archive_root_symlink = test_root / f"{prefix}-archive-root-symlink"
    shutil.copytree(active, archive_root_symlink)
    archive_target = archive_root_symlink / "archive-target"
    archive_target.mkdir()
    (archive_root_symlink / checker.ARCHIVE).symlink_to(
        archive_target, target_is_directory=True
    )
    require_error(checker, archive_root_symlink, binding, "archive root must be a regular")

    incomplete_active = test_root / f"{prefix}-incomplete-active-plus-archive"
    shutil.copytree(archived, incomplete_active)
    (incomplete_active / binding.active_change).mkdir(parents=True)
    require_error(checker, incomplete_active, binding, "missing its planning golden")

    broken_active = test_root / f"{prefix}-broken-active-plus-archive"
    shutil.copytree(archived, broken_active)
    broken_change = broken_active / binding.active_change
    broken_change.symlink_to(
        broken_active / "missing-active-change", target_is_directory=True
    )
    require_error(checker, broken_active, binding, "active OpenSpec change must be a regular")

    reauthorized = test_root / f"{prefix}-coordinated-reauthorized-drift"
    reauthorized.mkdir()
    subprocess.run(["git", "init", "-q", str(reauthorized)], check=True)
    subprocess.run(
        ["git", "-C", str(reauthorized), "config", "user.name", "Golden Test"],
        check=True,
    )
    subprocess.run(
        [
            "git",
            "-C",
            str(reauthorized),
            "config",
            "user.email",
            "golden-test@example.com",
        ],
        check=True,
    )
    subprocess.run(
        ["git", "-C", str(reauthorized), "commit", "-q", "--allow-empty", "-m", "base"],
        check=True,
    )
    base_sha = subprocess.run(
        ["git", "-C", str(reauthorized), "rev-parse", "HEAD"],
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()
    test_payload = payload.replace(
        f"# source_revision={binding.source_revision}".encode(),
        f"# source_revision={base_sha}".encode(),
        1,
    )
    test_binding = replace(
        binding,
        source_revision=base_sha,
        expected_sha256=hashlib.sha256(test_payload).hexdigest(),
    )
    write(reauthorized / test_binding.active, test_payload)
    subprocess.run(
        [
            "git",
            "-C",
            str(reauthorized),
            "add",
            str(test_binding.active),
        ],
        check=True,
    )
    subprocess.run(
        ["git", "-C", str(reauthorized), "commit", "-q", "-m", "planning"],
        check=True,
    )
    contract_head = subprocess.run(
        ["git", "-C", str(reauthorized), "rev-parse", "HEAD"],
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()
    receipt_path = reauthorized / test_binding.active_change / "preimplementation.json"
    receipt = {
        "schemaVersion": 1,
        "repository": "hyperledger-identus/sdk-rust",
        "issue": test_binding.issue,
        "change": test_binding.change_name,
        "branch": test_binding.branch,
        "baseRef": "origin/develop",
        "baseSha": base_sha,
        "contractHeadSha": contract_head,
        "createdAt": "2026-09-15T00:00:00.000Z",
        "researchReady": True,
        "constraintsReady": True,
        "strictValidation": True,
    }
    write(
        receipt_path,
        f"{json.dumps(receipt, indent=2)}\n".encode(),
    )
    write(reauthorized / test_binding.stable, test_payload)
    source_snapshot = os.environ.pop(checker.SOURCE_SNAPSHOT_ENV, None)
    try:
        if errors := checker.validate_binding(reauthorized, test_binding):
            raise AssertionError(f"valid hermetic receipt failed for {prefix}: {errors!r}")
        receipt["schemaVersion"] = True
        write(receipt_path, f"{json.dumps(receipt, indent=2)}\n".encode())
        errors = checker.validate_binding(reauthorized, test_binding)
        if not any("receipt identity is invalid" in error for error in errors):
            raise AssertionError(
                f"boolean schema version was accepted for {prefix}: {errors!r}"
            )
        receipt["schemaVersion"] = 1
        write(receipt_path, f"{json.dumps(receipt, indent=2)}\n".encode())
        receipt["issue"] = float(test_binding.issue)
        write(receipt_path, f"{json.dumps(receipt, indent=2)}\n".encode())
        errors = checker.validate_binding(reauthorized, test_binding)
        if not any("receipt identity is invalid" in error for error in errors):
            raise AssertionError(
                f"non-integer issue was accepted for {prefix}: {errors!r}"
            )
        receipt["issue"] = test_binding.issue
        write(receipt_path, f"{json.dumps(receipt, indent=2)}\n".encode())
    finally:
        if source_snapshot is not None:
            os.environ[checker.SOURCE_SNAPSHOT_ENV] = source_snapshot

    replace_both(reauthorized, test_binding, *drift)
    subprocess.run(
        [
            "git",
            "-C",
            str(reauthorized),
            "add",
            str(test_binding.stable),
            str(test_binding.active),
        ],
        check=True,
    )
    subprocess.run(
        ["git", "-C", str(reauthorized), "commit", "-q", "-m", "drift"],
        check=True,
    )
    drift_head = subprocess.run(
        ["git", "-C", str(reauthorized), "rev-parse", "HEAD"],
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()
    receipt["contractHeadSha"] = drift_head
    write(receipt_path, f"{json.dumps(receipt, indent=2)}\n".encode())
    changed_payload = (reauthorized / test_binding.stable).read_bytes()
    changed_binding = replace(
        test_binding,
        expected_sha256=hashlib.sha256(changed_payload).hexdigest(),
    )
    source_snapshot = os.environ.pop(checker.SOURCE_SNAPSHOT_ENV, None)
    try:
        errors = checker.validate_binding(reauthorized, changed_binding)
    finally:
        if source_snapshot is not None:
            os.environ[checker.SOURCE_SNAPSHOT_ENV] = source_snapshot
    if not any(
        marker in error
        for error in errors
        for marker in ("contract head is not planning-only", "planning diff is incomplete")
    ):
        raise AssertionError(
            f"post-drift receipt retarget was accepted for {prefix}: {errors!r}"
        )


def main() -> int:
    checker = load_checker()
    with tempfile.TemporaryDirectory(prefix="error-golden-") as temporary:
        test_root = Path(temporary)
        run_binding_cases(
            checker,
            test_root,
            checker.CREDENTIALS,
            (b"credential format is invalid", b"credential format was invalid"),
        )
        run_binding_cases(
            checker,
            test_root,
            checker.PRESENTATIONS,
            (b"presentation query id is invalid", b"presentation query id was invalid"),
        )
        run_binding_cases(
            checker,
            test_root,
            checker.JOSE,
            (b"JWS limits are invalid", b"JWS limits were invalid"),
        )
        run_binding_cases(
            checker,
            test_root,
            checker.OID4VCI,
            (b"OID4VCI limits are invalid", b"OID4VCI limits were invalid"),
        )

        combined = test_root / "combined-active-bindings"
        for binding in checker.BINDINGS:
            active_fixture(combined, binding, planning_payload(checker, binding))
        trusted_contracts = {
            binding.label: planning_payload(checker, binding)
            for binding in checker.BINDINGS
        }
        if errors := checker.validate(combined, trusted_contracts):
            raise AssertionError(f"combined binding validation failed: {errors!r}")

    print(
        "error-golden test: 101 credentials/presentations/JOSE/OID4VCI "
        "binding cases passed"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
