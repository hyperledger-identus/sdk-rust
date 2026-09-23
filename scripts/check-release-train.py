#!/usr/bin/env python3
"""Validate the protected three-crate release-train contract without network."""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path
from typing import Any


VERSION = "0.1.0-rc.1"
PACKAGE_ORDER = ("identus-derive", "identus-core", "identus-crypto")
PACKAGE_PATHS = {
    "identus-derive": Path("crates/derive"),
    "identus-core": Path("crates/core"),
    "identus-crypto": Path("crates/crypto"),
}
WORKFLOW = Path(".github/workflows/publish-crates.yml")
PUBLISHER = Path("scripts/publish-release-train.py")
DESCRIPTOR = Path("docs/release/crypto-candidate.toml")
ADR = Path("docs/adr/0134-activate-protected-crates-io-release-trains.md")
RELEASING = Path("RELEASING.md")
RELEASE_TAG = "v0.1.0-rc.1"
SUPERSEDED_RELEASE_TAG = "crypto-v0.1.0-rc.1"


def load_toml(path: Path, errors: list[str]) -> dict[str, Any]:
    try:
        with path.open("rb") as source:
            value = tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        errors.append(f"cannot read TOML {path}: {error}")
        return {}
    return value


def read(path: Path, errors: list[str]) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as error:
        errors.append(f"cannot read {path}: {error}")
        return ""


def validate(root: Path) -> list[str]:
    errors: list[str] = []
    workspace = load_toml(root / "Cargo.toml", errors)
    package_defaults = workspace.get("workspace", {}).get("package", {})
    if package_defaults.get("version") != "0.0.0":
        errors.append("unrelated workspace version default must remain 0.0.0")
    if package_defaults.get("publish") is not False:
        errors.append("unrelated workspace publication default must remain false")
    dependencies = workspace.get("workspace", {}).get("dependencies", {})
    for name in PACKAGE_ORDER:
        dependency = dependencies.get(name)
        if not isinstance(dependency, dict) or dependency.get("version") != f"={VERSION}":
            errors.append(f"workspace dependency must use exact release version: {name}")
        expected_path = PACKAGE_PATHS[name].as_posix()
        if not isinstance(dependency, dict) or dependency.get("path") != expected_path:
            errors.append(f"workspace dependency path differs: {name}")

    all_manifests = sorted((root / "crates").glob("*/Cargo.toml"))
    selected_paths = {root / path / "Cargo.toml" for path in PACKAGE_PATHS.values()}
    for path in all_manifests:
        manifest = load_toml(path, errors)
        package = manifest.get("package", {})
        name = package.get("name") if isinstance(package, dict) else None
        if path in selected_paths:
            if name not in PACKAGE_ORDER:
                errors.append(f"selected package identity differs: {path}")
                continue
            if package.get("version") != VERSION:
                errors.append(f"selected package version differs: {name}")
            if package.get("publish") != ["crates-io"]:
                errors.append(f"selected package registry permission differs: {name}")
            for field in (
                "description",
                "repository",
                "homepage",
                "documentation",
                "readme",
                "keywords",
                "categories",
            ):
                if not package.get(field):
                    errors.append(f"selected package metadata missing {field}: {name}")
        else:
            if package.get("version") != {"workspace": True}:
                errors.append(f"unrelated package overrides workspace version: {name}")
            if package.get("publish") != {"workspace": True}:
                errors.append(f"unrelated package overrides publication denial: {name}")

    descriptor = load_toml(root / DESCRIPTOR, errors)
    descriptor_expectations = {
        "version": VERSION,
        "publication": "release-gated",
        "release_tag": RELEASE_TAG,
        "release_workflow": WORKFLOW.as_posix(),
        "release_environment": "crates-io",
        "authentication_modes": ["bootstrap-token", "trusted-publishing"],
    }
    for field, expected in descriptor_expectations.items():
        if descriptor.get(field) != expected:
            errors.append(f"release descriptor differs: {field}")

    workflow = read(root / WORKFLOW, errors)
    required_workflow = (
        "workflow_dispatch:",
        "environment: crates-io",
        "can_admins_bypass",
        "prevent_self_review",
        "Verify signed tag identity through GitHub",
        "Rebind publish checkout to approved identity",
        "git merge-base --is-ancestor",
        "if: ${{ always() }}",
        "overwrite: true",
        "secrets.CARGO_PUBLISH",
        "inputs.authentication == 'bootstrap-token'",
        "inputs.authentication == 'trusted-publishing'",
        "rust-lang/crates-io-auth-action@c6f97d42243bad5fab37ca0427f495c86d5b1a18",
        "actions/attest-build-provenance@4d101475d8b20a2381f78447822ac1eab6504dd8",
        "scripts/publish-release-train.py",
        "gh release create",
        "gh release view",
        "expected_digest=\"sha256:",
    )
    for phrase in required_workflow:
        if phrase not in workflow:
            errors.append(f"release workflow is missing protected contract: {phrase}")
    trigger_prefix = workflow.split("concurrency:", 1)[0]
    for forbidden in ("pull_request:", "push:", "release:"):
        if forbidden in trigger_prefix:
            errors.append(f"release workflow has forbidden automatic trigger: {forbidden}")
    if workflow.count("secrets.CARGO_PUBLISH") != 1:
        errors.append("bootstrap secret must occur exactly once in the release workflow")
    if workflow.count("if: ${{ always() }}") != 2:
        errors.append("verify and publish receipts must both survive failed steps")
    candidate_name = "crates-io-candidate-${{ inputs.expected_sha }}"
    if workflow.count(candidate_name) != 2:
        errors.append("candidate artifact identity must be stable across attempts")
    for line in workflow.splitlines():
        if "crates-io-candidate-" in line and "github.run_attempt" in line:
            errors.append("candidate artifact identity must not include run_attempt")
    publish_job = workflow.split("  publish:", 1)[-1]
    if publish_job.find("Rebind publish checkout to approved identity") > publish_job.find(
        "Authenticate with crates.io trusted publishing"
    ):
        errors.append("publish identity must be rebound before registry authentication")
    for action in re.findall(r"uses:\s*([^\s#]+)", workflow):
        if not re.search(r"@[0-9a-f]{40}$", action):
            errors.append(f"release workflow action is not immutable: {action}")

    publisher = read(root / PUBLISHER, errors)
    required_publisher = (
        'PACKAGE_ORDER = ("identus-derive", "identus-core", "identus-crypto")',
        'VERSION = "0.1.0-rc.1"',
        f'RELEASE_TAG = "{RELEASE_TAG}"',
        'if os.environ.get("GITHUB_ACTIONS") != "true":',
        'if os.environ.get("GITHUB_REPOSITORY") != REPOSITORY:',
        'env.get("CARGO_REGISTRY_TOKEN", "")',
        '"cargo",\n                "publish"',
        'if authentication_class == "bootstrap-token" and all(observed.values()):',
        'if authentication_class == "trusted-publishing":',
        "def stage_publication_workspace(",
        'prefix=".identus-release-publication-"',
        'raise ReleaseError("publication staging must be outside every Git worktree")',
        '"status"] = "failed"',
    )
    for phrase in required_publisher:
        if phrase not in publisher:
            errors.append(f"publisher is missing fail-closed contract: {phrase}")
    if "--no-verify" not in publisher:
        errors.append("publisher must reproduce reviewed archives before live Cargo verification")
    publish_block = publisher.split("def publish(", 1)[-1]
    if '"--no-verify"' in publish_block.split("def write_receipt", 1)[0]:
        errors.append("live cargo publish must not use --no-verify")

    adr = read(root / ADR, errors)
    for phrase in (
        "signed immutable tag",
        "protected environment",
        "self-review prevention",
        "CARGO_PUBLISH",
        "official crates.io OIDC action",
        "never\nretag or overwrite",
    ):
        if phrase not in adr:
            errors.append(f"release ADR is missing protected decision: {phrase}")
    releasing = read(root / RELEASING, errors)
    for phrase in (
        f"git tag -s {RELEASE_TAG}",
        "publish-crates.yml",
        "crates-io",
        "bootstrap-token",
        "trusted-publishing",
        "identus-derive",
        "identus-core",
        "identus-crypto",
    ):
        if phrase not in releasing:
            errors.append(f"release runbook is missing exact operation: {phrase}")
    for label, text in (
        ("release runbook", releasing),
        ("release workflow", workflow),
        ("release descriptor", read(root / DESCRIPTOR, errors)),
        ("publisher", publisher),
    ):
        if SUPERSEDED_RELEASE_TAG in text:
            errors.append(
                f"{label} still names the superseded release tag: {SUPERSEDED_RELEASE_TAG}"
            )
    return errors


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    errors = validate(root)
    if errors:
        for error in errors:
            print(f"release-train: {error}", file=sys.stderr)
        print(f"release-train: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    print("release-train: protected three-package contract passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
