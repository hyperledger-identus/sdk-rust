#!/usr/bin/env bash

set -euo pipefail

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
"$repository_root/scripts/tests/bootstrap-inventory.py"

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
  docs/architecture/sdk-bootstrap-inventory.md
  docs/architecture/sdk-bootstrap-inventory.toml
  docs/architecture/sdk-support-policy.md
  docs/architecture/sdk-support-policy.toml
  docs/architecture/ssi-upstream-source-matrix.md
  docs/roadmap/ssi-upstream-dependency-backlog.csv
  docs/governance/agentic-sdlc.md
  docs/governance/repository-settings.md
  docs/adr/0003-delegate-develop-integration.md
  nix/checks/gates.toml
  nix/checks/rust-gates.nix
  openspec/config.yaml
  scripts/benchmark-support-policy.py
  scripts/factory
  scripts/check-factory.sh
  scripts/check-bootstrap-inventory.py
  scripts/check-support-policy.py
  scripts/check-ssi-upstream-backlog.py
  scripts/check-pr-policy.sh
  scripts/tests/factory-contract.sh
  scripts/tests/bootstrap-inventory.py
  scripts/tests/pr-policy.sh
  scripts/tests/support-policy.py
  .github/CODEOWNERS
  .github/ISSUE_TEMPLATE/component-change.yml
  .github/ISSUE_TEMPLATE/delivery-task.yml
  .github/pull_request_template.md
  .github/workflows/factory-contract.yml
  .github/workflows/pull-request-policy.yml
)

for relative_path in "${required_files[@]}"; do
  mkdir -p "$fixture_root/$(dirname "$relative_path")"
  case "$relative_path" in
    .github/CODEOWNERS | .github/ISSUE_TEMPLATE/component-change.yml | .github/ISSUE_TEMPLATE/delivery-task.yml | .github/pull_request_template.md | CODE_OF_CONDUCT.md | CONTRIBUTING.md | DCO.md | GOVERNANCE.md | LICENSE | MAINTAINERS.md | RELEASING.md | SECURITY.md | docs/architecture/sdk-bootstrap-inventory.md | docs/architecture/sdk-bootstrap-inventory.toml | docs/architecture/sdk-support-policy.md | docs/architecture/sdk-support-policy.toml | docs/architecture/ssi-upstream-source-matrix.md | docs/governance/repository-settings.md | docs/roadmap/ssi-upstream-dependency-backlog.csv | nix/checks/gates.toml | nix/checks/rust-gates.nix | scripts/benchmark-support-policy.py | scripts/check-bootstrap-inventory.py | scripts/check-support-policy.py | scripts/check-ssi-upstream-backlog.py | scripts/tests/bootstrap-inventory.py | scripts/tests/support-policy.py)
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
  "$fixture_root/scripts/check-pr-policy.sh" \
  "$fixture_root/scripts/check-support-policy.py" \
  "$fixture_root/scripts/check-ssi-upstream-backlog.py" \
  "$fixture_root/scripts/tests/bootstrap-inventory.py" \
  "$fixture_root/scripts/tests/factory-contract.sh" \
  "$fixture_root/scripts/tests/pr-policy.sh" \
  "$fixture_root/scripts/tests/support-policy.py"

for relative_path in Cargo.toml flake.nix flake.lock \
  docs/adr/0002-neoprism-toolchain-alignment.md nix/rust-toolchain.nix; do
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

printf 'factory-contract tests: passed\n'
