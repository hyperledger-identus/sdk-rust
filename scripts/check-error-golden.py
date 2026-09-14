#!/usr/bin/env python3
"""Validate immutable SDK error-golden bindings."""

from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path

ARCHIVE = Path("openspec/changes/archive")
CSV_HEADER = "error_type,variant,code_constant,constant_visibility,code,kind,capability,local_display,public_message,identus_display,source"
SHA_PATTERN = re.compile(r"^[0-9a-f]{40}$")
RECEIPT_KEYS = frozenset(
    {
        "schemaVersion",
        "repository",
        "issue",
        "change",
        "branch",
        "baseRef",
        "baseSha",
        "contractHeadSha",
        "createdAt",
        "researchReady",
        "constraintsReady",
        "strictValidation",
    }
)


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
    issue: int
    branch: str

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
    issue=278,
    branch="codex/refactor/issue-278",
)
PRESENTATIONS = GoldenBinding(
    label="presentations",
    change_name="decompose-presentation-error-contracts",
    file_name="presentations-error-contract-v1.csv",
    stable=Path("crates/presentations/tests/fixtures/presentations-error-contract-v1.csv"),
    expected_sha256="3941cbdb1b3eedb26243b5caf1b8a11c4646c3789a3cab1415834c48d3a8ba49",
    source_revision="105308771dbebceb473b99d9eb82b0fa0178ab09",
    generated_at="2026-09-15",
    issue=279,
    branch="codex/refactor/issue-279",
)
JOSE = GoldenBinding(
    label="jose",
    change_name="decompose-jose-error-contracts",
    file_name="jose-error-contract-v1.csv",
    stable=Path("crates/jose/tests/fixtures/jose-error-contract-v1.csv"),
    expected_sha256="528b29913876710a2ee806e30fef044657f3c6c38e7e3efff860cf71060b9592",
    source_revision="c32c1c8cd0194466a8c7fbfaa8c050f4bf2971a1",
    generated_at="2026-09-15",
    issue=280,
    branch="codex/refactor/issue-280",
)
BINDINGS = (CREDENTIALS, PRESENTATIONS, JOSE)
SOURCE_SNAPSHOT_ENV = "SDK_ERROR_GOLDEN_SOURCE_SNAPSHOT"


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


def read_contract_head_bytes(
    root: Path,
    binding: GoldenBinding,
    planning_path: Path,
    errors: list[str],
) -> bytes | None:
    """Read the planning golden from the immutable preflight Git commit."""
    if os.environ.get(SOURCE_SNAPSHOT_ENV) == "1":
        return None

    receipt_path = planning_path.parent.parent / "preimplementation.json"
    receipt_bytes = read_rooted_bytes(
        receipt_path,
        root,
        f"{binding.label} preimplementation receipt",
        errors,
    )
    if receipt_bytes is None:
        return None
    try:
        receipt = json.loads(receipt_bytes)
    except (UnicodeDecodeError, json.JSONDecodeError):
        errors.append(f"{binding.label} preimplementation receipt must be valid JSON")
        return None
    if not isinstance(receipt, dict):
        errors.append(f"{binding.label} preimplementation receipt must be a JSON object")
        return None

    contract_head = receipt.get("contractHeadSha")
    created_at = receipt.get("createdAt")
    if (
        len(receipt_bytes) > 8192
        or set(receipt) != RECEIPT_KEYS
        or type(receipt.get("schemaVersion")) is not int
        or receipt.get("schemaVersion") != 1
        or receipt.get("repository") != "hyperledger-identus/sdk-rust"
        or receipt.get("issue") != binding.issue
        or receipt.get("change") != binding.change_name
        or receipt.get("branch") != binding.branch
        or receipt.get("baseRef") != "origin/develop"
        or receipt.get("baseSha") != binding.source_revision
        or not isinstance(contract_head, str)
        or SHA_PATTERN.fullmatch(contract_head) is None
        or not isinstance(created_at, str)
        or receipt.get("researchReady") is not True
        or receipt.get("constraintsReady") is not True
        or receipt.get("strictValidation") is not True
    ):
        errors.append(f"{binding.label} preimplementation receipt identity is invalid")
        return None
    try:
        datetime.fromisoformat(created_at.replace("Z", "+00:00"))
    except ValueError:
        errors.append(f"{binding.label} preimplementation receipt timestamp is invalid")
        return None

    original_path = (
        Path("openspec/changes")
        / binding.change_name
        / "golden"
        / binding.file_name
    )
    try:
        top_level = subprocess.run(
            ["git", "-C", str(root), "rev-parse", "--show-toplevel"],
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        if Path(top_level).resolve() != root.resolve():
            errors.append(f"{binding.label} repository root does not match Git top-level")
            return None
        for older, newer, message in (
            (
                binding.source_revision,
                contract_head,
                "contract head must descend from its fixed base",
            ),
            (
                contract_head,
                "HEAD",
                "current head must descend from the contract head",
            ),
        ):
            outcome = subprocess.run(
                ["git", "-C", str(root), "merge-base", "--is-ancestor", older, newer],
                capture_output=True,
            )
            if outcome.returncode != 0:
                errors.append(f"{binding.label} preimplementation receipt {message}")
                return None

        changed = subprocess.run(
            [
                "git",
                "-C",
                str(root),
                "diff",
                "--name-only",
                f"{binding.source_revision}...{contract_head}",
            ],
            check=True,
            capture_output=True,
            text=True,
        ).stdout.splitlines()
        change_prefix = f"openspec/changes/{binding.change_name}/"
        invalid = [
            path
            for path in changed
            if not path.startswith(change_prefix) and not path.startswith("docs/adr/")
        ]
        if not changed or not any(path.startswith(change_prefix) for path in changed):
            errors.append(
                f"{binding.label} preimplementation receipt planning diff is incomplete"
            )
            return None
        if invalid:
            errors.append(
                f"{binding.label} preimplementation receipt contract head is not planning-only"
            )
            return None
        return subprocess.run(
            ["git", "-C", str(root), "show", f"{contract_head}:{original_path}"],
            check=True,
            capture_output=True,
        ).stdout
    except (OSError, subprocess.CalledProcessError):
        errors.append(
            f"{binding.label} planning golden must resolve from receipt contractHeadSha"
        )
        return None


def validate_binding(
    root: Path,
    binding: GoldenBinding,
    trusted_contract: bytes | None = None,
) -> list[str]:
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

    contract = (
        trusted_contract
        if trusted_contract is not None
        else read_contract_head_bytes(root, binding, planning_path, errors)
    )
    if contract is not None and planning != contract:
        errors.append(
            f"{binding.label} planning golden must match its receipt contractHeadSha blob"
        )

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


def validate(
    root: Path,
    trusted_contracts: dict[str, bytes] | None = None,
) -> list[str]:
    errors: list[str] = []
    for binding in BINDINGS:
        trusted = (
            trusted_contracts.get(binding.label)
            if trusted_contracts is not None
            else None
        )
        errors.extend(validate_binding(root, binding, trusted))
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
