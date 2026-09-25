#!/usr/bin/env python3
"""Mutation tests for independent release-candidate train policy."""

from __future__ import annotations

import importlib.util
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
        "docs/release/identus-did-0.1.0-rc.1.api.txt",
        "docs/release/identus-did-resolver-http-0.1.0-rc.1.api.txt",
        "docs/release/crypto-candidate.toml",
        "docs/adr/0153-use-primary-package-tags-for-independent-release-trains.md",
        "docs/adr/0154-use-first-candidate-api-snapshots-as-semver-origin.md",
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
