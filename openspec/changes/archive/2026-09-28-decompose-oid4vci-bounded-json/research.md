# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-28
Source retrieval date: 2026-09-28
Research blockers: none

## Problem and existing implementation

The current implementation is one 2,183-line source file with eleven
crate-private entry points, sixteen field records, one cursor scanner, twelve
protocol-object parsers, collection parsers, and shared lexical/object
mechanics. Public integration tests already exercise each entry point's valid,
duplicate, malformed, limit, and redaction behavior. The module concentration
is architectural; no missing wire feature or external implementation is being
ported.

## Normative sources

OpenID4VCI 1.0 Final remains the protocol source. Existing canonical OpenSpec
capabilities and error goldens own accepted wire behavior and error projection.
ADR 0115 and the code-health specification require semantic decomposition,
not metric gaming. Effective constraints `SDK-SEC-001` and `SDK-SEC-003`
preserve unsafe-code prohibition and bounded untrusted input.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Shared scanner plus four private protocol modules | `adopt` | Aligns ownership with independent protocol objects while single-owning lexical and resource invariants. | Two groups acquire the same normative change cadence and invariant set. |
| Keep one large file | `not-adopt` | Hides four independent reasons to change and weakens focused review. | The protocol groups collapse into one grammar in a future standard. |
| Split by arbitrary line ranges or one file per function | `not-adopt` | Changes file metrics without improving cohesion or review boundaries. | Never; boundaries must remain semantic. |
| Adopt a generic JSON/event-parser crate | `not-adopt` | Changes dependency, allocation, duplicate, error-precedence, and cleanup assumptions without consumer benefit. | A separate issue proves exact behavioral and dependency advantages. |
| Share scanner/parser code with OID4VP | `not-adopt` | Similar syntax does not establish shared limits, errors, ownership, or change cadence. | Differential evidence establishes identical invariants and owners. |

## Compatibility and dependency evidence

The crate-private `json` facade and all call sites remain unchanged. No public
or wire contract, Cargo feature, dependency, lockfile, MSRV, target, or error
golden changes. The direct and resolved dependency cone is identical. Parser
files remain inside `identus-oid4vci`; no new facade or cross-crate boundary is
introduced.

## Security, privacy and maintenance evidence

One scanner continues to own node/depth budgets, duplicate member detection,
complete-input checks, numeric/string scanning, and zeroizing decoded strings.
Protocol modules retain their exact limit types and static redaction-safe
errors. No unsafe or native code is introduced. The move narrows maintenance
review surfaces while preserving the existing supply-chain and release
posture. Rollback is a private-file recombination with no migration.

## Rejected or deferred candidates

Third-party parser adoption, a public parser API, cross-protocol abstraction,
resource-limit redesign, error cleanup, and `limits.rs` decomposition are
rejected or deferred. Issue #404 separately owns the limits-module concern.

## Open questions and blockers

There are no blockers. Review must verify exact error precedence and that
every protocol parser remains reachable only through the shared bounded root
contract. A new dependency, public type, or behavior delta blocks the slice.

## Evidence commands

Planning inspected revision `21d9f652d050cfc029a821aa6f522aa162ff0226`,
all `json.rs` entry points/methods, their public callers, OID4VCI integration
tests, error goldens, code-health policy, and issue #403. Before moving code,
run `cargo test -p identus-oid4vci --all-features`. Afterward run the same
suite, strict Clippy, formatting, conformance/error-golden checks, code-health
report verification, factory checks, relevant Nix gates, and protected CI.
Those implementation commands are unrun at planning time.
