#!/usr/bin/env python3
"""Build and verify the credential-free two-crate DID release candidate."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
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
ALLOWED_CARGO_OPERATIONS = frozenset({"check", "generate-lockfile", "package", "test"})
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


def sanitized_environment(cargo_home: Path) -> dict[str, str]:
    env = {
        key: value for key, value in os.environ.items()
        if "TOKEN" not in key.upper() and "SECRET" not in key.upper()
    }
    env["CARGO_HOME"] = str(cargo_home)
    env["CARGO_TERM_COLOR"] = "never"
    env["SOURCE_DATE_EPOCH"] = "1"
    return env


def build(root: Path, output: Path, requested: str | None, allow_dirty: bool) -> Path:
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
    revision, dirty = source_revision(root, requested, allow_dirty)
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
            receipt = {
                "schemaVersion": 1,
                "candidate": descriptor["candidate"],
                "releaseTag": descriptor["release_tag"],
                "publication": descriptor["publication"],
                "sourceRevision": revision,
                "sourceDirty": dirty,
                "tools": {"rustc": rustc, "cargo": cargo},
                "twoPassByteIdentical": True,
                "archives": archive_receipts,
                "verificationCommands": commands,
                "limitations": [
                    "candidate-only; canonical manifests remain unpublished",
                    "no registry upload, Git tag, GitHub release, or credentials",
                    "registry availability and platform matrix are separate promotion gates",
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
    return parser.parse_args()


def main() -> int:
    arguments = parse_args()
    try:
        receipt = build(
            arguments.root.resolve(), arguments.output.resolve(),
            arguments.revision, arguments.allow_dirty,
        )
    except (CandidateError, OSError, KeyError, tomllib.TOMLDecodeError) as error:
        print(f"did-candidate: {error}", file=sys.stderr)
        return 1
    print(receipt)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
