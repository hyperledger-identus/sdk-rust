# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-15
Source retrieval date: 2026-09-15
Research blockers: none

## Problem and existing implementation

The inspected repository is `hyperledger-identus/sdk-rust` at
`develop@c32c1c8cd0194466a8c7fbfaa8c050f4bf2971a1`. Issue #280 owns this
slice and #271 remains the umbrella.

`crates/jose/src/error.rs` is 471 physical/461 nonblank lines. Its public const
bridge occupies 214 lines: a 187-line code/message match plus a 24-line kind
match and wildcard. There are 51 fieldless non-exhaustive enum variants, 51
public constants, five kinds, one capability, identical static local/public
messages, and no sources. Whole JOSE production source is 3,264 physical/2,983
nonblank lines. Tests discover 57 cases (53 pass, four ignored diagnostics),
but `SizeOverflow` is the sole variant without a named test reference.

## Normative sources

Issues #271/#280, ADRs 0110/0116/0117/0118, canonical core-error and JOSE
specifications, and exact baseline source/tests control this internal refactor.
No external standard, donor repository, dependency, or mutable web source is
needed. The earlier ADRs are evidence, not authority for JOSE's record shape.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Four private three-field catalogues | `adopt` | Expresses five kinds once per row while centralizing uniform capability/message. |
| Keep the two existing matches | `not-adopt` | Retains two mapping sites and a silent wildcard default. |
| Reuse credentials/presentations shape | `not-adopt` | Repeats invariants or cannot express heterogeneous kind. |
| Shared error crate/trait or procedural macro | `not-adopt` | Adds coupling and a release/tooling axis. |
| Post-refactor generated golden | `not-adopt` | Lets implementation authorize its own drift. |

## Selected grouping

| Group | Count | Boundary |
| --- | ---: | --- |
| Compact/header | 15 | `InvalidLimits` through `SizeOverflow` |
| Algorithm/key/registry/signing | 11 | `UnsupportedAlgorithm` through `SignatureInvalid` |
| Proof/key/evidence | 16 | `InvalidProofClaims` through `KeyAttestationProviderUnavailable` |
| Proof/policy/time/replay | 9 | `InvalidProofPolicy` through `ProofReplayUnavailable` |

The kind distribution is 29 `InvalidInput`, 12 `VerificationFailed`, five
`Internal`, three `Unsupported`, and two `Crypto`. No other core kind exists in
the baseline, and no encryption, MAC, key-agreement, or broader JOSE scope is
invented.

## Compatibility and dependency evidence

Pin all 51 constant identities/paths/values, enum order/derives/non-exhaustive
marker, exact displays, five kinds, capability, const bridge, `From`, and
source-free semantics. Keep messages static and redacted; never capture tokens,
claims, headers, keys, signatures, identifiers, parser details, or causes.
Manifests, dependency graph, features, lockfile, Serde/binding surface, and
unsafe/native state stay unchanged.

## Security, privacy and maintenance evidence

All selected records contain only public codes, enum-like kinds, and static
redacted text. They cannot retain token, claim, header, key, signature,
identifier, parser, source, or cause data. Four crate-local catalogues improve
review locality without a shared release axis. Immutable fixed-size test data
cannot influence production behavior.

The planning golden contains 55 LF lines (51 rows), 13,353 bytes, and SHA-256
`528b29913876710a2ee806e30fef044657f3c6c38e7e3efff860cf71060b9592`.
Stable and planning copies must be independently checked and Git-receipt bound.

## Rejected or deferred candidates

The retained two-match design, prior record shapes, shared abstraction, and
post-refactor-generated oracle are `not-adopt`. A cross-crate helper remains
deferred until several completed slices demonstrate identical irreducible
mechanics; this slice supplies no such mandate.

## Gates and limitations

Run focused/default/minimal/all-feature tests, strict Clippy/docs/format,
public API and dependency diffs, existing conformance/vector/redaction tests,
direct WASM/Android/iOS package checks, workspace Nix tests, factory and source
contracts, and the expanded mutation suite. Target results are compilation
evidence only, not runtime, device, packaging, FFI, or binding support.

## Evidence commands

Source enumeration found 51 variants/constants and reconstructed 51 exact rows.
`wc` measured the current source/function/test surfaces; `sha256sum` produced
the immutable digest. Factory readiness, OpenSpec, implementation, public API,
target, and Nix commands remain deliberately unrun planning evidence until the
planning review and receipt sequence authorizes them.

## Open questions and blockers

No blocker remains. If implementation cannot eliminate the wildcard and second
mapping site while keeping all 51 rows exact, stop rather than weakening the
contract.
