#!/usr/bin/env python3
"""Validate the machine-readable SDK compatibility policy offline."""

from __future__ import annotations

import json
import re
import shlex
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path
from typing import Any


POLICY_PATH = Path("docs/architecture/sdk-support-policy.toml")
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


@dataclass(frozen=True)
class GateDefinition:
    path: Path
    body: str
    operation: str


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


def require_table(data: dict[str, Any], name: str, failures: list[str]) -> dict[str, Any]:
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


def imported_check_modules(root: Path, failures: list[str]) -> list[Path]:
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
        return []

    pending = [checks_entry]
    visited: set[Path] = set()
    while pending:
        path = pending.pop()
        if path in visited:
            continue
        if not path.is_file():
            failures.append(f"imported Nix check module does not exist: {path}")
            continue
        visited.add(path)
        text = nix_without_comments(path.read_text(encoding="utf-8"))
        for imports in re.finditer(r"imports\s*=\s*\[(.*?)\];", text, re.DOTALL):
            for relative in re.findall(r"\./([A-Za-z0-9_./-]+\.nix)", imports.group(1)):
                imported = (path.parent / relative).resolve()
                if not imported.is_relative_to(checks_root):
                    failures.append(
                        f"Nix check module {path} imports outside nix/checks: {relative}"
                    )
                    continue
                pending.append(imported)
    return sorted(visited)


def gate_sources(
    root: Path, failures: list[str]
) -> dict[str, list[GateDefinition]]:
    sources: dict[str, list[GateDefinition]] = {}
    gate_pattern = re.compile(
        r"^\s*(?:checks\.)?(rust-[A-Za-z0-9_-]+)\s*=\s*"
        r"(?:craneLib|msrvCraneLib)\.([A-Za-z0-9_-]+)\s*\{.*?^\s*\};",
        re.MULTILINE | re.DOTALL,
    )
    for path in imported_check_modules(root, failures):
        text = nix_without_comments(path.read_text(encoding="utf-8"))
        for match in gate_pattern.finditer(text):
            sources.setdefault(match.group(1), []).append(
                GateDefinition(path, match.group(0), match.group(2))
            )
    return sources


def validate_crane_command_attributes(root: Path, failures: list[str]) -> None:
    for path in sorted((root / "nix/checks").glob("*.nix")):
        text = nix_without_comments(path.read_text(encoding="utf-8"))
        if "craneLib.cargoBuild" in text and (
            "cargoBuildCommand" in text or "buildPhaseCargoCommand" in text
        ):
            failures.append(
                f"{path.relative_to(root)} passes an ignored custom command to craneLib.cargoBuild; use cargoExtraArgs or mkCargoDerivation"
            )


def validate_gate(
    gate: Any,
    evidence_token: str,
    sources: dict[str, list[GateDefinition]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or not gate:
        failures.append(f"{context} has an invalid gate")
        return
    definitions = sources.get(gate, [])
    if not definitions:
        failures.append(f"{context} references undefined Nix gate {gate}")
        return
    if len(definitions) != 1:
        paths = ", ".join(str(definition.path) for definition in definitions)
        failures.append(
            f"{context} gate {gate} must have exactly one definition, found {len(definitions)} in {paths}"
        )
        return
    if evidence_token and not any(
        evidence_token in definition.body for definition in definitions
    ):
        paths = ", ".join(str(definition.path) for definition in definitions)
        failures.append(
            f"{context} gate {gate} does not contain evidence token {evidence_token!r} in {paths}"
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
        gates.update(
            gate for gate in surface.get("gates", []) if isinstance(gate, str)
        )
        if isinstance(surface.get("msrv_gate"), str):
            gates.add(surface["msrv_gate"])
    return gates


def validate_gate_operations(
    policy: dict[str, Any],
    sources: dict[str, list[GateDefinition]],
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
    for gate in sorted(referenced & set(operations)):
        definitions = sources.get(gate, [])
        if len(definitions) != 1:
            continue
        definition = definitions[0]
        if definition.operation != operations[gate]:
            failures.append(
                f"gate {gate} uses Crane operation {definition.operation}, "
                f"expected {operations[gate]} in {definition.path}"
            )


def cargo_argument_tokens(definition: str) -> list[str]:
    values = re.findall(
        r'\bcargo(?:[A-Z][A-Za-z0-9]*)?ExtraArgs\s*=\s*"([^"]*)"\s*;',
        definition,
    )
    try:
        return [token for value in values for token in shlex.split(value)]
    except ValueError:
        return []


def cargo_option_values(tokens: list[str], *names: str) -> set[str]:
    values: set[str] = set()
    for index, token in enumerate(tokens):
        if token in names and index + 1 < len(tokens):
            values.add(tokens[index + 1])
            continue
        for name in names:
            prefix = f"{name}="
            if token.startswith(prefix) and token != prefix:
                values.add(token.removeprefix(prefix))
    return values


def cargo_feature_selection(tokens: list[str]) -> set[str]:
    features: set[str] = set()
    index = 0
    while index < len(tokens):
        token = tokens[index]
        inline = next(
            (
                token.removeprefix(prefix)
                for prefix in ("--features=", "-F=")
                if token.startswith(prefix)
            ),
            None,
        )
        if inline is not None:
            features.update(value for value in inline.split(",") if value)
            index += 1
            continue
        if token not in {"--features", "-F"}:
            index += 1
            continue
        index += 1
        while index < len(tokens) and not tokens[index].startswith("-"):
            features.update(value for value in tokens[index].split(",") if value)
            index += 1
    return features


def cargo_package_selection(
    tokens: list[str], workspace_packages: set[str]
) -> tuple[set[str], set[str], bool]:
    explicitly_selected = cargo_option_values(tokens, "-p", "--package")
    excluded = cargo_option_values(tokens, "--exclude")
    selects_workspace = "--workspace" in tokens or "--all" in tokens
    selected = set(workspace_packages) if selects_workspace else explicitly_selected
    return selected - excluded, excluded, selects_workspace


def validate_gate_target(
    gate: Any,
    expected_target: str,
    sources: dict[str, list[GateDefinition]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or len(sources.get(gate, [])) != 1:
        return
    definition = sources[gate][0]
    targets = cargo_option_values(
        cargo_argument_tokens(definition.body), "--target"
    )
    if targets != {expected_target}:
        failures.append(
            f"{context} gate {gate} uses Cargo targets {sorted(targets)}, "
            f"expected {[expected_target]} in {definition.path}"
        )


def validate_gate_cargo_selection(
    gate: Any,
    declared_packages: set[str],
    expects_workspace: bool,
    no_default_features: bool,
    declared_features: set[str],
    available_features: set[str] | None,
    workspace_packages: set[str],
    sources: dict[str, list[GateDefinition]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or len(sources.get(gate, [])) != 1:
        return
    gate_definition = sources[gate][0]
    path = gate_definition.path
    definition = gate_definition.body
    tokens = cargo_argument_tokens(definition)
    selected_packages, excluded_packages, selects_workspace = (
        cargo_package_selection(tokens, workspace_packages)
    )
    actual_no_default = "--no-default-features" in tokens
    selected_features = cargo_feature_selection(tokens)
    for match in re.finditer(
        r"cargoBuildFeatures\s*=\s*\[(.*?)\];", definition, re.DOTALL
    ):
        selected_features.update(
            re.findall(r'"([A-Za-z0-9_+./-]+)"', match.group(1))
        )
    if "--all-features" in tokens:
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
            f"no_default_features={no_default_features}, features={sorted(declared_features)} in {path}"
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
            failures.append(
                f"policy contains duplicate {identity_name} {identity!r}"
            )
            continue
        indexed[identity] = entry
    return indexed


def validate_msrv_builder(
    gate: Any,
    sources: dict[str, list[GateDefinition]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or len(sources.get(gate, [])) != 1:
        return
    gate_definition = sources[gate][0]
    path = gate_definition.path
    definition = gate_definition.body
    if not re.search(r"=\s*msrvCraneLib\.[A-Za-z0-9_-]+\s*\{", definition):
        failures.append(
            f"{context} gate {gate} is not built with msrvCraneLib in {path}"
        )


def validate_toolchains(
    root: Path,
    policy: dict[str, Any],
    cargo: dict[str, Any],
    gate_definitions: dict[str, list[GateDefinition]],
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
    systems = set(re.findall(r'"([^"]+)"', systems_match.group(1))) if systems_match else set()
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
            failures.append("root rust-overlay lock revision does not match support policy")
    except (OSError, KeyError, TypeError, json.JSONDecodeError) as error:
        failures.append(f"cannot validate flake.lock root inputs: {error}")

    adr = (root / "docs/adr/0002-neoprism-toolchain-alignment.md").read_text(
        encoding="utf-8"
    )
    for field in ("neoprism_revision", "nix_version"):
        value = str(toolchains.get(field, ""))
        if value and value not in adr:
            failures.append(f"ADR 0002 does not record policy {field} {value}")

    validate_gate("rust-msrv", "", gate_definitions, "MSRV policy", failures)


def validate_hosts(
    policy: dict[str, Any],
    sources: dict[str, list[GateDefinition]],
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
        gates = host.get("gates")
        if not isinstance(gates, list) or not gates:
            failures.append(f"host {system} must declare gates")
            continue
        for gate in gates:
            validate_gate(gate, "", sources, f"host {system}", failures)


def validate_targets(
    policy: dict[str, Any],
    packages: set[str],
    manifests: dict[str, Path],
    sources: dict[str, list[GateDefinition]],
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
                failures.append(
                    f"target {triple} must declare no_default_features"
                )
                no_default_features = False
            evidence_token = require_nonempty_string(
                target, "evidence_token", f"target {triple}", failures
            )
            validate_gate(
                target.get("gate"), evidence_token, sources, f"target {triple}", failures
            )
            validate_gate_target(
                target.get("gate"), triple, sources, f"target {triple}", failures
            )
            validate_gate_cargo_selection(
                target.get("gate"),
                set(declared_packages),
                False,
                no_default_features,
                set(declared_features),
                None,
                packages,
                sources,
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
    sources: dict[str, list[GateDefinition]],
    failures: list[str],
) -> None:
    features = policy.get("features")
    if not isinstance(features, list):
        failures.append("policy is missing [[features]] entries")
        return
    by_name = index_unique_policy_entries(
        features, "name", "feature surface", failures
    )
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
        if not isinstance(declared, list) or not all(isinstance(item, str) for item in declared):
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
        gates = surface.get("gates")
        token = require_nonempty_string(surface, "evidence_token", f"feature {name}", failures)
        if not isinstance(gates, list) or not gates:
            failures.append(f"feature surface {name} must declare gates")
            continue
        for gate in gates:
            validate_gate(gate, token, sources, f"feature {name}", failures)
            validate_gate_cargo_selection(
                gate,
                set(manifests) if package == "*" else {package},
                package == "*",
                surface.get("no_default_features"),
                set(declared),
                available_features,
                set(manifests),
                sources,
                f"feature {name}",
                failures,
            )

        msrv_gate = require_nonempty_string(
            surface, "msrv_gate", f"feature {name}", failures
        )
        validate_gate(msrv_gate, "", sources, f"feature {name} MSRV", failures)
        validate_msrv_builder(
            msrv_gate, sources, f"feature {name} MSRV", failures
        )
        validate_gate_cargo_selection(
            msrv_gate,
            set(manifests) if package == "*" else {package},
            package == "*",
            surface.get("no_default_features"),
            set(declared),
            available_features,
            set(manifests),
            sources,
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
    sources = gate_sources(root, failures)
    validate_crane_command_attributes(root, failures)
    validate_gate_operations(policy, sources, failures)
    validate_toolchains(root, policy, cargo, sources, failures)
    validate_hosts(policy, sources, failures)
    validate_targets(policy, packages, manifests, sources, failures)
    validate_features(policy, manifests, sources, failures)
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
        print(
            f"sdk-support-policy: {len(failures)} failure(s)", file=sys.stderr
        )
        return 1
    print("sdk-support-policy: contract passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
