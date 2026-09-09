#!/usr/bin/env bash

set -euo pipefail

if repository_root=$(git rev-parse --show-toplevel 2>/dev/null); then
  :
else
  repository_root=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
fi

nix_binary() {
  if command -v nix >/dev/null 2>&1; then
    command -v nix
  elif [[ -x /nix/var/nix/profiles/default/bin/nix ]]; then
    printf '%s\n' /nix/var/nix/profiles/default/bin/nix
  else
    printf 'bootstrap: Nix is required\n' >&2
    exit 1
  fi
}

run_in_shell() {
  local nix_cmd
  nix_cmd=$(nix_binary)
  "$nix_cmd" develop "$repository_root" --command "$@"
}

case "${1:-shell}" in
  shell)
    exec "$(nix_binary)" develop "$repository_root"
    ;;
  --pi)
    shift
    run_in_shell node scripts/factory-tools/audit-pi.mjs --enforce-config
    exec "$(nix_binary)" develop "$repository_root" --command pi "$@"
    ;;
  --audit-pi)
    shift
    run_in_shell node scripts/factory-tools/audit-pi.mjs "$@"
    ;;
  --configure-pi)
    shift
    run_in_shell node scripts/factory-tools/pi-policy.mjs apply --execute "$@"
    ;;
  --configure-git)
    shift
    run_in_shell node scripts/git-hooks/configure.mjs apply --execute "$@"
    ;;
  --check)
    shift
    run_in_shell bash -c 'scripts/factory check && node --test scripts/tests/factory-operations.mjs' bash "$@"
    ;;
  --)
    shift
    (($# > 0)) || { printf 'bootstrap: command required after --\n' >&2; exit 2; }
    exec "$(nix_binary)" develop "$repository_root" --command "$@"
    ;;
  --help | -h)
    printf '%s\n' \
      'Usage: ./bootstrap.sh [shell|--pi|--audit-pi|--configure-pi|--configure-git|--check|-- COMMAND]' \
      '' \
      'Configuration mutations are explicit. The default only enters the pinned Nix shell.'
    ;;
  *)
    printf 'bootstrap: unknown option: %s\n' "$1" >&2
    exit 2
    ;;
esac
