#!/usr/bin/env python3
"""Fail closed when native weekly slow evidence is missing, stale or unhealthy."""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
FACTORY_POLICY = ROOT / ".factory-policy.json"
SUPPORT_POLICY = ROOT / "docs/architecture/sdk-support-policy.toml"
SNAPSHOT_FIELDS = {
    "schemaVersion",
    "repository",
    "checkedAt",
    "defaultBranch",
    "runs",
}
RUN_FIELDS = {
    "databaseId",
    "attempt",
    "event",
    "status",
    "conclusion",
    "headBranch",
    "headSha",
    "createdAt",
    "url",
    "workflowName",
}
MAX_SNAPSHOT_BYTES = 65_536
MAX_RESPONSE_BYTES = 131_072
TIMEOUT_SECONDS = 30


class AuditError(Exception):
    pass


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Audit the latest native scheduled slow workflow"
    )
    parser.add_argument(
        "--snapshot", type=Path, help="strict snapshot for hermetic tests or replay"
    )
    return parser.parse_args()


def parse_utc(value: Any, context: str) -> datetime:
    if not isinstance(value, str) or not value.endswith("Z"):
        raise AuditError(f"{context} must be a UTC timestamp ending in Z")
    try:
        parsed = datetime.fromisoformat(value.removesuffix("Z") + "+00:00")
    except ValueError as error:
        raise AuditError(f"{context} is not a valid timestamp") from error
    if parsed.tzinfo is None:
        raise AuditError(f"{context} must carry timezone information")
    return parsed.astimezone(timezone.utc)


def load_repository() -> str:
    try:
        policy = json.loads(FACTORY_POLICY.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise AuditError(f"cannot read tracked factory policy: {error}") from error
    repository = policy.get("repository") if isinstance(policy, dict) else None
    if not isinstance(repository, str) or repository.count("/") != 1:
        raise AuditError("tracked factory repository is invalid")
    return repository


def freshness_hours() -> int:
    try:
        for line in SUPPORT_POLICY.read_text(encoding="utf-8").splitlines():
            if line.strip().startswith("slow_freshness_hours"):
                return int(line.split("=", 1)[1].strip())
    except (OSError, ValueError) as error:
        raise AuditError(f"cannot read slow freshness policy: {error}") from error
    raise AuditError("slow freshness policy is missing")


def validate_run(value: Any, repository: str, index: int) -> dict[str, Any]:
    if not isinstance(value, dict) or set(value) != RUN_FIELDS:
        raise AuditError(f"run {index} must contain exactly the documented fields")
    run_id = value["databaseId"]
    if isinstance(run_id, bool) or not isinstance(run_id, int) or run_id < 1:
        raise AuditError(f"run {index} databaseId must be a positive integer")
    attempt = value["attempt"]
    if isinstance(attempt, bool) or not isinstance(attempt, int) or attempt < 1:
        raise AuditError(f"run {run_id} attempt must be a positive integer")
    if value["event"] != "schedule":
        raise AuditError(f"run {run_id} event must be schedule")
    if value["status"] not in {
        "queued",
        "in_progress",
        "completed",
        "requested",
        "waiting",
        "pending",
    }:
        raise AuditError(f"run {run_id} has an unsupported status")
    conclusion = value["conclusion"]
    if conclusion is not None and not isinstance(conclusion, str):
        raise AuditError(f"run {run_id} conclusion must be a string or null")
    if value["headBranch"] != "develop":
        raise AuditError(
            f"run {run_id} headBranch must be protected develop"
        )
    sha = value["headSha"]
    if not isinstance(sha, str) or len(sha) != 40 or any(
        character not in "0123456789abcdef" for character in sha
    ):
        raise AuditError(f"run {run_id} headSha must be an exact lowercase SHA")
    parse_utc(value["createdAt"], f"run {run_id} createdAt")
    expected_url = f"https://github.com/{repository}/actions/runs/{run_id}"
    if value["url"] != expected_url:
        raise AuditError(f"run {run_id} url does not match repository and identity")
    if value["workflowName"] != "slow":
        raise AuditError(f"run {run_id} workflowName must be slow")
    return value


def validate_snapshot(document: Any, repository: str) -> dict[str, Any]:
    if not isinstance(document, dict) or set(document) != SNAPSHOT_FIELDS:
        raise AuditError("snapshot must contain exactly the documented fields")
    if document["schemaVersion"] != 1:
        raise AuditError("snapshot schemaVersion must be 1")
    if document["repository"] != repository:
        raise AuditError("snapshot repository does not match tracked factory policy")
    parse_utc(document["checkedAt"], "snapshot checkedAt")
    if not isinstance(document["defaultBranch"], str):
        raise AuditError("snapshot defaultBranch must be a string")
    runs = document["runs"]
    if not isinstance(runs, list) or len(runs) > 10:
        raise AuditError("snapshot runs must be a bounded array")
    for index, run in enumerate(runs):
        validate_run(run, repository, index)
    return document


def load_snapshot(path: Path, repository: str) -> dict[str, Any]:
    if path.is_symlink():
        raise AuditError("snapshot must not be a symbolic link")
    try:
        if path.stat().st_size > MAX_SNAPSHOT_BYTES:
            raise AuditError(f"snapshot exceeds {MAX_SNAPSHOT_BYTES} bytes")
        document = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise AuditError(f"cannot parse snapshot: {error}") from error
    return validate_snapshot(document, repository)


def gh_json(arguments: list[str], context: str) -> Any:
    gh = shutil.which("gh")
    if gh is None:
        raise AuditError("GitHub CLI is unavailable")
    try:
        result = subprocess.run(
            [gh, *arguments],
            check=False,
            capture_output=True,
            text=True,
            timeout=TIMEOUT_SECONDS,
        )
    except subprocess.TimeoutExpired as error:
        raise AuditError(f"GitHub {context} timed out") from error
    if result.returncode != 0:
        raise AuditError(f"GitHub {context} failed (exit {result.returncode})")
    if len(result.stdout.encode("utf-8")) > MAX_RESPONSE_BYTES:
        raise AuditError(f"GitHub {context} response exceeds bound")
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as error:
        raise AuditError(f"GitHub {context} returned malformed JSON") from error


def query_github(repository: str) -> dict[str, Any]:
    repository_data = gh_json(
        ["repo", "view", repository, "--json", "nameWithOwner,defaultBranchRef"],
        "repository lookup",
    )
    if not isinstance(repository_data, dict) or set(repository_data) != {
        "nameWithOwner",
        "defaultBranchRef",
    }:
        raise AuditError("GitHub repository response has unexpected fields")
    if repository_data["nameWithOwner"] != repository:
        raise AuditError("GitHub repository identity does not match policy")
    default_ref = repository_data["defaultBranchRef"]
    if not isinstance(default_ref, dict) or set(default_ref) != {"name"}:
        raise AuditError("GitHub default branch response is malformed")
    checked_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace(
        "+00:00", "Z"
    )
    if default_ref["name"] != "develop":
        return validate_snapshot(
            {
                "schemaVersion": 1,
                "repository": repository,
                "checkedAt": checked_at,
                "defaultBranch": default_ref["name"],
                "runs": [],
            },
            repository,
        )
    runs = gh_json(
        [
            "run",
            "list",
            "--repo",
            repository,
            "--workflow",
            "nix-checks.yml",
            "--event",
            "schedule",
            "--all",
            "--limit",
            "10",
            "--json",
            "databaseId,attempt,event,status,conclusion,headBranch,headSha,createdAt,url,workflowName",
        ],
        "slow-run lookup",
    )
    document = {
        "schemaVersion": 1,
        "repository": repository,
        "checkedAt": checked_at,
        "defaultBranch": default_ref["name"],
        "runs": runs,
    }
    return validate_snapshot(document, repository)


def audit(document: dict[str, Any], maximum_age_hours: int) -> dict[str, Any]:
    if document["defaultBranch"] != "develop":
        raise AuditError(
            f"default branch is {document['defaultBranch']!r}; expected protected develop"
        )
    runs = document["runs"]
    if not runs:
        raise AuditError("no native scheduled slow run exists")
    latest = max(runs, key=lambda run: parse_utc(run["createdAt"], "run createdAt"))
    run_id = latest["databaseId"]
    if latest["attempt"] != 1:
        raise AuditError(
            f"latest scheduled run {run_id} is rerun attempt {latest['attempt']}; "
            f"it is not natural schedule evidence: {latest['url']}"
        )
    if latest["status"] != "completed" or latest["conclusion"] != "success":
        raise AuditError(
            f"latest scheduled run {run_id} is "
            f"{latest['status']}/{latest['conclusion']}: {latest['url']}"
        )
    checked_at = parse_utc(document["checkedAt"], "snapshot checkedAt")
    created_at = parse_utc(latest["createdAt"], f"run {run_id} createdAt")
    if created_at > checked_at + timedelta(minutes=5):
        raise AuditError(f"latest scheduled run {run_id} is future-dated")
    age = checked_at - created_at
    if age > timedelta(hours=maximum_age_hours):
        raise AuditError(
            f"latest scheduled run {run_id} is stale ({int(age.total_seconds() // 3600)}h): "
            f"{latest['url']}"
        )
    return latest


def main() -> int:
    arguments = parse_arguments()
    try:
        repository = load_repository()
        document = (
            load_snapshot(arguments.snapshot, repository)
            if arguments.snapshot is not None
            else query_github(repository)
        )
        latest = audit(document, freshness_hours())
    except AuditError as error:
        print(f"weekly-slow-live: {error}", file=sys.stderr)
        return 1
    mode = "snapshot" if arguments.snapshot is not None else "live"
    print(
        "weekly-slow-live: latest scheduled slow run "
        f"{latest['databaseId']} passed at {latest['headSha']} ({mode})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
