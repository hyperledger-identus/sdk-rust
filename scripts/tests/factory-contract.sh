#!/usr/bin/env bash

set -euo pipefail
export PYTHONDONTWRITEBYTECODE=1

if repository_root=$(git rev-parse --show-toplevel 2>/dev/null); then
  :
else
  repository_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
fi
checker="$repository_root/scripts/check-factory.sh"
fixture_root=$(mktemp -d)
trap 'rm -rf "$fixture_root"' EXIT

"$repository_root/scripts/tests/ssi-upstream-backlog.py"
"$repository_root/scripts/tests/support-policy.py"
"$repository_root/scripts/tests/apollo-parity.py"
"$repository_root/scripts/tests/crypto-benchmark.py"
"$repository_root/scripts/tests/crypto-coverage.py"
"$repository_root/scripts/tests/bootstrap-inventory.py"
"$repository_root/scripts/tests/constraints.py"
"$repository_root/scripts/tests/openspec-archive.py"
"$repository_root/scripts/tests/research-readiness.py"

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
  docs/factory/metrics.md
  docs/factory/work-item-metrics-v1.schema.json
  docs/factory/research-readiness.md
  docs/architecture/sdk-rust-blueprint.md
  docs/architecture/sdk-bootstrap-inventory.md
  docs/architecture/sdk-bootstrap-inventory.toml
  docs/architecture/sdk-support-policy.md
  docs/architecture/sdk-support-policy.toml
  docs/architecture/apollo-crypto-parity.toml
  docs/architecture/ssi-upstream-source-matrix.md
  docs/roadmap/ssi-upstream-dependency-backlog.csv
  docs/governance/agentic-sdlc.md
  docs/governance/constraints-and-limitations.md
  docs/governance/repository-settings.md
  docs/governance/sdk-constraints.toml
  docs/adr/0001-bootstrap-branch-selection.md
  docs/adr/0003-delegate-develop-integration.md
  docs/adr/0062-use-a-rolling-near-current-msrv.md
  docs/adr/0063-make-material-constraints-explicit.md
  docs/adr/0064-separate-primary-rust-from-evidence-driven-msrv.md
  docs/adr/0081-use-temporary-rust-198-fast-slow-ci.md
  docs/adr/0087-enforce-first-party-unsafe-forbid.md
  docs/adr/0088-reject-unsafe-first-party-macro-output.md
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
  mkdir -p "$fixture_root/$(dirname "$relative_path")"
  case "$relative_path" in
    *)
      cp "$repository_root/$relative_path" "$fixture_root/$relative_path"
      ;;
  esac
done
chmod +x "$fixture_root/bootstrap.sh" "$fixture_root/scripts/factory" "$fixture_root/scripts/check-factory.sh" \
  "$fixture_root/scripts/benchmark-support-policy.py" \
  "$fixture_root/scripts/benchmark-crypto.sh" \
  "$fixture_root/scripts/coverage-crypto.sh" \
  "$fixture_root/scripts/check-bootstrap-inventory.py" \
  "$fixture_root/scripts/check-constraints.py" \
  "$fixture_root/scripts/check-crypto-benchmark.py" \
  "$fixture_root/scripts/report-crypto-coverage.py" \
  "$fixture_root/scripts/check-openspec-archive.py" \
  "$fixture_root/scripts/check-pr-policy.sh" \
  "$fixture_root/scripts/check-research-readiness.py" \
  "$fixture_root/scripts/check-support-policy.py" \
  "$fixture_root/scripts/check-apollo-parity.py" \
  "$fixture_root/scripts/check-ssi-upstream-backlog.py" \
  "$fixture_root/scripts/tests/bootstrap-inventory.py" \
  "$fixture_root/scripts/tests/constraints.py" \
  "$fixture_root/scripts/tests/crypto-benchmark.py" \
  "$fixture_root/scripts/tests/crypto-coverage.py" \
  "$fixture_root/scripts/tests/factory-contract.sh" \
  "$fixture_root/scripts/tests/factory-operations.mjs" \
  "$fixture_root/scripts/tests/openspec-archive.py" \
  "$fixture_root/scripts/tests/pr-policy.sh" \
  "$fixture_root/scripts/tests/research-readiness.py" \
  "$fixture_root/scripts/tests/support-policy.py"
chmod +x "$fixture_root/scripts/tests/apollo-parity.py"
chmod +x "$fixture_root/scripts/ci/"*.mjs "$fixture_root/scripts/factory-tools/"*.mjs \
  "$fixture_root/scripts/git-hooks/"*.mjs "$fixture_root/scripts/worktree-lifecycle.mjs" \
  "$fixture_root/.githooks/commit-msg" "$fixture_root/.githooks/pre-commit" \
  "$fixture_root/.githooks/pre-push"

for relative_path in Cargo.toml flake.nix flake.lock \
  docs/adr/0002-neoprism-toolchain-alignment.md nix/rust-toolchain.nix; do
  mkdir -p "$fixture_root/$(dirname "$relative_path")"
  cp "$repository_root/$relative_path" "$fixture_root/$relative_path"
done
for relative_path in .github/workflows/factory-contract.yml \
  .github/workflows/nix-checks.yml .github/workflows/crypto-fuzz.yml \
  .github/workflows/did-fuzz.yml .github/workflows/jws-fuzz.yml \
  nix/devshells/default.nix; do
  mkdir -p "$fixture_root/$(dirname "$relative_path")"
  cp "$repository_root/$relative_path" "$fixture_root/$relative_path"
done
mkdir -p "$fixture_root/nix"
cp -R "$repository_root/nix/checks/." "$fixture_root/nix/checks"
while IFS= read -r manifest; do
  relative_manifest=${manifest#"$repository_root/"}
  mkdir -p "$fixture_root/$(dirname "$relative_manifest")"
  cp "$manifest" "$fixture_root/$relative_manifest"
done < <(find "$repository_root/crates" -mindepth 2 -maxdepth 2 -name Cargo.toml -type f | sort)
while IFS= read -r source_file; do
  relative_source=${source_file#"$repository_root/"}
  mkdir -p "$fixture_root/$(dirname "$relative_source")"
  cp "$source_file" "$fixture_root/$relative_source"
done < <(find "$repository_root/crates" -type f -name '*.rs' | sort)

"$repository_root/scripts/tests/pr-policy.sh"

change_root="$fixture_root/openspec/changes/example-change"
mkdir -p "$change_root/specs/example-capability" "$fixture_root/.pi/chains"
for artifact in .openspec.yaml proposal.md design.md; do
  : >"$change_root/$artifact"
done
: >"$change_root/specs/example-capability/spec.md"
printf '%s\n' '- [ ] 1.1 Example task' >"$change_root/tasks.md"
cat >"$change_root/constraints.md" <<'EOF'
# Constraint and limitation impact

Impact class: routine
Decision status: not-required
Decision reference: not-required
Constraint blockers: none

## Existing entries affected

No effective entry changes.

## Introduced or changed constraints

No cross-cutting constraint is introduced.

## Introduced or changed limitations

No limitation is introduced.

## Consumer and product impact

No consumer or product outcome changes.

## Activation and rollback

There is no activation; rollback removes the fixture.

## Evidence

The deterministic factory test is the evidence.
EOF
cat >"$change_root/research.md" <<'EOF'
# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-07
Source retrieval date: 2026-09-07
Research blockers: none

## Problem and existing implementation

Not applicable to this fixture.

## Normative sources

Not applicable to this fixture.

## Candidate decisions

The fixture records `not-applicable`.

## Compatibility and dependency evidence

Not applicable to this fixture.

## Security, privacy and maintenance evidence

Not applicable to this fixture.

## Rejected or deferred candidates

Not applicable to this fixture.

## Open questions and blockers

None.

## Evidence commands

The factory contract test is the evidence.
EOF
: >"$fixture_root/.pi/chains/afk.yaml"

"$checker" "$fixture_root" >/dev/null

rm "$change_root/design.md"
if "$checker" "$fixture_root" >/dev/null 2>&1; then
  printf 'factory-contract test: missing artifact was accepted\n' >&2
  exit 1
fi
: >"$change_root/design.md"

printf '%s\n' 'argument: add-newtype-macro' >"$fixture_root/.pi/chains/afk.yaml"
if "$checker" "$fixture_root" >/dev/null 2>&1; then
  printf 'factory-contract test: hard-coded change was accepted\n' >&2
  exit 1
fi
: >"$fixture_root/.pi/chains/afk.yaml"

git -C "$fixture_root" init -q
git -C "$fixture_root" add .
mkdir -p "$fixture_root/nested"
: >"$fixture_root/nested/.env"
git -C "$fixture_root" add -f nested/.env
if "$checker" "$fixture_root" >/dev/null 2>&1; then
  printf 'factory-contract test: nested tracked local state was accepted\n' >&2
  exit 1
fi
git -C "$fixture_root" rm -q -f nested/.env

mkdir -p "$fixture_root/openspec/specs/example-capability" "$fixture_root/fake-bin"
cat >"$fixture_root/openspec/specs/example-capability/spec.md" <<'EOF'
# Example Specification

## Requirements

### Requirement: Existing behavior

The system SHALL preserve both behaviors.

#### Scenario: First behavior

- **WHEN** the first path runs
- **THEN** the first result remains

#### Scenario: Unrelated behavior

- **WHEN** the unrelated path runs
- **THEN** the unrelated result remains
EOF
cat >"$change_root/specs/example-capability/spec.md" <<'EOF'
## MODIFIED Requirements

### Requirement: Existing behavior

The system SHALL preserve only one behavior.

#### Scenario: First behavior

- **WHEN** the first path runs
- **THEN** the first result remains
EOF
printf '%s\n' '- [x] 1.1 Example task' >"$change_root/tasks.md"

# Build the same planning-only history required by the production preflight.
git -C "$fixture_root" rm -q -f --cached -r .
git -C "$fixture_root" branch -m codex/feat/issue-1
git -C "$fixture_root" config user.name 'Factory Fixture'
git -C "$fixture_root" config user.email 'factory-fixture@example.com'
git -C "$fixture_root" add . ':(exclude)openspec/changes/example-change/**'
git -C "$fixture_root" commit -q --no-gpg-sign -m 'test(factory): establish fixture base'
fixture_base_sha=$(git -C "$fixture_root" rev-parse HEAD)
git -C "$fixture_root" update-ref refs/remotes/origin/develop "$fixture_base_sha"
git -C "$fixture_root" add openspec/changes/example-change
git -C "$fixture_root" commit -q --no-gpg-sign -m 'spec(factory): define fixture contract'
fixture_contract_sha=$(git -C "$fixture_root" rev-parse HEAD)
cat >"$change_root/preimplementation.json" <<EOF
{
  "schemaVersion": 1,
  "repository": "hyperledger-identus/sdk-rust",
  "issue": 1,
  "change": "example-change",
  "branch": "codex/feat/issue-1",
  "baseRef": "origin/develop",
  "baseSha": "$fixture_base_sha",
  "contractHeadSha": "$fixture_contract_sha",
  "createdAt": "2026-09-09T00:00:00.000Z",
  "researchReady": true,
  "constraintsReady": true,
  "strictValidation": true
}
EOF
printf '#!%s\n' "$BASH" >"$fixture_root/fake-bin/openspec"
cat >>"$fixture_root/fake-bin/openspec" <<'EOF'
printf '%s\n' "$*" >>"$OPENSPEC_CALL_LOG"
EOF
chmod +x "$fixture_root/fake-bin/openspec"
: >"$fixture_root/openspec-calls.log"
canonical_before=$(git -C "$fixture_root" hash-object openspec/specs/example-capability/spec.md)
if (cd "$fixture_root" && \
  PATH="$fixture_root/fake-bin:$PATH" \
  OPENSPEC_CALL_LOG="$fixture_root/openspec-calls.log" \
  ./scripts/factory archive example-change >/dev/null 2>&1); then
  printf 'factory-contract test: lossy archive wrapper request was accepted\n' >&2
  exit 1
fi
canonical_after=$(git -C "$fixture_root" hash-object openspec/specs/example-capability/spec.md)
if [[ "$canonical_before" != "$canonical_after" || ! -d "$change_root" ]]; then
  printf 'factory-contract test: rejected archive mutated OpenSpec state\n' >&2
  exit 1
fi
if grep -Eq '^archive( |$)' "$fixture_root/openspec-calls.log"; then
  printf 'factory-contract test: OpenSpec archive ran before preservation failure\n' >&2
  exit 1
fi

cat >"$change_root/specs/example-capability/spec.md" <<'EOF'
## ADDED Requirements

### Requirement: New behavior

The system SHALL add one behavior.

#### Scenario: Added behavior

- **WHEN** the new path runs
- **THEN** the new result appears
EOF
: >"$fixture_root/openspec-calls.log"
expected_archive="$fixture_root/openspec/changes/archive/$(LC_ALL=C date -u +%F)-example-change"

mkdir -p "$expected_archive"
if collision_output=$(cd "$fixture_root" && \
  PATH="$fixture_root/fake-bin:$PATH" \
  OPENSPEC_CALL_LOG="$fixture_root/openspec-calls.log" \
  ./scripts/factory archive example-change 2>&1); then
  printf 'factory-contract test: archive destination collision was accepted\n' >&2
  exit 1
fi
if [[ -s "$fixture_root/openspec-calls.log" ]]; then
  printf 'factory-contract test: OpenSpec ran before destination collision failure\n' >&2
  exit 1
fi
if grep -Fq 'archived safely' <<<"$collision_output"; then
  printf 'factory-contract test: destination collision printed archive success\n' >&2
  exit 1
fi
rmdir "$expected_archive"

if no_op_output=$(cd "$fixture_root" && \
  PATH="$fixture_root/fake-bin:$PATH" \
  OPENSPEC_CALL_LOG="$fixture_root/openspec-calls.log" \
  ./scripts/factory archive example-change 2>&1); then
  printf 'factory-contract test: zero-exit no-op archive was accepted\n' >&2
  exit 1
fi
if [[ ! -d "$change_root" || -e "$expected_archive" ]]; then
  printf 'factory-contract test: zero-exit no-op changed archive state\n' >&2
  exit 1
fi
if ! grep -Eq '^archive example-change --yes$' "$fixture_root/openspec-calls.log"; then
  printf 'factory-contract test: no-op fixture did not reach OpenSpec archive\n' >&2
  printf 'factory-contract test: captured OpenSpec calls:\n' >&2
  cat "$fixture_root/openspec-calls.log" >&2
  printf 'factory-contract test: captured facade output:\n' >&2
  printf '%s\n' "$no_op_output" >&2
  exit 1
fi
if grep -Fq 'archived safely' <<<"$no_op_output"; then
  printf 'factory-contract test: zero-exit no-op printed archive success\n' >&2
  exit 1
fi

printf '#!%s\n' "$BASH" >"$fixture_root/fake-bin/openspec"
cat >>"$fixture_root/fake-bin/openspec" <<'EOF'
printf '%s\n' "$*" >>"$OPENSPEC_CALL_LOG"
if [[ "$*" == 'archive example-change --yes' ]]; then
  archive_date=${OPENSPEC_ARCHIVE_DATE:-$(LC_ALL=C date -u +%F)}
  mkdir -p "$OPENSPEC_FIXTURE_ROOT/openspec/changes/archive"
  archive_path="$OPENSPEC_FIXTURE_ROOT/openspec/changes/archive/$archive_date-example-change"
  case "${OPENSPEC_ARCHIVE_MODE:-complete}" in
    symlink)
      mv "$OPENSPEC_FIXTURE_ROOT/openspec/changes/example-change" \
        "$OPENSPEC_FIXTURE_ROOT/symlink-archive-target"
      ln -s "$OPENSPEC_FIXTURE_ROOT/symlink-archive-target" "$archive_path"
      ;;
    ambiguous)
      mv "$OPENSPEC_FIXTURE_ROOT/openspec/changes/example-change" "$archive_path"
      cp -R "$archive_path" \
        "$OPENSPEC_FIXTURE_ROOT/openspec/changes/archive/2000-01-03-example-change"
      ;;
    *)
      mv "$OPENSPEC_FIXTURE_ROOT/openspec/changes/example-change" "$archive_path"
      if [[ "${OPENSPEC_ARCHIVE_MODE:-complete}" == 'incomplete' ]]; then
        mv "$archive_path/research.md" "$archive_path/research.md.hidden"
      fi
      ;;
  esac
fi
EOF
chmod +x "$fixture_root/fake-bin/openspec"
: >"$fixture_root/openspec-calls.log"

if incomplete_output=$(cd "$fixture_root" && \
  PATH="$fixture_root/fake-bin:$PATH" \
  OPENSPEC_CALL_LOG="$fixture_root/openspec-calls.log" \
  OPENSPEC_FIXTURE_ROOT="$fixture_root" \
  OPENSPEC_ARCHIVE_MODE=incomplete \
  ./scripts/factory archive example-change 2>&1); then
  printf 'factory-contract test: incomplete archive was accepted\n' >&2
  exit 1
fi
if [[ -e "$change_root" || ! -d "$expected_archive" || -e "$expected_archive/research.md" ]]; then
  printf 'factory-contract test: incomplete archive fixture did not reach its postcondition\n' >&2
  exit 1
fi
if grep -Fq 'archived safely' <<<"$incomplete_output"; then
  printf 'factory-contract test: incomplete archive printed archive success\n' >&2
  exit 1
fi
mv "$expected_archive/research.md.hidden" "$expected_archive/research.md"
mv "$expected_archive" "$change_root"
: >"$fixture_root/openspec-calls.log"

symlink_archive="$fixture_root/openspec/changes/archive/2000-01-01-example-change"
if symlink_output=$(cd "$fixture_root" && \
  PATH="$fixture_root/fake-bin:$PATH" \
  OPENSPEC_CALL_LOG="$fixture_root/openspec-calls.log" \
  OPENSPEC_FIXTURE_ROOT="$fixture_root" \
  OPENSPEC_ARCHIVE_DATE=2000-01-01 \
  OPENSPEC_ARCHIVE_MODE=symlink \
  ./scripts/factory archive example-change 2>&1); then
  printf 'factory-contract test: symlink archive was accepted\n' >&2
  exit 1
fi
if [[ -e "$change_root" || ! -L "$symlink_archive" ]]; then
  printf 'factory-contract test: symlink archive fixture did not reach its postcondition\n' >&2
  exit 1
fi
if grep -Fq 'archived safely' <<<"$symlink_output"; then
  printf 'factory-contract test: symlink archive printed archive success\n' >&2
  exit 1
fi
rm "$symlink_archive"
mv "$fixture_root/symlink-archive-target" "$change_root"
: >"$fixture_root/openspec-calls.log"

ambiguous_archive_one="$fixture_root/openspec/changes/archive/2000-01-02-example-change"
ambiguous_archive_two="$fixture_root/openspec/changes/archive/2000-01-03-example-change"
if ambiguous_output=$(cd "$fixture_root" && \
  PATH="$fixture_root/fake-bin:$PATH" \
  OPENSPEC_CALL_LOG="$fixture_root/openspec-calls.log" \
  OPENSPEC_FIXTURE_ROOT="$fixture_root" \
  OPENSPEC_ARCHIVE_DATE=2000-01-02 \
  OPENSPEC_ARCHIVE_MODE=ambiguous \
  ./scripts/factory archive example-change 2>&1); then
  printf 'factory-contract test: ambiguous archive result was accepted\n' >&2
  exit 1
fi
if [[ -e "$change_root" || ! -d "$ambiguous_archive_one" || ! -d "$ambiguous_archive_two" ]]; then
  printf 'factory-contract test: ambiguous archive fixture did not reach its postcondition\n' >&2
  exit 1
fi
if grep -Fq 'archived safely' <<<"$ambiguous_output"; then
  printf 'factory-contract test: ambiguous archive printed archive success\n' >&2
  exit 1
fi
rm -r "$ambiguous_archive_two"
mv "$ambiguous_archive_one" "$change_root"
: >"$fixture_root/openspec-calls.log"

mismatched_archive="$fixture_root/openspec/changes/archive/2000-01-04-example-change"
success_output=$(cd "$fixture_root" && \
  PATH="$fixture_root/fake-bin:$PATH" \
  OPENSPEC_CALL_LOG="$fixture_root/openspec-calls.log" \
  OPENSPEC_FIXTURE_ROOT="$fixture_root" \
  OPENSPEC_ARCHIVE_DATE=2000-01-04 \
  ./scripts/factory archive example-change 2>&1)
if [[ -e "$change_root" || ! -d "$mismatched_archive" ]]; then
  printf 'factory-contract test: mismatched-date archive state transition is incomplete\n' >&2
  exit 1
fi
for artifact in .openspec.yaml proposal.md research.md constraints.md design.md tasks.md; do
  if [[ ! -f "$mismatched_archive/$artifact" ]]; then
    printf 'factory-contract test: successful archive lost %s\n' "$artifact" >&2
    exit 1
  fi
done
if [[ ! -d "$mismatched_archive/specs" ]]; then
  printf 'factory-contract test: successful archive lost capability deltas\n' >&2
  exit 1
fi
if ! grep -Fq 'factory: change archived safely: example-change' <<<"$success_output"; then
  printf 'factory-contract test: successful archive omitted success marker\n' >&2
  exit 1
fi

routing_root="$fixture_root/routing-repository"
mkdir -p "$routing_root/scripts/factory-tools" "$routing_root/fake-bin"
cp "$repository_root/scripts/factory" "$routing_root/scripts/factory"
cp "$repository_root/scripts/factory-tools/audit-pi.mjs" \
  "$routing_root/scripts/factory-tools/audit-pi.mjs"
chmod +x "$routing_root/scripts/factory"
git -C "$routing_root" init -q
routing_git_root=$(git -C "$routing_root" rev-parse --show-toplevel)

printf '#!%s\n' "$BASH" >"$routing_root/fake-bin/nix"
cat >>"$routing_root/fake-bin/nix" <<'EOF'
set -euo pipefail
printf '%s\n' "$*" >>"$FACTORY_NIX_LOG"
[[ "$1" == develop && "$3" == --command ]]
shift 3
IN_NIX_SHELL=1 exec "$@"
EOF
printf '#!%s\n' "$BASH" >"$routing_root/fake-bin/node"
cat >>"$routing_root/fake-bin/node" <<'EOF'
set -euo pipefail
printf 'argc=%s\n' "$#" >>"$FACTORY_NODE_LOG"
printf '<%s>\n' "$@" >>"$FACTORY_NODE_LOG"
exit "${FACTORY_NODE_EXIT:-0}"
EOF
chmod +x "$routing_root/fake-bin/nix" "$routing_root/fake-bin/node"
: >"$routing_root/nix.log"
: >"$routing_root/node.log"

(cd "$routing_root" && env -u IN_NIX_SHELL \
  PATH="$routing_root/fake-bin:$PATH" \
  FACTORY_NIX_LOG="$routing_root/nix.log" \
  FACTORY_NODE_LOG="$routing_root/node.log" \
  ./scripts/factory audit --json 'argument with spaces')
if [[ $(wc -l <"$routing_root/nix.log") -ne 1 ]] || \
  ! grep -Fq "develop $routing_git_root --command $routing_git_root/scripts/factory audit --json argument with spaces" \
    "$routing_root/nix.log" || \
  ! grep -Fq 'argc=3' "$routing_root/node.log" || \
  ! grep -Fxq '<argument with spaces>' "$routing_root/node.log"; then
  printf 'factory-contract test: outside audit did not preserve one Nix re-entry and argv\n' >&2
  printf 'factory-contract test: captured Nix calls:\n' >&2
  cat "$routing_root/nix.log" >&2
  printf 'factory-contract test: captured Node calls:\n' >&2
  cat "$routing_root/node.log" >&2
  exit 1
fi

: >"$routing_root/nix.log"
: >"$routing_root/node.log"
(cd "$routing_root" && \
  PATH="$routing_root/fake-bin:$PATH" \
  FACTORY_NIX_LOG="$routing_root/nix.log" \
  FACTORY_NODE_LOG="$routing_root/node.log" \
  IN_NIX_SHELL=1 ./scripts/factory audit --json)
if [[ -s "$routing_root/nix.log" ]] || ! grep -Fq 'argc=2' "$routing_root/node.log"; then
  printf 'factory-contract test: in-shell audit re-entered Nix or skipped direct execution\n' >&2
  exit 1
fi

if (cd "$routing_root" && env -u IN_NIX_SHELL \
  PATH="$routing_root/fake-bin:$PATH" \
  FACTORY_NIX_LOG="$routing_root/nix.log" \
  FACTORY_NODE_LOG="$routing_root/node.log" \
  FACTORY_NODE_EXIT=23 ./scripts/factory audit >/dev/null 2>&1); then
  printf 'factory-contract test: audit failure was not propagated across Nix\n' >&2
  exit 1
else
  audit_status=$?
fi
if [[ "$audit_status" -ne 23 ]]; then
  printf 'factory-contract test: audit failure changed exit status: %s\n' "$audit_status" >&2
  exit 1
fi

bootstrap_root="$fixture_root/bootstrap-repository"
mkdir -p "$bootstrap_root/scripts/tests" "$bootstrap_root/fake-bin"
printf '#!%s\n' "$BASH" >"$bootstrap_root/bootstrap.sh"
tail -n +2 "$repository_root/bootstrap.sh" >>"$bootstrap_root/bootstrap.sh"
chmod +x "$bootstrap_root/bootstrap.sh"
git -C "$bootstrap_root" init -q
printf '#!%s\n' "$BASH" >"$bootstrap_root/scripts/factory"
cat >>"$bootstrap_root/scripts/factory" <<'EOF'
set -euo pipefail
printf '%s\n' "$1" >>"$BOOTSTRAP_FACTORY_LOG"
if [[ "$1" == audit ]]; then
  exit "${BOOTSTRAP_AUDIT_EXIT:-0}"
fi
EOF
printf '#!%s\n' "$BASH" >"$bootstrap_root/fake-bin/nix"
cat >>"$bootstrap_root/fake-bin/nix" <<'EOF'
set -euo pipefail
[[ "$1" == develop && "$3" == --command ]]
shift 3
IN_NIX_SHELL=1 exec "$@"
EOF
printf '#!%s\n' "$BASH" >"$bootstrap_root/fake-bin/node"
cat >>"$bootstrap_root/fake-bin/node" <<'EOF'
set -euo pipefail
printf '%s\n' "$*" >>"$BOOTSTRAP_NODE_LOG"
EOF
chmod +x "$bootstrap_root/scripts/factory" "$bootstrap_root/fake-bin/nix" \
  "$bootstrap_root/fake-bin/node"
: >"$bootstrap_root/factory.log"
: >"$bootstrap_root/node.log"

if (cd "$bootstrap_root" && \
  PATH="$bootstrap_root/fake-bin:$PATH" \
  BOOTSTRAP_FACTORY_LOG="$bootstrap_root/factory.log" \
  BOOTSTRAP_NODE_LOG="$bootstrap_root/node.log" \
  BOOTSTRAP_AUDIT_EXIT=29 ./bootstrap.sh --check >/dev/null 2>&1); then
  printf 'factory-contract test: bootstrap accepted a failed runtime audit\n' >&2
  exit 1
else
  bootstrap_status=$?
fi
if [[ "$bootstrap_status" -ne 29 ]] || \
  [[ $(paste -sd, "$bootstrap_root/factory.log") != check,audit ]] || \
  [[ -s "$bootstrap_root/node.log" ]]; then
  printf 'factory-contract test: bootstrap did not fail fast after runtime audit\n' >&2
  exit 1
fi

: >"$bootstrap_root/factory.log"
(cd "$bootstrap_root" && \
  PATH="$bootstrap_root/fake-bin:$PATH" \
  BOOTSTRAP_FACTORY_LOG="$bootstrap_root/factory.log" \
  BOOTSTRAP_NODE_LOG="$bootstrap_root/node.log" \
  ./bootstrap.sh --check)
if [[ $(paste -sd, "$bootstrap_root/factory.log") != check,audit ]] || \
  ! grep -Fxq -- '--test scripts/tests/factory-operations.mjs' "$bootstrap_root/node.log"; then
  printf 'factory-contract test: bootstrap health stages were incomplete or unordered\n' >&2
  exit 1
fi

printf 'factory-contract tests: passed\n'
