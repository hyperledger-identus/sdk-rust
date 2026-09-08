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
  nix/checks/gates.toml
  nix/checks/rust-gates.nix
  openspec/config.yaml
  scripts/benchmark-support-policy.py
  scripts/factory
  scripts/check-factory.sh
  scripts/check-bootstrap-inventory.py
  scripts/check-constraints.py
  scripts/check-openspec-archive.py
  scripts/check-support-policy.py
  scripts/check-apollo-parity.py
  scripts/check-ssi-upstream-backlog.py
  scripts/check-pr-policy.sh
  scripts/check-research-readiness.py
  scripts/tests/factory-contract.sh
  scripts/tests/bootstrap-inventory.py
  scripts/tests/constraints.py
  scripts/tests/openspec-archive.py
  scripts/tests/pr-policy.sh
  scripts/tests/research-readiness.py
  scripts/tests/support-policy.py
  scripts/tests/apollo-parity.py
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
    .github/CODEOWNERS | .github/ISSUE_TEMPLATE/component-change.yml | .github/ISSUE_TEMPLATE/delivery-task.yml | .github/pull_request_template.md | CODE_OF_CONDUCT.md | CONTRIBUTING.md | DCO.md | GOVERNANCE.md | LICENSE | MAINTAINERS.md | RELEASING.md | SECURITY.md | docs/architecture/sdk-bootstrap-inventory.md | docs/architecture/sdk-bootstrap-inventory.toml | docs/architecture/sdk-rust-blueprint.md | docs/architecture/sdk-support-policy.md | docs/architecture/sdk-support-policy.toml | docs/architecture/apollo-crypto-parity.toml | docs/architecture/ssi-upstream-source-matrix.md | docs/factory/research-readiness.md | docs/governance/constraints-and-limitations.md | docs/governance/repository-settings.md | docs/governance/sdk-constraints.toml | docs/adr/0001-bootstrap-branch-selection.md | docs/adr/0003-delegate-develop-integration.md | docs/adr/0062-use-a-rolling-near-current-msrv.md | docs/adr/0063-make-material-constraints-explicit.md | docs/adr/0064-separate-primary-rust-from-evidence-driven-msrv.md | docs/adr/0081-use-temporary-rust-198-fast-slow-ci.md | docs/adr/0087-enforce-first-party-unsafe-forbid.md | docs/adr/0088-reject-unsafe-first-party-macro-output.md | docs/adr/0089-require-upstream-first-dependency-remediation.md | docs/roadmap/ssi-upstream-dependency-backlog.csv | nix/checks/gates.toml | nix/checks/rust-gates.nix | scripts/benchmark-support-policy.py | scripts/check-bootstrap-inventory.py | scripts/check-constraints.py | scripts/check-openspec-archive.py | scripts/check-research-readiness.py | scripts/factory | scripts/check-support-policy.py | scripts/check-apollo-parity.py | scripts/check-ssi-upstream-backlog.py | scripts/tests/bootstrap-inventory.py | scripts/tests/constraints.py | scripts/tests/openspec-archive.py | scripts/tests/research-readiness.py | scripts/tests/support-policy.py | scripts/tests/apollo-parity.py)
      cp "$repository_root/$relative_path" "$fixture_root/$relative_path"
      ;;
    *)
      : >"$fixture_root/$relative_path"
      ;;
  esac
done
chmod +x "$fixture_root/scripts/factory" "$fixture_root/scripts/check-factory.sh" \
  "$fixture_root/scripts/benchmark-support-policy.py" \
  "$fixture_root/scripts/check-bootstrap-inventory.py" \
  "$fixture_root/scripts/check-constraints.py" \
  "$fixture_root/scripts/check-openspec-archive.py" \
  "$fixture_root/scripts/check-pr-policy.sh" \
  "$fixture_root/scripts/check-research-readiness.py" \
  "$fixture_root/scripts/check-support-policy.py" \
  "$fixture_root/scripts/check-apollo-parity.py" \
  "$fixture_root/scripts/check-ssi-upstream-backlog.py" \
  "$fixture_root/scripts/tests/bootstrap-inventory.py" \
  "$fixture_root/scripts/tests/constraints.py" \
  "$fixture_root/scripts/tests/factory-contract.sh" \
  "$fixture_root/scripts/tests/openspec-archive.py" \
  "$fixture_root/scripts/tests/pr-policy.sh" \
  "$fixture_root/scripts/tests/research-readiness.py" \
  "$fixture_root/scripts/tests/support-policy.py"
chmod +x "$fixture_root/scripts/tests/apollo-parity.py"

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
expected_archive="$fixture_root/openspec/changes/archive/$(LC_ALL=C date +%F)-example-change"

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
  archive_date=$(LC_ALL=C date +%F)
  mkdir -p "$OPENSPEC_FIXTURE_ROOT/openspec/changes/archive"
  mv "$OPENSPEC_FIXTURE_ROOT/openspec/changes/example-change" \
    "$OPENSPEC_FIXTURE_ROOT/openspec/changes/archive/$archive_date-example-change"
  if [[ "${OPENSPEC_ARCHIVE_MODE:-complete}" == 'incomplete' ]]; then
    mv "$OPENSPEC_FIXTURE_ROOT/openspec/changes/archive/$archive_date-example-change/research.md" \
      "$OPENSPEC_FIXTURE_ROOT/openspec/changes/archive/$archive_date-example-change/research.md.hidden"
  fi
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

success_output=$(cd "$fixture_root" && \
  PATH="$fixture_root/fake-bin:$PATH" \
  OPENSPEC_CALL_LOG="$fixture_root/openspec-calls.log" \
  OPENSPEC_FIXTURE_ROOT="$fixture_root" \
  ./scripts/factory archive example-change 2>&1)
if [[ -e "$change_root" || ! -d "$expected_archive" ]]; then
  printf 'factory-contract test: successful archive state transition is incomplete\n' >&2
  exit 1
fi
for artifact in .openspec.yaml proposal.md research.md constraints.md design.md tasks.md; do
  if [[ ! -f "$expected_archive/$artifact" ]]; then
    printf 'factory-contract test: successful archive lost %s\n' "$artifact" >&2
    exit 1
  fi
done
if [[ ! -d "$expected_archive/specs" ]]; then
  printf 'factory-contract test: successful archive lost capability deltas\n' >&2
  exit 1
fi
if ! grep -Fq 'factory: change archived safely: example-change' <<<"$success_output"; then
  printf 'factory-contract test: successful archive omitted success marker\n' >&2
  exit 1
fi

printf 'factory-contract tests: passed\n'
