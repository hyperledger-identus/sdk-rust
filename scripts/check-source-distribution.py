#!/usr/bin/env python3
"""Validate the unreleased public-source distribution contract."""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path

GUIDE = Path("docs/architecture/source-distribution.md")
README = Path("README.md")
REPOSITORY = "https://github.com/hyperledger-identus/sdk-rust"
CANARY_REVISION = "04b45b7fceb094ae601e3cf7a3291de0a0248b57"
PACKAGES = {
    "identus-core": Path("crates/core/Cargo.toml"),
    "identus-derive": Path("crates/derive/Cargo.toml"),
    "identus-crypto": Path("crates/crypto/Cargo.toml"),
    "identus-did": Path("crates/did/Cargo.toml"),
    "identus-did-resolver-http": Path("crates/did-resolver-http/Cargo.toml"),
}
SHA = re.compile(r"^[0-9a-f]{40}$")
EXAMPLE = re.compile(
    r'^identus-[a-z0-9-]+ = \{ git = "([^"]+)", rev = "([^"]+)"[^}]*\}$',
    re.MULTILINE,
)


def load_toml(path: Path, errors: list[str]) -> dict[str, object]:
    try:
        with path.open("rb") as source:
            value = tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        errors.append(f"cannot read TOML {path}: {error}")
        return {}
    return value


def validate(root: Path) -> list[str]:
    errors: list[str] = []
    workspace = load_toml(root / "Cargo.toml", errors)
    package_policy = workspace.get("workspace", {})
    if isinstance(package_policy, dict):
        package_policy = package_policy.get("package", {})
    if not isinstance(package_policy, dict):
        errors.append("workspace.package must be a TOML table")
        package_policy = {}
    if package_policy.get("version") != "0.0.0":
        errors.append("workspace version must remain 0.0.0 for source-only alpha")
    if package_policy.get("publish") is not False:
        errors.append("workspace publish must remain false")
    if package_policy.get("rust-version") != "1.98.1":
        errors.append("consumer compiler floor must remain Rust 1.98.1")

    for expected_name, relative in PACKAGES.items():
        manifest = load_toml(root / relative, errors)
        package = manifest.get("package", {})
        if not isinstance(package, dict) or package.get("name") != expected_name:
            errors.append(f"package identity differs: {expected_name} at {relative}")
            continue
        version = package.get("version")
        publish = package.get("publish")
        if not isinstance(version, dict) or version.get("workspace") is not True:
            errors.append(f"{expected_name}: version must inherit the workspace")
        if not isinstance(publish, dict) or publish.get("workspace") is not True:
            errors.append(f"{expected_name}: publish must inherit the workspace")

    try:
        guide = (root / GUIDE).read_text(encoding="utf-8")
    except OSError as error:
        errors.append(f"cannot read distribution guide: {error}")
        guide = ""
    required_phrases = (
        REPOSITORY,
        "full lowercase 40-hex commit",
        "Cargo.lock",
        "flake.lock",
        "publish = false",
        "Rust 1.98.1",
        "no SemVer compatibility",
        "identus-apollo",
    )
    for phrase in required_phrases:
        if phrase not in guide:
            errors.append(f"distribution guide is missing required contract: {phrase}")
    for package in PACKAGES:
        if f"`{package}`" not in guide:
            errors.append(f"distribution guide is missing package: {package}")

    examples = EXAMPLE.findall(guide)
    if len(examples) != 2:
        errors.append("distribution guide must contain placeholder and canary examples")
    for git_url, revision in examples:
        if git_url != REPOSITORY:
            errors.append("source example must use the canonical public HTTPS repository")
        if revision != "SDK_COMMIT" and not SHA.fullmatch(revision):
            errors.append("source example revision must be a full lowercase Git SHA")
    if (REPOSITORY, CANARY_REVISION) not in examples:
        errors.append("distribution guide must retain the validated NeoPRISM canary")

    forbidden = (
        r"\bbranch\s*=",
        r"\btag\s*=",
        r"refs/pull/",
        r"rev\s*=\s*\"[0-9a-f]{7,39}\"",
    )
    for pattern in forbidden:
        if re.search(pattern, guide):
            errors.append(f"distribution guide contains mutable/short selector: {pattern}")

    try:
        readme = (root / README).read_text(encoding="utf-8")
    except OSError as error:
        errors.append(f"cannot read README: {error}")
        readme = ""
    if "[Alpha source distribution](docs/architecture/source-distribution.md)" not in readme:
        errors.append("README must link the alpha source-distribution guide")
    return errors


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    errors = validate(root)
    if errors:
        for error in errors:
            print(f"source-distribution: {error}", file=sys.stderr)
        print(f"source-distribution: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    print(f"source-distribution: {len(PACKAGES)} packages passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
