## Context

The factory facade already self-enters Nix for OpenSpec when required, while
the audit subcommand directly executes its Node implementation. Bootstrap has
a reusable `run_in_shell` boundary but its `--check` composition omits runtime
audit. The correction should reuse these boundaries and avoid a second runtime
detector.

## Decisions

### Route the audit through the facade exactly once

When `IN_NIX_SHELL` is absent, the audit subcommand resolves Nix through the
existing `nix_binary` helper and invokes the same absolute factory facade under
`nix develop <root> --command`. Inside a Nix shell it directly executes
`audit-pi.mjs`. The audit implementation remains the single source of runtime
truth.

### Compose bootstrap health checks from public factory commands

`bootstrap.sh --check` runs `scripts/factory check`, `scripts/factory audit`
and the operational Node tests in order inside one pinned shell. `set -e`
propagates the first failure, so a later successful command cannot hide a
failed audit.

### Test routing without launching a real nested Nix evaluation

The shell contract receives a hermetic fake Nix executable and fixture factory
root where needed. Tests assert preserved argv, one re-entry, direct in-shell
dispatch and non-zero propagation. The live repository commands remain the
end-to-end evidence.

## Risks and mitigations

- Recursive Nix invocation: the Nix shell marker selects direct execution.
- Audit under an unrelated Nix shell: direct execution intentionally lets the
  audit reject wrong Pi, wrappers or hooks.
- Quoting regression: commands pass separate arguments and do not evaluate
  issue or user text.
- Longer `--check`: runtime audit is expected health evidence; elapsed time is
  recorded in verification.

## Rollback

Revert the facade/bootstrap/test/documentation changes. Operators can continue
using `./bootstrap.sh --audit-pi` explicitly.
