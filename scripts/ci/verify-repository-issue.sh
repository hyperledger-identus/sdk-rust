#!/usr/bin/env bash

set -euo pipefail

repository=${GITHUB_REPOSITORY:-}
issue_number=${ISSUE_NUMBER:-}
max_attempts=${ISSUE_LOOKUP_MAX_ATTEMPTS:-3}
retry_delay_seconds=${ISSUE_LOOKUP_RETRY_DELAY_SECONDS:-2}

if [[ ! "$repository" =~ ^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+$ ]]; then
  printf 'pull-request-policy: invalid repository identity\n' >&2
  exit 1
fi
if [[ ! "$issue_number" =~ ^[1-9][0-9]*$ ]]; then
  printf 'pull-request-policy: invalid issue identity\n' >&2
  exit 1
fi
if [[ ! "$max_attempts" =~ ^[1-3]$ ]]; then
  printf 'pull-request-policy: invalid issue lookup attempt bound\n' >&2
  exit 1
fi
if [[ ! "$retry_delay_seconds" =~ ^[0-9]+$ || "$retry_delay_seconds" -gt 10 ]]; then
  printf 'pull-request-policy: invalid issue lookup retry delay\n' >&2
  exit 1
fi

private_tmp=$(mktemp -d)
response_file="$private_tmp/response.json"
error_file="$private_tmp/error.log"
trap 'rm -rf "$private_tmp"' EXIT

is_transient_failure() {
  grep -Eiq \
    'HTTP (408|425|429|499|5[0-9][0-9])([^0-9]|$)|timed? out|timeout|connection (reset|refused|closed)|temporary (failure|error)|temporarily unavailable|unexpected EOF|TLS handshake' \
    "$error_file"
}

expected_url="https://github.com/$repository/issues/$issue_number"
attempt=1
while [[ "$attempt" -le "$max_attempts" ]]; do
  : >"$response_file"
  : >"$error_file"
  if gh api "repos/$repository/issues/$issue_number" >"$response_file" 2>"$error_file"; then
    if ! jq -e 'type == "object"' "$response_file" >/dev/null 2>&1; then
      printf 'pull-request-policy: issue lookup returned an invalid response\n' >&2
      exit 1
    fi
    if jq -e 'has("pull_request")' "$response_file" >/dev/null; then
      printf 'pull-request-policy: #%s is not a repository issue\n' "$issue_number" >&2
      exit 1
    fi
    issue_url=$(jq -r '.html_url // empty' "$response_file")
    if [[ "$issue_url" != "$expected_url" ]]; then
      printf 'pull-request-policy: #%s is not the expected repository issue\n' \
        "$issue_number" >&2
      exit 1
    fi
    printf 'pull-request-policy: verified repository issue #%s\n' "$issue_number"
    exit 0
  fi

  if ! is_transient_failure; then
    printf 'pull-request-policy: issue lookup failed permanently on attempt %s\n' \
      "$attempt" >&2
    exit 1
  fi
  if [[ "$attempt" -eq "$max_attempts" ]]; then
    printf 'pull-request-policy: issue lookup failed after %s transient attempts\n' \
      "$max_attempts" >&2
    exit 1
  fi

  printf 'pull-request-policy: transient issue lookup failure; retrying (%s/%s)\n' \
    "$attempt" "$max_attempts" >&2
  sleep "$retry_delay_seconds"
  attempt=$((attempt + 1))
done

printf 'pull-request-policy: issue lookup exhausted unexpectedly\n' >&2
exit 1
