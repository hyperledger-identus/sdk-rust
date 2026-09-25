#!/usr/bin/env python3
"""Validate independent release-candidate train configuration without network."""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path
from typing import Any


INDEX = Path("docs/release/release-trains.toml")
DID_DESCRIPTOR = Path("docs/release/did-candidate.toml")
CRYPTO_DESCRIPTOR = Path("docs/release/crypto-candidate.toml")
BUILDER = Path("scripts/prepare-did-candidate.py")
ADR = Path("docs/adr/0153-use-primary-package-tags-for-independent-release-trains.md")
VERSION = "0.1.0-rc.1"
DID_PACKAGES = ("identus-did", "identus-did-resolver-http")
PACKAGE_PATHS = {
    "identus-did": Path("crates/did"),
    "identus-did-resolver-http": Path("crates/did-resolver-http"),
}
INDEX_KEYS = {"schema_version", "trains"}
TRAIN_KEYS = {"id", "lifecycle", "primary_package", "descriptor", "tag", "packages"}
DESCRIPTOR_KEYS = {
    "schema_version", "candidate", "version", "rust_version",
    "preparation_rust_version", "baseline_revision", "repository", "homepage",
    "license", "max_archive_bytes", "max_archive_members", "max_expansion_ratio",
    "max_evidence_bytes", "publication", "release_tag", "compatibility_status",
    "tools", "profiles", "packages",
}
PROFILE_KEYS = {"package", "name", "default_features", "features"}
PACKAGE_KEYS = {
    "name", "path", "description", "documentation", "readme", "keywords",
    "categories", "api_baseline", "candidate_dependencies", "published_dependencies",
    "features",
}
TOOL_KEYS = {"cargo_public_api", "cargo_semver_checks", "cargo_cyclonedx", "cyclonedx_spec"}
EXPECTED_TRAINS = (
    {
        "id": "crypto", "lifecycle": "published", "primary_package": "identus-crypto",
        "descriptor": CRYPTO_DESCRIPTOR.as_posix(), "tag": "v0.1.0-rc.1",
        "packages": ["identus-derive", "identus-core", "identus-crypto"],
    },
    {
        "id": "did", "lifecycle": "candidate-only", "primary_package": "identus-did",
        "descriptor": DID_DESCRIPTOR.as_posix(), "tag": "identus-did-v0.1.0-rc.1",
        "packages": list(DID_PACKAGES),
    },
)
EXPECTED_PROFILES = (
    ("identus-did", "default", True, ()),
    ("identus-did", "no-default-features", False, ()),
    ("identus-did-resolver-http", "default", True, ()),
    ("identus-did-resolver-http", "no-default-features", False, ()),
    ("identus-did-resolver-http", "all-features", True, ("openapi",)),
    ("identus-did-resolver-http", "openapi", False, ("openapi",)),
)
EXPECTED_INTERNAL = {
    "identus-did": {"identus-core", "identus-derive"},
    "identus-did-resolver-http": {"identus-core", "identus-did"},
}


def load_toml(path: Path, errors: list[str]) -> dict[str, Any]:
    try:
        with path.open("rb") as source:
            value = tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        errors.append(f"cannot read TOML {path}: {error}")
        return {}
    if not isinstance(value, dict):
        errors.append(f"TOML root is not a table: {path}")
        return {}
    return value


def read(path: Path, errors: list[str]) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as error:
        errors.append(f"cannot read {path}: {error}")
        return ""


def require_keys(value: dict[str, Any], expected: set[str], label: str, errors: list[str]) -> None:
    actual = set(value)
    if actual != expected:
        errors.append(
            f"{label} fields differ: missing={sorted(expected - actual)!r} "
            f"unexpected={sorted(actual - expected)!r}"
        )


def internal_dependencies(manifest: dict[str, Any]) -> set[str]:
    result: set[str] = set()
    for table_name in ("dependencies", "dev-dependencies", "build-dependencies"):
        table = manifest.get(table_name, {})
        if isinstance(table, dict):
            result.update(name for name in table if name.startswith("identus-"))
    return result


def validate(root: Path) -> list[str]:
    errors: list[str] = []
    index = load_toml(root / INDEX, errors)
    require_keys(index, INDEX_KEYS, "train index", errors)
    if index.get("schema_version") != 1:
        errors.append("train index schema_version must equal 1")
    trains = index.get("trains")
    if not isinstance(trains, list):
        errors.append("train index trains must be an array")
        trains = []
    ids: set[str] = set()
    tags: set[str] = set()
    descriptors: set[str] = set()
    owned: set[str] = set()
    for position, train in enumerate(trains):
        if not isinstance(train, dict):
            errors.append(f"train {position} must be a table")
            continue
        require_keys(train, TRAIN_KEYS, f"train {position}", errors)
        for field, seen in (("id", ids), ("tag", tags), ("descriptor", descriptors)):
            value = train.get(field)
            if not isinstance(value, str) or not value:
                errors.append(f"train {position} {field} must be a non-empty string")
            elif value in seen:
                errors.append(f"duplicate train {field}: {value}")
            else:
                seen.add(value)
        packages = train.get("packages")
        if not isinstance(packages, list) or not packages or not all(
            isinstance(item, str) and item for item in packages
        ):
            errors.append(f"train {position} packages must be a non-empty string array")
        else:
            if train.get("primary_package") not in packages:
                errors.append(f"train {position} primary_package is not owned")
            for package in packages:
                if package in owned:
                    errors.append(f"package belongs to multiple trains: {package}")
                owned.add(package)
        if train.get("id") != "crypto":
            expected_tag = f"{train.get('primary_package')}-v{VERSION}"
            if train.get("tag") != expected_tag:
                errors.append(f"train {position} tag must equal {expected_tag}")
    if trains != list(EXPECTED_TRAINS):
        errors.append("train index differs from the closed crypto and DID records")

    crypto = load_toml(root / CRYPTO_DESCRIPTOR, errors)
    if crypto.get("release_tag") != "v0.1.0-rc.1":
        errors.append("immutable crypto descriptor tag differs")
    if [row.get("name") for row in crypto.get("packages", []) if isinstance(row, dict)] != [
        "identus-derive", "identus-core", "identus-crypto"
    ]:
        errors.append("immutable crypto descriptor package order differs")

    descriptor = load_toml(root / DID_DESCRIPTOR, errors)
    require_keys(descriptor, DESCRIPTOR_KEYS, "DID descriptor", errors)
    expected_scalars = {
        "schema_version": 1,
        "candidate": "identus-did-0.1.0-rc.1",
        "version": VERSION,
        "rust_version": "1.89.0",
        "preparation_rust_version": "1.98.1",
        "baseline_revision": "96cf5f577b5f3585461d34589aad465297693af0",
        "repository": "https://github.com/hyperledger-identus/sdk-rust",
        "homepage": "https://hyperledger-identus.github.io/sdk-rust/",
        "license": "Apache-2.0",
        "max_archive_bytes": 1048576,
        "max_archive_members": 512,
        "max_expansion_ratio": 20,
        "max_evidence_bytes": 16777216,
        "publication": "candidate-only",
        "release_tag": "identus-did-v0.1.0-rc.1",
        "compatibility_status": "not-applicable-first-candidate",
    }
    for field, expected in expected_scalars.items():
        if descriptor.get(field) != expected:
            errors.append(f"DID descriptor differs: {field}")
    if not re.fullmatch(r"[0-9a-f]{40}", str(descriptor.get("baseline_revision", ""))):
        errors.append("DID descriptor baseline_revision must be a full lowercase SHA")
    tools = descriptor.get("tools")
    if not isinstance(tools, dict):
        errors.append("DID descriptor tools must be a table")
    else:
        require_keys(tools, TOOL_KEYS, "DID descriptor tools", errors)
        expected_tools = {
            "cargo_public_api": "0.52.0",
            "cargo_semver_checks": "0.50.0",
            "cargo_cyclonedx": "0.5.9",
            "cyclonedx_spec": "1.5",
        }
        if tools != expected_tools:
            errors.append("DID descriptor evidence tools differ")

    profiles = descriptor.get("profiles")
    observed_profiles: list[tuple[Any, Any, Any, tuple[Any, ...]]] = []
    if not isinstance(profiles, list):
        errors.append("DID descriptor profiles must be an array")
        profiles = []
    for position, profile in enumerate(profiles):
        if not isinstance(profile, dict):
            errors.append(f"DID profile {position} must be a table")
            continue
        require_keys(profile, PROFILE_KEYS, f"DID profile {position}", errors)
        features = profile.get("features")
        observed_profiles.append((
            profile.get("package"), profile.get("name"), profile.get("default_features"),
            tuple(features) if isinstance(features, list) else (None,),
        ))
    if tuple(observed_profiles) != EXPECTED_PROFILES:
        errors.append("DID descriptor profile matrix differs")

    packages = descriptor.get("packages")
    if not isinstance(packages, list):
        errors.append("DID descriptor packages must be an array")
        packages = []
    if [row.get("name") for row in packages if isinstance(row, dict)] != list(DID_PACKAGES):
        errors.append("DID descriptor package order differs")
    workspace = load_toml(root / "Cargo.toml", errors)
    workspace_dependencies = workspace.get("workspace", {}).get("dependencies", {})
    for position, package in enumerate(packages):
        if not isinstance(package, dict):
            errors.append(f"DID package {position} must be a table")
            continue
        require_keys(package, PACKAGE_KEYS, f"DID package {position}", errors)
        name = package.get("name")
        if name not in DID_PACKAGES:
            continue
        expected_path = PACKAGE_PATHS[name]
        if package.get("path") != expected_path.as_posix():
            errors.append(f"DID package path differs: {name}")
        manifest = load_toml(root / expected_path / "Cargo.toml", errors)
        metadata = manifest.get("package", {})
        if metadata.get("name") != name:
            errors.append(f"canonical package identity differs: {name}")
        if metadata.get("version") != {"workspace": True}:
            errors.append(f"candidate-only package overrides workspace version: {name}")
        if metadata.get("publish") != {"workspace": True}:
            errors.append(f"candidate-only package overrides publication denial: {name}")
        workspace_entry = workspace_dependencies.get(name)
        if not isinstance(workspace_entry, dict) or workspace_entry.get("path") != expected_path.as_posix():
            errors.append(f"workspace path differs: {name}")
        if isinstance(workspace_entry, dict) and "version" in workspace_entry:
            errors.append(f"candidate-only workspace dependency has a release version: {name}")
        actual_internal = internal_dependencies(manifest)
        staged = package.get("candidate_dependencies")
        published = package.get("published_dependencies")
        if not isinstance(staged, list) or not isinstance(published, list):
            errors.append(f"DID package dependency partitions must be arrays: {name}")
        else:
            if set(staged) & set(published):
                errors.append(f"DID package dependency partitions overlap: {name}")
            if set(staged) | set(published) != EXPECTED_INTERNAL[name]:
                errors.append(f"DID package dependency partition differs: {name}")
        features = manifest.get("features", {})
        actual_features = sorted(key for key in features if key != "default") if isinstance(features, dict) else []
        if package.get("features") != actual_features:
            errors.append(f"DID package feature declaration differs: {name}")
        readme = package.get("readme")
        if not isinstance(readme, str) or not (root / expected_path / readme).is_file():
            errors.append(f"DID package README is missing: {name}")
        api_baseline = package.get("api_baseline")
        expected_baseline = f"docs/release/{name}-{VERSION}.api.txt"
        if api_baseline != expected_baseline:
            errors.append(f"DID package API baseline path differs: {name}")
        elif not (root / api_baseline).is_file() or not (root / api_baseline).read_text(
            encoding="utf-8"
        ).strip():
            errors.append(f"DID package API baseline is missing or empty: {name}")
        for field in ("description", "documentation", "keywords", "categories"):
            if not package.get(field):
                errors.append(f"DID package release metadata missing {field}: {name}")

    builder = read(root / BUILDER, errors)
    required_builder = (
        "require_vcs_independent_build_scratch", "cargo", "package", "--workspace",
        "--locked", "--no-verify", "--allow-dirty", "inspect_archive",
        "verify_closure", "require_local_command", "ALLOWED_CARGO_OPERATIONS",
        "release_evidence", "public-api", "cyclonedx",
        "repositoryPolicy", "candidateSpecificScan", "os.replace", "candidate-receipt.json",
    )
    for phrase in required_builder:
        if phrase not in builder:
            errors.append(f"DID candidate builder is missing contract: {phrase}")
    forbidden_builder = (
        "cargo publish", "git tag", "gh release", "CARGO_REGISTRY_TOKEN",
        "CARGO_PUBLISH", "crates-io-auth-action",
    )
    for phrase in forbidden_builder:
        if phrase in builder:
            errors.append(f"candidate-only builder contains remote mutation capability: {phrase}")

    adr = read(root / ADR, errors)
    for phrase in (
        "`<primary-exact-cargo-package>-v<version>`",
        "`identus-did-v0.1.0-rc.1`", "candidate-only", "never moved",
    ):
        if phrase not in adr:
            errors.append(f"release-train ADR is missing decision evidence: {phrase}")
    api_adr = read(
        root / "docs/adr/0154-use-first-candidate-api-snapshots-as-semver-origin.md", errors
    )
    for phrase in (
        "cargo-public-api 0.52.0", "cargo-semver-checks 0.50.0",
        "cargo-cyclonedx 0.5.9", "`not-applicable`", "not a stable API promise",
    ):
        if phrase not in api_adr:
            errors.append(f"DID API-origin ADR is missing decision evidence: {phrase}")
    return errors


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    errors = validate(root)
    if errors:
        for error in errors:
            print(f"release-candidates: {error}", file=sys.stderr)
        print(f"release-candidates: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    print("release-candidates: closed crypto and DID train contract passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
