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
  README.md
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
  docs/architecture/code-health.md
  docs/architecture/code-health.toml
  docs/architecture/code-health-baseline.json
  docs/architecture/code-health-v2-migration.md
  crates/conformance/src/bin/code-health-classifier.rs
  docs/architecture/sdk-input-resource-boundaries.md
  docs/architecture/sdk-input-resource-boundaries.toml
  docs/architecture/source-distribution.md
  docs/release/crypto-candidate.toml
  docs/release/identus-crypto-0.1.0-rc.1.api.txt
  crates/derive/README.md
  crates/core/README.md
  crates/crypto/README.md
  crates/credentials/tests/fixtures/credentials-error-contract-v1.csv
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
  docs/adr/0113-prepare-isolated-unpublished-crypto-candidate.md
  docs/adr/0120-run-weekly-evidence-from-protected-develop.md
  docs/adr/0121-generate-candidate-rustdoc-json-before-api-rendering.md
  docs/adr/0122-use-aosp-image-for-android-runtime-proof.md
  docs/adr/0123-bind-android-build-to-the-exact-installed-ndk.md
  docs/adr/0124-separate-android-package-and-runtime-evidence.md
  docs/adr/0125-govern-sdk-input-resource-boundaries.md
  nix/checks/gates.toml
  nix/checks/rust-gates.nix
  nix/apps/crypto-candidate.nix
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
  scripts/check-error-golden.py
  scripts/check-bootstrap-inventory.py
  scripts/check-input-resource-boundaries.py
  scripts/check-constraints.py
  scripts/check-openspec-archive.py
  scripts/check-support-policy.py
  scripts/check-apollo-parity.py
  scripts/benchmark-crypto.sh
  scripts/coverage-crypto.sh
  scripts/check-crypto-benchmark.py
  scripts/report-crypto-coverage.py
  scripts/check-ssi-upstream-backlog.py
  scripts/check-ssi-upstream-backlog-live.py
  scripts/check-uniffi-did-android.sh
  scripts/check-weekly-slow-live.py
  scripts/check-source-distribution.py
  scripts/check-crypto-candidate.py
  scripts/code-health-audit.py
  scripts/prepare-crypto-candidate.py
  scripts/check-pr-policy.sh
  scripts/check-research-readiness.py
  scripts/tests/factory-contract.sh
  scripts/tests/error-golden.py
  scripts/tests/factory-operations.mjs
  scripts/tests/bootstrap-inventory.py
  scripts/tests/input-resource-boundaries.py
  scripts/tests/constraints.py
  scripts/tests/openspec-archive.py
  scripts/tests/pr-policy.sh
  scripts/tests/research-readiness.py
  scripts/tests/support-policy.py
  scripts/tests/ssi-upstream-backlog-live.py
  scripts/tests/weekly-slow-live.py
  scripts/tests/apollo-parity.py
  scripts/tests/crypto-benchmark.py
  scripts/tests/crypto-coverage.py
  scripts/tests/source-distribution.py
  scripts/tests/crypto-candidate.py
  scripts/tests/code-health-audit.py
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

for executable_path in bootstrap.sh scripts/factory scripts/benchmark-support-policy.py scripts/benchmark-crypto.sh scripts/coverage-crypto.sh scripts/check-factory.sh scripts/check-bootstrap-inventory.py scripts/check-input-resource-boundaries.py scripts/check-constraints.py scripts/check-error-golden.py scripts/check-crypto-benchmark.py scripts/report-crypto-coverage.py scripts/code-health-audit.py scripts/check-openspec-archive.py scripts/check-pr-policy.sh scripts/check-research-readiness.py scripts/check-support-policy.py scripts/check-apollo-parity.py scripts/check-ssi-upstream-backlog.py scripts/check-ssi-upstream-backlog-live.py scripts/check-uniffi-did-android.sh scripts/check-weekly-slow-live.py scripts/check-source-distribution.py scripts/check-crypto-candidate.py scripts/prepare-crypto-candidate.py scripts/ci/contribution-policy.mjs scripts/ci/target-plan.mjs scripts/factory-tools/audit-pi.mjs scripts/factory-tools/metrics.mjs scripts/factory-tools/pi-session-harvest.mjs scripts/factory-tools/strict-json.mjs scripts/factory-tools/supervisor.mjs scripts/factory-tools/pi-package-cache.mjs scripts/factory-tools/pi-policy.mjs scripts/factory-tools/preflight.mjs scripts/git-hooks/configure.mjs scripts/git-hooks/local-policy.mjs scripts/worktree-lifecycle.mjs scripts/tests/bootstrap-inventory.py scripts/tests/input-resource-boundaries.py scripts/tests/constraints.py scripts/tests/error-golden.py scripts/tests/crypto-benchmark.py scripts/tests/crypto-coverage.py scripts/tests/code-health-audit.py scripts/tests/source-distribution.py scripts/tests/crypto-candidate.py scripts/tests/factory-contract.sh scripts/tests/factory-operations.mjs scripts/tests/openspec-archive.py scripts/tests/pr-policy.sh scripts/tests/research-readiness.py scripts/tests/support-policy.py scripts/tests/apollo-parity.py scripts/tests/ssi-upstream-backlog.py scripts/tests/ssi-upstream-backlog-live.py scripts/tests/weekly-slow-live.py .githooks/commit-msg .githooks/pre-commit .githooks/pre-push; do
  if [[ -f "$factory_root/$executable_path" && ! -x "$factory_root/$executable_path" ]]; then
    report_failure "required executable bit is missing: $executable_path"
  fi
done

if [[ -x "$factory_root/scripts/check-bootstrap-inventory.py" && -f "$factory_root/docs/architecture/sdk-bootstrap-inventory.toml" ]]; then
  if ! "$factory_root/scripts/check-bootstrap-inventory.py" "$factory_root"; then
    report_failure "SDK bootstrap-inventory validation failed"
  fi
fi

if [[ -x "$factory_root/scripts/check-input-resource-boundaries.py" && -f "$factory_root/docs/architecture/sdk-input-resource-boundaries.toml" ]]; then
  if ! "$factory_root/scripts/check-input-resource-boundaries.py" "$factory_root"; then
    report_failure "SDK input-resource boundary validation failed"
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

if [[ -x "$factory_root/scripts/check-source-distribution.py" && -f "$factory_root/docs/architecture/source-distribution.md" ]]; then
  if ! "$factory_root/scripts/check-source-distribution.py" "$factory_root"; then
    report_failure "source-distribution validation failed"
  fi
fi

if [[ -x "$factory_root/scripts/check-crypto-candidate.py" && -f "$factory_root/docs/release/crypto-candidate.toml" ]]; then
  if ! "$factory_root/scripts/check-crypto-candidate.py" "$factory_root"; then
    report_failure "unpublished crypto-candidate validation failed"
  fi
fi

if [[ -x "$factory_root/scripts/check-error-golden.py" ]]; then
  if ! "$factory_root/scripts/check-error-golden.py" "$factory_root"; then
    report_failure "immutable error-golden validation failed"
  fi
fi

factory_workflow="$factory_root/.github/workflows/factory-contract.yml"
if [[ -f "$factory_workflow" ]]; then
  hosted_error_block=$(sed -n \
    '/^      - name: Verify preflight-bound error goldens$/,/^      - name: Install Nix$/p' \
    "$factory_workflow")
  if ! grep -Fxq '          env -u SDK_ERROR_GOLDEN_SOURCE_SNAPSHOT' \
      <<<"$hosted_error_block" ||
    ! grep -Fxq '          python3 scripts/check-error-golden.py .' \
      <<<"$hosted_error_block"; then
    report_failure "fast hosted CI must run Git-backed error-golden validation before Nix"
  fi

  hosted_code_health_block=$(sed -n \
    '/^      - name: Verify pinned code-health source evidence$/,/^      - name: Run fast factory and Rust gates$/p' \
    "$factory_workflow")
  if ! grep -Fq \
      'nix develop --command python3 scripts/code-health-audit.py' \
      <<<"$hosted_code_health_block"; then
    report_failure "fast hosted code-health validation must use the pinned Nix shell"
  fi
fi

if [[ -x "$factory_root/scripts/code-health-audit.py" && -f "$factory_root/docs/architecture/code-health-baseline.json" ]]; then
  if ! python3 "$factory_root/scripts/code-health-audit.py" --root "$factory_root" --check-report "$factory_root/docs/architecture/code-health-baseline.json" --policy-only; then
    report_failure "code-health report validation failed"
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
