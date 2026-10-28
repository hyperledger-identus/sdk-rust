# Design

## One fail-closed guard

Add a repository-owned script that receives the repository root and optional
clock override used only by deterministic tests. It validates an ISO date,
rejects dates after 2027-01-31, obtains the locked inverse tree for
`basic-toml@0.1.10`, and byte-compares its normalized output with the reviewed
seven-node path. It then runs generator `cargo deny` and `cargo audit` with
exactly one ignored advisory ID.

The workflow calls this script instead of spelling the two supply-chain
commands inline. Host binding generation and execution remain separate commands
after the guard and are unchanged.

## Testability

The script exposes command paths through test-only environment variables so a
fixture can prove invocation arguments and failure order without network or a
mutable advisory database. Tests cover current date success, expiry failure,
malformed date, graph drift, audit failure and deny failure. Production CI does
not set those variables.

## Documentation and ownership

ADR 0191 owns the rationale and exit rules. A knowledge-base page records the
live command, accepted path, natural-run semantics and upstream monitoring
trigger. Issue #564 remains open until protected CI is green; upstream removal
or expiry requires a new issue and preimplementation receipt.

## Failure policy

The guard never falls back to an unscoped audit. Any unexpected output, missing
tool, changed dependency, failed policy check, additional advisory or expired
date returns non-zero before host E2E execution. The natural schedule is not
manually dispatched or rerun as a substitute for the next immutable receipt.
