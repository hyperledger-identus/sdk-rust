#!/usr/bin/env bash

set -euo pipefail

if repository_root=$(git rev-parse --show-toplevel 2>/dev/null); then
  :
else
  repository_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
fi
verifier="$repository_root/scripts/ci/verify-repository-issue.sh"
fixture_root=$(mktemp -d)
fake_bin="$fixture_root/bin"
counter_file="$fixture_root/attempts"
stderr_file="$fixture_root/stderr"
mkdir -p "$fake_bin"
trap 'rm -rf "$fixture_root"' EXIT

cat >"$fake_bin/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

count=0
if [[ -f "$FAKE_GH_COUNTER" ]]; then
  count=$(cat "$FAKE_GH_COUNTER")
fi
count=$((count + 1))
printf '%s\n' "$count" >"$FAKE_GH_COUNTER"

case "$FAKE_GH_MODE" in
  success)
    printf '%s\n' '{"html_url":"https://github.com/acme/sdk/issues/474"}'
    ;;
  transient-then-success)
    if [[ "$count" -eq 1 ]]; then
      printf '%s\n' 'HTTP 499: provider cancelled request' >&2
      exit 1
    fi
    printf '%s\n' '{"html_url":"https://github.com/acme/sdk/issues/474"}'
    ;;
  transient-always)
    printf '%s\n' 'HTTP 503: service unavailable' >&2
    exit 1
    ;;
  permanent)
    printf '%s\n' 'HTTP 404: Not Found' >&2
    exit 1
    ;;
  pull-request)
    printf '%s\n' '{"html_url":"https://github.com/acme/sdk/pull/474","pull_request":{}}'
    ;;
  mismatch)
    printf '%s\n' '{"html_url":"https://github.com/other/sdk/issues/474"}'
    ;;
  *)
    printf 'unexpected fake mode: %s\n' "$FAKE_GH_MODE" >&2
    exit 2
    ;;
esac
EOF
chmod +x "$fake_bin/gh"

run_verifier() {
  PATH="$fake_bin:$PATH" \
    FAKE_GH_COUNTER="$counter_file" \
    FAKE_GH_MODE="$1" \
    GITHUB_REPOSITORY=acme/sdk \
    ISSUE_NUMBER=474 \
    ISSUE_LOOKUP_MAX_ATTEMPTS=3 \
    ISSUE_LOOKUP_RETRY_DELAY_SECONDS=0 \
    "$verifier"
}

assert_success() {
  local mode=$1
  local expected_attempts=$2
  : >"$counter_file"
  if ! run_verifier "$mode" >/dev/null 2>"$stderr_file"; then
    printf 'repository-issue-verification test: rejected %s\n' "$mode" >&2
    cat "$stderr_file" >&2
    exit 1
  fi
  grep -qx "$expected_attempts" "$counter_file"
}

assert_failure() {
  local mode=$1
  local expected_attempts=$2
  local expected_diagnostic=$3
  : >"$counter_file"
  if run_verifier "$mode" >/dev/null 2>"$stderr_file"; then
    printf 'repository-issue-verification test: accepted %s\n' "$mode" >&2
    exit 1
  fi
  grep -qx "$expected_attempts" "$counter_file"
  grep -Fq "$expected_diagnostic" "$stderr_file"
}

assert_success success 1
assert_success transient-then-success 2
assert_failure transient-always 3 'failed after 3 transient attempts'
assert_failure permanent 1 'lookup failed permanently on attempt 1'
assert_failure pull-request 1 'is not a repository issue'
assert_failure mismatch 1 'is not the expected repository issue'

printf 'repository-issue-verification tests: passed\n'
