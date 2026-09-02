#!/usr/bin/env bash

set -euo pipefail

pr_base_ref=${PR_BASE_REF:-}
pr_draft=${PR_DRAFT:-}
pr_body=${PR_BODY:-}
failures=0

report_failure() {
  printf 'pull-request-policy: %s\n' "$1" >&2
  failures=$((failures + 1))
}

if [[ "$pr_base_ref" != "develop" ]]; then
  report_failure "base branch must be develop"
fi

if [[ "$pr_draft" != "false" ]]; then
  report_failure "pull request must be ready, not draft"
fi

issue_pattern='^[[:space:]]*-[[:space:]]*Issue:[[:space:]]*#[0-9]+([[:space:]]|$)|^[[:space:]]*(Closes|Fixes|Resolves|Refs|References)[[:space:]]+#[0-9]+([[:space:]]|$)'
if issue_line=$(printf '%s\n' "$pr_body" | grep -Eim1 "$issue_pattern"); then
  issue_suffix=${issue_line#*#}
  issue_number=${issue_suffix%%[!0-9]*}
else
  issue_number=
  report_failure "body must reference the corresponding repository issue"
fi

review_pattern='^[[:space:]]*-[[:space:]]*Local review:[[:space:]]*(passed|complete|completed)([[:space:][:punct:]]|$)'
if ! printf '%s\n' "$pr_body" | grep -Eiq "$review_pattern"; then
  report_failure "body must record completed local review"
fi

if ((failures > 0)); then
  printf 'pull-request-policy: %d failure(s)\n' "$failures" >&2
  exit 1
fi

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  printf 'issue-number=%s\n' "$issue_number" >>"$GITHUB_OUTPUT"
fi

printf 'pull-request-policy: passed\n'
