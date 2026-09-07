#!/usr/bin/env bash

set -euo pipefail

if repository_root=$(git rev-parse --show-toplevel 2>/dev/null); then
  :
else
  repository_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
fi
checker="$repository_root/scripts/check-pr-policy.sh"
valid_body=$'- Issue: #16\n- Local review: passed by a fresh review pass\n- Constraint impact: routine\n- Limitations: none'
output_file=$(mktemp)
trap 'rm -f "$output_file"' EXIT

assert_rejected() {
  local description=$1
  shift

  if "$@" >/dev/null 2>&1; then
    printf 'pr-policy test: accepted %s\n' "$description" >&2
    exit 1
  fi
}

GITHUB_OUTPUT="$output_file" PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY="$valid_body" "$checker" >/dev/null
grep -qx 'issue-number=16' "$output_file"

assert_rejected "a pull request targeting main" \
  env PR_BASE_REF=main PR_DRAFT=false PR_BODY="$valid_body" "$checker"
assert_rejected "a draft pull request" \
  env PR_BASE_REF=develop PR_DRAFT=true PR_BODY="$valid_body" "$checker"
assert_rejected "a pull request without an issue" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY='- Local review: passed locally' "$checker"
assert_rejected "a pull request without local review" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY=$'- Issue: #16\n- Constraint impact: routine\n- Limitations: none' "$checker"
assert_rejected "a pull request without constraint impact" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY=$'- Issue: #16\n- Local review: passed\n- Limitations: none' "$checker"
assert_rejected "a pull request without limitations" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY=$'- Issue: #16\n- Local review: passed\n- Constraint impact: routine' "$checker"
assert_rejected "an unchanged pull request template" \
  env PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY=$'- Issue: <!-- Required: #123 -->\n- Local review: <!-- Required: passed -->\n- Constraint impact: <!-- Required: none/routine/material -->\n- Limitations: <!-- Required -->' \
  "$checker"

PR_BASE_REF=develop PR_DRAFT=false \
  PR_BODY=$'Closes #16\n- Local review: completed by maintainer\n- Constraint impact: none\n- Limitations: none' \
  "$checker" >/dev/null

printf 'pr-policy tests: passed\n'
