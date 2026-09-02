#!/usr/bin/env python3
"""Validate the machine-readable SDK compatibility policy offline."""

from __future__ import annotations

import json
import re
import sys
import tomllib
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


def imported_check_modules(root: Path, failures: list[str]) -> list[Path]:
    checks_root = (root / "nix/checks").resolve()
    pending = [checks_root / "default.nix"]
    visited: set[Path] = set()
    while pending:
        path = pending.pop()
        if path in visited:
            continue
        if not path.is_file():
            failures.append(f"imported Nix check module does not exist: {path}")
            continue
        visited.add(path)
        text = path.read_text(encoding="utf-8")
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
) -> dict[str, list[tuple[Path, str]]]:
    sources: dict[str, list[tuple[Path, str]]] = {}
    gate_pattern = re.compile(
        r"^\s*(?:checks\.)?(rust-[A-Za-z0-9_-]+)\s*=\s*"
        r"(?:craneLib|msrvCraneLib)\.[A-Za-z0-9_-]+\s*\{.*?^\s*\};",
        re.MULTILINE | re.DOTALL,
    )
    for path in imported_check_modules(root, failures):
        text = path.read_text(encoding="utf-8")
        for match in gate_pattern.finditer(text):
            sources.setdefault(match.group(1), []).append((path, match.group(0)))
    return sources


def validate_crane_command_attributes(root: Path, failures: list[str]) -> None:
    for path in sorted((root / "nix/checks").glob("*.nix")):
        text = path.read_text(encoding="utf-8")
        if "craneLib.cargoBuild" in text and (
            "cargoBuildCommand" in text or "buildPhaseCargoCommand" in text
        ):
            failures.append(
                f"{path.relative_to(root)} passes an ignored custom command to craneLib.cargoBuild; use cargoExtraArgs or mkCargoDerivation"
            )


def validate_gate(
    gate: Any,
    evidence_token: str,
    sources: dict[str, list[tuple[Path, str]]],
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
        paths = ", ".join(str(path) for path, _ in definitions)
        failures.append(
            f"{context} gate {gate} must have exactly one definition, found {len(definitions)} in {paths}"
        )
        return
    if evidence_token and not any(evidence_token in text for _, text in definitions):
        paths = ", ".join(str(path) for path, _ in definitions)
        failures.append(
            f"{context} gate {gate} does not contain evidence token {evidence_token!r} in {paths}"
        )


def cargo_package_selection(definition: str) -> set[str]:
    return set(
        re.findall(
            r'(?:^|[\s"])(?:-p|--package)(?:=|\s+)([A-Za-z0-9_-]+)',
            definition,
        )
    )


def validate_gate_cargo_selection(
    gate: Any,
    declared_packages: set[str],
    no_default_features: bool,
    declared_features: set[str],
    available_features: set[str] | None,
    sources: dict[str, list[tuple[Path, str]]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or len(sources.get(gate, [])) != 1:
        return
    path, definition = sources[gate][0]
    selected_packages = cargo_package_selection(definition)
    actual_no_default = bool(
        re.search(r'(?:^|[\s"])--no-default-features(?:\s|"|$)', definition)
    )
    selected_features: set[str] = set()
    for match in re.finditer(
        r'(?:^|[\s"])--features(?:=|\s+)([A-Za-z0-9_+./,-]+)', definition
    ):
        selected_features.update(
            value for value in match.group(1).split(",") if value
        )
    for match in re.finditer(
        r"cargoBuildFeatures\s*=\s*\[(.*?)\];", definition, re.DOTALL
    ):
        selected_features.update(
            re.findall(r'"([A-Za-z0-9_+./-]+)"', match.group(1))
        )
    if re.search(r'(?:^|[\s"])--all-features(?:\s|"|$)', definition):
        selected_features = (
            set(available_features)
            if available_features is not None
            else {"<all-features>"}
        )
    if (
        selected_packages != declared_packages
        or actual_no_default != no_default_features
        or selected_features != declared_features
    ):
        failures.append(
            f"{context} gate {gate} selects packages {sorted(selected_packages)}, "
            f"no_default_features={actual_no_default}, features={sorted(selected_features)}; "
            f"expected packages {sorted(declared_packages)}, "
            f"no_default_features={no_default_features}, features={sorted(declared_features)} in {path}"
        )


def validate_msrv_builder(
    gate: Any,
    sources: dict[str, list[tuple[Path, str]]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or len(sources.get(gate, [])) != 1:
        return
    path, definition = sources[gate][0]
    if not re.search(r"=\s*msrvCraneLib\.[A-Za-z0-9_-]+\s*\{", definition):
        failures.append(
            f"{context} gate {gate} is not built with msrvCraneLib in {path}"
        )


def validate_toolchains(
    root: Path,
    policy: dict[str, Any],
    cargo: dict[str, Any],
    gate_definitions: dict[str, list[tuple[Path, str]]],
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

    rust_nix = (root / "nix/rust-toolchain.nix").read_text(encoding="utf-8")
    for token in [
        f'nightly."{str(toolchains.get("etalon", "")).removeprefix("nightly-")}"',
        f'stable."{toolchains.get("msrv", "")}"',
    ]:
        if token not in rust_nix:
            failures.append(f"nix/rust-toolchain.nix does not select {token}")

    for target, tier in REQUIRED_TARGETS.items():
        if tier == "compile-checked" and f'"{target}"' not in rust_nix:
            failures.append(f"Nix Rust toolchain is missing target component {target}")

    flake = (root / "flake.nix").read_text(encoding="utf-8")
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
    sources: dict[str, list[tuple[Path, str]]],
    failures: list[str],
) -> None:
    hosts = policy.get("hosts")
    if not isinstance(hosts, list):
        failures.append("policy is missing [[hosts]] entries")
        return
    by_system = {host.get("nix_system"): host for host in hosts if isinstance(host, dict)}
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
    sources: dict[str, list[tuple[Path, str]]],
    failures: list[str],
) -> None:
    targets = policy.get("targets")
    if not isinstance(targets, list):
        failures.append("policy is missing [[targets]] entries")
        return
    by_triple = {target.get("triple"): target for target in targets if isinstance(target, dict)}
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
            validate_gate_cargo_selection(
                target.get("gate"),
                set(declared_packages),
                no_default_features,
                set(declared_features),
                None,
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
    sources: dict[str, list[tuple[Path, str]]],
    failures: list[str],
) -> None:
    features = policy.get("features")
    if not isinstance(features, list):
        failures.append("policy is missing [[features]] entries")
        return
    by_name = {surface.get("name"): surface for surface in features if isinstance(surface, dict)}
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
                set() if package == "*" else {package},
                surface.get("no_default_features"),
                set(declared),
                available_features,
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
            set() if package == "*" else {package},
            surface.get("no_default_features"),
            set(declared),
            available_features,
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
