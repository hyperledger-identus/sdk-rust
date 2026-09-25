#!/usr/bin/env python3
"""Build and verify the credential-free two-crate DID release candidate."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import time
import tomllib
from pathlib import Path, PurePosixPath
from typing import Any


INDEX = Path("docs/release/release-trains.toml")
DESCRIPTOR = Path("docs/release/did-candidate.toml")
PACKAGE_ORDER = ("identus-did", "identus-did-resolver-http")
PACKAGE_PATHS = {
    "identus-did": Path("crates/did"),
    "identus-did-resolver-http": Path("crates/did-resolver-http"),
}
EXTERNAL_DEPENDENCIES = (
    "axum", "base64", "bs58", "fluent-uri", "headers-accept", "mediatype",
    "serde", "serde_json", "tokio", "tower", "uriparse", "utoipa",
)
ALLOWED_SUFFIXES = {".rs", ".md"}
ALLOWED_CARGO_OPERATIONS = frozenset({
    "check", "cyclonedx", "generate-lockfile", "package", "public-api", "rustdoc",
    "test",
})
SHA = re.compile(r"^[0-9a-f]{40}$")


class CandidateError(RuntimeError):
    pass


def require_local_command(command: list[str]) -> None:
    if not command:
        raise CandidateError("empty candidate command")
    executable = Path(command[0]).name
    if executable == "git" and command[1:] in (["rev-parse", "HEAD"], ["status", "--porcelain"]):
        return
    if executable in {"cargo", "rustc"} and command[1:] == ["--version"]:
        return
    if executable == "rustc" and command[1:] == ["-vV"]:
        return
    if executable == "cargo" and command[1:] == ["semver-checks", "--version"]:
        return
    if executable == "cargo" and len(command) >= 2 and command[1] in ALLOWED_CARGO_OPERATIONS:
        return
    if (
        Path(command[0]).resolve() == Path(sys.executable).resolve()
        and len(command) == 3
        and Path(command[1]).name == "check-release-candidates.py"
    ):
        return
    raise CandidateError(f"candidate command is not allowlisted: {executable}")


def run(command: list[str], *, cwd: Path, env: dict[str, str]) -> str:
    require_local_command(command)
    result = subprocess.run(
        command, cwd=cwd, env=env, check=False, text=True,
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
    )
    if result.returncode != 0:
        rendered = " ".join(command)
        raise CandidateError(f"command failed ({result.returncode}): {rendered}\n{result.stdout}")
    return result.stdout


def run_stdout(command: list[str], *, cwd: Path, env: dict[str, str]) -> str:
    require_local_command(command)
    result = subprocess.run(
        command, cwd=cwd, env=env, check=False, text=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
    )
    if result.returncode != 0:
        rendered = " ".join(command)
        raise CandidateError(
            f"command failed ({result.returncode}): {rendered}\n{result.stdout}{result.stderr}"
        )
    return result.stdout


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as source:
        value = tomllib.load(source)
    if not isinstance(value, dict):
        raise CandidateError(f"TOML root is not a table: {path}")
    return value


def require_tool_version(tool: str, output: str, expected: str) -> str:
    normalized = output.strip()
    match = re.fullmatch(rf"{re.escape(tool)} ([0-9]+\.[0-9]+\.[0-9]+)(?: .*)?", normalized)
    if match is None:
        raise CandidateError(f"cannot parse {tool} version: {normalized!r}")
    if match.group(1) != expected:
        raise CandidateError(
            f"candidate preparation requires {tool} {expected}; found {match.group(1)}"
        )
    return normalized


def require_subcommand_version(command: list[str], expected: str, root: Path, env: dict[str, str]) -> str:
    output = run(command, cwd=root, env=env).strip()
    if not output or output.split()[-1] != expected:
        raise CandidateError(f"tool version differs; expected {expected}: {output}")
    return output


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def source_revision(root: Path, requested: str | None, allow_dirty: bool) -> tuple[str, bool]:
    env = os.environ.copy()
    head = run(["git", "rev-parse", "HEAD"], cwd=root, env=env).strip()
    revision = requested or head
    if not SHA.fullmatch(revision) or revision != head:
        raise CandidateError("source revision must be repository HEAD as a full lowercase Git SHA")
    dirty = bool(run(["git", "status", "--porcelain"], cwd=root, env=env))
    if dirty and not allow_dirty:
        raise CandidateError("candidate evidence requires a clean Git worktree")
    return revision, dirty


def require_vcs_independent_build_scratch(root: Path, scratch: Path) -> None:
    repository = root.resolve()
    candidate = scratch.resolve()
    if candidate == repository or repository in candidate.parents:
        raise CandidateError("candidate build scratch must be outside the source repository")
    for ancestor in (candidate, *candidate.parents):
        try:
            (ancestor / ".git").lstat()
        except FileNotFoundError:
            continue
        except OSError as error:
            raise CandidateError("cannot validate candidate scratch VCS boundary") from error
        raise CandidateError("candidate build scratch must not be inside a Git worktree")


def dependency_line(root_manifest: str, name: str) -> str:
    match = re.search(rf"(?m)^{re.escape(name)}\s*=.*$", root_manifest)
    if match is None:
        raise CandidateError(f"canonical workspace dependency is missing: {name}")
    return match.group(0)


def render_root_manifest(root: Path, descriptor: dict[str, Any]) -> str:
    canonical = (root / "Cargo.toml").read_text(encoding="utf-8")
    version = descriptor["version"]
    lines = [
        "[workspace]", 'resolver = "3"',
        'members = [ "crates/did", "crates/did-resolver-http" ]', "",
        "[workspace.dependencies]",
        f'identus-core              = "={version}"',
        f'identus-derive            = "={version}"',
        f'identus-did               = {{ path = "crates/did", version = "={version}" }}',
    ]
    lines.extend(dependency_line(canonical, name) for name in EXTERNAL_DEPENDENCIES)
    lines.extend([
        "", "[workspace.package]", 'edition = "2024"',
        f'rust-version = "{descriptor["rust_version"]}"',
        f'version = "{version}"', f'license = "{descriptor["license"]}"',
        "publish = true", f'repository = "{descriptor["repository"]}"',
        f'homepage = "{descriptor["homepage"]}"', "",
        "[workspace.lints.rust]", 'unsafe_code = "forbid"', 'warnings = "deny"', "",
        "[workspace.lints.clippy]", 'all = { level = "deny", priority = -1 }', "",
    ])
    return "\n".join(lines)


def toml_array(values: list[str]) -> str:
    return "[ " + ", ".join(json.dumps(value) for value in values) + " ]"


def render_package_manifest(source: str, package: dict[str, Any]) -> str:
    try:
        manifest = tomllib.loads(source)
    except tomllib.TOMLDecodeError as error:
        raise CandidateError(f"cannot parse {package['path']}/Cargo.toml") from error
    metadata = manifest.get("package", {})
    if metadata.get("name") != package["name"]:
        raise CandidateError(f"canonical package identity differs: {package['name']}")
    if metadata.get("version") != {"workspace": True}:
        raise CandidateError(f"canonical package version was activated: {package['name']}")
    if metadata.get("publish") != {"workspace": True}:
        raise CandidateError(f"canonical package publication was activated: {package['name']}")
    insertion = "\n".join([
        f'description            = {json.dumps(package["description"])}',
        'repository.workspace   = true',
        'homepage.workspace     = true',
        f'documentation          = {json.dumps(package["documentation"])}',
        f'readme                 = {json.dumps(package["readme"])}',
        f'keywords               = {toml_array(package["keywords"])}',
        f'categories             = {toml_array(package["categories"])}',
    ])
    marker = "publish.workspace      = true"
    if source.count(marker) != 1:
        raise CandidateError(f"canonical publish marker differs: {package['name']}")
    return source.replace(marker, f"{marker}\n{insertion}", 1)


def copy_package(root: Path, stage: Path, package: dict[str, Any]) -> None:
    relative = Path(package["path"])
    source_root = root / relative
    target_root = stage / relative
    for source in sorted(source_root.rglob("*")):
        if source.is_symlink():
            raise CandidateError(f"candidate package contains symlink: {source.relative_to(root)}")
        if source.is_dir():
            continue
        if source.name != "Cargo.toml" and source.suffix not in ALLOWED_SUFFIXES:
            raise CandidateError(f"candidate package contains unapproved file: {source.relative_to(root)}")
        destination = target_root / source.relative_to(source_root)
        destination.parent.mkdir(parents=True, exist_ok=True)
        if source.name == "Cargo.toml":
            destination.write_text(
                render_package_manifest(source.read_text(encoding="utf-8"), package),
                encoding="utf-8",
            )
        else:
            shutil.copyfile(source, destination)
    shutil.copyfile(root / "LICENSE", target_root / "LICENSE")


def create_stage(root: Path, parent: Path, descriptor: dict[str, Any]) -> Path:
    stage = parent / "workspace"
    stage.mkdir(parents=True)
    (stage / "Cargo.toml").write_text(render_root_manifest(root, descriptor), encoding="utf-8")
    packages = descriptor.get("packages", [])
    if [package.get("name") for package in packages] != list(PACKAGE_ORDER):
        raise CandidateError("descriptor package order differs")
    for package in packages:
        copy_package(root, stage, package)
    return stage


def assemble(stage: Path, descriptor: dict[str, Any], env: dict[str, str]) -> dict[str, Path]:
    run(["cargo", "generate-lockfile", "--manifest-path", str(stage / "Cargo.toml")], cwd=stage, env=env)
    run([
        "cargo", "package", "--workspace", "--locked", "--no-verify", "--allow-dirty",
        "--manifest-path", str(stage / "Cargo.toml"),
        "--target-dir", str(stage / "target"),
    ], cwd=stage, env=env)
    archives: dict[str, Path] = {}
    for name in PACKAGE_ORDER:
        archive = stage / "target/package" / f"{name}-{descriptor['version']}.crate"
        if not archive.is_file():
            raise CandidateError(f"Cargo did not create expected archive: {archive.name}")
        archives[name] = archive
    return archives


def safe_members(archive: Path, prefix: str, descriptor: dict[str, Any]) -> list[tarfile.TarInfo]:
    maximum = descriptor["max_archive_bytes"]
    if archive.stat().st_size > maximum:
        raise CandidateError(f"archive exceeds byte limit: {archive.name}")
    with tarfile.open(archive, "r:gz") as package:
        members = package.getmembers()
    if not members or len(members) > descriptor["max_archive_members"]:
        raise CandidateError(f"archive has invalid member count: {archive.name}")
    names: set[str] = set()
    expanded_bytes = 0
    for member in members:
        path = PurePosixPath(member.name)
        if path.is_absolute() or ".." in path.parts or not path.parts or path.parts[0] != prefix:
            raise CandidateError(f"unsafe archive path: {member.name}")
        if not (member.isfile() or member.isdir()):
            raise CandidateError(f"archive contains link or special file: {member.name}")
        if member.name in names:
            raise CandidateError(f"archive contains duplicate path: {member.name}")
        names.add(member.name)
        expanded_bytes += member.size
    if expanded_bytes > maximum * descriptor["max_expansion_ratio"]:
        raise CandidateError(f"archive exceeds expanded-byte limit: {archive.name}")
    return members


def archive_files(archive: Path, descriptor: dict[str, Any]) -> list[str]:
    prefix = archive.name.removesuffix(".crate")
    return sorted(
        member.name for member in safe_members(archive, prefix, descriptor) if member.isfile()
    )


def extract_safe(archive: Path, destination: Path, descriptor: dict[str, Any]) -> Path:
    prefix = archive.name.removesuffix(".crate")
    members = safe_members(archive, prefix, descriptor)
    with tarfile.open(archive, "r:gz") as package:
        for member in members:
            target = destination / member.name
            if member.isdir():
                target.mkdir(parents=True, exist_ok=True)
                continue
            target.parent.mkdir(parents=True, exist_ok=True)
            source = package.extractfile(member)
            if source is None:
                raise CandidateError(f"cannot read archive member: {member.name}")
            target.write_bytes(source.read())
    return destination / prefix


def dependency_tables(manifest: dict[str, Any]) -> list[dict[str, Any]]:
    tables: list[dict[str, Any]] = []
    for key in ("dependencies", "dev-dependencies", "build-dependencies"):
        value = manifest.get(key, {})
        if isinstance(value, dict):
            tables.append(value)
    target = manifest.get("target", {})
    if isinstance(target, dict):
        for value in target.values():
            if isinstance(value, dict):
                tables.extend(dependency_tables(value))
    return tables


def inspect_archive(archive: Path, package: dict[str, Any], descriptor: dict[str, Any]) -> None:
    prefix = f"{package['name']}-{descriptor['version']}"
    members = safe_members(archive, prefix, descriptor)
    names = {member.name for member in members}
    required = {
        f"{prefix}/Cargo.toml", f"{prefix}/Cargo.toml.orig", f"{prefix}/README.md",
        f"{prefix}/LICENSE", f"{prefix}/src/lib.rs",
    }
    if missing := sorted(required - names):
        raise CandidateError(f"{archive.name} misses required files: {', '.join(missing)}")
    with tarfile.open(archive, "r:gz") as contents:
        source = contents.extractfile(f"{prefix}/Cargo.toml")
        if source is None:
            raise CandidateError(f"cannot inspect normalized manifest: {archive.name}")
        manifest = tomllib.loads(source.read().decode("utf-8"))
    metadata = manifest.get("package", {})
    expected_metadata = {
        "name": package["name"], "version": descriptor["version"],
        "description": package["description"], "repository": descriptor["repository"],
        "homepage": descriptor["homepage"], "documentation": package["documentation"],
        "readme": package["readme"], "keywords": package["keywords"],
        "categories": package["categories"], "license": descriptor["license"],
        "rust-version": descriptor["rust_version"],
    }
    for field, expected in expected_metadata.items():
        if metadata.get(field) != expected:
            raise CandidateError(f"{package['name']}: normalized metadata differs: {field}")
    expected_internal = set(package["candidate_dependencies"]) | set(package["published_dependencies"])
    observed_internal: set[str] = set()
    for table in dependency_tables(manifest):
        for dependency, value in table.items():
            if isinstance(value, dict) and ("git" in value or "path" in value):
                raise CandidateError(
                    f"{package['name']}: normalized dependency {dependency} retains git/path"
                )
            if dependency.startswith("identus-"):
                observed_internal.add(dependency)
                version = value if isinstance(value, str) else value.get("version")
                if version != f"={descriptor['version']}":
                    raise CandidateError(
                        f"{package['name']}: {dependency} is not exact candidate version"
                    )
    if observed_internal != expected_internal:
        raise CandidateError(f"{package['name']}: normalized internal dependencies differ")


def profile_arguments(profile: dict[str, Any]) -> list[str]:
    arguments: list[str] = []
    if not profile["default_features"]:
        arguments.append("--no-default-features")
    if profile["features"]:
        arguments.extend(["--features", ",".join(profile["features"])])
    return arguments


def verify_closure(
    archives: dict[str, Path], parent: Path, descriptor: dict[str, Any], env: dict[str, str]
) -> list[list[str]]:
    verify = parent / "verify"
    packages = {
        name: extract_safe(archive, verify / "packages", descriptor)
        for name, archive in archives.items()
    }
    members = [packages[name].relative_to(verify).as_posix() for name in PACKAGE_ORDER]
    manifest = [
        "[workspace]", 'resolver = "3"', f"members = {toml_array(members)}", "",
        "[patch.crates-io]",
        f'identus-did = {{ path = {json.dumps(packages["identus-did"].relative_to(verify).as_posix())} }}',
        "",
    ]
    (verify / "Cargo.toml").write_text("\n".join(manifest), encoding="utf-8")
    run(["cargo", "generate-lockfile"], cwd=verify, env=env)
    commands: list[list[str]] = []
    for profile in descriptor["profiles"]:
        for operation in ("check", "test"):
            command = ["cargo", operation, "--locked", "-p", profile["package"]]
            command.extend(profile_arguments(profile))
            commands.append(command)
            run(command, cwd=verify, env=env)
    return commands


def require_bounded_evidence(path: Path, descriptor: dict[str, Any]) -> None:
    if path.is_symlink() or not path.is_file():
        raise CandidateError(f"evidence is not a regular file: {path.name}")
    size = path.stat().st_size
    if size == 0 or size > descriptor["max_evidence_bytes"]:
        raise CandidateError(f"evidence has invalid byte size: {path.name}")


def component_license_ids(component: dict[str, Any]) -> set[str]:
    identifiers: set[str] = set()
    licenses = component.get("licenses", [])
    if not isinstance(licenses, list):
        return identifiers
    for entry in licenses:
        if not isinstance(entry, dict):
            continue
        license_value = entry.get("license")
        if isinstance(license_value, dict) and isinstance(license_value.get("id"), str):
            identifiers.add(license_value["id"])
        if isinstance(entry.get("expression"), str):
            identifiers.add(entry["expression"])
    return identifiers


def normalize_local_sbom_references(value: Any) -> Any:
    if isinstance(value, str) and value.startswith("path+file://") and "#" in value:
        return f"path+file:///candidate#{value.split('#', 1)[1]}"
    if isinstance(value, list):
        return [normalize_local_sbom_references(item) for item in value]
    if isinstance(value, dict):
        return {
            key: normalize_local_sbom_references(item) for key, item in value.items()
        }
    return value


def validate_cyclonedx(
    sbom: dict[str, Any], package: dict[str, Any], descriptor: dict[str, Any]
) -> None:
    name = package["name"]
    if sbom.get("bomFormat") != "CycloneDX" or sbom.get("specVersion") != descriptor["tools"]["cyclonedx_spec"]:
        raise CandidateError(f"CycloneDX format/specification differs: {name}")
    component = sbom.get("metadata", {}).get("component", {})
    if not isinstance(component, dict):
        raise CandidateError(f"CycloneDX root component is missing: {name}")
    if component.get("name") != name or component.get("version") != descriptor["version"]:
        raise CandidateError(f"CycloneDX component identity differs: {name}")
    if descriptor["license"] not in component_license_ids(component):
        raise CandidateError(f"CycloneDX component license differs: {name}")

    components = sbom.get("components", [])
    dependencies = sbom.get("dependencies", [])
    if not isinstance(components, list) or not isinstance(dependencies, list):
        raise CandidateError(f"CycloneDX dependency graph is malformed: {name}")
    references = [component.get("bom-ref")]
    references.extend(
        dependency.get("bom-ref") for dependency in components if isinstance(dependency, dict)
    )
    if any(not isinstance(reference, str) or not reference for reference in references):
        raise CandidateError(f"CycloneDX component reference is missing: {name}")
    if len(references) != len(set(references)):
        raise CandidateError(f"CycloneDX component reference is duplicated: {name}")
    known = set(references)
    graph_references: list[str] = []
    for dependency in dependencies:
        if not isinstance(dependency, dict) or not isinstance(dependency.get("ref"), str):
            raise CandidateError(f"CycloneDX dependency graph is malformed: {name}")
        graph_references.append(dependency["ref"])
        depends_on = dependency.get("dependsOn", [])
        if not isinstance(depends_on, list) or not all(
            isinstance(reference, str) and reference in known for reference in depends_on
        ):
            raise CandidateError(f"CycloneDX dependency identity differs: {name}")
    if len(graph_references) != len(set(graph_references)) or not set(graph_references) <= known:
        raise CandidateError(f"CycloneDX dependency identity differs: {name}")
    if component["bom-ref"] not in graph_references:
        raise CandidateError(f"CycloneDX root dependency is missing: {name}")
    if "git+" in json.dumps(sbom, sort_keys=True):
        raise CandidateError(f"CycloneDX evidence contains a Git source: {name}")


def release_evidence(
    stage: Path,
    root: Path,
    output: Path,
    descriptor: dict[str, Any],
    env: dict[str, str],
    initialize_api: bool,
) -> tuple[dict[str, str], list[dict[str, Any]]]:
    tools = descriptor["tools"]
    versions = {
        "cargoPublicApi": require_subcommand_version(
            ["cargo", "public-api", "--version"], tools["cargo_public_api"], root, env
        ),
        "cargoSemverChecks": require_subcommand_version(
            ["cargo", "semver-checks", "--version"],
            tools["cargo_semver_checks"], root, env,
        ),
        "cargoCyclonedx": require_subcommand_version(
            ["cargo", "cyclonedx", "--version"], tools["cargo_cyclonedx"], root, env
        ),
        "cyclonedxSpec": tools["cyclonedx_spec"],
    }
    evidence: list[dict[str, Any]] = []
    api_target = stage / "target/public-api"
    api_env = env | {"RUSTC_BOOTSTRAP": "1"}
    for package in descriptor["packages"]:
        name = package["name"]
        run([
            "cargo", "rustdoc", "--manifest-path", str(stage / package["path"] / "Cargo.toml"),
            "--all-features", "--lib", "--target-dir", str(api_target), "--",
            "-Z", "unstable-options", "--output-format", "json",
        ], cwd=stage, env=api_env)
        api_json = api_target / "doc" / f"{name.replace('-', '_')}.json"
        require_bounded_evidence(api_json, descriptor)
        api = run_stdout([
            "cargo", "public-api", "--rustdoc-json", str(api_json),
            "-sss", "--color=never",
        ], cwd=stage, env=env)
        api_bytes = api.encode("utf-8")
        if not api.strip() or len(api_bytes) > descriptor["max_evidence_bytes"]:
            raise CandidateError(f"public API evidence has invalid byte size: {name}")
        baseline = root / package["api_baseline"]
        if not initialize_api and baseline.read_text(encoding="utf-8") != api:
            raise CandidateError(f"public API differs from committed candidate baseline: {name}")
        api_output = output / f"{name}.public-api.txt"
        api_output.write_text(api, encoding="utf-8")
        evidence.append({
            "kind": "public-api", "package": name, "file": api_output.name,
            "sha256": sha256(api_output), "bytes": api_output.stat().st_size,
            "baseline": package["api_baseline"],
        })

        override = f"{name}-candidate"
        run([
            "cargo", "cyclonedx", "--manifest-path", str(stage / package["path"] / "Cargo.toml"),
            "--format", "json", "--spec-version", tools["cyclonedx_spec"],
            "--all-features", "--override-filename", override,
        ], cwd=stage, env=env)
        matches = sorted((stage / package["path"]).glob(f"{override}*.json"))
        if len(matches) != 1:
            raise CandidateError(f"expected one CycloneDX document for {name}; found {len(matches)}")
        sbom_source = matches[0]
        require_bounded_evidence(sbom_source, descriptor)
        try:
            sbom = json.loads(sbom_source.read_text(encoding="utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError) as error:
            raise CandidateError(f"invalid CycloneDX JSON: {name}") from error
        sbom = normalize_local_sbom_references(sbom)
        validate_cyclonedx(sbom, package, descriptor)
        sbom_output = output / f"{name}.cdx.json"
        sbom_output.write_text(
            json.dumps(sbom, indent=2, sort_keys=False) + "\n", encoding="utf-8"
        )
        evidence.append({
            "kind": "cyclonedx", "package": name, "file": sbom_output.name,
            "sha256": sha256(sbom_output), "bytes": sbom_output.stat().st_size,
            "specVersion": tools["cyclonedx_spec"],
        })
    return versions, evidence


def sanitized_environment(cargo_home: Path) -> dict[str, str]:
    env = {
        key: value for key, value in os.environ.items()
        if "TOKEN" not in key.upper() and "SECRET" not in key.upper()
    }
    env["CARGO_HOME"] = str(cargo_home)
    env["CARGO_TERM_COLOR"] = "never"
    env["SOURCE_DATE_EPOCH"] = "1"
    return env


def matrix_host(descriptor: dict[str, Any], rustc_verbose: str) -> dict[str, Any]:
    match = re.search(r"(?m)^host: (\S+)$", rustc_verbose)
    if match is None:
        raise CandidateError("cannot determine rustc host triple")
    rust_triple = match.group(1)
    identities = {
        ("Linux", "x86_64"): ("linux", "x86_64-unknown-linux-gnu"),
        ("Darwin", "arm64"): ("macos", "aarch64-apple-darwin"),
    }
    identity = identities.get((platform.system(), platform.machine()))
    if identity is None or rust_triple != identity[1]:
        raise CandidateError(
            f"unsupported DID matrix host: {platform.system()}/{platform.machine()}/{rust_triple}"
        )
    host = next(
        (row for row in descriptor["matrix_hosts"] if row.get("name") == identity[0]), None
    )
    if not isinstance(host, dict):
        raise CandidateError(f"DID matrix host is not declared: {identity[0]}")
    return host | {"rust_triple": rust_triple}


def matrix_profiles(descriptor: dict[str, Any], package: str) -> list[dict[str, Any]]:
    return [profile for profile in descriptor["profiles"] if profile.get("package") == package]


def matrix_command(
    operation: str, package: str, profile: dict[str, Any], target: str | None = None
) -> list[str]:
    command = ["cargo", operation, "--locked", "-p", package]
    command.extend(profile_arguments(profile))
    if target is not None:
        command.extend(["--target", target])
    return command


def write_json_atomic(path: Path, value: dict[str, Any], maximum: int) -> None:
    payload = (json.dumps(value, indent=2, sort_keys=True) + "\n").encode("utf-8")
    if not payload or len(payload) > maximum:
        raise CandidateError(f"matrix receipt exceeds byte limit: {path.name}")
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(dir=path.parent, prefix=f".{path.name}.", delete=False) as stream:
        temporary = Path(stream.name)
        stream.write(payload)
        stream.flush()
        os.fsync(stream.fileno())
    try:
        os.replace(temporary, path)
    finally:
        temporary.unlink(missing_ok=True)


def build_matrix_lane(
    root: Path,
    output: Path,
    requested: str | None,
    allow_dirty: bool,
    toolchain_class: str,
) -> Path:
    run(
        [sys.executable, str(root / "scripts/check-release-candidates.py"), str(root)],
        cwd=root,
        env=os.environ.copy(),
    )
    descriptor = load_toml(root / DESCRIPTOR)
    matrix = descriptor["matrix"]
    if toolchain_class not in {"primary", "msrv"}:
        raise CandidateError("matrix toolchain must be primary or msrv")
    revision, dirty = source_revision(root, requested, allow_dirty)
    started = time.monotonic()
    with tempfile.TemporaryDirectory(prefix="identus-did-matrix-") as temporary:
        scratch = Path(temporary)
        require_vcs_independent_build_scratch(root, scratch)
        env = sanitized_environment(scratch / "cargo-home")
        expected_version = matrix[f"{toolchain_class}_rust_version"]
        rustc = require_tool_version(
            "rustc", run(["rustc", "--version"], cwd=root, env=env), expected_version
        )
        cargo = require_tool_version(
            "cargo", run(["cargo", "--version"], cwd=root, env=env), expected_version
        )
        host = matrix_host(descriptor, run(["rustc", "-vV"], cwd=root, env=env))
        stage = create_stage(root, scratch / "candidate", descriptor)
        run(["cargo", "generate-lockfile"], cwd=stage, env=env)
        lock = stage / "Cargo.lock"
        rows: list[dict[str, Any]] = []
        host_operation = host[f"{toolchain_class}_operation"]
        for package in host["packages"]:
            for profile in matrix_profiles(descriptor, package):
                command = matrix_command(host_operation, package, profile)
                run(command, cwd=stage, env=env)
                rows.append({
                    "scope": "host", "package": package, "profile": profile["name"],
                    "operation": host_operation, "target": host["rust_triple"],
                    "tier": "host-tested" if host_operation == "test" else "host-compiled",
                    "status": "passed", "command": command,
                })
        unsupported: list[dict[str, Any]] = []
        for target in descriptor["matrix_targets"]:
            if target["runner_host"] != host["name"]:
                continue
            for package in target["packages"]:
                for profile in matrix_profiles(descriptor, package):
                    command = matrix_command(target["operation"], package, profile, target["triple"])
                    run(command, cwd=stage, env=env)
                    rows.append({
                        "scope": "target", "package": package, "profile": profile["name"],
                        "operation": target["operation"], "target": target["triple"],
                        "tier": target["tier"], "status": "passed", "command": command,
                    })
            for package in target["unsupported_packages"]:
                unsupported.append({
                    "package": package, "target": target["triple"],
                    "status": "not-supported", "limitation": target["limitation"],
                })
        receipt = {
            "schemaVersion": matrix["schema_version"],
            "candidate": descriptor["candidate"],
            "sourceRevision": revision,
            "sourceDirty": dirty,
            "host": {
                "name": host["name"], "nixSystem": host["nix_system"],
                "rustTriple": host["rust_triple"],
            },
            "toolchain": {
                "class": toolchain_class, "rustVersion": expected_version,
                "rustc": rustc, "cargo": cargo,
            },
            "lock": {"file": "Cargo.lock", "sha256": sha256(lock)},
            "rows": rows,
            "unsupported": unsupported,
            "limitations": [
                "portable rows are compile-only and make no runtime or packaging claim",
                "the HTTP resolver adapter is host-only",
            ],
            "elapsedSeconds": round(time.monotonic() - started, 3),
        }
        receipt_path = output / f"{host['name']}-{toolchain_class}.json"
        if output.exists():
            raise CandidateError(f"output already exists: {output}")
        write_json_atomic(receipt_path, receipt, matrix["max_receipt_bytes"])
        return receipt_path


def require_exact_keys(value: dict[str, Any], expected: set[str], label: str) -> None:
    if set(value) != expected:
        raise CandidateError(f"{label} fields differ")


def expected_lane_rows(
    descriptor: dict[str, Any], host: dict[str, Any], toolchain_class: str
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    operation = host[f"{toolchain_class}_operation"]
    rows: list[dict[str, Any]] = []
    for package in host["packages"]:
        for profile in matrix_profiles(descriptor, package):
            command = matrix_command(operation, package, profile)
            rows.append({
                "scope": "host", "package": package, "profile": profile["name"],
                "operation": operation, "target": host["rust_triple"],
                "tier": "host-tested" if operation == "test" else "host-compiled",
                "status": "passed", "command": command,
            })
    unsupported: list[dict[str, Any]] = []
    for target in descriptor["matrix_targets"]:
        if target["runner_host"] != host["name"]:
            continue
        for package in target["packages"]:
            for profile in matrix_profiles(descriptor, package):
                command = matrix_command(target["operation"], package, profile, target["triple"])
                rows.append({
                    "scope": "target", "package": package, "profile": profile["name"],
                    "operation": target["operation"], "target": target["triple"],
                    "tier": target["tier"], "status": "passed", "command": command,
                })
        for package in target["unsupported_packages"]:
            unsupported.append({
                "package": package, "target": target["triple"],
                "status": "not-supported", "limitation": target["limitation"],
            })
    return rows, unsupported


def aggregate_matrix(
    root: Path, output: Path, requested: str | None, lane_paths: list[Path]
) -> Path:
    run(
        [sys.executable, str(root / "scripts/check-release-candidates.py"), str(root)],
        cwd=root,
        env=os.environ.copy(),
    )
    descriptor = load_toml(root / DESCRIPTOR)
    matrix = descriptor["matrix"]
    head = run(["git", "rev-parse", "HEAD"], cwd=root, env=os.environ.copy()).strip()
    revision = requested or head
    if not SHA.fullmatch(revision) or revision != head:
        raise CandidateError("matrix aggregate revision must equal repository HEAD")
    expected_identities = {
        (host["name"], toolchain_class)
        for host in descriptor["matrix_hosts"]
        for toolchain_class in ("primary", "msrv")
    }
    if len(lane_paths) != len(expected_identities):
        raise CandidateError("matrix aggregate requires exactly four lane receipts")
    lanes: list[dict[str, Any]] = []
    identities: set[tuple[str, str]] = set()
    lock_hashes: set[str] = set()
    for path in lane_paths:
        require_bounded_evidence(path, {"max_evidence_bytes": matrix["max_receipt_bytes"]})
        try:
            lane = json.loads(path.read_text(encoding="utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError) as error:
            raise CandidateError(f"invalid matrix lane receipt: {path.name}") from error
        if not isinstance(lane, dict):
            raise CandidateError(f"matrix lane receipt is not an object: {path.name}")
        require_exact_keys(lane, {
            "schemaVersion", "candidate", "sourceRevision", "sourceDirty", "host",
            "toolchain", "lock", "rows", "unsupported", "limitations", "elapsedSeconds",
        }, "matrix lane")
        if lane["schemaVersion"] != matrix["schema_version"]:
            raise CandidateError("matrix lane schema differs")
        if lane["candidate"] != descriptor["candidate"] or lane["sourceRevision"] != revision:
            raise CandidateError("matrix lane source identity differs")
        if lane["sourceDirty"] is not False:
            raise CandidateError("matrix lane source must be clean")
        if not isinstance(lane.get("host"), dict):
            raise CandidateError("matrix lane host is malformed")
        if not isinstance(lane.get("toolchain"), dict):
            raise CandidateError("matrix lane toolchain is malformed")
        if not isinstance(lane.get("lock"), dict):
            raise CandidateError("matrix lane lock is malformed")
        require_exact_keys(lane["host"], {"name", "nixSystem", "rustTriple"}, "matrix host")
        require_exact_keys(
            lane["toolchain"], {"class", "rustVersion", "rustc", "cargo"},
            "matrix toolchain",
        )
        require_exact_keys(lane["lock"], {"file", "sha256"}, "matrix lock")
        if lane["lock"]["file"] != "Cargo.lock":
            raise CandidateError("matrix lane lock identity differs")
        if lane["limitations"] != [
            "portable rows are compile-only and make no runtime or packaging claim",
            "the HTTP resolver adapter is host-only",
        ]:
            raise CandidateError("matrix lane limitations differ")
        if (
            not isinstance(lane["elapsedSeconds"], (int, float))
            or isinstance(lane["elapsedSeconds"], bool)
            or lane["elapsedSeconds"] < 0
        ):
            raise CandidateError("matrix lane elapsed time is invalid")
        host_name = lane.get("host", {}).get("name")
        toolchain_class = lane.get("toolchain", {}).get("class")
        identity = (host_name, toolchain_class)
        if identity in identities or identity not in expected_identities:
            raise CandidateError("matrix lane identity is duplicate or unexpected")
        identities.add(identity)
        host = next(row for row in descriptor["matrix_hosts"] if row["name"] == host_name)
        expected_triples = {
            "linux": "x86_64-unknown-linux-gnu", "macos": "aarch64-apple-darwin",
        }
        expected_host = {
            "name": host_name, "nixSystem": host["nix_system"],
            "rustTriple": expected_triples[host_name],
        }
        if lane["host"] != expected_host:
            raise CandidateError("matrix lane host differs")
        expected_version = matrix[f"{toolchain_class}_rust_version"]
        toolchain = lane["toolchain"]
        if (
            not isinstance(toolchain, dict)
            or toolchain.get("rustVersion") != expected_version
            or not str(toolchain.get("rustc", "")).startswith(f"rustc {expected_version}")
            or not str(toolchain.get("cargo", "")).startswith(f"cargo {expected_version}")
        ):
            raise CandidateError("matrix lane compiler identity differs")
        expected_rows, expected_unsupported = expected_lane_rows(
            descriptor, host | {"rust_triple": expected_triples[host_name]}, toolchain_class
        )
        if lane["rows"] != expected_rows:
            raise CandidateError("matrix lane rows differ or overclaim support")
        if lane["unsupported"] != expected_unsupported:
            raise CandidateError("matrix lane unsupported boundary differs")
        lock_hash = lane.get("lock", {}).get("sha256")
        if not isinstance(lock_hash, str) or not re.fullmatch(r"[0-9a-f]{64}", lock_hash):
            raise CandidateError("matrix lane lock hash is invalid")
        lock_hashes.add(lock_hash)
        lanes.append(lane)
    if identities != expected_identities:
        raise CandidateError("matrix lane set is incomplete")
    if len(lock_hashes) != 1:
        raise CandidateError("matrix lanes did not use one staged lockfile")
    aggregate = {
        "schemaVersion": matrix["schema_version"],
        "candidate": descriptor["candidate"],
        "sourceRevision": revision,
        "sourceDirty": False,
        "lockSha256": next(iter(lock_hashes)),
        "laneCount": len(lanes),
        "status": "passed",
        "lanes": sorted(
            lanes, key=lambda row: (row["host"]["name"], row["toolchain"]["class"])
        ),
        "claim": "host-tested plus explicitly bounded portable compile support",
        "limitations": [
            "portable rows are compile-only and make no runtime or packaging claim",
            "identus-did-resolver-http remains host-only",
        ],
    }
    if output.exists():
        raise CandidateError(f"output already exists: {output}")
    write_json_atomic(output, aggregate, matrix["max_receipt_bytes"])
    return output


def build(
    root: Path, output: Path, requested: str | None, allow_dirty: bool, initialize_api: bool
) -> Path:
    run(
        [sys.executable, str(root / "scripts/check-release-candidates.py"), str(root)],
        cwd=root,
        env=os.environ.copy(),
    )
    descriptor = load_toml(root / DESCRIPTOR)
    index = load_toml(root / INDEX)
    did_train = next((train for train in index.get("trains", []) if train.get("id") == "did"), None)
    if not isinstance(did_train, dict) or did_train.get("descriptor") != DESCRIPTOR.as_posix():
        raise CandidateError("DID train index binding differs")
    revision, dirty = source_revision(root, requested, allow_dirty or initialize_api)
    dirty = dirty or initialize_api
    started = time.monotonic()
    output_parent = output.resolve().parent
    output_parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="identus-did-candidate-") as temporary:
        scratch = Path(temporary)
        require_vcs_independent_build_scratch(root, scratch)
        env = sanitized_environment(scratch / "cargo-home")
        expected_tool = descriptor["preparation_rust_version"]
        rustc = require_tool_version("rustc", run(["rustc", "--version"], cwd=root, env=env), expected_tool)
        cargo = require_tool_version("cargo", run(["cargo", "--version"], cwd=root, env=env), expected_tool)
        passes: list[dict[str, Path]] = []
        for pass_name in ("first", "second"):
            stage = create_stage(root, scratch / pass_name, descriptor)
            passes.append(assemble(stage, descriptor, env))
        for name in PACKAGE_ORDER:
            if sha256(passes[0][name]) != sha256(passes[1][name]):
                raise CandidateError(f"two-pass archive bytes differ: {name}")
        for package in descriptor["packages"]:
            inspect_archive(passes[0][package["name"]], package, descriptor)
        commands = verify_closure(passes[0], scratch, descriptor, env)
        staged_output = Path(tempfile.mkdtemp(prefix=f".{output.name}.tmp-", dir=output_parent))
        try:
            archive_receipts = []
            for name in PACKAGE_ORDER:
                source = passes[0][name]
                destination = staged_output / source.name
                shutil.copyfile(source, destination)
                archive_receipts.append({
                    "package": name, "file": destination.name,
                    "sha256": sha256(destination), "bytes": destination.stat().st_size,
                    "files": archive_files(destination, descriptor),
                })
            evidence_tools, evidence = release_evidence(
                scratch / "first" / "workspace", root, staged_output,
                descriptor, env, initialize_api,
            )
            receipt = {
                "schemaVersion": 1,
                "candidate": descriptor["candidate"],
                "releaseTag": descriptor["release_tag"],
                "publication": descriptor["publication"],
                "sourceRevision": revision,
                "sourceDirty": dirty,
                "tools": {"rustc": rustc, "cargo": cargo, **evidence_tools},
                "compatibility": {
                    "status": descriptor["compatibility_status"],
                    "futureTool": descriptor["tools"]["cargo_semver_checks"],
                    "claim": "first candidate API origin; no stable SemVer promise",
                },
                "repositoryPolicy": {
                    "licenseAdvisoryAuthority": ["Cargo.lock", "deny.toml"],
                    "candidateSpecificScan": "not-run",
                    "reason": (
                        "local candidate SBOM evidence is deterministic; repository "
                        "license/advisory gates remain a separate promotion control"
                    ),
                },
                "twoPassByteIdentical": True,
                "archives": archive_receipts,
                "evidence": evidence,
                "verificationCommands": commands,
                "limitations": [
                    "candidate-only; canonical manifests remain unpublished",
                    "no registry upload, Git tag, GitHub release, or credentials",
                    "registry availability and platform matrix are separate promotion gates",
                    "local SBOMs are not signed provenance or vulnerability-free claims",
                ],
                "elapsedSeconds": round(time.monotonic() - started, 3),
            }
            receipt_path = staged_output / "candidate-receipt.json"
            receipt_path.write_text(
                json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
            )
            if output.exists():
                raise CandidateError(f"output already exists: {output}")
            os.replace(staged_output, output)
        finally:
            if staged_output.exists():
                shutil.rmtree(staged_output)
    return output / "candidate-receipt.json"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path("."))
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--revision")
    parser.add_argument("--allow-dirty", action="store_true")
    parser.add_argument("--initialize-api", action="store_true")
    parser.add_argument("--matrix-toolchain", choices=("primary", "msrv"))
    parser.add_argument("--aggregate-matrix", action="store_true")
    parser.add_argument("--lane-receipt", action="append", type=Path, default=[])
    return parser.parse_args()


def main() -> int:
    arguments = parse_args()
    try:
        root = arguments.root.resolve()
        output = arguments.output.resolve()
        if arguments.aggregate_matrix:
            if arguments.matrix_toolchain or arguments.allow_dirty or arguments.initialize_api:
                raise CandidateError("aggregate mode does not accept lane/build options")
            receipt = aggregate_matrix(
                root, output, arguments.revision,
                [path.resolve() for path in arguments.lane_receipt],
            )
        elif arguments.matrix_toolchain:
            if arguments.lane_receipt or arguments.initialize_api:
                raise CandidateError("matrix lane mode does not accept aggregate/build options")
            receipt = build_matrix_lane(
                root, output, arguments.revision, arguments.allow_dirty,
                arguments.matrix_toolchain,
            )
        else:
            if arguments.lane_receipt:
                raise CandidateError("candidate build does not accept lane receipts")
            receipt = build(
                root, output, arguments.revision,
                arguments.allow_dirty, arguments.initialize_api,
            )
    except (CandidateError, OSError, KeyError, tomllib.TOMLDecodeError) as error:
        print(f"did-candidate: {error}", file=sys.stderr)
        return 1
    print(receipt)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
