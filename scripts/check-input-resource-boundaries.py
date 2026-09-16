#!/usr/bin/env python3
"""Validate the SDK input-resource boundary inventory without network access."""

from __future__ import annotations

import argparse
import re
import sys
import tomllib
from pathlib import Path
from typing import Any


MAX_FILE_BYTES = 256 * 1024
MAX_BOUNDARIES = 256
MAX_ARRAY_ITEMS = 64
MAX_VALUE_BYTES = 4_096
MAX_SOURCE_FILES_PER_PACKAGE = 512
MAX_SOURCE_FILE_BYTES = 2 * 1_024 * 1_024
TOP_LEVEL_KEYS = {
    "schema_version",
    "repository",
    "owner_issue",
    "baseline_revision",
    "boundaries",
}
BOUNDARY_KEYS = {
    "id",
    "package",
    "surface",
    "disposition",
    "limits",
    "evidence",
    "outer_obligation",
    "consumer_impact",
    "review_triggers",
}
DISPOSITIONS = {
    "sdk-enforced",
    "fixed-or-no-input",
    "caller-budgeted-work",
    "outer-preallocation",
    "known-unbounded-compatibility",
}
IDENTIFIER = re.compile(r"[a-z0-9][a-z0-9-]*\Z")
REVISION = re.compile(r"[0-9a-f]{40}\Z")
ISSUE = re.compile(r"#[1-9][0-9]*\Z")
LIMIT = re.compile(r"[A-Za-z][A-Za-z0-9_]*=(?:[1-9][0-9]*|explicit)\Z")
PUBLIC_RESOURCE_LIMIT = re.compile(
    rb"(?m)^pub const ((?:[A-Z0-9]+_)*(?:MAX|MIN)_[A-Z0-9_]+)\s*:"
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


def read_toml(path: Path, label: str, failures: list[str]) -> dict[str, Any] | None:
    try:
        if path.is_symlink():
            failures.append(f"{label} must not be a symlink: {path}")
            return None
        size = path.stat().st_size
        if size > MAX_FILE_BYTES:
            failures.append(f"{label} exceeds {MAX_FILE_BYTES} bytes: {size}")
            return None
        content = path.read_bytes()
        return tomllib.loads(content.decode("utf-8"))
    except FileNotFoundError:
        failures.append(f"missing {label}: {path}")
    except UnicodeDecodeError:
        failures.append(f"{label} is not valid UTF-8: {path}")
    except tomllib.TOMLDecodeError as error:
        failures.append(f"malformed {label}: {error}")
    except OSError as error:
        failures.append(f"cannot read {label}: {error}")
    return None


def bounded_string(value: Any, label: str, failures: list[str]) -> str | None:
    if not isinstance(value, str) or not value.strip():
        failures.append(f"{label} must be a non-empty string")
        return None
    if len(value.encode("utf-8")) > MAX_VALUE_BYTES:
        failures.append(f"{label} exceeds {MAX_VALUE_BYTES} UTF-8 bytes")
        return None
    return value


def bounded_string_array(value: Any, label: str, failures: list[str]) -> list[str]:
    if not isinstance(value, list):
        failures.append(f"{label} must be an array")
        return []
    if len(value) > MAX_ARRAY_ITEMS:
        failures.append(f"{label} exceeds {MAX_ARRAY_ITEMS} entries")
        return []
    result: list[str] = []
    for index, item in enumerate(value):
        parsed = bounded_string(item, f"{label}[{index}]", failures)
        if parsed is not None:
            result.append(parsed)
    if len(set(result)) != len(result):
        failures.append(f"{label} contains duplicate entries")
    return result


def implemented_package_paths(root: Path, failures: list[str]) -> dict[str, Path]:
    source = root / "docs/architecture/sdk-bootstrap-inventory.toml"
    document = read_toml(source, "bootstrap inventory", failures)
    if document is None:
        return {}
    packages = document.get("packages")
    if not isinstance(packages, list):
        failures.append("bootstrap inventory packages must be an array")
        return {}
    implemented: dict[str, Path] = {}
    for index, package in enumerate(packages):
        if not isinstance(package, dict):
            failures.append(f"bootstrap package[{index}] must be a table")
            continue
        if package.get("classification") == "implemented":
            name = package.get("name")
            package_path = package.get("path")
            if isinstance(name, str) and isinstance(package_path, str):
                relative = Path(package_path)
                if relative.is_absolute() or ".." in relative.parts:
                    failures.append(
                        f"bootstrap package path must be repository-relative: {package_path}"
                    )
                    continue
                implemented[name] = relative
    return implemented


def public_resource_limits(
    root: Path, package_paths: dict[str, Path], failures: list[str]
) -> dict[str, set[str]]:
    discovered: dict[str, set[str]] = {}
    for package, relative in package_paths.items():
        source_root = root / relative
        source_files = sorted(source_root.rglob("*.rs"))
        if len(source_files) > MAX_SOURCE_FILES_PER_PACKAGE:
            failures.append(
                f"{package} exceeds {MAX_SOURCE_FILES_PER_PACKAGE} Rust source files"
            )
            continue
        names: set[str] = set()
        for source in source_files:
            if not source.is_file() or has_symlink_component(source, root):
                failures.append(f"{package} source is missing or symlinked: {source}")
                continue
            try:
                size = source.stat().st_size
                if size > MAX_SOURCE_FILE_BYTES:
                    failures.append(
                        f"{package} source exceeds {MAX_SOURCE_FILE_BYTES} bytes: {source}"
                    )
                    continue
                content = source.read_bytes()
            except OSError as error:
                failures.append(f"cannot read {package} source {source}: {error}")
                continue
            names.update(
                match.group(1).decode("ascii")
                for match in PUBLIC_RESOURCE_LIMIT.finditer(content)
            )
        discovered[package] = names
    return discovered


def validate(root: Path, inventory_path: Path) -> list[str]:
    failures: list[str] = []
    document = read_toml(inventory_path, "resource-boundary inventory", failures)
    if document is None:
        return failures
    if set(document) != TOP_LEVEL_KEYS:
        failures.append(
            "resource-boundary inventory must contain exactly: "
            + ", ".join(sorted(TOP_LEVEL_KEYS))
        )
    if document.get("schema_version") != 1:
        failures.append("schema_version must be 1")
    if document.get("repository") != "hyperledger-identus/sdk-rust":
        failures.append("repository must be hyperledger-identus/sdk-rust")
    owner_issue = bounded_string(document.get("owner_issue"), "owner_issue", failures)
    if owner_issue is not None and ISSUE.fullmatch(owner_issue) is None:
        failures.append("owner_issue must be a positive #issue reference")
    revision = bounded_string(
        document.get("baseline_revision"), "baseline_revision", failures
    )
    if revision is not None and REVISION.fullmatch(revision) is None:
        failures.append("baseline_revision must be a lowercase 40-hex Git revision")

    boundaries = document.get("boundaries")
    if not isinstance(boundaries, list) or not boundaries:
        failures.append("boundaries must be a non-empty array")
        return failures
    if len(boundaries) > MAX_BOUNDARIES:
        failures.append(f"boundaries exceeds {MAX_BOUNDARIES} entries")
        return failures

    package_paths = implemented_package_paths(root, failures)
    expected_packages = set(package_paths)
    covered_packages: set[str] = set()
    identifiers: set[str] = set()
    declared_limit_names: dict[str, set[str]] = {
        package: set() for package in expected_packages
    }

    for index, boundary in enumerate(boundaries):
        label = f"boundaries[{index}]"
        if not isinstance(boundary, dict):
            failures.append(f"{label} must be a table")
            continue
        if set(boundary) != BOUNDARY_KEYS:
            failures.append(f"{label} must contain exactly: {', '.join(sorted(BOUNDARY_KEYS))}")

        identifier = bounded_string(boundary.get("id"), f"{label}.id", failures)
        if identifier is not None:
            if IDENTIFIER.fullmatch(identifier) is None:
                failures.append(f"{label}.id must be lowercase kebab-case")
            if identifier in identifiers:
                failures.append(f"duplicate boundary id: {identifier}")
            identifiers.add(identifier)

        package = bounded_string(boundary.get("package"), f"{label}.package", failures)
        if package is not None:
            if package not in expected_packages:
                failures.append(f"{label} references non-implemented package: {package}")
            else:
                covered_packages.add(package)

        bounded_string(boundary.get("surface"), f"{label}.surface", failures)
        bounded_string(
            boundary.get("outer_obligation"), f"{label}.outer_obligation", failures
        )
        bounded_string(
            boundary.get("consumer_impact"), f"{label}.consumer_impact", failures
        )
        triggers = bounded_string_array(
            boundary.get("review_triggers"), f"{label}.review_triggers", failures
        )
        if not triggers:
            failures.append(f"{label}.review_triggers must not be empty")

        disposition = bounded_string(
            boundary.get("disposition"), f"{label}.disposition", failures
        )
        if disposition is not None and disposition not in DISPOSITIONS:
            failures.append(f"{label} has unknown disposition: {disposition}")

        limits = bounded_string_array(boundary.get("limits"), f"{label}.limits", failures)
        for limit in limits:
            if LIMIT.fullmatch(limit) is None:
                failures.append(f"{label} has invalid limit declaration: {limit}")
            elif package in declared_limit_names:
                declared_limit_names[package].add(limit.partition("=")[0])
        if disposition == "sdk-enforced" and not limits:
            failures.append(f"{label} sdk-enforced boundary requires explicit limits")
        if disposition in DISPOSITIONS - {"sdk-enforced"} and limits:
            failures.append(f"{label} non-enforced boundary must not claim SDK limits")

        evidence = bounded_string_array(
            boundary.get("evidence"), f"{label}.evidence", failures
        )
        if not evidence:
            failures.append(f"{label}.evidence must not be empty")
        for item in evidence:
            relative = Path(item)
            if relative.is_absolute() or ".." in relative.parts:
                failures.append(f"{label} evidence must be repository-relative: {item}")
                continue
            candidate = root / relative
            if not candidate.is_file() or has_symlink_component(candidate, root):
                failures.append(f"{label} evidence is missing or symlinked: {item}")

    missing = sorted(expected_packages - covered_packages)
    if missing:
        failures.append("implemented packages missing boundary coverage: " + ", ".join(missing))
    for package, names in public_resource_limits(root, package_paths, failures).items():
        omitted = sorted(names - declared_limit_names[package])
        if omitted:
            failures.append(
                f"{package} public resource limits missing from inventory: "
                + ", ".join(omitted)
            )
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    parser.add_argument("--inventory")
    args = parser.parse_args()
    root = Path(args.root).resolve()
    inventory_path = (
        Path(args.inventory).resolve()
        if args.inventory
        else root / "docs/architecture/sdk-input-resource-boundaries.toml"
    )
    failures = validate(root, inventory_path)
    if failures:
        for failure in failures:
            print(f"input-resource-boundaries: {failure}", file=sys.stderr)
        print(
            f"input-resource-boundaries: {len(failures)} failure(s)", file=sys.stderr
        )
        return 1
    with inventory_path.open("rb") as source:
        count = len(tomllib.load(source)["boundaries"])
    print(f"input-resource-boundaries: {count} boundaries passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
