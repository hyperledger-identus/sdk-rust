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
  CODE_OF_CONDUCT.md
  CONTRIBUTING.md
  DCO.md
  GOVERNANCE.md
  LICENSE
  MAINTAINERS.md
  RELEASING.md
  SECURITY.md
  docs/factory/README.md
  docs/factory/operations.md
  docs/factory/recovery.md
  docs/factory/supervisor.md
  docs/factory/metrics.md
  docs/factory/work-item-metrics-v1.schema.json
  docs/factory/work-item-metrics-v2.schema.json
  docs/factory/pi-usage-v1.schema.json
  docs/factory/supervisor-invocation-v1.schema.json
  docs/factory/supervisor-heartbeat-v1.schema.json
  docs/factory/worker-handoff-v1.schema.json
  docs/factory/research-readiness.md
  docs/governance/constraints-and-limitations.md
  docs/governance/sdk-constraints.toml
  docs/architecture/sdk-bootstrap-inventory.md
  docs/architecture/sdk-bootstrap-inventory.toml
  docs/architecture/sdk-support-policy.md
  docs/architecture/sdk-support-policy.toml
  docs/architecture/apollo-crypto-parity.toml
  docs/architecture/ssi-upstream-source-matrix.md
  docs/roadmap/ssi-upstream-dependency-backlog.csv
  docs/governance/agentic-sdlc.md
  docs/governance/repository-settings.md
  docs/adr/0003-delegate-develop-integration.md
  docs/adr/0081-use-temporary-rust-198-fast-slow-ci.md
  docs/adr/0087-enforce-first-party-unsafe-forbid.md
  docs/adr/0089-require-upstream-first-dependency-remediation.md
  docs/adr/0095-use-dependency-free-crypto-benchmark-harness.md
  docs/adr/0096-adopt-cargo-llvm-cov-for-apollo-evidence.md
  docs/adr/0108-operationalize-guidance-based-ai-factory.md
  nix/checks/gates.toml
  nix/checks/rust-gates.nix
  openspec/config.yaml
  scripts/benchmark-support-policy.py
  scripts/factory
  bootstrap.sh
  .factory-policy.json
  .devloops
  .pi/settings.json
  .pi/package-runtime/package.json
  .pi/package-runtime/package-lock.json
  .pi/delivery-profiles.json
  .pi/subagent-policy.json
  .pi/agents/dev-loop.agent.md
  .pi/agents/planner.agent.md
  .pi/agents/developer.agent.md
  .pi/agents/reviewer.agent.md
  .pi/agents/quality.agent.md
  .github/contribution-policy.json
  .github/ISSUE_TEMPLATE/factory-work-item.yml
  .githooks/commit-msg
  .githooks/pre-commit
  .githooks/pre-push
  scripts/ci/contribution-policy.mjs
  scripts/ci/target-plan.mjs
  scripts/factory-tools/audit-pi.mjs
  scripts/factory-tools/metrics.mjs
  scripts/factory-tools/pi-session-harvest.mjs
  scripts/factory-tools/strict-json.mjs
  scripts/factory-tools/supervisor.mjs
  scripts/factory-tools/pi-package-cache.mjs
  scripts/factory-tools/pi-policy.mjs
  scripts/factory-tools/preflight.mjs
  scripts/git-hooks/configure.mjs
  scripts/git-hooks/local-policy.mjs
  scripts/worktree-lifecycle.mjs
  scripts/check-factory.sh
  scripts/check-bootstrap-inventory.py
  scripts/check-constraints.py
  scripts/check-openspec-archive.py
  scripts/check-support-policy.py
  scripts/check-apollo-parity.py
  scripts/benchmark-crypto.sh
  scripts/coverage-crypto.sh
  scripts/check-crypto-benchmark.py
  scripts/report-crypto-coverage.py
  scripts/check-ssi-upstream-backlog.py
  scripts/check-pr-policy.sh
  scripts/check-research-readiness.py
  scripts/tests/factory-contract.sh
  scripts/tests/factory-operations.mjs
  scripts/tests/bootstrap-inventory.py
  scripts/tests/constraints.py
  scripts/tests/openspec-archive.py
  scripts/tests/pr-policy.sh
  scripts/tests/research-readiness.py
  scripts/tests/support-policy.py
  scripts/tests/apollo-parity.py
  scripts/tests/crypto-benchmark.py
  scripts/tests/crypto-coverage.py
  .github/CODEOWNERS
  .github/ISSUE_TEMPLATE/component-change.yml
  .github/ISSUE_TEMPLATE/delivery-task.yml
  .github/pull_request_template.md
  .github/workflows/factory-contract.yml
  .github/workflows/nix-checks.yml
  .github/workflows/pull-request-policy.yml
)

for relative_path in "${required_files[@]}"; do
  if [[ ! -f "$factory_root/$relative_path" ]]; then
    report_failure "missing required file: $relative_path"
  fi
done

for executable_path in bootstrap.sh scripts/factory scripts/benchmark-support-policy.py scripts/benchmark-crypto.sh scripts/coverage-crypto.sh scripts/check-factory.sh scripts/check-bootstrap-inventory.py scripts/check-constraints.py scripts/check-crypto-benchmark.py scripts/report-crypto-coverage.py scripts/check-openspec-archive.py scripts/check-pr-policy.sh scripts/check-research-readiness.py scripts/check-support-policy.py scripts/check-apollo-parity.py scripts/check-ssi-upstream-backlog.py scripts/ci/contribution-policy.mjs scripts/ci/target-plan.mjs scripts/factory-tools/audit-pi.mjs scripts/factory-tools/metrics.mjs scripts/factory-tools/pi-session-harvest.mjs scripts/factory-tools/strict-json.mjs scripts/factory-tools/supervisor.mjs scripts/factory-tools/pi-package-cache.mjs scripts/factory-tools/pi-policy.mjs scripts/factory-tools/preflight.mjs scripts/git-hooks/configure.mjs scripts/git-hooks/local-policy.mjs scripts/worktree-lifecycle.mjs scripts/tests/bootstrap-inventory.py scripts/tests/constraints.py scripts/tests/crypto-benchmark.py scripts/tests/crypto-coverage.py scripts/tests/factory-contract.sh scripts/tests/factory-operations.mjs scripts/tests/openspec-archive.py scripts/tests/pr-policy.sh scripts/tests/research-readiness.py scripts/tests/support-policy.py scripts/tests/apollo-parity.py .githooks/commit-msg .githooks/pre-commit .githooks/pre-push; do
  if [[ -f "$factory_root/$executable_path" && ! -x "$factory_root/$executable_path" ]]; then
    report_failure "required executable bit is missing: $executable_path"
  fi
done

if [[ -x "$factory_root/scripts/check-bootstrap-inventory.py" && -f "$factory_root/docs/architecture/sdk-bootstrap-inventory.toml" ]]; then
  if ! "$factory_root/scripts/check-bootstrap-inventory.py" "$factory_root"; then
    report_failure "SDK bootstrap-inventory validation failed"
  fi
fi

if [[ -x "$factory_root/scripts/check-research-readiness.py" ]]; then
  if ! "$factory_root/scripts/check-research-readiness.py" "$factory_root"; then
    report_failure "research-readiness validation failed"
  fi
fi

if [[ -x "$factory_root/scripts/check-constraints.py" ]]; then
  if ! "$factory_root/scripts/check-constraints.py" "$factory_root"; then
    report_failure "constraint-governance validation failed"
  fi
fi

if [[ -x "$factory_root/scripts/check-ssi-upstream-backlog.py" && -f "$factory_root/docs/roadmap/ssi-upstream-dependency-backlog.csv" ]]; then
  if ! "$factory_root/scripts/check-ssi-upstream-backlog.py" "$factory_root/docs/roadmap/ssi-upstream-dependency-backlog.csv"; then
    report_failure "SSI upstream backlog validation failed"
  fi
fi

if [[ -x "$factory_root/scripts/check-support-policy.py" && -f "$factory_root/docs/architecture/sdk-support-policy.toml" ]]; then
  if ! "$factory_root/scripts/check-support-policy.py" "$factory_root"; then
    report_failure "SDK support-policy validation failed"
  fi
fi

if [[ -x "$factory_root/scripts/check-apollo-parity.py" && -f "$factory_root/docs/architecture/apollo-crypto-parity.toml" ]]; then
  if ! "$factory_root/scripts/check-apollo-parity.py" "$factory_root"; then
    report_failure "Apollo parity-manifest validation failed"
  fi
fi

changes_root="$factory_root/openspec/changes"
if [[ ! -d "$changes_root" ]]; then
  report_failure "missing OpenSpec changes directory"
else
  while IFS= read -r -d '' change_dir; do
    change_name=$(basename "$change_dir")
    for artifact in .openspec.yaml proposal.md research.md constraints.md design.md tasks.md; do
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

if [[ -x "$factory_root/scripts/check-openspec-archive.py" ]]; then
  if ! "$factory_root/scripts/check-openspec-archive.py" "$factory_root"; then
    report_failure "OpenSpec archive-preservation validation failed"
  fi
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
