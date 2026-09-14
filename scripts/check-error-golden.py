#!/usr/bin/env python3
"""Validate immutable SDK error-golden bindings."""

from __future__ import annotations

import hashlib
import os
import sys
from dataclasses import dataclass
from pathlib import Path

ARCHIVE = Path("openspec/changes/archive")
CSV_HEADER = "error_type,variant,code_constant,constant_visibility,code,kind,capability,local_display,public_message,identus_display,source"


@dataclass(frozen=True)
class GoldenBinding:
    """One stable fixture and its single active-or-archived planning oracle."""

    label: str
    change_name: str
    file_name: str
    stable: Path
    expected_sha256: str
    source_revision: str
    generated_at: str

    @property
    def active_change(self) -> Path:
        return Path("openspec/changes") / self.change_name

    @property
    def active(self) -> Path:
        return self.active_change / "golden" / self.file_name

    @property
    def expected_prefix(self) -> tuple[str, ...]:
        return (
            "# source_repository=hyperledger-identus/sdk-rust",
            f"# source_revision={self.source_revision}",
            f"# generated_at={self.generated_at}",
            CSV_HEADER,
        )


CREDENTIALS = GoldenBinding(
    label="credentials",
    change_name="decompose-public-error-contracts",
    file_name="credentials-error-contract-v1.csv",
    stable=Path("crates/credentials/tests/fixtures/credentials-error-contract-v1.csv"),
    expected_sha256="6148a00b22bdb8551d4df9369c7a9fcf819e1cf1c2654edc8654feef82227c7c",
    source_revision="353030a7f263b9a1fba9deac0312ed228e61d761",
    generated_at="2026-09-14",
)
PRESENTATIONS = GoldenBinding(
    label="presentations",
    change_name="decompose-presentation-error-contracts",
    file_name="presentations-error-contract-v1.csv",
    stable=Path("crates/presentations/tests/fixtures/presentations-error-contract-v1.csv"),
    expected_sha256="3941cbdb1b3eedb26243b5caf1b8a11c4646c3789a3cab1415834c48d3a8ba49",
    source_revision="105308771dbebceb473b99d9eb82b0fa0178ab09",
    generated_at="2026-09-15",
)
BINDINGS = (CREDENTIALS, PRESENTATIONS)


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


def resolve_planning_golden(
    root: Path, binding: GoldenBinding, errors: list[str]
) -> Path | None:
    active_change = root / binding.active_change
    active = root / binding.active
    candidates: list[Path] = []
    if os.path.lexists(active_change):
        if has_symlink_component(active_change, root) or not active_change.is_dir():
            errors.append(
                f"{binding.label} active OpenSpec change must be a regular directory: "
                f"{active_change}"
            )
        elif not os.path.lexists(active):
            errors.append(
                f"{binding.label} active OpenSpec change is missing its planning golden: "
                f"{active}"
            )
        elif is_regular_without_symlinks(
            active, root, f"{binding.label} active planning golden", errors
        ):
            candidates.append(active)

    archive_root = root / ARCHIVE
    if os.path.lexists(archive_root):
        if has_symlink_component(archive_root, root) or not archive_root.is_dir():
            errors.append(f"OpenSpec archive root must be a regular directory: {archive_root}")
        else:
            for directory in sorted(archive_root.iterdir()):
                if not directory.name.endswith(f"-{binding.change_name}"):
                    continue
                if directory.is_symlink() or not directory.is_dir():
                    errors.append(
                        f"{binding.label} matching archived change must be a regular directory: "
                        f"{directory}"
                    )
                    continue
                candidate = directory / "golden" / binding.file_name
                if not os.path.lexists(candidate):
                    errors.append(
                        f"{binding.label} matching archived change is missing its planning golden: "
                        f"{candidate}"
                    )
                elif is_regular_without_symlinks(
                    candidate,
                    root,
                    f"{binding.label} archived planning golden",
                    errors,
                ):
                    candidates.append(candidate)

    if len(candidates) != 1:
        errors.append(
            f"{binding.label} expected exactly one active or archived planning golden; "
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


def validate_binding(root: Path, binding: GoldenBinding) -> list[str]:
    errors: list[str] = []
    stable = read_rooted_bytes(
        root / binding.stable,
        root,
        f"{binding.label} stable error fixture",
        errors,
    )
    planning_path = resolve_planning_golden(root, binding, errors)
    planning = (
        read_rooted_bytes(
            planning_path,
            root,
            f"{binding.label} planning error golden",
            errors,
        )
        if planning_path is not None
        else None
    )
    if stable is None or planning is None:
        return errors

    if stable != planning:
        errors.append(
            f"{binding.label} stable error fixture must match the planning golden "
            "byte-for-byte"
        )

    for label, payload in (
        (f"{binding.label} stable fixture", stable),
        (f"{binding.label} planning golden", planning),
    ):
        digest = hashlib.sha256(payload).hexdigest()
        if digest != binding.expected_sha256:
            errors.append(
                f"{label} SHA-256 must be {binding.expected_sha256}; found {digest}"
            )

    try:
        lines = stable.decode("utf-8").splitlines()
    except UnicodeDecodeError:
        errors.append(f"{binding.label} stable error fixture must be UTF-8")
        return errors
    for index, expected in enumerate(binding.expected_prefix):
        actual = lines[index] if index < len(lines) else None
        if actual != expected:
            label = (
                "source_repository provenance",
                "source_revision provenance",
                "generated_at provenance",
                "CSV header",
            )[index]
            errors.append(
                f"{binding.label} stable error fixture has unexpected {label}"
            )
    return errors


def validate(root: Path) -> list[str]:
    errors: list[str] = []
    for binding in BINDINGS:
        errors.extend(validate_binding(root, binding))
    return errors


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    errors = validate(root)
    if errors:
        for error in errors:
            print(f"error-golden: {error}", file=sys.stderr)
        print(f"error-golden: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    bindings = ", ".join(
        f"{binding.label}={binding.expected_sha256}" for binding in BINDINGS
    )
    print(f"error-golden: immutable bindings passed ({bindings})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
