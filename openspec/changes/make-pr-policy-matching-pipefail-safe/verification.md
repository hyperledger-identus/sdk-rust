# Verification

## Focused behavior

At implementation head `8c265134f773060ce719c7eee5a0d9c197ecce3b`:

```text
scripts/tests/pr-policy.sh
  passed on macOS Bash 3.2

/nix/store/...-bash-interactive-5.3p15/bin/bash scripts/tests/pr-policy.sh
  passed on pinned Linux Bash 5.3

node --test scripts/tests/factory-operations.mjs
  55 passed, 0 failed

bash -n scripts/check-pr-policy.sh scripts/tests/pr-policy.sh
git diff --check
  passed
```

The focused regressions validate 60 KiB bodies with required metadata at the
beginning and end, exact issue extraction, unchanged negative cases, the real
file-backed `scripts/factory delivery pr-preflight` route, and removal of the
private temporary representation.

## Factory and OpenSpec

```text
scripts/factory check
  86 OpenSpec/factory items passed, 0 failed

scripts/factory preflight make-pr-policy-matching-pipefail-safe \
  --issue 374 --validate-receipt
  exact planning receipt passed
```

## Cleaned-source Nix evidence

The Darwin Nix `factory-contract` derivation reached and passed the relevant
`pr-policy tests` under patched pinned interpreters. It then entered the known
nested factory-contract self-test and was stopped; it is not reported as a
complete local Nix receipt. The direct factory check and focused cross-Bash
evidence are green. The protected hosted exact-head `fast` lane
must complete before merge.

## Unchanged surfaces

No Rust source, Cargo metadata, dependency graph, lockfile, feature, target,
workflow, public API, wire format, release asset or consumer repository
changed. Rust build/test matrices are therefore delegated to the unchanged
weekly slow line rather than duplicated for this factory-only PR.
