#!/usr/bin/env python3
"""Validate the unpublished identus-crypto candidate contract."""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path
from typing import Any

DESCRIPTOR = Path("docs/release/crypto-candidate.toml")
API_BASELINE = Path("docs/release/identus-crypto-0.1.0-rc.1.api.txt")
ADR = Path("docs/adr/0113-prepare-isolated-unpublished-crypto-candidate.md")
TOOLCHAIN_ADR = Path("docs/adr/0121-generate-candidate-rustdoc-json-before-api-rendering.md")
RUNNER = Path("scripts/prepare-crypto-candidate.py")
VERSION = "0.1.0-rc.1"
BASELINE = "8110277c24714206436ae4a3fe678bdc58a84736"
PACKAGE_ORDER = ("identus-derive", "identus-core", "identus-crypto")
PACKAGE_PATHS = {
    "identus-derive": Path("crates/derive"),
    "identus-core": Path("crates/core"),
    "identus-crypto": Path("crates/crypto"),
}
TOOLS = {
    "cargo_public_api": "0.52.0",
    "cargo_semver_checks": "0.50.0",
    "cargo_cyclonedx": "0.5.9",
    "cyclonedx_spec": "1.5",
}
PROFILES = ("default", "all-features", "no-default-features", "hash-only", "kmp-compat")


def load_toml(path: Path, errors: list[str]) -> dict[str, Any]:
    try:
        with path.open("rb") as source:
            value = tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        errors.append(f"cannot read TOML {path}: {error}")
        return {}
    return value


def read(path: Path, errors: list[str]) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as error:
        errors.append(f"cannot read {path}: {error}")
        return ""


def validate(root: Path) -> list[str]:
    errors: list[str] = []
    descriptor = load_toml(root / DESCRIPTOR, errors)
    expected_scalars: dict[str, object] = {
        "schema_version": 1,
        "candidate": "identus-crypto-0.1.0-rc.1",
        "version": VERSION,
        "rust_version": "1.89.0",
        "preparation_rust_version": "1.98.1",
        "baseline_revision": BASELINE,
        "repository": "https://github.com/hyperledger-identus/sdk-rust",
        "license": "Apache-2.0",
        "publication": "prohibited",
    }
    for key, expected in expected_scalars.items():
        if descriptor.get(key) != expected:
            errors.append(f"descriptor {key} must equal {expected!r}")
    maximum = descriptor.get("max_archive_bytes")
    if not isinstance(maximum, int) or maximum <= 0 or maximum > 1024 * 1024:
        errors.append("descriptor max_archive_bytes must be within 1 MiB")
    if descriptor.get("tools") != TOOLS:
        errors.append("descriptor tools must retain the reviewed exact versions")

    packages = descriptor.get("packages", [])
    names = tuple(row.get("name") for row in packages if isinstance(row, dict))
    if names != PACKAGE_ORDER:
        errors.append("candidate package order/scope must remain derive, core, crypto")
    for row in packages if isinstance(packages, list) else []:
        if not isinstance(row, dict) or row.get("name") not in PACKAGE_PATHS:
            continue
        name = row["name"]
        expected_path = PACKAGE_PATHS[name].as_posix()
        if row.get("path") != expected_path:
            errors.append(f"{name}: descriptor path must be {expected_path}")
        for field in ("description", "documentation", "readme", "keywords", "categories"):
            if not row.get(field):
                errors.append(f"{name}: descriptor metadata missing {field}")

    profiles = descriptor.get("profiles", [])
    profile_names = tuple(row.get("name") for row in profiles if isinstance(row, dict))
    if profile_names != PROFILES:
        errors.append("candidate verification profiles differ from the reviewed set")

    workspace = load_toml(root / "Cargo.toml", errors).get("workspace", {})
    package_policy = workspace.get("package", {}) if isinstance(workspace, dict) else {}
    if package_policy.get("version") != "0.0.0":
        errors.append("canonical workspace version must remain 0.0.0")
    if package_policy.get("publish") is not False:
        errors.append("canonical workspace publish must remain false")
    if package_policy.get("rust-version") != "1.89.0":
        errors.append("canonical Rust version must remain the selected 1.89.0 MSRV")

    for name, package_path in PACKAGE_PATHS.items():
        manifest = load_toml(root / package_path / "Cargo.toml", errors)
        package = manifest.get("package", {})
        if not isinstance(package, dict) or package.get("name") != name:
            errors.append(f"canonical package identity differs: {name}")
        readme = read(root / package_path / "README.md", errors)
        for phrase in ("unpublished", "0.1.0-rc.1", "not published to crates.io"):
            if phrase not in readme:
                errors.append(f"{name}: README is missing candidate warning: {phrase}")

    adr = read(root / ADR, errors)
    for phrase in ("#266", "archive-closure verification", "publish = false"):
        if phrase not in adr:
            errors.append(f"candidate ADR is missing decision evidence: {phrase}")
    support_adr = read(root / "docs/adr/0133-select-rc1-compiler-support-matrix.md", errors)
    for phrase in ("Rust 1.89.0", "Rust 1.98.1", "0.1.x", "hash-only"):
        if phrase not in support_adr:
            errors.append(f"support-matrix ADR is missing decision evidence: {phrase}")
    toolchain_adr = read(root / TOOLCHAIN_ADR, errors)
    for phrase in ("#276", "cargo rustdoc", "cargo-public-api 0.52.0", "RUSTC_BOOTSTRAP=1", "Rust 1.98.1"):
        if phrase not in toolchain_adr:
            errors.append(f"candidate toolchain ADR is missing decision evidence: {phrase}")

    baseline = read(root / API_BASELINE, errors)
    if len(baseline.strip().splitlines()) < 5:
        errors.append("public API baseline must be committed and non-trivial")

    runner = read(root / RUNNER, errors)
    prohibited_commands = (
        r'\[\s*["\']cargo["\']\s*,\s*["\']publish["\']',
        r'\[\s*["\']git["\']\s*,\s*["\'](?:push|tag)["\']',
        r'api/v1/crates',
    )
    for pattern in prohibited_commands:
        if re.search(pattern, runner):
            errors.append(f"candidate runner contains prohibited remote mutation: {pattern}")
    required_staging_contract = (
        'TemporaryDirectory(prefix=".identus-crypto-candidate-build-")',
        "require_vcs_independent_build_scratch(root, scratch)",
        'prefix=f".{output.name}-stage-", dir=output.parent',
        "staging_output.rename(output)",
    )
    for phrase in required_staging_contract:
        if phrase not in runner:
            errors.append(f"candidate runner is missing staging boundary: {phrase}")
    if re.search(
        r'TemporaryDirectory\(\s*prefix="\.identus-crypto-candidate-build-"\s*,\s*dir=',
        runner,
    ):
        errors.append("candidate build scratch must not be rooted in the output destination")
    required_api_contract = (
        'api_target = stage / "target/public-api"',
        'api_env = env | {"RUSTC_BOOTSTRAP": "1"}',
        '"cargo", "rustdoc", "--manifest-path"',
        '"-Z", "unstable-options", "--output-format", "json"',
        'api_json = api_target / "doc/identus_crypto.json"',
        'if not api_json.is_file():',
        '"cargo", "public-api", "--rustdoc-json", str(api_json)',
    )
    for phrase in required_api_contract:
        if phrase not in runner:
            errors.append(f"candidate runner is missing stable public-API boundary: {phrase}")
    return errors


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    errors = validate(root)
    if errors:
        for error in errors:
            print(f"crypto-candidate: {error}", file=sys.stderr)
        print(f"crypto-candidate: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    print("crypto-candidate: unpublished three-package contract passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
