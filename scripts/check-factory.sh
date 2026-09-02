#!/usr/bin/env bash

set -euo pipefail

factory_root=${1:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}
failures=0

report_failure() {
  printf 'factory-contract: %s\n' "$1" >&2
  failures=$((failures + 1))
}

required_files=(
  AGENTS.md
  CONTRIBUTING.md
  docs/factory/README.md
  docs/governance/agentic-sdlc.md
  docs/governance/repository-settings.md
  docs/adr/0003-delegate-develop-integration.md
  openspec/config.yaml
  scripts/factory
  scripts/check-factory.sh
  scripts/check-pr-policy.sh
  scripts/tests/factory-contract.sh
  scripts/tests/pr-policy.sh
  .github/ISSUE_TEMPLATE/component-change.yml
  .github/ISSUE_TEMPLATE/delivery-task.yml
  .github/pull_request_template.md
  .github/workflows/factory-contract.yml
  .github/workflows/pull-request-policy.yml
)

for relative_path in "${required_files[@]}"; do
  if [[ ! -f "$factory_root/$relative_path" ]]; then
    report_failure "missing required file: $relative_path"
  fi
done

for executable_path in scripts/factory scripts/check-factory.sh scripts/check-pr-policy.sh scripts/tests/factory-contract.sh scripts/tests/pr-policy.sh; do
  if [[ -f "$factory_root/$executable_path" && ! -x "$factory_root/$executable_path" ]]; then
    report_failure "required executable bit is missing: $executable_path"
  fi
done

changes_root="$factory_root/openspec/changes"
if [[ ! -d "$changes_root" ]]; then
  report_failure "missing OpenSpec changes directory"
else
  while IFS= read -r -d '' change_dir; do
    change_name=$(basename "$change_dir")
    for artifact in .openspec.yaml proposal.md design.md tasks.md; do
      if [[ ! -f "$change_dir/$artifact" ]]; then
        report_failure "active change $change_name is missing $artifact"
      fi
    done

    if ! find "$change_dir/specs" -mindepth 2 -maxdepth 2 -type f -name spec.md -print -quit 2>/dev/null | grep -q .; then
      report_failure "active change $change_name has no capability spec"
    fi

    if [[ -f "$change_dir/tasks.md" ]] && ! grep -Eq '^- \[[ xX]\] [0-9]+\.[0-9]+ ' "$change_dir/tasks.md"; then
      report_failure "active change $change_name has no parseable task checkbox"
    fi
  done < <(find "$changes_root" -mindepth 1 -maxdepth 1 -type d ! -name archive -print0)
fi

if [[ -f "$factory_root/.pi/chains/afk.yaml" ]] && grep -q 'add-newtype-macro' "$factory_root/.pi/chains/afk.yaml"; then
  report_failure ".pi/chains/afk.yaml contains a hard-coded historical change"
fi

if git -C "$factory_root" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  while IFS= read -r tracked_path; do
    case "$tracked_path" in
      .env | .env.* | */.env | */.env.* | .codex/* | */.codex/* | .idea/* | */.idea/* | .vscode/* | */.vscode/* | .mcp.json | */.mcp.json | mcp.json | */mcp.json | mcp_servers.json | */mcp_servers.json | .claude/settings.local.json | */.claude/settings.local.json | */.DS_Store | .DS_Store)
        if [[ "$(basename "$tracked_path")" != ".env.example" ]]; then
          report_failure "tracked personal or secret-bearing state: $tracked_path"
        fi
        ;;
    esac
  done < <(git -C "$factory_root" ls-files)
fi

if ((failures > 0)); then
  printf 'factory-contract: %d failure(s)\n' "$failures" >&2
  exit 1
fi

printf 'factory-contract: structure passed\n'
