# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-15
Source retrieval date: 2026-09-15
Research blockers: none

## Problem and existing implementation

The inspected repository is `hyperledger-identus/sdk-rust` at
`develop@105308771dbebceb473b99d9eb82b0fa0178ab09`. Issue #279 owns this
slice and #271 remains the umbrella.

`crates/presentations/src/error.rs` is 417 physical lines. Its private
`contract()` occupies 190 lines and has 48 wildcard-free arms. One fieldless,
non-exhaustive public enum and 48 public code constants are involved. The
public bridge is `const`; `Display` and the bridge already share the same
tuple. All messages are static and identical across local/public surfaces,
all kinds are `InvalidInput`, all capabilities are `presentation`, and
`std::error::Error::source()` is empty.

The current 205-line integration test lists all 48 variants and codes and
checks kind/capability/non-empty redaction. It does not pin exact messages,
complete public display, constant name/visibility, or source behavior.

## Normative sources

The controlling sources are issues #271/#279, ADRs 0110/0116/0117, canonical
core-error and presentation specifications, and the exact baseline source and
tests. No external standard, donor repository, dependency, or mutable web
source is needed for this internal behavior-preserving refactor.

ADR 0116 is evidence, not authority: it explicitly applies only to credentials
and requires later crates to decide independently. ADR 0117 selects the leaner
shape for this homogeneous catalogue.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Five private two-field domain catalogues | adopt | Improves review locality without repeating uniform kind/capability. |
| Credentials five-field record unchanged | not-adopt | Repeats invariant data and mistakes a pilot shape for a standard. |
| Keep one 190-line match | not-adopt | Behavior is correct but the measured review hotspot remains. |
| Shared error crate/trait or procedural macro | not-adopt | Adds coupling and a release/tooling axis before two crate-local results exist. |
| Post-refactor generated golden | not-adopt | Lets the implementation bless its own drift. |

## Selected grouping

| Group | Variants | Count |
| --- | --- | ---: |
| Request and query | `InvalidQueryId` through `DuplicateQueryId` | 13 |
| Candidate matching | `InvalidCandidateClaims` through `CandidateRequestMismatch` | 9 |
| Disclosure selection | `InvalidSelectionClaims` through `DisclosureRequestMismatch` | 13 |
| Artifact assembly | `InvalidArtifactBindings` through `MissingArtifactSelection` | 9 |
| Lifecycle | `InvalidLifecyclePhase` through `InvalidProtocolTransition` | 4 |

## Compatibility and dependency evidence

- Pin all 48 constant identities/paths/values, exact displays, kind,
  capability, const bridge, and source-free semantics.
- Preserve enum order/derives/non-exhaustive marker, `From`, `Display`, `Error`,
  public modules and absence of Serde/bindings.
- Keep messages `&'static str`; never capture claims, credentials, holder or
  verifier inputs, identifiers, parser details, or causes.
- Keep manifests, dependency graph, features and lockfile unchanged.
- Prove the planning/stable golden copies are immutable, independent,
  unambiguous, regular and non-symlinked before and after archive.

## Security, privacy and maintenance evidence

All records contain only public static codes and `&'static str`. They cannot
capture claims, credentials, holder/verifier inputs, identifiers, parser
details, sources or causes. The fixed-size golden is test-only, and malformed,
duplicated, mutated or aliased inputs fail rather than influencing production.
The five groups preserve domain ownership without a new shared release axis.

## Rejected or deferred candidates

The candidate table records the five-field pilot copy, retained large match,
shared trait/crate or procedural macro, and generated golden as `not-adopt`.
A cross-crate compile-time helper remains deferred until the presentation and
JOSE results show genuinely identical irreducible mechanics.

## Gates and limitations

Run focused/default/minimal/all-feature tests, strict Clippy, docs, format,
public API comparison, direct WASM/Android/iOS package checks, workspace Nix
tests, factory and source-filter gates. The current generic Nix target package
lists do not directly include presentations, so direct package compile checks
are required and do not imply runtime/device/binding support.

No retryability field exists in the current core error model. This slice does
not invent one. It also makes no runtime, consumer, presentation-exchange,
wire, localization, or downstream adoption claim.

The exact planning golden contains 52 LF lines (48 data rows), 15,892 bytes,
and SHA-256
`3941cbdb1b3eedb26243b5caf1b8a11c4646c3789a3cab1415834c48d3a8ba49`.

## Evidence commands

- Source audit found 48 enum variants and 48 public constants.
- The existing contract measured 190 physical lines and 48 match arms.
- `awk` schema validation found 48 rows and zero non-11-column rows.
- `shasum -a 256` produced the immutable digest above.
- `git diff 353030a..1053087 -- crates/presentations` is empty, confirming the
  audit remained valid after the credentials-only merge.
- Implementation, public API, target and Nix commands remain deliberately
  unrun planning evidence.

## Open questions and blockers

No blocker remains. If the five-way split cannot reduce the largest review unit
without obscuring ownership or adding decision sites, implementation stops and
records retention of the current router rather than forcing ADR 0117.
