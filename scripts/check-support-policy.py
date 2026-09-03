#!/usr/bin/env python3
"""Validate the machine-readable SDK compatibility policy offline."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any

import tomllib

POLICY_PATH = Path("docs/architecture/sdk-support-policy.toml")
GATE_MANIFEST_PATH = Path("nix/checks/gates.toml")
GATE_FIELDS = {
    "name",
    "operation",
    "toolchain",
    "source",
    "artifacts",
    "locked",
    "workspace",
    "packages",
    "exclude_packages",
    "lib",
    "all_targets",
    "no_default_features",
    "all_features",
    "features",
    "target",
    "extra_args",
}
ALLOWED_OPERATIONS = {
    "cargoAudit",
    "cargoBuild",
    "cargoClippy",
    "cargoDeny",
    "cargoDoc",
    "cargoFmt",
    "cargoNextest",
}
OPERATIONS_WITH_CARGO_SELECTION = {
    "cargoBuild",
    "cargoClippy",
    "cargoDoc",
    "cargoNextest",
}
OPERATION_EXTRA_ARGS = {
    "cargoAudit": [],
    "cargoBuild": [],
    "cargoClippy": ["--", "-D", "warnings"],
    "cargoDeny": [],
    "cargoDoc": ["--no-deps"],
    "cargoFmt": [],
    "cargoNextest": ["--no-fail-fast", "--no-tests=pass"],
}
REQUIRED_HOSTS = {
    "x86_64-linux": "x86_64-unknown-linux-gnu",
    "aarch64-darwin": "aarch64-apple-darwin",
}
REQUIRED_TARGETS = {
    "wasm32-unknown-unknown": "compile-checked",
    "aarch64-linux-android": "compile-checked",
    "aarch64-apple-ios": "compile-checked",
    "x86_64-pc-windows-msvc": "planned",
    "wasm32-wasip1": "planned",
}
REQUIRED_COMPILE_PACKAGES = {
    "identus-core",
    "identus-crypto",
    "identus-did",
    "identus-adapters-entropy",
}
REQUIRED_FEATURE_SURFACES = {
    "workspace-default",
    "crypto-minimal",
    "crypto-kmp-compat",
    "entropy-minimal",
    "entropy-deterministic",
    "entropy-system-random",
    "entropy-all",
}
ALLOWED_TIERS = {"host-tested", "compile-checked", "planned", "not-supported"}


def load_toml(path: Path, failures: list[str]) -> dict[str, Any]:
    try:
        with path.open("rb") as source:
            value = tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        failures.append(f"cannot read {path}: {error}")
        return {}
    if not isinstance(value, dict):
        failures.append(f"{path} is not a TOML table")
        return {}
    return value


def require_table(
    data: dict[str, Any], name: str, failures: list[str]
) -> dict[str, Any]:
    value = data.get(name)
    if not isinstance(value, dict):
        failures.append(f"policy is missing [{name}]")
        return {}
    return value


def require_nonempty_string(
    table: dict[str, Any], field: str, context: str, failures: list[str]
) -> str:
    value = table.get(field)
    if not isinstance(value, str) or not value.strip():
        failures.append(f"{context} is missing non-empty {field}")
        return ""
    return value


def workspace_packages(
    root: Path, cargo: dict[str, Any], failures: list[str]
) -> tuple[set[str], dict[str, Path]]:
    dependencies = cargo.get("workspace", {}).get("dependencies", {})
    if not isinstance(dependencies, dict):
        failures.append("Cargo.toml is missing [workspace.dependencies]")
        return set(), {}

    packages: set[str] = set()
    manifests: dict[str, Path] = {}
    for alias, value in dependencies.items():
        if not isinstance(value, dict) or "path" not in value:
            continue
        manifest_path = root / str(value["path"]) / "Cargo.toml"
        manifest = load_toml(manifest_path, failures)
        package_name = manifest.get("package", {}).get("name")
        if not isinstance(package_name, str):
            failures.append(f"{manifest_path} has no package.name")
            continue
        packages.add(package_name)
        manifests[package_name] = manifest_path
        if alias != package_name:
            failures.append(
                f"workspace dependency alias {alias} does not match package {package_name}"
            )
    return packages, manifests


def nix_without_comments(text: str) -> str:
    without_blocks = re.sub(r"/\*.*?\*/", "", text, flags=re.DOTALL)
    return re.sub(r"#.*$", "", without_blocks, flags=re.MULTILINE)


def validate_gate_wiring(root: Path, failures: list[str]) -> None:
    checks_root = (root / "nix/checks").resolve()
    checks_entry = checks_root / "default.nix"
    flake_path = root / "flake.nix"
    flake = nix_without_comments(flake_path.read_text(encoding="utf-8"))
    root_imports = re.search(
        r"flake-parts\.lib\.mkFlake\s+\{[^{}]*\}\s+\{\s*"
        r"imports\s*=\s*\[(.*?)\];",
        flake,
        re.DOTALL,
    )
    flake_imports: set[Path] = set()
    if root_imports is not None:
        for relative in re.findall(r"\./([A-Za-z0-9_./-]+)", root_imports.group(1)):
            imported = (flake_path.parent / relative).resolve()
            if imported.is_dir():
                imported = imported / "default.nix"
            flake_imports.add(imported)
    if checks_entry not in flake_imports:
        failures.append("flake.nix does not import the nix/checks module")
        return

    default_nix = nix_without_comments(checks_entry.read_text(encoding="utf-8"))
    imports_match = re.search(r"\bimports\s*=\s*\[(.*?)\];", default_nix, re.DOTALL)
    check_imports: set[Path] = set()
    if imports_match is not None:
        for relative in re.findall(r"\./([A-Za-z0-9_./-]+)", imports_match.group(1)):
            check_imports.add((checks_entry.parent / relative).resolve())
    generator_path = checks_root / "rust-gates.nix"
    if generator_path not in check_imports:
        failures.append("nix/checks/default.nix does not import rust-gates.nix")

    if not generator_path.is_file():
        failures.append("nix/checks/rust-gates.nix does not exist")
        return
    generator = nix_without_comments(generator_path.read_text(encoding="utf-8"))
    manifest_binding = re.search(
        r"\bmanifest\s*=\s*builtins\.fromTOML\s*"
        r"\(builtins\.readFile\s+\./gates\.toml\)\s*;",
        generator,
    )
    generated_binding = re.search(
        r"\bgeneratedChecks\s*=\s*listToAttrs\s*\(\s*map\s*\("
        r".*?\)\s*manifest\.gates\s*\)\s*;",
        generator,
        re.DOTALL,
    )
    published_result = re.search(
        r"\bin\s*\{\s*checks\s*=\s*generatedChecks\s*;\s*\}\s*;\s*\}\s*$",
        generator,
    )
    if manifest_binding is None:
        failures.append("rust-gates.nix does not parse gates.toml as manifest")
    if generated_binding is None:
        failures.append(
            "rust-gates.nix does not derive generatedChecks from manifest.gates"
        )
    if published_result is None:
        failures.append(
            "rust-gates.nix does not return generatedChecks as top-level checks"
        )

    duplicates = sorted(
        path.relative_to(root)
        for path in checks_root.glob("rust-*.nix")
        if path.name != "rust-gates.nix"
    )
    if duplicates:
        failures.append(
            f"hand-written Rust gate modules duplicate the manifest: {duplicates}"
        )


def string_list(value: Any, context: str, failures: list[str]) -> list[str]:
    if not isinstance(value, list) or not all(
        isinstance(item, str) and item for item in value
    ):
        failures.append(f"{context} must be a list of non-empty strings")
        return []
    if len(value) != len(set(value)):
        failures.append(f"{context} must not contain duplicates")
    return value


def load_gate_manifest(
    root: Path,
    workspace: set[str],
    manifests: dict[str, Path],
    failures: list[str],
) -> dict[str, dict[str, Any]]:
    path = root / GATE_MANIFEST_PATH
    manifest = load_toml(path, failures)
    if manifest.get("schema_version") != 1:
        failures.append("gate manifest schema_version must be 1")
    if set(manifest) != {"schema_version", "gates"}:
        failures.append(
            f"gate manifest has unknown top-level fields {sorted(set(manifest) - {'schema_version', 'gates'})}"
        )
    raw_gates = manifest.get("gates")
    if not isinstance(raw_gates, list) or not raw_gates:
        failures.append("gate manifest is missing [[gates]] entries")
        return {}

    gates: dict[str, dict[str, Any]] = {}
    for index, gate in enumerate(raw_gates):
        context = f"gate manifest entry {index}"
        if not isinstance(gate, dict):
            failures.append(f"{context} must be a table")
            continue
        unknown = set(gate) - GATE_FIELDS
        missing = GATE_FIELDS - set(gate)
        if unknown or missing:
            failures.append(
                f"{context} fields do not match schema: missing={sorted(missing)}, unknown={sorted(unknown)}"
            )
        name = gate.get("name")
        if not isinstance(name, str) or re.fullmatch(r"rust-[a-z0-9-]+", name) is None:
            failures.append(f"{context} has invalid name {name!r}")
            continue
        if name in gates:
            failures.append(f"gate manifest contains duplicate gate {name!r}")
            continue
        gates[name] = {}

        operation = gate.get("operation")
        if not isinstance(operation, str) or operation not in ALLOWED_OPERATIONS:
            failures.append(f"gate {name} has unsupported operation {operation!r}")
            operation = ""
        gate["operation"] = operation
        toolchain = gate.get("toolchain")
        if not isinstance(toolchain, str) or toolchain not in {"etalon", "msrv"}:
            failures.append(f"gate {name} has invalid toolchain {toolchain!r}")
            toolchain = ""
        gate["toolchain"] = toolchain
        source = gate.get("source")
        if not isinstance(source, str) or source not in {"rust", "repository"}:
            failures.append(f"gate {name} has invalid source {source!r}")
            source = ""
        gate["source"] = source
        artifacts = gate.get("artifacts")
        if not isinstance(artifacts, str) or artifacts not in {
            "none",
            "etalon",
            "msrv",
        }:
            failures.append(f"gate {name} has invalid artifacts {artifacts!r}")
            artifacts = ""
        gate["artifacts"] = artifacts

        for field in (
            "locked",
            "workspace",
            "lib",
            "all_targets",
            "no_default_features",
            "all_features",
        ):
            value = gate.get(field)
            if not isinstance(value, bool):
                failures.append(f"gate {name}.{field} must be a boolean")
                gate[field] = False
        packages = string_list(gate.get("packages"), f"gate {name}.packages", failures)
        excluded = string_list(
            gate.get("exclude_packages"), f"gate {name}.exclude_packages", failures
        )
        features = string_list(gate.get("features"), f"gate {name}.features", failures)
        extra_args = string_list(
            gate.get("extra_args"), f"gate {name}.extra_args", failures
        )
        target = gate.get("target")
        if not isinstance(target, str):
            failures.append(f"gate {name}.target must be a string")
            target = ""
        gate["packages"] = packages
        gate["exclude_packages"] = excluded
        gate["features"] = features
        gate["extra_args"] = extra_args
        gate["target"] = target
        gates[name] = gate

        unknown_packages = (set(packages) | set(excluded)) - workspace
        if unknown_packages:
            failures.append(
                f"gate {name} names unknown packages {sorted(unknown_packages)}"
            )
        if gate.get("workspace") and packages:
            failures.append(
                f"gate {name} cannot select workspace and explicit packages"
            )
        if (
            operation in OPERATIONS_WITH_CARGO_SELECTION
            and not gate.get("workspace")
            and not packages
        ):
            failures.append(
                f"gate {name} operation {operation} requires explicit workspace or packages"
            )
        if excluded and not gate.get("workspace"):
            failures.append(f"gate {name} exclusions require workspace=true")
        if gate.get("all_features") and features:
            failures.append(
                f"gate {name} cannot select all_features and named features"
            )
        if gate.get("lib") and gate.get("all_targets"):
            failures.append(f"gate {name} cannot select lib and all_targets")
        if gate.get("no_default_features") and gate.get("all_features"):
            failures.append(
                f"gate {name} cannot select no_default_features and all_features"
            )
        if target and operation != "cargoBuild":
            failures.append(f"gate {name} target selection requires cargoBuild")

        has_selection = bool(
            gate.get("locked")
            or gate.get("workspace")
            or packages
            or excluded
            or gate.get("lib")
            or gate.get("all_targets")
            or gate.get("no_default_features")
            or gate.get("all_features")
            or features
            or target
            or gate.get("extra_args")
        )
        if operation not in OPERATIONS_WITH_CARGO_SELECTION and has_selection:
            failures.append(
                f"gate {name} operation {operation} cannot carry Cargo selection"
            )
        if (
            operation in OPERATION_EXTRA_ARGS
            and extra_args != OPERATION_EXTRA_ARGS[operation]
        ):
            failures.append(
                f"gate {name} operation {operation} requires extra_args={OPERATION_EXTRA_ARGS[operation]!r}"
            )
        if (
            operation in OPERATIONS_WITH_CARGO_SELECTION
            and gate.get("source") != "rust"
        ):
            failures.append(f"gate {name} operation {operation} requires rust source")
        if operation in OPERATIONS_WITH_CARGO_SELECTION and gate.get(
            "artifacts"
        ) != gate.get("toolchain"):
            failures.append(
                f"gate {name} operation {operation} artifacts must match its toolchain"
            )
        if operation in {"cargoAudit", "cargoDeny"} and (
            gate.get("source") != "repository" or gate.get("artifacts") != "none"
        ):
            failures.append(
                f"gate {name} operation {operation} requires repository source and no artifacts"
            )
        if operation == "cargoFmt" and (
            gate.get("source") != "rust" or gate.get("artifacts") != "none"
        ):
            failures.append(
                f"gate {name} cargoFmt requires rust source and no artifacts"
            )
        if gate.get("toolchain") == "msrv" and (
            operation != "cargoBuild" or gate.get("artifacts") != "msrv"
        ):
            failures.append(
                f"gate {name} MSRV toolchain requires cargoBuild and MSRV artifacts"
            )
        if gate.get("toolchain") == "etalon" and gate.get("artifacts") == "msrv":
            failures.append(f"gate {name} etalon toolchain cannot use MSRV artifacts")
        if gate.get("artifacts") == "etalon" and gate.get("toolchain") != "etalon":
            failures.append(f"gate {name} etalon artifacts require etalon toolchain")

        for feature in features:
            package_name, separator, feature_name = feature.partition("/")
            if separator:
                candidate_package = package_name
            elif len(packages) == 1:
                candidate_package = packages[0]
                feature_name = feature
            else:
                failures.append(
                    f"gate {name} feature {feature!r} requires one package or package/feature syntax"
                )
                continue
            manifest_path = manifests.get(candidate_package)
            if manifest_path is None:
                continue
            package_manifest = load_toml(manifest_path, failures)
            available = package_manifest.get("features", {})
            if not isinstance(available, dict) or feature_name not in available:
                failures.append(f"gate {name} references missing feature {feature!r}")
    return gates


def validate_gate(
    gate: Any,
    evidence_token: str,
    gates: dict[str, dict[str, Any]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or not gate:
        failures.append(f"{context} has an invalid gate")
        return
    definition = gates.get(gate)
    if definition is None:
        failures.append(f"{context} references undefined Nix gate {gate}")
        return
    evidence = " ".join(
        [
            *definition.get("packages", []),
            *definition.get("features", []),
            str(definition.get("target", "")),
            *definition.get("extra_args", []),
            "--no-default-features" if definition.get("no_default_features") else "",
            "--all-features" if definition.get("all_features") else "",
        ]
    )
    if evidence_token and evidence_token not in evidence:
        failures.append(
            f"{context} gate {gate} does not contain evidence token {evidence_token!r} in structured execution data"
        )


def referenced_policy_gates(policy: dict[str, Any]) -> set[str]:
    gates: set[str] = set()
    for host in policy.get("hosts", []):
        if isinstance(host, dict):
            gates.update(
                gate for gate in host.get("gates", []) if isinstance(gate, str)
            )
    for target in policy.get("targets", []):
        if isinstance(target, dict) and isinstance(target.get("gate"), str):
            gates.add(target["gate"])
    for surface in policy.get("features", []):
        if not isinstance(surface, dict):
            continue
        gates.update(gate for gate in surface.get("gates", []) if isinstance(gate, str))
        if isinstance(surface.get("msrv_gate"), str):
            gates.add(surface["msrv_gate"])
    return gates


def validate_gate_operations(
    policy: dict[str, Any],
    gates: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    raw_operations = require_table(policy, "gate_operations", failures)
    operations: dict[str, str] = {}
    for gate, operation in raw_operations.items():
        if not isinstance(operation, str) or not operation:
            failures.append(f"gate_operations.{gate} must be a non-empty string")
            continue
        operations[gate] = operation

    referenced = referenced_policy_gates(policy)
    if set(operations) != referenced:
        failures.append(
            "gate_operations keys do not match policy gates: "
            f"missing={sorted(referenced - set(operations))}, "
            f"extra={sorted(set(operations) - referenced)}"
        )
    if set(gates) != referenced:
        failures.append(
            "gate manifest names do not match policy gates: "
            f"missing={sorted(referenced - set(gates))}, "
            f"extra={sorted(set(gates) - referenced)}"
        )
    for gate in sorted(referenced & set(operations)):
        definition = gates.get(gate)
        if definition is None:
            continue
        if definition.get("operation") != operations[gate]:
            failures.append(
                f"gate {gate} uses Crane operation {definition.get('operation')}, "
                f"expected {operations[gate]} in {GATE_MANIFEST_PATH}"
            )


def validate_gate_target(
    gate: Any,
    expected_target: str,
    gates: dict[str, dict[str, Any]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or gate not in gates:
        return
    target = gates[gate].get("target")
    if target != expected_target:
        failures.append(
            f"{context} gate {gate} uses Cargo target {target!r}, "
            f"expected {expected_target!r} in {GATE_MANIFEST_PATH}"
        )


def validate_gate_cargo_selection(
    gate: Any,
    declared_packages: set[str],
    expects_workspace: bool,
    no_default_features: bool,
    declared_features: set[str],
    available_features: set[str] | None,
    workspace_packages: set[str],
    gates: dict[str, dict[str, Any]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or gate not in gates:
        return
    definition = gates[gate]
    selects_workspace = definition.get("workspace") is True
    excluded_packages = set(definition.get("exclude_packages", []))
    selected_packages = (
        set(workspace_packages)
        if selects_workspace
        else set(definition.get("packages", []))
    ) - excluded_packages
    actual_no_default = definition.get("no_default_features") is True
    selected_features = set(definition.get("features", []))
    if definition.get("all_features") is True:
        selected_features = (
            set(available_features)
            if available_features is not None
            else {"<all-features>"}
        )
    if (
        selected_packages != declared_packages
        or selects_workspace != expects_workspace
        or bool(excluded_packages)
        or actual_no_default != no_default_features
        or selected_features != declared_features
    ):
        failures.append(
            f"{context} gate {gate} selects packages {sorted(selected_packages)}, "
            f"workspace={selects_workspace}, excludes={sorted(excluded_packages)}, "
            f"no_default_features={actual_no_default}, "
            f"features={sorted(selected_features)}; "
            f"expected packages {sorted(declared_packages)}, "
            f"workspace={expects_workspace}, "
            f"no_default_features={no_default_features}, features={sorted(declared_features)} in {GATE_MANIFEST_PATH}"
        )


def index_unique_policy_entries(
    entries: list[Any],
    key: str,
    identity_name: str,
    failures: list[str],
) -> dict[str, dict[str, Any]]:
    indexed: dict[str, dict[str, Any]] = {}
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        identity = entry.get(key)
        if not isinstance(identity, str):
            continue
        if identity in indexed:
            failures.append(f"policy contains duplicate {identity_name} {identity!r}")
            continue
        indexed[identity] = entry
    return indexed


def validate_msrv_builder(
    gate: Any,
    gates: dict[str, dict[str, Any]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or gate not in gates:
        return
    if gates[gate].get("toolchain") != "msrv":
        failures.append(
            f"{context} gate {gate} is not built with the MSRV toolchain in {GATE_MANIFEST_PATH}"
        )


def validate_toolchains(
    root: Path,
    policy: dict[str, Any],
    cargo: dict[str, Any],
    gates: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    toolchains = require_table(policy, "toolchains", failures)
    expected = {
        "msrv": "1.85.0",
        "etalon": "nightly-2026-03-18",
        "neoprism_revision": "8becb225132efb1d9302b2c5f6ed4d87b84e8685",
        "nixpkgs_revision": "c27cdad491a991b11ed731760aa2ef8db0cb0410",
        "rust_overlay_revision": "f17186f52e82ec5cf40920b58eac63b78692ac7c",
        "nix_version": "2.34.8",
    }
    for field, expected_value in expected.items():
        actual = require_nonempty_string(toolchains, field, "[toolchains]", failures)
        if actual and actual != expected_value:
            failures.append(
                f"toolchains.{field} must be {expected_value}, found {actual}"
            )

    cargo_msrv = cargo.get("workspace", {}).get("package", {}).get("rust-version")
    if cargo_msrv != toolchains.get("msrv"):
        failures.append(
            f"Cargo workspace rust-version {cargo_msrv!r} does not match policy MSRV {toolchains.get('msrv')!r}"
        )

    rust_nix = nix_without_comments(
        (root / "nix/rust-toolchain.nix").read_text(encoding="utf-8")
    )
    for token in [
        f'nightly."{str(toolchains.get("etalon", "")).removeprefix("nightly-")}"',
        f'stable."{toolchains.get("msrv", "")}"',
    ]:
        if token not in rust_nix:
            failures.append(f"nix/rust-toolchain.nix does not select {token}")

    msrv = re.escape(str(toolchains.get("msrv", "")))
    if not re.search(
        rf'msrvToolchain\s*=\s*pkgs\.rust-bin\.stable\."{msrv}"\.minimal\s*;',
        rust_nix,
    ):
        failures.append(
            "nix/rust-toolchain.nix does not bind msrvToolchain to the policy stable toolchain"
        )
    if not re.search(
        r"msrvCraneLib\s*=\s*\(inputs\.crane\.mkLib\s+pkgs\)"
        r"\.overrideToolchain\s+msrvToolchain\s*;",
        rust_nix,
    ):
        failures.append(
            "nix/rust-toolchain.nix does not wire msrvCraneLib to msrvToolchain"
        )

    for target, tier in REQUIRED_TARGETS.items():
        if tier == "compile-checked" and f'"{target}"' not in rust_nix:
            failures.append(f"Nix Rust toolchain is missing target component {target}")

    flake = nix_without_comments((root / "flake.nix").read_text(encoding="utf-8"))
    systems_match = re.search(r"systems\s*=\s*\[(.*?)\];", flake, re.DOTALL)
    systems = (
        set(re.findall(r'"([^"]+)"', systems_match.group(1)))
        if systems_match
        else set()
    )
    if systems != set(REQUIRED_HOSTS):
        failures.append(
            f"flake systems {sorted(systems)} do not match policy hosts {sorted(REQUIRED_HOSTS)}"
        )

    lock_path = root / "flake.lock"
    try:
        lock = json.loads(lock_path.read_text(encoding="utf-8"))
        nodes = lock["nodes"]
        root_inputs = nodes["root"]["inputs"]
        nixpkgs_rev = nodes[root_inputs["nixpkgs"]]["locked"]["rev"]
        overlay_rev = nodes[root_inputs["rust-overlay"]]["locked"]["rev"]
        if nixpkgs_rev != toolchains.get("nixpkgs_revision"):
            failures.append("root nixpkgs lock revision does not match support policy")
        if overlay_rev != toolchains.get("rust_overlay_revision"):
            failures.append(
                "root rust-overlay lock revision does not match support policy"
            )
    except (OSError, KeyError, TypeError, json.JSONDecodeError) as error:
        failures.append(f"cannot validate flake.lock root inputs: {error}")

    adr = (root / "docs/adr/0002-neoprism-toolchain-alignment.md").read_text(
        encoding="utf-8"
    )
    for field in ("neoprism_revision", "nix_version"):
        value = str(toolchains.get(field, ""))
        if value and value not in adr:
            failures.append(f"ADR 0002 does not record policy {field} {value}")

    validate_gate("rust-msrv", "", gates, "MSRV policy", failures)


def validate_hosts(
    policy: dict[str, Any],
    gates: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    hosts = policy.get("hosts")
    if not isinstance(hosts, list):
        failures.append("policy is missing [[hosts]] entries")
        return
    by_system = index_unique_policy_entries(
        hosts, "nix_system", "host system", failures
    )
    if set(by_system) != set(REQUIRED_HOSTS):
        failures.append(
            f"policy hosts {sorted(str(value) for value in by_system)} do not match required hosts {sorted(REQUIRED_HOSTS)}"
        )
    for system, target in REQUIRED_HOSTS.items():
        host = by_system.get(system, {})
        if host.get("rust_target") != target:
            failures.append(f"host {system} must map to Rust target {target}")
        if host.get("tier") != "host-tested":
            failures.append(f"host {system} must be host-tested")
        require_nonempty_string(host, "limitation", f"host {system}", failures)
        gate_names = host.get("gates")
        if not isinstance(gate_names, list) or not gate_names:
            failures.append(f"host {system} must declare gates")
            continue
        for gate in gate_names:
            validate_gate(gate, "", gates, f"host {system}", failures)


def validate_targets(
    policy: dict[str, Any],
    packages: set[str],
    manifests: dict[str, Path],
    gates: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    targets = policy.get("targets")
    if not isinstance(targets, list):
        failures.append("policy is missing [[targets]] entries")
        return
    by_triple = index_unique_policy_entries(
        targets, "triple", "target triple", failures
    )
    if set(by_triple) != set(REQUIRED_TARGETS):
        failures.append(
            f"policy targets {sorted(str(value) for value in by_triple)} do not match required targets {sorted(REQUIRED_TARGETS)}"
        )
    for triple, required_tier in REQUIRED_TARGETS.items():
        target = by_triple.get(triple, {})
        tier = target.get("tier")
        if tier not in ALLOWED_TIERS:
            failures.append(f"target {triple} has invalid tier {tier!r}")
        elif tier != required_tier:
            failures.append(f"target {triple} must be {required_tier}, found {tier}")
        require_nonempty_string(target, "surface", f"target {triple}", failures)
        require_nonempty_string(target, "limitation", f"target {triple}", failures)
        declared_packages = target.get("packages")
        if not isinstance(declared_packages, list):
            failures.append(f"target {triple} has no package list")
            continue
        unknown = set(declared_packages) - packages
        if unknown:
            failures.append(f"target {triple} names unknown packages {sorted(unknown)}")
        if tier == "compile-checked":
            if set(declared_packages) != REQUIRED_COMPILE_PACKAGES:
                failures.append(
                    f"target {triple} compile package set does not match {sorted(REQUIRED_COMPILE_PACKAGES)}"
                )
            declared_features = target.get("features")
            if not isinstance(declared_features, list) or not all(
                isinstance(item, str) for item in declared_features
            ):
                failures.append(f"target {triple} has an invalid features list")
                declared_features = []
            for qualified_feature in declared_features:
                package_name, separator, feature_name = qualified_feature.partition("/")
                if not separator or not package_name or not feature_name:
                    failures.append(
                        f"target {triple} feature {qualified_feature!r} must use package/feature syntax"
                    )
                    continue
                if package_name not in declared_packages:
                    failures.append(
                        f"target {triple} feature {qualified_feature!r} names an unselected package"
                    )
                    continue
                manifest_path = manifests.get(package_name)
                if manifest_path is None:
                    continue
                manifest = load_toml(manifest_path, failures)
                available = manifest.get("features", {})
                if not isinstance(available, dict) or feature_name not in available:
                    failures.append(
                        f"target {triple} references missing feature {qualified_feature}"
                    )
            no_default_features = target.get("no_default_features")
            if not isinstance(no_default_features, bool):
                failures.append(f"target {triple} must declare no_default_features")
                no_default_features = False
            evidence_token = require_nonempty_string(
                target, "evidence_token", f"target {triple}", failures
            )
            validate_gate(
                target.get("gate"), evidence_token, gates, f"target {triple}", failures
            )
            validate_gate_target(
                target.get("gate"), triple, gates, f"target {triple}", failures
            )
            validate_gate_cargo_selection(
                target.get("gate"),
                set(declared_packages),
                False,
                no_default_features,
                set(declared_features),
                None,
                packages,
                gates,
                f"target {triple}",
                failures,
            )
        elif declared_packages:
            failures.append(f"planned target {triple} must not claim eligible packages")
        elif "gate" in target:
            failures.append(f"planned target {triple} must not claim a required gate")


def validate_features(
    policy: dict[str, Any],
    manifests: dict[str, Path],
    gates: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    features = policy.get("features")
    if not isinstance(features, list):
        failures.append("policy is missing [[features]] entries")
        return
    by_name = index_unique_policy_entries(features, "name", "feature surface", failures)
    if set(by_name) != REQUIRED_FEATURE_SURFACES:
        failures.append(
            f"policy feature surfaces {sorted(str(value) for value in by_name)} do not match required surfaces {sorted(REQUIRED_FEATURE_SURFACES)}"
        )

    for name in REQUIRED_FEATURE_SURFACES:
        surface = by_name.get(name, {})
        package = surface.get("package")
        if package != "*" and package not in manifests:
            failures.append(f"feature surface {name} names unknown package {package!r}")
            continue
        declared = surface.get("features")
        if not isinstance(declared, list) or not all(
            isinstance(item, str) for item in declared
        ):
            failures.append(f"feature surface {name} has an invalid features list")
            continue
        if not isinstance(surface.get("no_default_features"), bool):
            failures.append(f"feature surface {name} must declare no_default_features")
        available_features: set[str] | None = None
        if package != "*":
            manifest = load_toml(manifests[package], failures)
            available = manifest.get("features", {})
            if not isinstance(available, dict):
                failures.append(f"package {package} has an invalid features table")
                available = {}
            available_features = set(available) - {"default"}
            for feature in declared:
                if feature not in available:
                    failures.append(
                        f"feature surface {name} references missing {package} feature {feature}"
                    )
        gate_names = surface.get("gates")
        token = require_nonempty_string(
            surface, "evidence_token", f"feature {name}", failures
        )
        if not isinstance(gate_names, list) or not gate_names:
            failures.append(f"feature surface {name} must declare gates")
            continue
        for gate in gate_names:
            validate_gate(gate, token, gates, f"feature {name}", failures)
            validate_gate_cargo_selection(
                gate,
                set(manifests) if package == "*" else {package},
                package == "*",
                surface.get("no_default_features"),
                set(declared),
                available_features,
                set(manifests),
                gates,
                f"feature {name}",
                failures,
            )

        msrv_gate = require_nonempty_string(
            surface, "msrv_gate", f"feature {name}", failures
        )
        validate_gate(msrv_gate, "", gates, f"feature {name} MSRV", failures)
        validate_msrv_builder(msrv_gate, gates, f"feature {name} MSRV", failures)
        validate_gate_cargo_selection(
            msrv_gate,
            set(manifests) if package == "*" else {package},
            package == "*",
            surface.get("no_default_features"),
            set(declared),
            available_features,
            set(manifests),
            gates,
            f"feature {name} MSRV",
            failures,
        )


def validate_deferred_dimensions(
    policy: dict[str, Any], packages: set[str], failures: list[str]
) -> None:
    ffi = require_table(policy, "ffi", failures)
    if ffi.get("status") != "not-supported":
        failures.append("ffi.status must remain not-supported in this bootstrap policy")
    if ffi.get("package") not in packages:
        failures.append("ffi.package must name an existing workspace placeholder")
    require_nonempty_string(ffi, "reason", "[ffi]", failures)

    for dimension in ("binary_size", "build_time"):
        table = require_table(policy, dimension, failures)
        if table.get("status") != "measurement-only":
            failures.append(f"{dimension}.status must be measurement-only")
        require_nonempty_string(table, "measurement_point", f"[{dimension}]", failures)
        require_nonempty_string(table, "reason", f"[{dimension}]", failures)


def validate(root: Path) -> list[str]:
    failures: list[str] = []
    policy = load_toml(root / POLICY_PATH, failures)
    cargo = load_toml(root / "Cargo.toml", failures)
    if policy.get("schema_version") != 1:
        failures.append("policy schema_version must be 1")
    require_nonempty_string(policy, "policy_revision", "policy", failures)

    packages, manifests = workspace_packages(root, cargo, failures)
    gates = load_gate_manifest(root, packages, manifests, failures)
    validate_gate_wiring(root, failures)
    validate_gate_operations(policy, gates, failures)
    validate_toolchains(root, policy, cargo, gates, failures)
    validate_hosts(policy, gates, failures)
    validate_targets(policy, packages, manifests, gates, failures)
    validate_features(policy, manifests, gates, failures)
    validate_deferred_dimensions(policy, packages, failures)
    return sorted(set(failures))


def main() -> int:
    root = (
        Path(sys.argv[1]).resolve()
        if len(sys.argv) > 1
        else Path(__file__).resolve().parents[1]
    )
    failures = validate(root)
    if failures:
        for failure in failures:
            print(f"sdk-support-policy: {failure}", file=sys.stderr)
        print(f"sdk-support-policy: {len(failures)} failure(s)", file=sys.stderr)
        return 1
    print("sdk-support-policy: contract passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
