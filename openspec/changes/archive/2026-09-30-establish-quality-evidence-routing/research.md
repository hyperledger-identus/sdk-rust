# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-30
Source retrieval date: 2026-09-30
Research blockers: none

## Problem and existing implementation

The current implementation has four mature but separate evidence mechanisms:

- deterministic property-shaped DID grammar, constructor/deserializer, limit,
  allocation, and redaction tests in `crates/did/tests/did_syntax.rs`;
- bounded `did` and `did_url` libFuzzer targets, corpora, dictionaries, a
  pinned sanitizer shell, and a weekly workflow;
- measurement-only benchmark contracts for crypto and repository tooling; and
- the #420 language-neutral DID packet exercised by
  `crates/conformance/tests/cross_language_did_vectors.rs`.

The factory diff plan decides required PR and recommended slow infrastructure
from changed paths. It does not describe why a capability needs a specific
quality technique, which command/selector is authoritative, when evidence is
fresh, or who owns missing evidence. ADR 0173 and issue #501 assign that gap to
an independent A1 record consumed later by #422 and #492.

## Normative sources

- SDK-Rust base `d3543e3eec42dd519c57a7250950c01fa628145a` contains the DID
  syntax tests, #420 packet/catalog, DID fuzz targets and factory lane policy.
- Cross-language packet `did.syntax.v1` contains 22 positive, negative,
  boundary, redaction, and consumer-regression vectors with immutable source
  provenance.
- The latest natural DID fuzz schedule available at planning time is successful
  run `36517752658` on protected `develop` revision
  `d27e455901c501d9611abbe0065e6a9b7270dd57`, completed 2026-09-29.
- ADR 0127 keeps `fast` as the required active-development line and slow
  evidence as weekly/manual production-promotion input.
- ADR 0173 requires every A1 declaration to cover property, fuzz, benchmark,
  and differential dispositions while preserving standards/evidence authority.

The primary source URL is
https://github.com/hyperledger-identus/sdk-rust/tree/d3543e3eec42dd519c57a7250950c01fa628145a.
The exact version and features assessed are registry schema v1, DID packet v1,
the featureless generic `identus-did` parser surface, the
`identus-conformance` test-only loader, `cargo-fuzz 0.13.2`, and the pinned
`nightly-2026-03-18` sanitizer shell.

License and provenance are explicit: all assessed repository content is
Apache-2.0, and every source above is bound to its immutable revision or run.
The scheduled run is an exact receipt for existing repository-owned code; it
is not copied into the tree.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Closed TOML declaration registry | `adopt` | Matches existing offline governance contracts and remains reviewable without a new runtime dependency. | Cross-repository consumers require a generated interchange schema. |
| Python standard-library validator and renderer | `adopt` | Reuses the established factory pattern and can resolve repository paths plus #420 vector IDs offline. | Scale or consumers justify a compiled/shared validator. |
| Extend existing test frameworks | `adopt` | Declarations point to exact commands and selectors rather than wrapping or replacing Cargo/libFuzzer. | A technique cannot be expressed without unsafe string execution. |
| Put property, fuzz, benchmark, and differential work in every PR | `not-adopt` | Ignores risk/cost and conflicts with the accepted fast/slow split. | CI policy is superseded by measured evidence. |
| Treat coverage or a generic CI URL as complete evidence | `not-adopt` | Neither identifies the exercised invariant, input domain, oracle, budget, or target. | Never without an exact declaration and receipt. |
| Require a DID parser benchmark for A1 | `not-adopt` | No consumer latency/throughput outcome or regression budget exists; measuring it here would create vanity evidence. | A named consumer or release profile defines a comparable operation and budget. |
| Import an external quality-orchestration dependency | `not-adopt` | Adds supply-chain/coupling cost without replacing the existing test engines. | At least two independent repositories need the same executable registry engine. |

## Compatibility and dependency evidence

The registry is repository metadata. Its direct and resolved dependency cone
is unchanged: it adds no Cargo dependency, public Rust API, wire format,
persisted data, target, unsafe code, or production edge.
Rust 1.89.0 MSRV, Rust 1.98.1 primary/etalon, the existing fuzz nightly, and
the required `fast` status remain unchanged.

Each active declaration contains exactly one obligation for each of the four
classes. Evidence is either satisfied with exact source/run receipts, required
with visible owned debt, or reviewed not applicable. Class-specific detail
keys prevent a selector plus generic prose from satisfying the wrong technique.
The DID seed uses #420 vector IDs rather than duplicating packet payload.

The planned factory quality view is declarative and non-executing. It groups
commands into focused, fast, slow, and release routes but cannot promote slow
evidence into a required PR status or authorize a release. The existing diff
plan and branch protections remain authoritative for integration.

The facade boundary is the factory command and checked-in rendering; no
registry type crosses into a Rust crate or consumer API. Public and wire
compatibility therefore remain unchanged. Protocol or draft currency is not
applicable to the quality metadata itself; referenced DID behavior remains
bound to the final DID Core 1.0 packet selected by #420 rather than a moving
draft.

## Security, privacy and maintenance evidence

The registry admits commands and locators as data only; the validator never
executes them. It rejects control characters, unsafe paths, symlinks,
unresolved selectors/vectors, unknown fields, duplicate IDs, incomplete class
sets, incoherent route/cadence pairs, malformed run receipts, stale receipts,
unreviewed `not-applicable` outcomes, and open debt without an owner and issue.

Receipts contain public repository revisions, dates, paths, and GitHub run
URLs only. They contain no prompts, session identifiers, credentials, private
fixtures, personal data, production identifiers, or secrets. Deterministic
rendering prevents the human plan from drifting from the machine registry.

Unsafe and native-code evidence is unchanged: authored code remains under the
workspace unsafe prohibition, while the existing libFuzzer sanitizer runtime
is an isolated tooling input rather than a production native dependency. The
supply-chain evidence adds only repository-owned Python/TOML/Markdown and the
already pinned workflow/toolchain identities.

The declaration is snapshot evidence evaluated at a recorded date. A bounded
freshness receipt proves that evidence was current at that evaluation; release
and #422 must separately require a current exact-candidate receipt rather than
claiming this historical snapshot is perpetually fresh.

Maintenance, release, and security posture are unchanged. Before merge,
rollback removes the registry/checker/template additions. After stable IDs
merge, rollback or correction uses an explicit schema version and replacement
record rather than rewriting historical receipts.

## Rejected or deferred candidates

Automatic workflow dispatch, benchmark threshold invention, coverage gates,
consumer E2E, generated language bindings, remote-source retrieval during CI,
and conversion of existing domain-specific evidence registries are deferred.
Future declarations may reference those assets after their owning issues define
stable identifiers and exact evidence.

## Open questions and blockers

None. The three satisfied DID classes, reviewed benchmark disposition, stable
#420 packet, lane split, freshness semantics, debt ownership, and downstream
stop boundary are sufficiently bounded for implementation.

## Evidence commands

Planning evidence uses the factory research, constraint, and strict validation
commands for this selected change. Implementation will add focused
validator/render/mutation commands before the full factory gate; those commands
are intentionally unrun at planning time.
