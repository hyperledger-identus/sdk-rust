#!/usr/bin/env python3

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path
from typing import Any


INVENTORY_PATH = Path("docs/architecture/sdk-bootstrap-inventory.toml")
TOP_LEVEL_KEYS = {
    "schema_version",
    "public_api_inventory",
    "governance_documents",
    "repository",
    "authority",
    "canonical_policies",
    "packages",
}
REQUIRED_GOVERNANCE_DOCUMENTS = {
    ".github/CODEOWNERS",
    ".github/ISSUE_TEMPLATE/component-change.yml",
    ".github/ISSUE_TEMPLATE/delivery-task.yml",
    ".github/pull_request_template.md",
    "CODE_OF_CONDUCT.md",
    "CONTRIBUTING.md",
    "DCO.md",
    "GOVERNANCE.md",
    "LICENSE",
    "MAINTAINERS.md",
    "RELEASING.md",
    "SECURITY.md",
    "docs/governance/repository-settings.md",
}
REPOSITORY_CONTRACT = {
    "name": "hyperledger-identus/sdk-rust",
    "active_branch": "develop",
    "reserved_branch": "main",
    "baseline_revision": "662f8d7d2b9b9a151365c6bb889cd614bca625f7",
    "release_state": "unreleased",
    "publication_state": "disabled",
    "parent_issue": "#4",
    "delivery_issue": "#25",
    "namespace_issue": "#3",
    "live_controls_state": "external-action-required",
    "live_controls_issue": "#26",
}
AUTHORITY_CONTRACT = {
    "maintainers": "canonical-identus-policy",
    "routine_delivery": "issue-linked-reviewed-green-ci-develop",
    "release": "assigned-human-release-manager-with-second-maintainer",
    "repository_admin": "human-maintainers",
    "security_response": "identus-security-response-team",
}
POLICY_PATHS = {"MAINTAINERS.md", "CONTRIBUTING.md", "SECURITY.md", "DCO.md"}
POLICY_KEYS = {"repository", "revision", "path", "blob_sha"}
PACKAGE_KEYS = {
    "name",
    "path",
    "layer",
    "classification",
    "public_api",
    "owner_issue",
    "summary",
}
PUBLIC_API_BY_CLASS = {
    "implemented": "experimental",
    "verification": "internal",
    "placeholder": "none",
}
LAYER_NAMES = {
    "Foundation": "foundation",
    "DomainPrimitives": "domain-primitives",
    "CredentialSemantics": "credential-semantics",
    "ProtocolSemantics": "protocol-semantics",
    "Orchestration": "orchestration",
    "OuterBoundary": "outer-boundary",
    "Verification": "verification",
}
ISSUE_PATTERN = re.compile(r"#[1-9][0-9]*\Z")
SHA_PATTERN = re.compile(r"[0-9a-f]{40}\Z")
DEPENDENCY_SECTIONS = ("dependencies", "dev-dependencies", "build-dependencies")


def load_toml(path: Path, failures: list[str], label: str) -> dict[str, Any]:
    try:
        with path.open("rb") as source:
            value = tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        failures.append(f"cannot read {label} {path}: {error}")
        return {}
    if not isinstance(value, dict):
        failures.append(f"{label} must be a TOML table")
        return {}
    return value


def validate_exact_keys(
    value: object, expected: set[str], label: str, failures: list[str]
) -> dict[str, Any]:
    if not isinstance(value, dict):
        failures.append(f"{label} must be a table")
        return {}
    actual = set(value)
    missing = sorted(expected - actual)
    unknown = sorted(actual - expected)
    if missing:
        failures.append(f"{label} missing keys: {','.join(missing)}")
    if unknown:
        failures.append(f"{label} unknown keys: {','.join(unknown)}")
    return value


def safe_relative_path(value: object, label: str, failures: list[str]) -> Path | None:
    if not isinstance(value, str) or not value:
        failures.append(f"{label} must be a non-empty relative path")
        return None
    path = Path(value)
    if path.is_absolute() or ".." in path.parts or path == Path("."):
        failures.append(f"{label} must stay inside the repository: {value}")
        return None
    return path


def manifest_path_dependencies(
    manifest: dict[str, Any], manifest_dir: Path
) -> list[tuple[str, Path]]:
    dependencies: list[tuple[str, Path]] = []

    def collect(table: object) -> None:
        if not isinstance(table, dict):
            return
        for name, value in table.items():
            if isinstance(value, dict) and isinstance(value.get("path"), str):
                dependencies.append((name, (manifest_dir / value["path"]).resolve()))

    for section in DEPENDENCY_SECTIONS:
        collect(manifest.get(section))
    targets = manifest.get("target")
    if isinstance(targets, dict):
        for target in targets.values():
            if isinstance(target, dict):
                for section in DEPENDENCY_SECTIONS:
                    collect(target.get(section))
    return dependencies


def workspace_packages(root: Path, failures: list[str]) -> dict[str, Path]:
    root_manifest = load_toml(root / "Cargo.toml", failures, "workspace manifest")
    workspace = root_manifest.get("workspace")
    if not isinstance(workspace, dict):
        failures.append("workspace manifest is missing [workspace]")
        return {}

    package_defaults = workspace.get("package")
    if not isinstance(package_defaults, dict) or package_defaults.get("publish") is not False:
        failures.append("workspace.package.publish must be false")

    members = workspace.get("members")
    if not isinstance(members, list) or not members or not all(isinstance(item, str) for item in members):
        failures.append("workspace.members must be a non-empty string array")
        return {}

    exclusions = workspace.get("exclude", [])
    if not isinstance(exclusions, list) or not all(isinstance(item, str) for item in exclusions):
        failures.append("workspace.exclude must be a string array when present")
        return {}

    member_dirs: set[Path] = set()
    for pattern in members:
        for candidate in root.glob(pattern):
            if candidate.is_dir() and (candidate / "Cargo.toml").is_file():
                member_dirs.add(candidate.resolve())
    for pattern in exclusions:
        for candidate in root.glob(pattern):
            member_dirs.discard(candidate.resolve())

    dependency_sources: list[tuple[str, dict[str, Any], Path]] = [
        (
            "workspace",
            {"dependencies": workspace.get("dependencies", {})},
            root.resolve(),
        )
    ]
    for member_dir in sorted(member_dirs):
        dependency_sources.append(
            (
                member_dir.relative_to(root.resolve()).as_posix(),
                load_toml(
                    member_dir / "Cargo.toml", failures, "package manifest"
                ),
                member_dir,
            )
        )
    for source, manifest, manifest_dir in dependency_sources:
        for dependency, dependency_path in manifest_path_dependencies(
            manifest, manifest_dir
        ):
            try:
                dependency_path.relative_to(root.resolve())
            except ValueError:
                continue
            if dependency_path not in member_dirs:
                failures.append(
                    f"{source}: in-tree path dependency {dependency} must be an explicit workspace member: "
                    f"{dependency_path.relative_to(root.resolve()).as_posix()}"
                )

    packages: dict[str, Path] = {}
    for member_dir in sorted(member_dirs):
        manifest = load_toml(member_dir / "Cargo.toml", failures, "package manifest")
        package = manifest.get("package")
        if not isinstance(package, dict) or not isinstance(package.get("name"), str):
            failures.append(f"package manifest has no package.name: {member_dir}")
            continue
        name = package["name"]
        if name in packages:
            failures.append(f"duplicate workspace package name: {name}")
        packages[name] = member_dir.relative_to(root.resolve())

        if package.get("publish") != {"workspace": True}:
            failures.append(f"{name}: package.publish.workspace must be true")

    return packages


def rulebook_layers(root: Path, failures: list[str]) -> dict[str, str]:
    path = root / "crates/conformance/src/rulebook.rs"
    try:
        source = path.read_text(encoding="utf-8")
    except OSError as error:
        failures.append(f"cannot read layer rulebook {path}: {error}")
        return {}

    layers: dict[str, str] = {}
    block_pattern = re.compile(
        r"LayerRule\s*\{\s*layer:\s*Layer::([A-Za-z]+),"
        r"\s*members:\s*&\[(.*?)\],\s*allowed_target_layers:",
        re.DOTALL,
    )
    for match in block_pattern.finditer(source):
        rust_layer, members = match.groups()
        layer = LAYER_NAMES.get(rust_layer)
        if layer is None:
            failures.append(f"unknown rulebook layer: {rust_layer}")
            continue
        for name in re.findall(r'name:\s*"([^"]+)"', members):
            if name in layers:
                failures.append(f"duplicate rulebook package: {name}")
            layers[name] = layer

    if not layers:
        failures.append("layer rulebook contains no discoverable packages")
    return layers


def validate_governance(
    root: Path, inventory: dict[str, Any], failures: list[str]
) -> None:
    documents = inventory.get("governance_documents")
    if not isinstance(documents, list) or not all(isinstance(item, str) for item in documents):
        failures.append("governance_documents must be a string array")
        return
    duplicates = sorted({item for item in documents if documents.count(item) > 1})
    if duplicates:
        failures.append(f"duplicate governance documents: {','.join(duplicates)}")
    actual = set(documents)
    missing = sorted(REQUIRED_GOVERNANCE_DOCUMENTS - actual)
    unknown = sorted(actual - REQUIRED_GOVERNANCE_DOCUMENTS)
    if missing:
        failures.append(f"missing governance document entries: {','.join(missing)}")
    if unknown:
        failures.append(f"unknown governance document entries: {','.join(unknown)}")
    for relative in documents:
        path = safe_relative_path(relative, "governance document", failures)
        if path is not None and (not (root / path).is_file() or (root / path).stat().st_size == 0):
            failures.append(f"governance document is missing or empty: {relative}")

    policies = inventory.get("canonical_policies")
    if not isinstance(policies, list):
        failures.append("canonical_policies must be an array of tables")
        return
    seen_paths: set[str] = set()
    for index, raw_policy in enumerate(policies, start=1):
        policy = validate_exact_keys(raw_policy, POLICY_KEYS, f"canonical policy {index}", failures)
        path = policy.get("path")
        if isinstance(path, str):
            if path in seen_paths:
                failures.append(f"duplicate canonical policy path: {path}")
            seen_paths.add(path)
        if policy.get("repository") != "hyperledger-identus/.github":
            failures.append(f"canonical policy {index}: unexpected repository")
        if policy.get("revision") != "main":
            failures.append(f"canonical policy {index}: revision must be main")
        blob_sha = policy.get("blob_sha")
        if not isinstance(blob_sha, str) or not SHA_PATTERN.fullmatch(blob_sha):
            failures.append(f"canonical policy {index}: blob_sha must be 40 lowercase hex characters")
    if seen_paths != POLICY_PATHS:
        failures.append(
            "canonical policy paths must be exactly: " + ",".join(sorted(POLICY_PATHS))
        )


def placeholder_source_is_minimal(
    root: Path, package_name: str, package_path: Path, failures: list[str]
) -> None:
    package_root = root / package_path
    rust_files = sorted(
        path.relative_to(package_root).as_posix()
        for path in package_root.rglob("*.rs")
        if path.is_file()
    )
    if rust_files != ["src/lib.rs"]:
        failures.append(
            f"{package_name}: placeholder Rust files must be exactly src/lib.rs; received {','.join(rust_files)}"
        )
        return
    source_path = package_root / "src/lib.rs"
    try:
        source = source_path.read_text(encoding="utf-8")
    except OSError as error:
        failures.append(f"{package_name}: cannot read placeholder source: {error}")
        return
    if not re.search(r"(?m)^\s*//!", source):
        failures.append(f"{package_name}: placeholder must have crate documentation")
    executable_docs = re.search(
        r"(?m)^\s*//[!/]\s*(?:`{3,}|~{3,})", source
    ) or re.search(r"(?m)^\s*//[!/](?: {4,}|\t)\S", source)
    if executable_docs:
        failures.append(
            f"{package_name}: placeholder documentation must not contain executable code blocks"
        )
    stripped = re.sub(r"(?m)^\s*//[/!].*(?:\n|\Z)", "", source)
    shape = re.compile(
        r'\s*use\s+identus_core::Component;\s*'
        r'pub\s+const\s+COMPONENT:\s*Component\s*=\s*Component\s*\{\s*'
        rf'name:\s*"{re.escape(package_name)}",\s*'
        r'summary:\s*"[^"\n]+",\s*\};\s*',
        re.DOTALL,
    )
    if shape.fullmatch(stripped) is None:
        failures.append(f"{package_name}: placeholder source exceeds the COMPONENT marker shape")


def implemented_source_has_api(
    root: Path, package_name: str, package_path: Path, failures: list[str]
) -> None:
    public_items: set[str] = set()
    for source_path in (root / package_path / "src").rglob("*.rs"):
        try:
            source = source_path.read_text(encoding="utf-8")
        except OSError as error:
            failures.append(f"{package_name}: cannot read source {source_path}: {error}")
            continue
        public_items.update(
            re.findall(
                r"(?m)^\s*pub\s+(?:struct|enum|trait|type|fn|mod|use|const|static)\s+([A-Za-z_][A-Za-z0-9_]*)",
                source,
            )
        )
    public_items.discard("COMPONENT")
    if not public_items:
        failures.append(f"{package_name}: implemented classification has no public non-marker item")


def validate_packages(
    root: Path, inventory: dict[str, Any], failures: list[str]
) -> None:
    workspace = workspace_packages(root, failures)
    layers = rulebook_layers(root, failures)
    raw_packages = inventory.get("packages")
    if not isinstance(raw_packages, list):
        failures.append("packages must be an array of tables")
        return

    entries: dict[str, tuple[dict[str, Any], Path]] = {}
    seen_paths: set[Path] = set()
    for index, raw_entry in enumerate(raw_packages, start=1):
        entry = validate_exact_keys(raw_entry, PACKAGE_KEYS, f"package entry {index}", failures)
        name = entry.get("name")
        if not isinstance(name, str) or not name:
            failures.append(f"package entry {index}: name must be non-empty")
            continue
        path = safe_relative_path(entry.get("path"), f"{name}: path", failures)
        if path is None:
            continue
        if name in entries:
            failures.append(f"duplicate inventory package: {name}")
        if path in seen_paths:
            failures.append(f"duplicate inventory package path: {path.as_posix()}")
        entries[name] = (entry, path)
        seen_paths.add(path)

        classification = entry.get("classification")
        if classification not in PUBLIC_API_BY_CLASS:
            failures.append(f"{name}: unknown classification {classification}")
        elif entry.get("public_api") != PUBLIC_API_BY_CLASS[classification]:
            failures.append(
                f"{name}: public_api must be {PUBLIC_API_BY_CLASS[classification]} for {classification}"
            )
        owner_issue = entry.get("owner_issue")
        if not isinstance(owner_issue, str) or not ISSUE_PATTERN.fullmatch(owner_issue):
            failures.append(f"{name}: owner_issue must match #N")
        if not isinstance(entry.get("summary"), str) or not entry["summary"].strip():
            failures.append(f"{name}: summary must be non-empty")

    inventory_names = set(entries)
    workspace_names = set(workspace)
    if inventory_names != workspace_names:
        missing = sorted(workspace_names - inventory_names)
        unknown = sorted(inventory_names - workspace_names)
        if missing:
            failures.append(f"inventory missing workspace packages: {','.join(missing)}")
        if unknown:
            failures.append(f"inventory has unknown workspace packages: {','.join(unknown)}")

    if set(layers) != workspace_names:
        failures.append("layer rulebook package set does not match Cargo workspace packages")

    for name, (entry, inventory_path) in entries.items():
        workspace_path = workspace.get(name)
        if workspace_path is None:
            continue
        if inventory_path != workspace_path:
            failures.append(
                f"{name}: inventory path {inventory_path.as_posix()} does not match {workspace_path.as_posix()}"
            )
        expected_layer = layers.get(name)
        if entry.get("layer") != expected_layer:
            failures.append(
                f"{name}: inventory layer {entry.get('layer')} does not match rulebook {expected_layer}"
            )

        manifest = load_toml(root / workspace_path / "Cargo.toml", failures, f"{name} manifest")
        classification = entry.get("classification")
        if classification == "placeholder":
            package = manifest.get("package")
            if isinstance(package, dict) and "build" in package:
                failures.append(f"{name}: placeholder must not declare package.build")
            dependencies = manifest.get("dependencies")
            if not isinstance(dependencies, dict) or set(dependencies) != {"identus-core"}:
                actual = sorted(dependencies) if isinstance(dependencies, dict) else []
                failures.append(
                    f"{name}: placeholder dependencies must be exactly identus-core; received {','.join(actual)}"
                )
            elif dependencies.get("identus-core") != {"workspace": True}:
                failures.append(
                    f"{name}: identus-core must inherit the workspace dependency"
                )
            for section in (
                "dev-dependencies",
                "build-dependencies",
                "features",
                "target",
                "lib",
                "bin",
                "example",
                "test",
                "bench",
            ):
                if section in manifest:
                    failures.append(f"{name}: placeholder must not declare {section}")
            placeholder_source_is_minimal(root, name, workspace_path, failures)
        elif classification == "implemented":
            implemented_source_has_api(root, name, workspace_path, failures)
        elif classification == "verification" and name != "identus-conformance":
            failures.append(f"{name}: only identus-conformance may be verification-only")


def validate(root: Path) -> list[str]:
    failures: list[str] = []
    inventory = load_toml(root / INVENTORY_PATH, failures, "bootstrap inventory")
    if not inventory:
        return failures

    validate_exact_keys(inventory, TOP_LEVEL_KEYS, "bootstrap inventory", failures)
    if inventory.get("schema_version") != 1:
        failures.append("schema_version must be 1")
    repository = validate_exact_keys(
        inventory.get("repository"), set(REPOSITORY_CONTRACT), "repository", failures
    )
    for key, expected in REPOSITORY_CONTRACT.items():
        if repository.get(key) != expected:
            failures.append(f"repository.{key} must be {expected}")
    authority = validate_exact_keys(
        inventory.get("authority"), set(AUTHORITY_CONTRACT), "authority", failures
    )
    for key, expected in AUTHORITY_CONTRACT.items():
        if authority.get(key) != expected:
            failures.append(f"authority.{key} must be {expected}")

    public_inventory_path = safe_relative_path(
        inventory.get("public_api_inventory"), "public_api_inventory", failures
    )
    if public_inventory_path is not None:
        public_path = root / public_inventory_path
        if not public_path.is_file() or public_path.stat().st_size == 0:
            failures.append(f"public API inventory is missing or empty: {public_inventory_path}")
        else:
            public_text = public_path.read_text(encoding="utf-8")
            for raw_package in inventory.get("packages", []):
                if isinstance(raw_package, dict) and isinstance(raw_package.get("name"), str):
                    if f"`{raw_package['name']}`" not in public_text:
                        failures.append(
                            f"public API inventory does not mention {raw_package['name']}"
                        )

    validate_governance(root, inventory, failures)
    validate_packages(root, inventory, failures)
    return failures


def main() -> int:
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path(__file__).resolve().parents[1]
    failures = validate(root)
    if failures:
        for failure in failures:
            print(f"sdk-bootstrap-inventory: {failure}", file=sys.stderr)
        return 1
    inventory = load_toml(root / INVENTORY_PATH, [], "bootstrap inventory")
    print(
        "sdk-bootstrap-inventory: contract passed "
        f"({len(inventory.get('packages', []))} packages)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
