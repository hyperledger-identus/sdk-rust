#!/usr/bin/env bash

set -euo pipefail

pr_base_ref=${PR_BASE_REF:-}
pr_draft=${PR_DRAFT:-}
pr_body=${PR_BODY:-}
failures=0
pr_body_file=$(mktemp "${TMPDIR:-/tmp}/sdk-rust-pr-body.XXXXXX")
trap 'rm -f "$pr_body_file"' EXIT
chmod 600 "$pr_body_file"
printf '%s\n' "$pr_body" >"$pr_body_file"

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
if issue_line=$(grep -Eim1 "$issue_pattern" "$pr_body_file"); then
  issue_suffix=${issue_line#*#}
  issue_number=${issue_suffix%%[!0-9]*}
else
  issue_number=
  report_failure "body must reference the corresponding repository issue"
fi

review_pattern='^[[:space:]]*-[[:space:]]*Local review:[[:space:]]*(passed|complete|completed)([[:space:][:punct:]]|$)'
if ! grep -Eiq "$review_pattern" "$pr_body_file"; then
  report_failure "body must record completed local review"
fi

constraint_pattern='^[[:space:]]*-[[:space:]]*Constraint impact:[[:space:]]*(none|routine|material)([[:space:][:punct:]]|$)'
if ! grep -Eiq "$constraint_pattern" "$pr_body_file"; then
  report_failure "body must classify constraint impact as none, routine or material"
fi

limitation_pattern='^[[:space:]]*-[[:space:]]*Limitations:[[:space:]]*[^[:space:]<].*$'
if ! grep -Eiq "$limitation_pattern" "$pr_body_file"; then
  report_failure "body must state none or describe the introduced limitation"
fi

disposition_pattern='^[[:space:]]*-[[:space:]]*Compatibility disposition:[[:space:]]*(consumer-visible|behavior-neutral)([[:space:][:punct:]]|$)'
if disposition_line=$(grep -Eim1 "$disposition_pattern" "$pr_body_file"); then
  if grep -Eiq 'behavior-neutral' <<<"$disposition_line"; then
    disposition=behavior-neutral
  else
    disposition=consumer-visible
  fi
else
  disposition=
  report_failure "body must classify compatibility disposition as consumer-visible or behavior-neutral"
fi

ledger_pattern='^[[:space:]]*-[[:space:]]*Change-ledger IDs:[[:space:]]*[^[:space:]<].*$'
if ledger_line=$(grep -Eim1 "$ledger_pattern" "$pr_body_file"); then
  ledger_ids=${ledger_line#*:}
  ledger_ids=${ledger_ids#"${ledger_ids%%[![:space:]]*}"}
  ledger_ids=${ledger_ids%$'\r'}
else
  ledger_ids=
  report_failure "body must state stable change-ledger IDs or none"
fi

if [[ "$disposition" == "behavior-neutral" && ! "$ledger_ids" =~ ^[Nn][Oo][Nn][Ee]([[:space:][:punct:]]|$) ]]; then
  report_failure "behavior-neutral pull requests must record change-ledger IDs as none"
elif [[ "$disposition" == "consumer-visible" && "$ledger_ids" =~ ^[Nn][Oo][Nn][Ee]([[:space:][:punct:]]|$) ]]; then
  report_failure "consumer-visible pull requests must record at least one change-ledger ID"
elif [[ "$disposition" == "consumer-visible" && ! "$ledger_ids" =~ ^[a-z][a-z0-9]*([.-][a-z0-9]+)*([[:space:]]*,[[:space:]]*[a-z][a-z0-9]*([.-][a-z0-9]+)*)*$ ]]; then
  report_failure "consumer-visible change-ledger IDs must be stable lowercase dotted IDs"
fi

impact_pattern="^[[:space:]]*-[[:space:]]*OpenSpec \`?compatibility-impact\\.toml\`?:[[:space:]]*(present|grandfathered|not applicable)([[:space:][:punct:]]|$)"
if ! grep -Eiq "$impact_pattern" "$pr_body_file"; then
  report_failure "body must record the OpenSpec compatibility-impact declaration state"
elif [[ "$disposition" == "consumer-visible" ]] && grep -Eiq "OpenSpec \`?compatibility-impact\\.toml\`?:[[:space:]]*not applicable" "$pr_body_file"; then
  report_failure "consumer-visible pull requests require a present compatibility-impact declaration"
fi

if ((failures > 0)); then
  printf 'pull-request-policy: %d failure(s)\n' "$failures" >&2
  exit 1
fi

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  printf 'issue-number=%s\n' "$issue_number" >>"$GITHUB_OUTPUT"
fi

printf 'pull-request-policy: passed\n'
