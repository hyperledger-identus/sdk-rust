#!/usr/bin/env python3
"""Mutation tests for independent release-candidate train policy."""

from __future__ import annotations

import importlib.util
import hashlib
import io
import json
import shutil
import tarfile
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-release-candidates.py"
BUILDER = ROOT / "scripts/prepare-did-candidate.py"


def load_checker():
    spec = importlib.util.spec_from_file_location("release_candidates_checker", CHECKER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load release-candidates checker")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def load_builder():
    spec = importlib.util.spec_from_file_location("did_candidate_builder", BUILDER)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load DID candidate builder")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def copy_fixture(destination: Path) -> None:
    files = (
        "Cargo.toml",
        "docs/release/release-trains.toml",
        "docs/release/did-candidate.toml",
        "docs/release/did-candidate.lock",
        "docs/release/identus-did-0.1.0-rc.1.api.txt",
        "docs/release/identus-did-resolver-http-0.1.0-rc.1.api.txt",
        "docs/release/crypto-candidate.toml",
        "docs/adr/0153-use-primary-package-tags-for-independent-release-trains.md",
        "docs/adr/0154-use-first-candidate-api-snapshots-as-semver-origin.md",
        "docs/adr/0155-qualify-staged-did-candidate-matrix.md",
        ".github/workflows/nix-checks.yml",
        "nix/apps/default.nix",
        "nix/apps/did-candidate.nix",
        "nix/apps/did-candidate-matrix-primary.nix",
        "nix/apps/did-candidate-matrix-msrv.nix",
        "scripts/prepare-did-candidate.py",
        "crates/did/Cargo.toml",
        "crates/did/README.md",
        "crates/did-resolver-http/Cargo.toml",
        "crates/did-resolver-http/README.md",
    )
    for relative in files:
        target = destination / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / relative, target)


def replace(path: Path, old: str, new: str) -> None:
    source = path.read_text(encoding="utf-8")
    if old not in source:
        raise AssertionError(f"fixture text not found: {old}")
    path.write_text(source.replace(old, new, 1), encoding="utf-8")


def append(path: Path, text: str) -> None:
    path.write_text(path.read_text(encoding="utf-8") + text, encoding="utf-8")


def replace_lock_and_rebind(root: Path, old: str, new: str) -> None:
    lock = root / "docs/release/did-candidate.lock"
    replace(lock, old, new)
    rebind_lock_digest(root)


def rebind_lock_digest(root: Path) -> None:
    lock = root / "docs/release/did-candidate.lock"
    digest = hashlib.sha256(lock.read_bytes()).hexdigest()
    descriptor = root / "docs/release/did-candidate.toml"
    replace(
        descriptor,
        'staged_lock_sha256       = "1f1d4206e2ced5bd74675d654876684536dd8f82bf79cd6de4c2fa67db904447"',
        f'staged_lock_sha256       = "{digest}"',
    )


def move_cleanliness_step_after_msrv(root: Path) -> None:
    path = root / ".github/workflows/nix-checks.yml"
    source = path.read_text(encoding="utf-8")
    cleanliness = """      - name: Verify primary qualification preserved a clean checkout
        run: |
          dirty_paths="$(git status --porcelain --untracked-files=all)"
          if test -n "$dirty_paths"; then
            echo "::error::primary DID candidate qualification dirtied the checkout"
            printf '%s\\n' "$dirty_paths"
            exit 1
          fi

"""
    msrv = """      - name: Qualify staged DID sources with the MSRV
        run: >-
          nix run .#did-candidate-matrix-msrv --
          --output "$RUNNER_TEMP/did-matrix/msrv"
          --revision "$GITHUB_SHA"

"""
    if cleanliness not in source or msrv not in source:
        raise AssertionError("fixture DID lane boundary not found")
    source = source.replace(cleanliness, "", 1).replace(msrv, msrv + cleanliness, 1)
    path.write_text(source, encoding="utf-8")


def require_rejection(checker, fixture: Path, mutation, expected: str) -> None:
    mutation(fixture)
    errors = checker.validate(fixture)
    if not any(expected in error for error in errors):
        raise AssertionError(f"mutation was accepted; expected {expected!r}: {errors!r}")


def main() -> int:
    checker = load_checker()
    builder = load_builder()
    with tempfile.TemporaryDirectory(prefix="release-candidates-policy-") as temporary:
        boundary = Path(temporary) / "boundary"
        repository = boundary / "repository"
        (repository / ".git").mkdir(parents=True)
        try:
            builder.require_vcs_independent_build_scratch(repository, repository / "scratch")
        except builder.CandidateError:
            pass
        else:
            raise AssertionError("repository-contained candidate scratch was accepted")
        external = boundary / "external"
        external.mkdir()
        builder.require_vcs_independent_build_scratch(repository, external)
        builder.require_local_command(["git", "status", "--porcelain"])
        builder.require_local_command(["cargo", "package", "--workspace"])
        builder.require_local_command(["cargo", "semver-checks", "--version"])
        staged_lock = builder.staged_lock_path(ROOT, builder.load_toml(ROOT / builder.DESCRIPTOR))
        installed_stage = boundary / "installed-stage"
        installed_stage.mkdir()
        installed_hash = builder.install_staged_lock(
            ROOT, installed_stage, builder.load_toml(ROOT / builder.DESCRIPTOR)
        )
        if installed_hash != builder.sha256(staged_lock) or (
            installed_stage / "Cargo.lock"
        ).read_bytes() != staged_lock.read_bytes():
            raise AssertionError("installed staged lock identity differs")
        corrupted_stage = boundary / "corrupted-stage"
        corrupted_stage.mkdir()
        original_copyfile = builder.shutil.copyfile
        try:
            builder.shutil.copyfile = lambda _source, destination: Path(destination).write_bytes(
                b"copy drift"
            )
            try:
                builder.install_staged_lock(
                    ROOT, corrupted_stage, builder.load_toml(ROOT / builder.DESCRIPTOR)
                )
            except builder.CandidateError as error:
                if str(error) != "installed staged lock digest differs":
                    raise
            else:
                raise AssertionError("corrupted staged lock copy was accepted")
        finally:
            builder.shutil.copyfile = original_copyfile
        for forbidden_command in (
            ["git", "push"], ["cargo", "publish"], ["gh", "release", "create"],
            ["cargo", "semver-checks", "check-release"],
        ):
            try:
                builder.require_local_command(forbidden_command)
            except builder.CandidateError:
                pass
            else:
                raise AssertionError(f"remote-mutation command was accepted: {forbidden_command}")

        unsafe_archive = boundary / "package.crate"
        with tarfile.open(unsafe_archive, "w:gz") as archive:
            member = tarfile.TarInfo("../escape")
            payload = b"unsafe"
            member.size = len(payload)
            archive.addfile(member, io.BytesIO(payload))
        limits = {
            "max_archive_bytes": 1024 * 1024,
            "max_archive_members": 16,
            "max_expansion_ratio": 20,
        }
        try:
            builder.safe_members(unsafe_archive, "package", limits)
        except builder.CandidateError:
            pass
        else:
            raise AssertionError("path-traversing archive member was accepted")

        evidence = boundary / "evidence.json"
        evidence.write_text("{}", encoding="utf-8")
        builder.require_bounded_evidence(
            evidence, {"max_evidence_bytes": evidence.stat().st_size}
        )
        try:
            builder.require_bounded_evidence(evidence, {"max_evidence_bytes": 1})
        except builder.CandidateError:
            pass
        else:
            raise AssertionError("oversized evidence was accepted")

        package = {"name": "identus-did"}
        descriptor = {
            "version": "0.1.0-rc.1", "license": "Apache-2.0",
            "tools": {"cyclonedx_spec": "1.5"},
        }
        root_ref = "pkg:cargo/identus-did@0.1.0-rc.1"
        dependency_ref = "pkg:cargo/serde@1.0.0"
        sbom = {
            "bomFormat": "CycloneDX", "specVersion": "1.5",
            "metadata": {"component": {
                "name": "identus-did", "version": "0.1.0-rc.1", "bom-ref": root_ref,
                "licenses": [{"license": {"id": "Apache-2.0"}}],
            }},
            "components": [{"name": "serde", "version": "1.0.0", "bom-ref": dependency_ref}],
            "dependencies": [
                {"ref": root_ref, "dependsOn": [dependency_ref]},
                {"ref": dependency_ref, "dependsOn": []},
            ],
        }
        builder.validate_cyclonedx(sbom, package, descriptor)
        local = f"path+file:///tmp/random/workspace/crates/did#{root_ref}"
        normalized = builder.normalize_local_sbom_references(
            {"ref": local, "nested": [local, "registry+https://example.invalid#serde@1"]}
        )
        if normalized != {
            "ref": f"path+file:///candidate#{root_ref}",
            "nested": [
                f"path+file:///candidate#{root_ref}",
                "registry+https://example.invalid#serde@1",
            ],
        }:
            raise AssertionError("local CycloneDX references were not normalized deterministically")
        for mutation, expected in (
            (lambda value: value["metadata"]["component"].update(name="other"), "identity"),
            (lambda value: value["metadata"]["component"].update(licenses=[]), "license"),
            (lambda value: value["components"][0].update(**{"bom-ref": root_ref}), "duplicated"),
            (lambda value: value["components"][0].update(purl="git+https://example.invalid/repo"), "Git source"),
        ):
            mutated = json.loads(json.dumps(sbom))
            mutation(mutated)
            try:
                builder.validate_cyclonedx(mutated, package, descriptor)
            except builder.CandidateError as error:
                if expected not in str(error):
                    raise AssertionError(f"unexpected CycloneDX rejection: {error}") from error
            else:
                raise AssertionError(f"invalid CycloneDX evidence was accepted: {expected}")

        descriptor = builder.load_toml(ROOT / builder.DESCRIPTOR)
        revision = "b" * 40
        original_run = builder.run

        def aggregate_run(command, *, cwd, env):
            if command == ["git", "rev-parse", "HEAD"]:
                return revision + "\n"
            if len(command) == 3 and Path(command[1]).name == "check-release-candidates.py":
                return "release-candidates: test fixture passed\n"
            return original_run(command, cwd=cwd, env=env)

        builder.run = aggregate_run
        lane_paths: list[Path] = []
        lock_hash = "a" * 64
        host_triples = {
            "linux": "x86_64-unknown-linux-gnu", "macos": "aarch64-apple-darwin",
        }
        for host in descriptor["matrix_hosts"]:
            for toolchain_class in ("primary", "msrv"):
                version = descriptor["matrix"][f"{toolchain_class}_rust_version"]
                full_host = host | {"rust_triple": host_triples[host["name"]]}
                rows, unsupported = builder.expected_lane_rows(
                    descriptor, full_host, toolchain_class
                )
                lane = {
                    "schemaVersion": 1,
                    "candidate": descriptor["candidate"],
                    "sourceRevision": revision,
                    "sourceDirty": False,
                    "host": {
                        "name": host["name"], "nixSystem": host["nix_system"],
                        "rustTriple": full_host["rust_triple"],
                    },
                    "toolchain": {
                        "class": toolchain_class, "rustVersion": version,
                        "rustc": f"rustc {version} (test)",
                        "cargo": f"cargo {version} (test)",
                    },
                    "lock": {"file": "Cargo.lock", "sha256": lock_hash},
                    "rows": rows,
                    "unsupported": unsupported,
                    "limitations": [
                        "portable rows are compile-only and make no runtime or packaging claim",
                        "the HTTP resolver adapter is host-only",
                    ],
                    "elapsedSeconds": 1.0,
                }
                path = boundary / f"{host['name']}-{toolchain_class}.json"
                path.write_text(json.dumps(lane), encoding="utf-8")
                lane_paths.append(path)
        aggregate = boundary / "aggregate.json"
        builder.aggregate_matrix(ROOT, aggregate, revision, lane_paths)
        result = json.loads(aggregate.read_text(encoding="utf-8"))
        if result["status"] != "passed" or result["laneCount"] != 4:
            raise AssertionError("valid closed matrix did not aggregate")
        overclaim = json.loads(lane_paths[0].read_text(encoding="utf-8"))
        overclaim["rows"].append({
            "scope": "target", "package": "identus-did-resolver-http",
            "profile": "default", "operation": "check",
            "target": "wasm32-unknown-unknown", "tier": "compile-checked",
            "status": "passed", "command": ["cargo", "check"],
        })
        overclaim_path = boundary / "overclaim.json"
        overclaim_path.write_text(json.dumps(overclaim), encoding="utf-8")
        try:
            builder.aggregate_matrix(
                ROOT, boundary / "rejected-overclaim.json", revision,
                [overclaim_path, *lane_paths[1:]],
            )
        except builder.CandidateError as error:
            if "overclaim" not in str(error):
                raise AssertionError(f"unexpected matrix rejection: {error}") from error
        else:
            raise AssertionError("portable HTTP overclaim was accepted")
        builder.run = original_run

        fixture = Path(temporary) / "valid"
        copy_fixture(fixture)
        if errors := checker.validate(fixture):
            raise AssertionError(f"valid fixture failed: {errors!r}")
        cases = (
            (
                lambda root: append(root / "docs/release/release-trains.toml", "\nunexpected = true\n"),
                "train 1 fields differ",
            ),
            (
                lambda root: replace(
                    root / "docs/release/release-trains.toml",
                    'id              = "did"', 'id              = "crypto"',
                ),
                "duplicate train id",
            ),
            (
                lambda root: replace(
                    root / "docs/release/release-trains.toml",
                    'tag             = "identus-did-v0.1.0-rc.1"',
                    'tag             = "did-v0.1.0-rc.1"',
                ),
                "tag must equal identus-did-v0.1.0-rc.1",
            ),
            (
                lambda root: replace(
                    root / "docs/release/did-candidate.toml",
                    'publication              = "candidate-only"',
                    'publication              = "release-gated"',
                ),
                "DID descriptor differs: publication",
            ),
            (
                lambda root: replace(
                    root / "docs/release/did-candidate.toml",
                    'cargo_public_api    = "0.52.0"',
                    'cargo_public_api    = "0.53.0"',
                ),
                "DID descriptor evidence tools differ",
            ),
            (
                lambda root: replace(
                    root / "docs/release/did-candidate.toml",
                    'compatibility_status     = "not-applicable-first-candidate"',
                    'compatibility_status     = "compatible"',
                ),
                "DID descriptor differs: compatibility_status",
            ),
            (
                lambda root: (root / "docs/release/did-candidate.lock").unlink(),
                "DID staged lock must be a regular repository file",
            ),
            (
                lambda root: append(root / "docs/release/did-candidate.lock", "\n# drift\n"),
                "DID staged lock digest differs",
            ),
            (
                lambda root: replace_lock_and_rebind(
                    root,
                    'name = "identus-did"\nversion = "0.1.0-rc.1"',
                    'name = "identus-did"\nversion = "0.1.0-rc.2"',
                ),
                "DID staged lock candidate package identity differs",
            ),
            (
                lambda root: replace_lock_and_rebind(
                    root,
                    'source = "registry+https://github.com/rust-lang/crates.io-index"',
                    'source = "git+https://example.invalid/dependency"',
                ),
                "DID staged lock contains a non-crates.io source",
            ),
            (
                lambda root: replace_lock_and_rebind(
                    root,
                    'source = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "31b698c5f9a010f6573133b09e0de5408834d0c82f8d7475a89fc1867a71cd90"',
                    "",
                ),
                "DID staged lock registry package is missing source provenance",
            ),
            (
                lambda root: (
                    replace(
                        root / "docs/release/did-candidate.lock",
                        '[[package]]\nname = "identus-did"\nversion = "0.1.0-rc.1"',
                        '[[package]]\nname = "identus-did"\nversion = "0.1.0-rc.0"\n\n'
                        '[[package]]\nname = "identus-did"\nversion = "0.1.0-rc.1"',
                    ),
                    rebind_lock_digest(root),
                ),
                "DID staged lock contains a duplicate candidate package name",
            ),
            (
                lambda root: replace(
                    root / "docs/release/did-candidate.toml",
                    'primary_rust_version = "1.98.1"',
                    'primary_rust_version = "1.99.0"',
                ),
                "DID descriptor compiler matrix differs",
            ),
            (
                lambda root: replace(
                    root / "docs/release/did-candidate.toml",
                    'packages             = [ "identus-did" ]',
                    'packages             = [ "identus-did", "identus-did-resolver-http" ]',
                ),
                "overclaims portable packages",
            ),
            (
                lambda root: replace(
                    root / "docs/release/did-candidate.toml",
                    'runner_host          = "linux"', 'runner_host          = "macos"',
                ),
                "DID descriptor portable target matrix differs",
            ),
            (
                lambda root: append(
                    root / ".github/workflows/nix-checks.yml", "\npull_request:\n",
                ),
                "slow workflow must not gain a pull_request trigger",
            ),
            (
                lambda root: replace(
                    root / ".github/workflows/nix-checks.yml",
                    '--output "$RUNNER_TEMP/did-matrix/primary"',
                    "--output artifacts/did-matrix/primary",
                ),
                "slow workflow writes DID matrix evidence inside the checkout",
            ),
            (
                lambda root: replace(
                    root / ".github/workflows/nix-checks.yml",
                    "git status --porcelain --untracked-files=all",
                    "git diff --quiet",
                ),
                "slow workflow is missing DID matrix contract: git status --porcelain",
            ),
            (
                move_cleanliness_step_after_msrv,
                "slow workflow must verify checkout cleanliness between DID lanes",
            ),
            (
                lambda root: replace(
                    root / ".github/workflows/nix-checks.yml",
                    "printf '%s\\n' \"$dirty_paths\"",
                    ': "$dirty_paths"',
                ),
                "slow workflow is missing DID matrix contract: printf",
            ),
            (
                lambda root: replace(
                    root / ".github/workflows/nix-checks.yml",
                    "exit 1",
                    "true",
                ),
                "slow workflow is missing DID matrix contract: exit 1",
            ),
            (
                lambda root: replace(
                    root / ".github/workflows/nix-checks.yml",
                    "path: ${{ runner.temp }}/did-matrix",
                    "path: artifacts/did-matrix",
                ),
                "slow workflow is missing DID matrix contract: path: ${{ runner.temp }}/did-matrix",
            ),
            (
                lambda root: replace(
                    root / "docs/release/did-candidate.toml",
                    'api_baseline           = "docs/release/identus-did-0.1.0-rc.1.api.txt"',
                    'api_baseline           = "docs/release/missing.api.txt"',
                ),
                "DID package API baseline path differs: identus-did",
            ),
            (
                lambda root: replace(
                    root / "docs/release/did-candidate.toml",
                    'path                   = "crates/did"', 'path                   = "crates/did2"',
                ),
                "DID package path differs",
            ),
            (
                lambda root: replace(
                    root / "docs/release/did-candidate.toml",
                    'published_dependencies = [ "identus-core", "identus-derive" ]',
                    'published_dependencies = [ "identus-core" ]',
                ),
                "DID package dependency partition differs",
            ),
            (
                lambda root: replace(
                    root / "docs/release/did-candidate.toml",
                    'features               = [ "openapi" ]', 'features               = [  ]',
                ),
                "DID package feature declaration differs",
            ),
            (
                lambda root: replace(
                    root / "crates/did/Cargo.toml",
                    "version.workspace      = true", 'version                = "0.1.0-rc.1"',
                ),
                "candidate-only package overrides workspace version",
            ),
            (
                lambda root: replace(
                    root / "crates/did-resolver-http/Cargo.toml",
                    "publish.workspace      = true", 'publish                = [ "crates-io" ]',
                ),
                "candidate-only package overrides publication denial",
            ),
            (
                lambda root: replace(
                    root / "Cargo.toml",
                    'identus-did                = { path = "crates/did" }',
                    'identus-did                = { path = "crates/did", version = "=0.1.0-rc.1" }',
                ),
                "candidate-only workspace dependency has a release version",
            ),
            (
                lambda root: append(
                    root / "scripts/prepare-did-candidate.py", "\n# cargo publish\n",
                ),
                "candidate-only builder contains remote mutation capability",
            ),
            (
                lambda root: replace(
                    root / "scripts/prepare-did-candidate.py",
                    "lock_hash = install_staged_lock(root, stage, descriptor)",
                    'run(["cargo", "generate-lockfile", "--manifest-path", '
                    'str(stage / "Cargo.toml")], cwd=stage, env=env)',
                ),
                "DID candidate lock generation must stay inside extracted closure verification",
            ),
            (
                lambda root: replace(
                    root / "docs/release/crypto-candidate.toml",
                    'release_tag              = "v0.1.0-rc.1"',
                    'release_tag              = "identus-crypto-v0.1.0-rc.1"',
                ),
                "immutable crypto descriptor tag differs",
            ),
        )
        for position, (mutation, expected) in enumerate(cases):
            case = Path(temporary) / f"case-{position}"
            shutil.copytree(fixture, case)
            require_rejection(checker, case, mutation, expected)
    print("release-candidates test: mutation cases passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
