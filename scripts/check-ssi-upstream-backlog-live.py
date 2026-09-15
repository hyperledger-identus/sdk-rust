#!/usr/bin/env python3

import argparse
import csv
import json
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Any


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_BACKLOG = REPOSITORY_ROOT / "docs/roadmap/ssi-upstream-dependency-backlog.csv"
OFFLINE_CHECKER = REPOSITORY_ROOT / "scripts/check-ssi-upstream-backlog.py"
FACTORY_POLICY = REPOSITORY_ROOT / ".factory-policy.json"
SNAPSHOT_FIELDS = {"schemaVersion", "repository", "issues"}
ISSUE_FIELDS = {"number", "state", "url"}
ISSUE_STATES = {"OPEN", "CLOSED"}
MAX_SNAPSHOT_BYTES = 65_536
MAX_GITHUB_RESPONSE_BYTES = 16_384
GITHUB_TIMEOUT_SECONDS = 30


class AuditError(Exception):
    pass


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Audit SSI backlog issue ownership against live GitHub state"
    )
    parser.add_argument(
        "--snapshot",
        type=Path,
        help="strict issue-state snapshot for hermetic test or incident replay",
    )
    return parser.parse_args()


def load_repository() -> str:
    try:
        document = json.loads(FACTORY_POLICY.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise AuditError(f"cannot read tracked factory policy: {error}") from error
    repository = document.get("repository") if isinstance(document, dict) else None
    if not isinstance(repository, str) or repository.count("/") != 1:
        raise AuditError("tracked factory repository is invalid")
    return repository


def validate_offline(backlog: Path) -> None:
    result = subprocess.run(
        [sys.executable, str(OFFLINE_CHECKER), str(backlog)],
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        detail = result.stderr.strip().splitlines()
        suffix = f": {detail[0]}" if detail else ""
        raise AuditError(f"offline backlog validation failed{suffix}")


def load_rows(backlog: Path) -> list[dict[str, str]]:
    try:
        with backlog.open(encoding="utf-8", newline="") as source:
            return list(csv.DictReader(source, strict=True))
    except (OSError, csv.Error) as error:
        raise AuditError(f"cannot read backlog: {error}") from error


def required_issue_numbers(rows: list[dict[str, str]]) -> list[int]:
    try:
        return sorted({int(row["issue"].removeprefix("#")) for row in rows})
    except (KeyError, ValueError) as error:
        raise AuditError("backlog changed after offline validation") from error


def validate_issue_record(value: Any, context: str) -> tuple[int, str]:
    if not isinstance(value, dict) or set(value) != ISSUE_FIELDS:
        raise AuditError(f"{context} must contain exactly number, state and url")
    number = value["number"]
    state = value["state"]
    url = value["url"]
    if isinstance(number, bool) or not isinstance(number, int) or number < 1:
        raise AuditError(f"{context} number must be a positive integer")
    if state not in ISSUE_STATES:
        raise AuditError(f"{context} state must be OPEN or CLOSED")
    if not isinstance(url, str) or not url.startswith("https://github.com/"):
        raise AuditError(f"{context} url must be an HTTPS GitHub issue URL")
    return number, state


def parse_snapshot(
    document: Any, repository: str, expected_numbers: list[int]
) -> dict[int, str]:
    if not isinstance(document, dict) or set(document) != SNAPSHOT_FIELDS:
        raise AuditError(
            "snapshot must contain exactly schemaVersion, repository and issues"
        )
    if document["schemaVersion"] != 1:
        raise AuditError("snapshot schemaVersion must be 1")
    if document["repository"] != repository:
        raise AuditError("snapshot repository does not match tracked factory policy")
    issues = document["issues"]
    if not isinstance(issues, list) or len(issues) > len(expected_numbers):
        raise AuditError("snapshot issues must be a bounded array")

    states: dict[int, str] = {}
    for index, issue in enumerate(issues):
        number, state = validate_issue_record(issue, f"snapshot issue {index}")
        if number in states:
            raise AuditError(f"snapshot contains duplicate issue #{number}")
        expected_url = f"https://github.com/{repository}/issues/{number}"
        if issue["url"] != expected_url:
            raise AuditError(f"snapshot issue #{number} url does not match repository")
        states[number] = state

    expected = set(expected_numbers)
    actual = set(states)
    missing = sorted(expected - actual)
    unexpected = sorted(actual - expected)
    if missing:
        raise AuditError(
            "snapshot is missing referenced issues: "
            + ", ".join(f"#{number}" for number in missing)
        )
    if unexpected:
        raise AuditError(
            "snapshot contains unreferenced issues: "
            + ", ".join(f"#{number}" for number in unexpected)
        )
    return states


def load_snapshot(
    path: Path, repository: str, expected_numbers: list[int]
) -> dict[int, str]:
    if path.is_symlink():
        raise AuditError("snapshot must not be a symbolic link")
    try:
        size = path.stat().st_size
    except OSError as error:
        raise AuditError(f"cannot inspect snapshot: {error}") from error
    if size > MAX_SNAPSHOT_BYTES:
        raise AuditError(f"snapshot exceeds {MAX_SNAPSHOT_BYTES} bytes")
    try:
        document = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise AuditError(f"cannot parse snapshot: {error}") from error
    return parse_snapshot(document, repository, expected_numbers)


def query_github(repository: str, issue_numbers: list[int]) -> dict[int, str]:
    gh = shutil.which("gh")
    if gh is None:
        raise AuditError("GitHub CLI is unavailable")

    states: dict[int, str] = {}
    for number in issue_numbers:
        try:
            result = subprocess.run(
                [
                    gh,
                    "issue",
                    "view",
                    str(number),
                    "--repo",
                    repository,
                    "--json",
                    "number,state,url",
                ],
                check=False,
                capture_output=True,
                text=True,
                timeout=GITHUB_TIMEOUT_SECONDS,
            )
        except subprocess.TimeoutExpired as error:
            raise AuditError(f"GitHub lookup timed out for issue #{number}") from error
        if result.returncode != 0:
            raise AuditError(
                f"GitHub could not resolve issue #{number} (exit {result.returncode})"
            )
        if len(result.stdout.encode("utf-8")) > MAX_GITHUB_RESPONSE_BYTES:
            raise AuditError(f"GitHub response exceeds bound for issue #{number}")
        try:
            document = json.loads(result.stdout)
        except json.JSONDecodeError as error:
            raise AuditError(
                f"GitHub returned malformed JSON for issue #{number}"
            ) from error
        actual_number, state = validate_issue_record(
            document, f"GitHub issue #{number}"
        )
        if actual_number != number:
            raise AuditError(
                f"GitHub returned issue #{actual_number} for requested issue #{number}"
            )
        expected_url = f"https://github.com/{repository}/issues/{number}"
        if document["url"] != expected_url:
            raise AuditError(f"GitHub issue #{number} url does not match repository")
        states[number] = state
    return states


def audit_rows(rows: list[dict[str, str]], states: dict[int, str]) -> list[str]:
    failures: list[str] = []
    try:
        for row in rows:
            number = int(row["issue"].removeprefix("#"))
            if row["delivery_status"] == "in_progress" and states[number] != "OPEN":
                failures.append(
                    f"{row['id']}: in_progress owner #{number} is {states[number]}"
                )
    except (KeyError, ValueError) as error:
        raise AuditError("backlog or issue state changed during audit") from error
    return failures


def main() -> int:
    arguments = parse_arguments()
    try:
        repository = load_repository()
        validate_offline(DEFAULT_BACKLOG)
        rows = load_rows(DEFAULT_BACKLOG)
        issue_numbers = required_issue_numbers(rows)
        if arguments.snapshot is None:
            states = query_github(repository, issue_numbers)
            mode = "live"
        else:
            states = load_snapshot(arguments.snapshot, repository, issue_numbers)
            mode = "snapshot"
        failures = audit_rows(rows, states)
    except AuditError as error:
        print(f"ssi-upstream-backlog-live: {error}", file=sys.stderr)
        return 1

    if failures:
        for failure in failures:
            print(f"ssi-upstream-backlog-live: {failure}", file=sys.stderr)
        return 1
    print(
        "ssi-upstream-backlog-live: "
        f"{len(rows)} rows and {len(states)} issues passed ({mode})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
