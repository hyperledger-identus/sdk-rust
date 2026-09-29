#!/usr/bin/env python3
"""Validate independent release-candidate train configuration without network."""

from __future__ import annotations

import ast
import hashlib
import re
import sys
import tomllib
from pathlib import Path
from typing import Any


INDEX = Path("docs/release/release-trains.toml")
DID_DESCRIPTOR = Path("docs/release/did-candidate.toml")
DID_STAGED_LOCK = Path("docs/release/did-candidate.lock")
CRYPTO_DESCRIPTOR = Path("docs/release/crypto-candidate.toml")
BUILDER = Path("scripts/prepare-did-candidate.py")
ADR = Path("docs/adr/0153-use-primary-package-tags-for-independent-release-trains.md")
MATRIX_ADR = Path("docs/adr/0155-qualify-staged-did-candidate-matrix.md")
MATRIX_PRIMARY_APP = Path("nix/apps/did-candidate-matrix-primary.nix")
MATRIX_MSRV_APP = Path("nix/apps/did-candidate-matrix-msrv.nix")
DID_CANDIDATE_APP = Path("nix/apps/did-candidate.nix")
APPS = Path("nix/apps/default.nix")
SLOW_WORKFLOW = Path(".github/workflows/nix-checks.yml")
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
    "preparation_rust_version", "baseline_revision", "staged_lock",
    "staged_lock_sha256", "repository", "homepage",
    "license", "max_archive_bytes", "max_archive_members", "max_expansion_ratio",
    "max_evidence_bytes", "publication", "release_tag", "compatibility_status",
    "tools", "matrix", "matrix_hosts", "matrix_targets", "profiles", "packages",
}
PROFILE_KEYS = {"package", "name", "default_features", "features"}
PACKAGE_KEYS = {
    "name", "path", "description", "documentation", "readme", "keywords",
    "categories", "api_baseline", "candidate_dependencies", "published_dependencies",
    "features",
}
TOOL_KEYS = {"cargo_public_api", "cargo_semver_checks", "cargo_cyclonedx", "cyclonedx_spec"}
MATRIX_KEYS = {
    "schema_version", "primary_rust_version", "msrv_rust_version", "max_receipt_bytes",
}
MATRIX_HOST_KEYS = {
    "name", "nix_system", "github_runner", "packages", "primary_operation",
    "msrv_operation",
}
MATRIX_TARGET_KEYS = {
    "triple", "runner_host", "packages", "unsupported_packages", "operation", "tier",
    "limitation",
}
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
EXPECTED_STAGED_IDENTUS = {
    "identus-core": (VERSION, "registry+https://github.com/rust-lang/crates.io-index"),
    "identus-derive": (VERSION, "registry+https://github.com/rust-lang/crates.io-index"),
    "identus-did": (VERSION, None),
    "identus-did-resolver-http": (VERSION, None),
}
EXPECTED_STAGED_PATH_PACKAGES = {"identus-did", "identus-did-resolver-http"}
EXPECTED_MATRIX = {
    "schema_version": 1,
    "primary_rust_version": "1.98.1",
    "msrv_rust_version": "1.89.0",
    "max_receipt_bytes": 262144,
}
EXPECTED_MATRIX_HOSTS = [
    {
        "name": "linux", "nix_system": "x86_64-linux", "github_runner": "ubuntu-latest",
        "packages": list(DID_PACKAGES), "primary_operation": "test", "msrv_operation": "check",
    },
    {
        "name": "macos", "nix_system": "aarch64-darwin", "github_runner": "macos-latest",
        "packages": list(DID_PACKAGES), "primary_operation": "test", "msrv_operation": "check",
    },
]
EXPECTED_MATRIX_TARGETS = [
    {
        "triple": "wasm32-unknown-unknown", "runner_host": "linux",
        "packages": ["identus-did"], "unsupported_packages": ["identus-did-resolver-http"],
        "operation": "check", "tier": "compile-checked",
        "limitation": "No browser or runtime claim; browser runtime evidence is independent.",
    },
    {
        "triple": "aarch64-linux-android", "runner_host": "linux",
        "packages": ["identus-did"], "unsupported_packages": ["identus-did-resolver-http"],
        "operation": "check", "tier": "compile-checked",
        "limitation": "No Android device or emulator claim; runtime evidence is independent.",
    },
    {
        "triple": "aarch64-apple-ios", "runner_host": "macos",
        "packages": ["identus-did"], "unsupported_packages": ["identus-did-resolver-http"],
        "operation": "check", "tier": "compile-checked",
        "limitation": "No iOS simulator, device, or binding claim; runtime evidence is independent.",
    },
]


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


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def named_calls(node: ast.AST, name: str) -> list[ast.Call]:
    return [
        candidate
        for candidate in ast.walk(node)
        if isinstance(candidate, ast.Call)
        and isinstance(candidate.func, ast.Name)
        and candidate.func.id == name
    ]


def literal_command(call: ast.Call) -> list[str] | None:
    if not call.args or not isinstance(call.args[0], (ast.List, ast.Tuple)):
        return None
    values: list[str] = []
    for element in call.args[0].elts:
        if not isinstance(element, ast.Constant) or not isinstance(element.value, str):
            break
        values.append(element.value)
    return values


def literal_string_argument(call: ast.Call, position: int) -> str | None:
    if len(call.args) <= position:
        return None
    value = call.args[position]
    return value.value if isinstance(value, ast.Constant) and isinstance(value.value, str) else None


def command_tokens(call: ast.Call) -> list[str | None]:
    if not call.args or not isinstance(call.args[0], (ast.List, ast.Tuple)):
        return []
    return [
        element.value
        if isinstance(element, ast.Constant) and isinstance(element.value, str)
        else None
        for element in call.args[0].elts
    ]


def cargo_option_precedes_separator(call: ast.Call, option: str) -> bool:
    tokens = command_tokens(call)
    separator = tokens.index("--") if "--" in tokens else len(tokens)
    return option in tokens[2:separator]


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
        "staged_lock": DID_STAGED_LOCK.as_posix(),
        "staged_lock_sha256": "1f1d4206e2ced5bd74675d654876684536dd8f82bf79cd6de4c2fa67db904447",
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
    staged_lock = root / DID_STAGED_LOCK
    if staged_lock.is_symlink() or not staged_lock.is_file():
        errors.append("DID staged lock must be a regular repository file")
    elif staged_lock.stat().st_size == 0 or staged_lock.stat().st_size > 262144:
        errors.append("DID staged lock has an invalid byte size")
    else:
        if sha256(staged_lock) != descriptor.get("staged_lock_sha256"):
            errors.append("DID staged lock digest differs")
        lock = load_toml(staged_lock, errors)
        if set(lock) != {"version", "package"} or lock.get("version") != 4:
            errors.append("DID staged lock shape or format differs")
        lock_packages = lock.get("package")
        if not isinstance(lock_packages, list):
            errors.append("DID staged lock packages must be an array")
            lock_packages = []
        observed_identus: dict[str, tuple[Any, Any]] = {}
        identities: set[tuple[Any, Any, Any]] = set()
        for position, package in enumerate(lock_packages):
            if not isinstance(package, dict):
                errors.append(f"DID staged lock package {position} is malformed")
                continue
            name = package.get("name")
            version = package.get("version")
            source = package.get("source")
            identity = (name, version, source)
            if identity in identities:
                errors.append("DID staged lock contains a duplicate package identity")
            identities.add(identity)
            if source is not None:
                if source != "registry+https://github.com/rust-lang/crates.io-index":
                    errors.append("DID staged lock contains a non-crates.io source")
                if not re.fullmatch(r"[0-9a-f]{64}", str(package.get("checksum", ""))):
                    errors.append("DID staged lock registry checksum is invalid")
            elif name not in EXPECTED_STAGED_PATH_PACKAGES:
                errors.append("DID staged lock registry package is missing source provenance")
            if isinstance(name, str) and name.startswith("identus-"):
                if name in observed_identus:
                    errors.append("DID staged lock contains a duplicate candidate package name")
                else:
                    observed_identus[name] = (version, source)
        if observed_identus != EXPECTED_STAGED_IDENTUS:
            errors.append("DID staged lock candidate package identity differs")
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

    matrix = descriptor.get("matrix")
    if not isinstance(matrix, dict):
        errors.append("DID descriptor matrix must be a table")
    else:
        require_keys(matrix, MATRIX_KEYS, "DID descriptor matrix", errors)
        if matrix != EXPECTED_MATRIX:
            errors.append("DID descriptor compiler matrix differs")

    matrix_hosts = descriptor.get("matrix_hosts")
    if not isinstance(matrix_hosts, list):
        errors.append("DID descriptor matrix_hosts must be an array")
        matrix_hosts = []
    for position, host in enumerate(matrix_hosts):
        if not isinstance(host, dict):
            errors.append(f"DID matrix host {position} must be a table")
        else:
            require_keys(host, MATRIX_HOST_KEYS, f"DID matrix host {position}", errors)
    if matrix_hosts != EXPECTED_MATRIX_HOSTS:
        errors.append("DID descriptor host matrix differs")

    matrix_targets = descriptor.get("matrix_targets")
    if not isinstance(matrix_targets, list):
        errors.append("DID descriptor matrix_targets must be an array")
        matrix_targets = []
    for position, target in enumerate(matrix_targets):
        if not isinstance(target, dict):
            errors.append(f"DID matrix target {position} must be a table")
        else:
            require_keys(target, MATRIX_TARGET_KEYS, f"DID matrix target {position}", errors)
            if target.get("packages") != ["identus-did"]:
                errors.append(f"DID matrix target {position} overclaims portable packages")
            if target.get("unsupported_packages") != ["identus-did-resolver-http"]:
                errors.append(f"DID matrix target {position} HTTP support boundary differs")
    if matrix_targets != EXPECTED_MATRIX_TARGETS:
        errors.append("DID descriptor portable target matrix differs")

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
        "--matrix-toolchain", "--aggregate-matrix", "build_matrix_lane",
        "aggregate_matrix", "matrix receipt exceeds byte limit", "install_staged_lock",
        "staged_lock_sha256", "--refresh-staged-lock", "refresh_staged_lock",
        "generate_lockfile", "matrix lane lock differs from descriptor identity",
        '"sha256": passes[0][1]',
        'command[1:3] == ["generate-lockfile", "--manifest-path"]',
        'Path(command[3]) != cwd / "Cargo.toml"',
        "'metadata', '--locked', *sys.argv[2:]",
        'cyclonedx_env = env | {"CARGO": str(locked_cargo)}',
        "release evidence staged lock digest differs",
        "Rustdoc changed the staged lock",
        "public API extraction changed the staged lock",
        "CycloneDX changed the staged lock",
        "archive installed staged lock digest differs",
        "matrix installed staged lock digest differs",
        "output.mkdir(mode=0o700)", "reserved output was modified during refresh",
        "shutil.rmtree(output)",
    )
    for phrase in required_builder:
        if phrase not in builder:
            errors.append(f"DID candidate builder is missing contract: {phrase}")
    try:
        builder_tree = ast.parse(builder)
    except SyntaxError:
        errors.append("DID candidate builder is not valid Python")
        builder_tree = ast.Module(body=[], type_ignores=[])
    functions = {
        node.name: node
        for node in ast.walk(builder_tree)
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))
    }
    install_calls = named_calls(builder_tree, "install_staged_lock")
    if len(install_calls) != 2 or any(
        len(named_calls(functions.get(owner, ast.Module(body=[], type_ignores=[])), "install_staged_lock")) != 1
        for owner in ("assemble", "build_matrix_lane")
    ):
        errors.append("DID candidate builder must install the staged lock in archive and matrix paths")
    generation_literals = [
        node for node in ast.walk(builder_tree)
        if isinstance(node, ast.Constant) and node.value == "generate-lockfile"
    ]
    generation_function = functions.get(
        "generate_lockfile", ast.Module(body=[], type_ignores=[])
    )
    privileged_runs = [
        call for call in named_calls(builder_tree, "run")
        if any(keyword.arg == "lock_generation_purpose" for keyword in call.keywords)
    ]
    generation_runs = named_calls(generation_function, "run")
    direct_generation_runs = [
        call
        for runner in ("run", "run_stdout")
        for call in named_calls(builder_tree, runner)
        if (literal_command(call) or [])[:2] == ["cargo", "generate-lockfile"]
    ]
    generation_calls = named_calls(builder_tree, "generate_lockfile")
    allowed_callers = {
        "verify_closure": "extracted-closure",
        "refresh_staged_lock": "staged-refresh",
    }
    generation_call_contract = all(
        len(named_calls(functions.get(owner, ast.Module(body=[], type_ignores=[])), "generate_lockfile")) == 1
        and literal_string_argument(
            named_calls(functions[owner], "generate_lockfile")[0], 2
        ) == purpose
        for owner, purpose in allowed_callers.items()
    )
    generation_run_contract = (
        len(generation_runs) == 1
        and (literal_command(generation_runs[0]) or [])[:3]
        == ["cargo", "generate-lockfile", "--manifest-path"]
        and any(
            keyword.arg == "lock_generation_purpose"
            and isinstance(keyword.value, ast.Name)
            and keyword.value.id == "purpose"
            for keyword in generation_runs[0].keywords
        )
    )
    if (
        len(generation_literals) != 2
        or len(generation_calls) != 2
        or not generation_call_contract
        or not generation_run_contract
        or privileged_runs != generation_runs
        or direct_generation_runs != generation_runs
    ):
        errors.append(
            "DID candidate lock generation must stay inside closed closure/refresh capabilities"
        )
    staged_operations = {
        "rustdoc": [
            call for call in named_calls(builder_tree, "run")
            if (literal_command(call) or [])[:2] == ["cargo", "rustdoc"]
        ],
        "cyclonedx": [
            call for call in named_calls(builder_tree, "run")
            if (literal_command(call) or [])[:2] == ["cargo-cyclonedx", "cyclonedx"]
        ],
    }
    rustdoc_calls = staged_operations["rustdoc"]
    cyclonedx_calls = staged_operations["cyclonedx"]
    cyclonedx_env_bound = (
        len(cyclonedx_calls) == 1
        and any(
            keyword.arg == "env"
            and isinstance(keyword.value, ast.Name)
            and keyword.value.id == "cyclonedx_env"
            for keyword in cyclonedx_calls[0].keywords
        )
    )
    if (
        len(rustdoc_calls) != 1
        or not cargo_option_precedes_separator(rustdoc_calls[0], "--locked")
        or len(cyclonedx_calls) != 1
        or not cyclonedx_env_bound
    ):
        errors.append("DID staged Cargo evidence operations must use --locked")
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

    matrix_adr = read(root / MATRIX_ADR, errors)
    for phrase in (
        "staged sources", "Rust 1.98.1", "MSRV 1.89.0", "compile-only claims",
        "explicitly unsupported", "exact four receipts", "not authorize dispatch or rerun",
    ):
        if phrase not in matrix_adr:
            errors.append(f"DID matrix ADR is missing decision evidence: {phrase}")

    primary_app = read(root / MATRIX_PRIMARY_APP, errors)
    msrv_app = read(root / MATRIX_MSRV_APP, errors)
    candidate_app = read(root / DID_CANDIDATE_APP, errors)
    apps = read(root / APPS, errors)
    for source, label, phrases in (
        (primary_app, "primary matrix app", ("toolchain", "--matrix-toolchain primary")),
        (msrv_app, "MSRV matrix app", ("msrvToolchain", "--matrix-toolchain msrv")),
        (
            candidate_app,
            "candidate app",
            (
                "cargoCyclonedx",
                "cargoPublicApi",
                "cargoSemverChecks",
                "toolchain",
                "prepare-did-candidate.py",
            ),
        ),
        (
            apps, "matrix app registration",
            ("did-candidate =", "did-candidate-matrix-primary", "did-candidate-matrix-msrv"),
        ),
    ):
        for phrase in phrases:
            if phrase not in source:
                errors.append(f"DID {label} is missing contract: {phrase}")

    workflow = read(root / SLOW_WORKFLOW, errors)
    required_workflow = (
        "did-candidate-matrix:", "did-candidate-matrix-primary",
        "did-candidate-matrix-msrv", "did-matrix-${{ matrix.host }}-${{ github.sha }}-",
        '--output "$RUNNER_TEMP/did-matrix/primary"',
        "Verify primary qualification preserved a clean checkout",
        "git status --porcelain --untracked-files=all",
        "primary DID candidate qualification dirtied the checkout",
        "printf '%s\\n' \"$dirty_paths\"",
        "exit 1",
        '--output "$RUNNER_TEMP/did-matrix/msrv"',
        "path: ${{ runner.temp }}/did-matrix",
        "actions/download-artifact@3e5f45b2cfb9172054b4087a40e8e0b5a5461e7c",
        "--aggregate-matrix", "DID_MATRIX_RESULT", '"did-candidate-matrix"',
        "artifacts/slow-run",
    )
    for phrase in required_workflow:
        if phrase not in workflow:
            errors.append(f"slow workflow is missing DID matrix contract: {phrase}")
    if "--output artifacts/did-matrix/" in workflow:
        errors.append("slow workflow writes DID matrix evidence inside the checkout")
    ordered_boundary = (
        '--output "$RUNNER_TEMP/did-matrix/primary"',
        "Verify primary qualification preserved a clean checkout",
        '--output "$RUNNER_TEMP/did-matrix/msrv"',
    )
    boundary_positions = tuple(workflow.find(phrase) for phrase in ordered_boundary)
    if all(position >= 0 for position in boundary_positions) and boundary_positions != tuple(
        sorted(boundary_positions)
    ):
        errors.append("slow workflow must verify checkout cleanliness between DID lanes")
    if "pull_request:" in workflow:
        errors.append("slow workflow must not gain a pull_request trigger")
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
