#!/usr/bin/env python3
"""Verify and, when explicitly authorized, publish one SDK-Rust crate train."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


PACKAGE_ORDER = ("identus-derive", "identus-core", "identus-crypto")
VERSION = "0.1.0-rc.1"
RELEASE_TAG = "crypto-v0.1.0-rc.1"
REPOSITORY = "hyperledger-identus/sdk-rust"
AUTHENTICATION_CLASSES = ("bootstrap-token", "trusted-publishing")
FULL_SHA = re.compile(r"^[0-9a-f]{40}$")
USER_AGENT = "hyperledger-identus-sdk-rust-release/0.1"


class ReleaseError(RuntimeError):
    pass


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def run(command: list[str], *, cwd: Path, env: dict[str, str]) -> str:
    result = subprocess.run(
        command,
        cwd=cwd,
        env=env,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    if result.returncode != 0:
        raise ReleaseError(
            f"command failed ({result.returncode}): {' '.join(command)}\n{result.stdout}"
        )
    return result.stdout


def read_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ReleaseError(f"cannot read JSON {path}: {error}") from error
    if not isinstance(value, dict):
        raise ReleaseError(f"JSON document must be an object: {path}")
    return value


def registry_version(name: str, version: str) -> dict[str, Any] | None:
    url = f"https://crates.io/api/v1/crates/{name}/{version}"
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            value = json.load(response)
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return None
        raise ReleaseError(f"crates.io lookup failed for {name}: HTTP {error.code}") from error
    except (OSError, json.JSONDecodeError) as error:
        raise ReleaseError(f"crates.io lookup failed for {name}: {error}") from error
    version_data = value.get("version") if isinstance(value, dict) else None
    if not isinstance(version_data, dict):
        raise ReleaseError(f"crates.io returned malformed version data for {name}")
    return version_data


def require_candidate(
    evidence: Path, expected_sha: str, release_tag: str
) -> tuple[dict[str, Any], Path, dict[str, str]]:
    if not FULL_SHA.fullmatch(expected_sha):
        raise ReleaseError("expected source revision must be a full lowercase Git SHA")
    if release_tag != RELEASE_TAG:
        raise ReleaseError(f"release tag must equal {RELEASE_TAG}")
    receipt = read_json(evidence / "receipt.json")
    expected_fields: dict[str, object] = {
        "schemaVersion": 1,
        "candidate": "identus-crypto-0.1.0-rc.1",
        "version": VERSION,
        "sourceRevision": expected_sha,
        "sourceDirty": False,
        "publication": "release-gated",
        "releaseTag": RELEASE_TAG,
        "releaseWorkflow": ".github/workflows/publish-crates.yml",
        "releaseEnvironment": "crates-io",
        "authenticationModes": list(AUTHENTICATION_CLASSES),
        "verification": "release-eligible-archive-closure",
        "declaredMsrv": "1.89.0",
    }
    for field, expected in expected_fields.items():
        if receipt.get(field) != expected:
            raise ReleaseError(f"candidate receipt differs for {field}")

    packages = receipt.get("packages")
    if not isinstance(packages, list):
        raise ReleaseError("candidate receipt packages must be an array")
    names = tuple(row.get("name") for row in packages if isinstance(row, dict))
    if names != PACKAGE_ORDER:
        raise ReleaseError("candidate receipt package scope/order differs")
    digests: dict[str, str] = {}
    for row in packages:
        if not isinstance(row, dict):
            raise ReleaseError("candidate package receipt must be an object")
        name = row["name"]
        if row.get("version") != VERSION:
            raise ReleaseError(f"candidate version differs for {name}")
        archive = evidence / "packages" / f"{name}-{VERSION}.crate"
        if not archive.is_file() or archive.is_symlink():
            raise ReleaseError(f"candidate archive is missing or unsafe: {archive}")
        actual = sha256(archive)
        if row.get("sha256") != actual:
            raise ReleaseError(f"candidate archive checksum differs for {name}")
        digests[name] = actual

    workspace = evidence / "publication" / "workspace"
    if not (workspace / "Cargo.toml").is_file() or not (workspace / "Cargo.lock").is_file():
        raise ReleaseError("candidate publication workspace is incomplete")
    return receipt, workspace, digests


def verify_repackaged(
    workspace: Path, digests: dict[str, str], env: dict[str, str]
) -> None:
    with tempfile.TemporaryDirectory(prefix="sdk-rust-release-package-") as temporary:
        target = Path(temporary)
        run(
            [
                "cargo",
                "package",
                "--workspace",
                "--locked",
                "--no-verify",
                "--allow-dirty",
                "--manifest-path",
                str(workspace / "Cargo.toml"),
                "--target-dir",
                str(target),
            ],
            cwd=workspace,
            env=env,
        )
        for name in PACKAGE_ORDER:
            archive = target / "package" / f"{name}-{VERSION}.crate"
            if not archive.is_file() or sha256(archive) != digests[name]:
                raise ReleaseError(f"publication workspace does not reproduce {name}")


def stage_publication_workspace(source: Path, destination: Path) -> Path:
    """Copy reviewed source into a VCS-independent Cargo publication workspace."""
    if source.is_symlink() or not source.is_dir():
        raise ReleaseError("publication workspace source is missing or unsafe")
    if destination.exists():
        raise ReleaseError("publication staging destination already exists")
    for ancestor in (destination.resolve(), *destination.resolve().parents):
        marker = ancestor / ".git"
        try:
            marker.lstat()
        except FileNotFoundError:
            continue
        except OSError as error:
            raise ReleaseError("cannot validate publication VCS boundary") from error
        raise ReleaseError("publication staging must be outside every Git worktree")
    for path in source.rglob("*"):
        if path.is_symlink():
            raise ReleaseError("publication workspace contains a symlink")
    shutil.copytree(source, destination)
    return destination


def wait_for_registry(name: str, expected: str) -> dict[str, Any]:
    for attempt in range(30):
        value = registry_version(name, VERSION)
        if value is not None:
            checksum = value.get("checksum")
            if checksum != expected:
                raise ReleaseError(
                    f"published registry checksum differs for {name}: {checksum}"
                )
            return value
        if attempt < 29:
            time.sleep(10)
    raise ReleaseError(f"published version did not appear in time: {name} {VERSION}")


def publish(
    workspace: Path,
    digests: dict[str, str],
    authentication_class: str,
    env: dict[str, str],
    results: list[dict[str, Any]],
) -> None:
    if os.environ.get("GITHUB_ACTIONS") != "true":
        raise ReleaseError("live publication is restricted to GitHub Actions")
    if os.environ.get("GITHUB_REPOSITORY") != REPOSITORY:
        raise ReleaseError("live publication repository identity differs")
    token = env.get("CARGO_REGISTRY_TOKEN", "")
    if not token:
        raise ReleaseError("selected registry credential is unavailable")

    observed = {name: registry_version(name, VERSION) for name in PACKAGE_ORDER}
    if authentication_class == "bootstrap-token" and all(observed.values()):
        raise ReleaseError("bootstrap mode is forbidden after the complete version exists")
    if authentication_class == "trusted-publishing":
        for name in PACKAGE_ORDER:
            base_url = f"https://crates.io/api/v1/crates/{name}"
            request = urllib.request.Request(base_url, headers={"User-Agent": USER_AGENT})
            try:
                urllib.request.urlopen(request, timeout=30).close()
            except urllib.error.HTTPError as error:
                if error.code == 404:
                    raise ReleaseError(
                        f"trusted publishing cannot create missing namespace: {name}"
                    ) from error
                raise

    for name in PACKAGE_ORDER:
        existing = observed[name]
        if existing is not None:
            if existing.get("checksum") != digests[name]:
                raise ReleaseError(f"existing version checksum differs for {name}")
            results.append(
                {
                    "name": name,
                    "version": VERSION,
                    "result": "already-published",
                    "sha256": digests[name],
                    "url": f"https://crates.io/crates/{name}/{VERSION}",
                }
            )
            continue
        run(
            [
                "cargo",
                "publish",
                "--locked",
                "--manifest-path",
                str(workspace / "Cargo.toml"),
                "--package",
                name,
                "--registry",
                "crates-io",
            ],
            cwd=workspace,
            env=env,
        )
        version = wait_for_registry(name, digests[name])
        results.append(
            {
                "name": name,
                "version": VERSION,
                "result": "published",
                "sha256": digests[name],
                "registryChecksum": version.get("checksum"),
                "url": f"https://crates.io/crates/{name}/{VERSION}",
            }
        )


def write_receipt(path: Path, receipt: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--evidence", required=True)
    parser.add_argument("--expected-sha", required=True)
    parser.add_argument("--release-tag", required=True)
    parser.add_argument("--auth-class", choices=AUTHENTICATION_CLASSES, required=True)
    parser.add_argument("--receipt-output", required=True)
    parser.add_argument("--publish", action="store_true")
    args = parser.parse_args()

    evidence = Path(args.evidence).resolve()
    output = Path(args.receipt_output).resolve()
    results: list[dict[str, Any]] = []
    release_receipt: dict[str, Any] = {
        "schemaVersion": 1,
        "repository": REPOSITORY,
        "sourceRevision": args.expected_sha,
        "releaseTag": args.release_tag,
        "version": VERSION,
        "authenticationClass": args.auth_class,
        "mode": "publish" if args.publish else "verify-only",
        "workflowRun": os.environ.get("GITHUB_RUN_ID"),
        "workflowAttempt": os.environ.get("GITHUB_RUN_ATTEMPT"),
        "workflowRunUrl": (
            f"https://github.com/{REPOSITORY}/actions/runs/{os.environ['GITHUB_RUN_ID']}"
            if os.environ.get("GITHUB_RUN_ID")
            else None
        ),
        "startedAt": datetime.now(timezone.utc).isoformat(),
        "packages": results,
        "status": "running",
    }
    try:
        _, evidence_workspace, digests = require_candidate(
            evidence, args.expected_sha, args.release_tag
        )
        env = os.environ.copy()
        env.update({"SOURCE_DATE_EPOCH": "1", "CARGO_TERM_COLOR": "never"})
        with tempfile.TemporaryDirectory(
            prefix=".identus-release-publication-"
        ) as temporary:
            workspace = stage_publication_workspace(
                evidence_workspace, Path(temporary) / "workspace"
            )
            verify_repackaged(workspace, digests, env)
            if args.publish:
                publish(workspace, digests, args.auth_class, env, results)
                release_receipt["status"] = "published"
            else:
                release_receipt["packages"] = [
                    {
                        "name": name,
                        "version": VERSION,
                        "result": "verified",
                        "sha256": digests[name],
                    }
                    for name in PACKAGE_ORDER
                ]
                release_receipt["status"] = "verified"
    except (KeyError, OSError, ReleaseError) as error:
        release_receipt["status"] = "failed"
        release_receipt["error"] = str(error)
        release_receipt["finishedAt"] = datetime.now(timezone.utc).isoformat()
        write_receipt(output, release_receipt)
        print(f"release-train: {error}", file=sys.stderr)
        return 1
    release_receipt["finishedAt"] = datetime.now(timezone.utc).isoformat()
    write_receipt(output, release_receipt)
    print(f"release-train: {release_receipt['status']}; receipt written to {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
