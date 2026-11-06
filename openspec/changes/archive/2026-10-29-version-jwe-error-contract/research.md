# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

At exact base `c66d506690d3cd6f119f9ae6430b285f98115d10`,
`JweEncryptionError` is a separate public non-exhaustive eight-variant enum.
Its wildcard-free `to_identus_error` match owns stable inline codes, two kinds,
the shared `jose` capability, static messages, and source-free conversion.
`crates/jose/tests/jwe_port.rs` checks the category list only for a code prefix
and a generic redaction canary.

`JoseError` already has a private declarative contract and immutable historical
golden under ADR 0118. ADR 0137 permits a suffix for that same enum without
rewriting its baseline. Those patterns are evidence, not a requirement to
couple the later JWE enum to the JWS-oriented fixture.

## Normative sources

The exact repository base and accepted ADRs 0118, 0137, 0181, and 0192 are the
authorities for current public behavior and compatibility policy. RFC 7516 is
the primary JWE protocol source, but it deliberately does not define an SDK
error taxonomy. Issue #539 authorizes this bounded evidence change.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Append JWE rows to `jose-error-contract-v2.csv` | `not-adopt` | Couples two enum order/version axes and makes one fixture's type scope ambiguous. |
| Rewrite the historical JOSE golden | `not-adopt` | Destroys independent historical evidence. |
| Independent JWE v1 golden with the shared schema | `adopt` | Preserves exact behavior and lets each public enum evolve independently. |
| Publish JWE code constants | `not-adopt` | Adds API without consumer need; the contract can pin literal stable codes. |
| Generate the golden from current source | `not-adopt` | Lets implementation authorize its own drift. |
| Keep only ad hoc tests | `not-adopt` | Does not bind provenance, order, or complete exact behavior. |

## Compatibility and dependency evidence

The adopted design changes no public item, enum order, manifest, feature,
dependency, lockfile, target policy, allocation path, or runtime result. The
golden uses the existing eleven-column schema and records `none` for code
constant name and visibility because no such public API exists.

The planning golden is independently authored from the exact base, committed
before implementation, and later copied byte-for-byte to the stable test path.
The factory checker binds it to the planning commit and fixed SHA-256.

## Security, privacy and maintenance evidence

All messages remain static and reveal no key, plaintext, ciphertext, compact
JWE, state, or engine diagnostic. Exact local/public/full display and empty
source behavior are tested for every category. The implementation remains
engine-neutral and contains no cryptographic composition.

One private record and one wildcard-free match keep mapping decisions local.
The eight-row inventory is small enough to review directly. A future variant
must be mapped at compile time and tested as an append-only suffix; changing a
v1 row requires a new compatibility decision.

## Rejected or deferred candidates

All non-adopted candidates in the decision table are rejected for this slice.
Public code constants remain a future option only if an adopter demonstrates a
source-level need. A broader shared error framework remains out of scope. The
production JWE engine is deferred to #566 and cannot be inferred from this
compatibility work.

## Open questions and blockers

There are no blockers. The only future policy question is whether an externally
released consumer will require named public JWE code constants; the current
string-code contract is sufficient and adding constants now would be
speculative API expansion.

## Evidence commands

Focused evidence uses `cargo test -p identus-jose --all-features`, the immutable
golden checker and its mutation suite, `scripts/factory check`, workspace
format/Clippy/tests/docs, `nix flake check`, public API/dependency diffs, and a
distinct Claude or Codex CLI review. No network or downstream mutation is
needed for the behavior-neutral contract.
