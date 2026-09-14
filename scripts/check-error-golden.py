#!/usr/bin/env python3
"""Validate the immutable credentials error-golden binding."""

from __future__ import annotations

import hashlib
import os
import sys
from pathlib import Path

CHANGE_NAME = "decompose-public-error-contracts"
FILE_NAME = "credentials-error-contract-v1.csv"
STABLE = Path("crates/credentials/tests/fixtures") / FILE_NAME
ACTIVE_CHANGE = Path("openspec/changes") / CHANGE_NAME
ACTIVE = ACTIVE_CHANGE / "golden" / FILE_NAME
ARCHIVE = Path("openspec/changes/archive")
EXPECTED_SHA256 = "6148a00b22bdb8551d4df9369c7a9fcf819e1cf1c2654edc8654feef82227c7c"
EXPECTED_PREFIX = (
    "# source_repository=hyperledger-identus/sdk-rust",
    "# source_revision=353030a7f263b9a1fba9deac0312ed228e61d761",
    "# generated_at=2026-09-14",
    "error_type,variant,code_constant,constant_visibility,code,kind,capability,local_display,public_message,identus_display,source",
)


def has_symlink_component(path: Path, root: Path) -> bool:
    try:
        relative = path.relative_to(root)
    except ValueError:
        return True
    current = root
    for component in relative.parts:
        current /= component
        if current.is_symlink():
            return True
    return False


def is_regular_without_symlinks(
    path: Path, root: Path, label: str, errors: list[str]
) -> bool:
    if has_symlink_component(path, root):
        errors.append(f"{label} must not contain symlinked path components: {path}")
        return False
    if not path.is_file():
        errors.append(f"{label} must be a regular file: {path}")
        return False
    return True


def resolve_planning_golden(root: Path, errors: list[str]) -> Path | None:
    active_change = root / ACTIVE_CHANGE
    active = root / ACTIVE
    candidates: list[Path] = []
    if os.path.lexists(active_change):
        if has_symlink_component(active_change, root) or not active_change.is_dir():
            errors.append(
                f"active OpenSpec change must be a regular directory: {active_change}"
            )
        elif not os.path.lexists(active):
            errors.append(f"active OpenSpec change is missing its planning golden: {active}")
        elif is_regular_without_symlinks(
            active, root, "active planning golden", errors
        ):
            candidates.append(active)

    archive_root = root / ARCHIVE
    if os.path.lexists(archive_root):
        if has_symlink_component(archive_root, root) or not archive_root.is_dir():
            errors.append(f"OpenSpec archive root must be a regular directory: {archive_root}")
        else:
            for directory in sorted(archive_root.iterdir()):
                if not directory.name.endswith(f"-{CHANGE_NAME}"):
                    continue
                if directory.is_symlink() or not directory.is_dir():
                    errors.append(
                        "matching archived change must be a regular directory: "
                        f"{directory}"
                    )
                    continue
                candidate = directory / "golden" / FILE_NAME
                if not os.path.lexists(candidate):
                    errors.append(
                        f"matching archived change is missing its planning golden: {candidate}"
                    )
                elif is_regular_without_symlinks(
                    candidate, root, "archived planning golden", errors
                ):
                    candidates.append(candidate)

    if len(candidates) != 1:
        errors.append(
            "expected exactly one active or archived planning golden; "
            f"found {len(candidates)}"
        )
        return None
    return candidates[0]


def read_rooted_bytes(
    path: Path, root: Path, label: str, errors: list[str]
) -> bytes | None:
    if not is_regular_without_symlinks(path, root, label, errors):
        return None
    try:
        return path.read_bytes()
    except OSError as error:
        errors.append(f"cannot read {label} {path}: {error}")
        return None


def validate(root: Path) -> list[str]:
    errors: list[str] = []
    stable = read_rooted_bytes(root / STABLE, root, "stable error fixture", errors)
    planning_path = resolve_planning_golden(root, errors)
    planning = (
        read_rooted_bytes(planning_path, root, "planning error golden", errors)
        if planning_path is not None
        else None
    )
    if stable is None or planning is None:
        return errors

    if stable != planning:
        errors.append("stable error fixture must match the planning golden byte-for-byte")

    for label, payload in (("stable fixture", stable), ("planning golden", planning)):
        digest = hashlib.sha256(payload).hexdigest()
        if digest != EXPECTED_SHA256:
            errors.append(
                f"{label} SHA-256 must be {EXPECTED_SHA256}; found {digest}"
            )

    try:
        lines = stable.decode("utf-8").splitlines()
    except UnicodeDecodeError:
        errors.append("stable error fixture must be UTF-8")
        return errors
    for index, expected in enumerate(EXPECTED_PREFIX):
        actual = lines[index] if index < len(lines) else None
        if actual != expected:
            label = (
                "source_repository provenance",
                "source_revision provenance",
                "generated_at provenance",
                "CSV header",
            )[index]
            errors.append(f"stable error fixture has unexpected {label}")
    return errors


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    errors = validate(root)
    if errors:
        for error in errors:
            print(f"error-golden: {error}", file=sys.stderr)
        print(f"error-golden: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    print(f"error-golden: immutable binding passed ({EXPECTED_SHA256})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
