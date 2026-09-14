#!/usr/bin/env python3
"""Build and verify the unpublished three-crate cryptography candidate."""

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

DESCRIPTOR = Path("docs/release/crypto-candidate.toml")
API_BASELINE = Path("docs/release/identus-crypto-0.1.0-rc.1.api.txt")
PACKAGE_ORDER = ("identus-derive", "identus-core", "identus-crypto")
INTERNAL_DEPENDENCIES = {
    "identus-core": ("identus-derive",),
    "identus-crypto": ("identus-core", "identus-derive"),
}
EXTERNAL_DEPENDENCIES = (
    "base64", "bip39", "coset", "ed25519-bip32", "ed25519-dalek", "hex",
    "hmac", "k256", "p256", "pbkdf2", "proc-macro2", "quote", "serde",
    "serde_json", "sha2", "syn", "trybuild", "x25519-dalek", "zeroize",
)
ALLOWED_SUFFIXES = {".rs", ".stderr", ".md"}
SHA = re.compile(r"^[0-9a-f]{40}$")


class CandidateError(RuntimeError):
    pass


def run(command: list[str], *, cwd: Path, env: dict[str, str]) -> str:
    result = subprocess.run(
        command, cwd=cwd, env=env, check=False, text=True,
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
    )
    if result.returncode != 0:
        rendered = " ".join(command)
        raise CandidateError(f"command failed ({result.returncode}): {rendered}\n{result.stdout}")
    return result.stdout


def run_stdout(command: list[str], *, cwd: Path, env: dict[str, str]) -> str:
    """Return stdout without contaminating generated evidence with diagnostics."""
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


def load_descriptor(root: Path) -> dict[str, Any]:
    with (root / DESCRIPTOR).open("rb") as source:
        return tomllib.load(source)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def source_revision(root: Path, requested: str | None, allow_dirty: bool) -> tuple[str, bool]:
    head = run(["git", "rev-parse", "HEAD"], cwd=root, env=os.environ.copy()).strip()
    revision = requested or head
    if not SHA.fullmatch(revision):
        raise CandidateError("source revision must be a full lowercase Git SHA")
    if revision != head:
        raise CandidateError("requested source revision must equal repository HEAD")
    dirty = bool(run(["git", "status", "--porcelain"], cwd=root, env=os.environ.copy()))
    if dirty and not allow_dirty:
        raise CandidateError("candidate evidence requires a clean Git worktree")
    return revision, dirty


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
        'members = [ "crates/core", "crates/derive", "crates/crypto" ]', "",
        "[workspace.dependencies]",
        f'identus-core   = {{ path = "crates/core", version = "={version}" }}',
        f'identus-derive = {{ path = "crates/derive", version = "={version}" }}',
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
    name_line = re.search(rf'(?m)^name\s*=\s*"{re.escape(package["name"])}"\s*$', source)
    if name_line is None:
        raise CandidateError(f"cannot locate package name in {package['path']}/Cargo.toml")
    metadata = "\n".join([
        name_line.group(0),
        f'description = {json.dumps(package["description"])}',
        "repository.workspace = true",
        "homepage.workspace = true",
        f'documentation = {json.dumps(package["documentation"])}',
        f'readme = {json.dumps(package["readme"])}',
        f'keywords = {toml_array(package["keywords"])}',
        f'categories = {toml_array(package["categories"])}',
    ])
    return source[: name_line.start()] + metadata + source[name_line.end() :]


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
            rendered = render_package_manifest(source.read_text(encoding="utf-8"), package)
            destination.write_text(rendered, encoding="utf-8")
        else:
            shutil.copyfile(source, destination)
    shutil.copyfile(root / "LICENSE", target_root / "LICENSE")


def create_stage(root: Path, parent: Path, descriptor: dict[str, Any]) -> Path:
    stage = parent / "workspace"
    stage.mkdir(parents=True)
    (stage / "Cargo.toml").write_text(render_root_manifest(root, descriptor), encoding="utf-8")
    shutil.copyfile(root / "Cargo.lock", stage / "Cargo.lock")
    for package in descriptor["packages"]:
        copy_package(root, stage, package)
    return stage


def assemble(stage: Path, descriptor: dict[str, Any], env: dict[str, str]) -> dict[str, Path]:
    run(["cargo", "generate-lockfile", "--manifest-path", str(stage / "Cargo.toml")], cwd=stage, env=env)
    # Cargo 1.98 can stage mutually dependent, unpublished workspace crates in
    # one operation. Packaging a dependent crate alone incorrectly asks the
    # public registry for the sibling candidate before it exists there.
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


def safe_members(archive: Path, prefix: str, maximum: int) -> list[tarfile.TarInfo]:
    if archive.stat().st_size > maximum:
        raise CandidateError(f"archive exceeds byte limit: {archive.name}")
    with tarfile.open(archive, "r:gz") as package:
        members = package.getmembers()
    if not members or len(members) > 512:
        raise CandidateError(f"archive has invalid member count: {archive.name}")
    names: set[str] = set()
    expanded_bytes = 0
    for member in members:
        path = PurePosixPath(member.name)
        if path.is_absolute() or ".." in path.parts or path.parts[0] != prefix:
            raise CandidateError(f"unsafe archive path: {member.name}")
        if not (member.isfile() or member.isdir()):
            raise CandidateError(f"archive contains link or special file: {member.name}")
        if member.name in names:
            raise CandidateError(f"archive contains duplicate path: {member.name}")
        names.add(member.name)
        expanded_bytes += member.size
    if expanded_bytes > maximum * 20:
        raise CandidateError(f"archive exceeds expanded-byte limit: {archive.name}")
    return members


def extract_safe(archive: Path, destination: Path, maximum: int) -> Path:
    prefix = archive.name.removesuffix(".crate")
    members = safe_members(archive, prefix, maximum)
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
    members = safe_members(archive, prefix, descriptor["max_archive_bytes"])
    names = {member.name for member in members}
    required = {f"{prefix}/{name}" for name in ("Cargo.toml", "Cargo.toml.orig", "README.md", "LICENSE", "src/lib.rs")}
    if missing := sorted(required - names):
        raise CandidateError(f"{archive.name} misses required files: {', '.join(missing)}")
    with tarfile.open(archive, "r:gz") as contents:
        source = contents.extractfile(f"{prefix}/Cargo.toml")
        if source is None:
            raise CandidateError(f"cannot inspect normalized manifest: {archive.name}")
        manifest = tomllib.loads(source.read().decode("utf-8"))
    metadata = manifest.get("package", {})
    expected_metadata = {
        "description": package["description"],
        "repository": descriptor["repository"],
        "homepage": descriptor["homepage"],
        "documentation": package["documentation"],
        "readme": package["readme"],
        "keywords": package["keywords"],
        "categories": package["categories"],
        "license": descriptor["license"],
        "rust-version": descriptor["rust_version"],
    }
    for field, expected in expected_metadata.items():
        if metadata.get(field) != expected:
            raise CandidateError(f"{package['name']}: normalized metadata differs: {field}")
    if metadata.get("version") != descriptor["version"]:
        raise CandidateError(f"{package['name']}: normalized candidate version differs")
    for table in dependency_tables(manifest):
        for dependency, value in table.items():
            if isinstance(value, dict) and ("git" in value or "path" in value):
                raise CandidateError(f"{package['name']}: normalized dependency {dependency} retains git/path")
            if dependency in INTERNAL_DEPENDENCIES.get(package["name"], ()):
                version = value if isinstance(value, str) else value.get("version")
                if version != f"={descriptor['version']}":
                    raise CandidateError(f"{package['name']}: {dependency} is not exact candidate version")


def verify_closure(archives: dict[str, Path], root: Path, descriptor: dict[str, Any], env: dict[str, str]) -> None:
    verify = root / "verify"
    packages = {name: extract_safe(path, verify / "packages", descriptor["max_archive_bytes"]) for name, path in archives.items()}
    consumer = verify / "consumer"
    (consumer / "src").mkdir(parents=True)
    crypto_features = tomllib.loads((packages["identus-crypto"] / "Cargo.toml").read_text(encoding="utf-8"))["features"]
    all_features = sorted(name for name in crypto_features if name != "default")
    consumer_manifest = "\n".join([
        "[package]", 'name = "candidate-consumer"', 'version = "0.0.0"', 'edition = "2024"', "publish = false", "",
        "[dependencies]", f'identus-crypto = {{ version = "={descriptor["version"]}", default-features = false }}', "",
        "[features]", 'default = [ "identus-crypto/default" ]',
        f'all-crypto = {toml_array([f"identus-crypto/{name}" for name in all_features])}',
        'kmp-compat = [ "identus-crypto/kmp-compat" ]', "",
    ])
    (consumer / "Cargo.toml").write_text(consumer_manifest, encoding="utf-8")
    # The consumer intentionally has no source-level API dependency so the
    # no-default-features profile remains a meaningful compile check.
    (consumer / "src/main.rs").write_text("fn main() {}\n", encoding="utf-8")
    member_paths = [packages[name].relative_to(verify).as_posix() for name in PACKAGE_ORDER]
    root_manifest = ["[workspace]", 'resolver = "3"', f"members = {toml_array([*member_paths, 'consumer'])}", "", "[patch.crates-io]"]
    root_manifest.extend(f'{name} = {{ path = "{packages[name].relative_to(verify).as_posix()}" }}' for name in PACKAGE_ORDER)
    (verify / "Cargo.toml").write_text("\n".join(root_manifest) + "\n", encoding="utf-8")
    run(["cargo", "generate-lockfile", "--manifest-path", str(verify / "Cargo.toml")], cwd=verify, env=env)
    commands = [
        ["cargo", "check", "--locked", "-p", "candidate-consumer"],
        ["cargo", "check", "--locked", "-p", "candidate-consumer", "--no-default-features"],
        ["cargo", "check", "--locked", "-p", "candidate-consumer", "--no-default-features", "--features", "all-crypto"],
        ["cargo", "check", "--locked", "-p", "candidate-consumer", "--no-default-features", "--features", "kmp-compat"],
        ["cargo", "test", "--locked", "-p", "identus-crypto", "--all-features"],
    ]
    for command in commands:
        run(command, cwd=verify, env=env)


def tool_version(command: list[str], expected: str, cwd: Path, env: dict[str, str]) -> str:
    output = run(command, cwd=cwd, env=env).strip()
    if not output or output.split()[-1] != expected:
        raise CandidateError(f"tool version differs; expected {expected}: {output}")
    return output


def release_evidence(stage: Path, root: Path, output: Path, descriptor: dict[str, Any], env: dict[str, str], initialize_api: bool) -> dict[str, str]:
    tools = descriptor["tools"]
    versions = {
        "cargo_public_api": tool_version(["cargo", "public-api", "--version"], tools["cargo_public_api"], root, env),
        "cargo_semver_checks": tool_version(["cargo", "semver-checks", "--version"], tools["cargo_semver_checks"], root, env),
        "cargo_cyclonedx": tool_version(["cargo", "cyclonedx", "--version"], tools["cargo_cyclonedx"], root, env),
    }
    # cargo-public-api uses rustdoc JSON, which is not stable yet. Keep the
    # compiler itself at the 1.98.1 etalon and scope the documented bootstrap
    # escape hatch to this inspection subprocess only.
    api_env = env | {"RUSTC_BOOTSTRAP": "1"}
    api = run_stdout([
        "cargo", "public-api", "--manifest-path", str(stage / "crates/crypto/Cargo.toml"),
        "--all-features", "-sss", "--color=never",
    ], cwd=stage, env=api_env)
    api_path = root / API_BASELINE
    if initialize_api:
        api_path.parent.mkdir(parents=True, exist_ok=True)
        api_path.write_text(api, encoding="utf-8")
    elif not api_path.is_file() or api_path.read_text(encoding="utf-8") != api:
        raise CandidateError("public API differs from the committed candidate baseline")
    (output / "identus-crypto.public-api.txt").write_text(api, encoding="utf-8")
    run([
        "cargo", "semver-checks", "check-release", "--manifest-path", str(root / "Cargo.toml"),
        "-p", "identus-crypto", "--baseline-rev", descriptor["baseline_revision"], "--all-features",
    ], cwd=root, env=env)
    run([
        "cargo", "cyclonedx", "--manifest-path", str(stage / "crates/crypto/Cargo.toml"),
        "--format", "json", "--spec-version", tools["cyclonedx_spec"], "--all-features", "--all",
        "--override-filename", "identus-crypto-candidate",
    ], cwd=stage, env=env)
    sboms = sorted(stage.rglob("identus-crypto-candidate*.json"))
    if len(sboms) != len(PACKAGE_ORDER):
        found = ", ".join(path.relative_to(stage).as_posix() for path in sboms)
        raise CandidateError(f"expected three CycloneDX JSON documents, found {len(sboms)}: {found}")
    seen: set[str] = set()
    for path in sboms:
        sbom = json.loads(path.read_text(encoding="utf-8"))
        component = sbom.get("metadata", {}).get("component", {})
        name = component.get("name")
        if name not in PACKAGE_ORDER or component.get("version") != descriptor["version"]:
            raise CandidateError(f"CycloneDX component does not bind the candidate: {path}")
        if name in seen:
            raise CandidateError(f"duplicate CycloneDX component: {name}")
        seen.add(name)
        shutil.copyfile(path, output / f"{name}.cdx.json")
    if seen != set(PACKAGE_ORDER):
        raise CandidateError("CycloneDX evidence does not cover the candidate closure")
    return versions


def prepare(args: argparse.Namespace) -> Path:
    root = Path(__file__).resolve().parents[1]
    descriptor = load_descriptor(root)
    revision, dirty = source_revision(root, args.source_revision, args.allow_dirty or args.initialize_api)
    dirty = dirty or args.initialize_api
    output = Path(args.output).resolve()
    if output.exists():
        raise CandidateError(f"output path already exists: {output}")
    output.parent.mkdir(parents=True, exist_ok=True)
    start = time.monotonic()
    env = os.environ.copy()
    env.update({"SOURCE_DATE_EPOCH": "1", "CARGO_TERM_COLOR": "never"})
    with tempfile.TemporaryDirectory(prefix=".identus-crypto-candidate-", dir=output.parent) as temporary:
        scratch = Path(temporary)
        first_stage = create_stage(root, scratch / "first", descriptor)
        second_stage = create_stage(root, scratch / "second", descriptor)
        first = assemble(first_stage, descriptor, env)
        second = assemble(second_stage, descriptor, env)
        digests = {name: sha256(first[name]) for name in PACKAGE_ORDER}
        for name in PACKAGE_ORDER:
            if first[name].read_bytes() != second[name].read_bytes():
                raise CandidateError(f"non-deterministic Cargo archive: {name}")
            package = next(row for row in descriptor["packages"] if row["name"] == name)
            inspect_archive(first[name], package, descriptor)
        verify_closure(first, scratch, descriptor, env)
        staging_output = scratch / "output"
        (staging_output / "packages").mkdir(parents=True)
        for name in PACKAGE_ORDER:
            shutil.copyfile(first[name], staging_output / "packages" / first[name].name)
        versions: dict[str, str] = {}
        if not args.package_only:
            versions = release_evidence(first_stage, root, staging_output, descriptor, env, args.initialize_api)
        receipt = {
            "schemaVersion": 1,
            "candidate": descriptor["candidate"],
            "version": descriptor["version"],
            "sourceRevision": revision,
            "sourceDirty": dirty,
            "publication": "prohibited",
            "verification": "unpublished-archive-closure",
            "rustVersion": run(["rustc", "--version"], cwd=root, env=env).strip(),
            "cargoVersion": run(["cargo", "--version"], cwd=root, env=env).strip(),
            "tools": versions,
            "profiles": [row["name"] for row in descriptor["profiles"]],
            "packages": [
                {"name": name, "version": descriptor["version"], "sha256": digests[name], "bytes": first[name].stat().st_size}
                for name in PACKAGE_ORDER
            ],
            "limitations": [
                "unpublished; no registry resolution or cargo publish dry-run",
                "Rust 1.98.1 is candidate-preparation evidence, not the publication MSRV",
                "public API rendering scopes RUSTC_BOOTSTRAP=1 to rustdoc JSON inspection",
                "no tag, signature, attestation, foreign-language package, main promotion, or downstream migration",
            ],
            "durationSeconds": round(time.monotonic() - start, 3),
        }
        (staging_output / "receipt.json").write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        staging_output.rename(output)
    return output


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", required=True)
    parser.add_argument("--source-revision")
    parser.add_argument("--allow-dirty", action="store_true")
    parser.add_argument("--package-only", action="store_true")
    parser.add_argument("--initialize-api", action="store_true")
    args = parser.parse_args()
    try:
        output = prepare(args)
    except (CandidateError, OSError, KeyError, tomllib.TOMLDecodeError) as error:
        print(f"crypto-candidate: {error}", file=sys.stderr)
        return 1
    print(f"crypto-candidate: evidence written to {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
