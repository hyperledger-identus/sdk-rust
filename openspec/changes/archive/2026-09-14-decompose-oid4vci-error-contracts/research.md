# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-15
Source retrieval date: 2026-09-15
Research blockers: none

## Problem and existing implementation

The inspected repository is `hyperledger-identus/sdk-rust` at
`develop@6217384f85ff72003a7b94482bf7879f4587be02`. Issue #277 owns this
slice and #271 remains the umbrella.

`crates/oid4vci/src/error.rs` is 1,381 physical/1,371 nonblank lines. Its
public const `to_identus_error()` bridge occupies 860 lines and its one
wildcard-free match contains 171 arms. The fieldless non-exhaustive public enum
has 171 variants and 171 public constants. There are 167 `InvalidInput` and
four `Unsupported` kinds, one `oid4vci` capability, identical static
local/public messages, and no error sources. Whole OID4VCI production source
is 8,526 physical/7,852 nonblank lines.

The current integration suite discovers 192 tests: 191 pass and one ignored
diagnostic at the assessed base. Twenty-one metadata-area variants have no
direct named test reference, so existing tests are not a complete exact error
compatibility oracle.

## Normative sources

Issues #271/#277, ADRs 0110/0116/0117/0118/0119, canonical core-error and
OID4VCI capability specifications, and exact baseline source/tests control this
internal behavior-preserving refactor. The published OID4VCI profile informs
the existing implementation but is not reinterpreted here. No donor
repository, new dependency, or mutable web source is needed.

The earlier error-contract ADRs provide evidence, not cross-crate authority.
ADR 0119 selects the OID4VCI-owned grouping and preserves existing typed wire
error models rather than merging wire and SDK diagnostic taxonomies.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Six private three-field protocol catalogues | `adopt` | Makes each protocol responsibility reviewable while expressing the four varying kinds explicitly. |
| Keep the current 857-arm match | `not-adopt` | Behavior is correct but the measured review hotspot remains. |
| Reuse credentials/presentations record | `not-adopt` | Repeats invariants or cannot express heterogeneous kind. |
| Share JOSE's private type or introduce a shared crate/trait | `not-adopt` | Couples independent domain taxonomies and release cadence for no runtime benefit. |
| Generate public errors or wire models from a macro/schema | `not-adopt` | Expands compatibility and build-tooling risk beyond maintenance scope. |
| Post-refactor generated golden | `not-adopt` | Lets implementation authorize its own drift. |

## Rejected or deferred candidates

The retained monolithic match, prior crate record shapes, shared JOSE type,
shared crate/trait, public or build-time generation, and post-refactor oracle
are `not-adopt` for the reasons in the candidate table. A workspace-wide
compile-time helper remains deferred until multiple completed slices show
identical irreducible mechanics and measurable value; similarity between the
private JOSE and OID4VCI fields is not sufficient authority to add that
coupling.

## Selected grouping

| Group | Range | Count | InvalidInput / Unsupported |
| --- | --- | ---: | ---: |
| Offer transport and JSON | `InvalidLimits` through `UnsafeReferenceUri` | 12 | 11 / 1 |
| Offer semantics and grants | `InvalidSemanticLimits` through `TransactionCodeDescriptionTooLarge` | 21 | 21 / 0 |
| Issuer and authorization-server metadata | `InvalidMetadataLimits` through `TokenEndpointRequired` | 39 | 39 / 0 |
| Token request, response, and errors | `InvalidTransactionCodeInputLimits` through `TokenErrorUriTooLarge` | 34 | 34 / 0 |
| Credential, nonce, and HTTP | `InvalidCredentialErrorResponseLimits` through `CredentialRequestBodyTooLarge` | 36 | 34 / 2 |
| Deferred and immediate issuance | `InvalidDeferredCredentialRequestLimits` through `CredentialResponseExceedsProofCount` | 29 | 28 / 1 |

The four `Unsupported` variants remain exactly `UnsupportedTransport`,
`CredentialRequestAuthorizationDetailsUnsupported`,
`CredentialRequestTokenTypeUnsupported`, and
`DeferredCredentialResponseUnsupported`.

## Compatibility and dependency evidence

- Pin all 171 constant identities, paths, values, exact displays, kinds,
  capability, enum order, derives, non-exhaustive marker, public const bridge,
  `From`, `Display`, `Error`, and source-free semantics.
- Preserve every typed token/credential error response model, Serde name and
  shape, transport status/content-type rule, parsing limit, and protocol
  branch. No canonical OID4VCI requirement is modified.
- Keep messages `&'static str`; never capture offers, tokens, credentials,
  nonces, authorization details, identifiers, URIs, parser details, or causes.
- Keep the manifest, empty default feature, `fluent-uri`, `identus-core`,
  `identus-jose`, `serde_json`, and `zeroize` dependency edges and lockfile
  unchanged.
- Preserve issues #7 and #168 behavior by excluding their implementation and
  specifications from this maintenance slice.

## Security, privacy and maintenance evidence

The selected record contains only public code, enum-like kind, and static
redacted text. It cannot retain protocol input or secret data. Six private
catalogues reduce review coupling without exposing a taxonomy or adding a
shared release axis. The golden is fixed-size test evidence and never affects
production conversion.

The mapping-site count truthfully remains one, wildcard defaults remain zero,
and behavioral decisions remain 171. The expected improvement is a small
public delegator and six catalogues of at most 39 records. Total production
lines may increase and must be disclosed.

## Golden evidence

The exact candidate golden was reconstructed in enum order from the assessed
source after proving that the earlier preview source has no
`crates/oid4vci/**` delta. It contains 175 LF lines (three provenance lines,
one header, and 171 rows), 59,380 bytes, and SHA-256
`2c9e03381744b11902eb8b8bbc08934374781fad99d6561573cfcd62490bcd39`.
Every row uses the existing 11-column schema. Before implementation, that byte
sequence must be added as the active planning golden, independently
cross-checked, committed alone with the other planning artifacts, and bound by
the durable receipt.

## Test-gap evidence

Current tests do not directly name these 21 metadata-area variants:
`InvalidMetadataLimits`, `CredentialEndpointTooLarge`,
`UnsafeCredentialEndpoint`, `InvalidAuthorizationServers`,
`DuplicateAuthorizationServer`, `InvalidCredentialConfigurations`,
`InvalidCredentialFormat`, `CredentialFormatTooLarge`,
`InvalidAuthorizationServerMetadataLimits`,
`AuthorizationServerMetadataTooLarge`,
`InvalidAuthorizationServerMetadata`,
`AuthorizationServerMetadataIssuerMismatch`,
`AuthorizationEndpointTooLarge`, `UnsafeAuthorizationEndpoint`,
`TokenEndpointTooLarge`, `UnsafeTokenEndpoint`, `InvalidGrantTypes`,
`GrantTypeTooLarge`, `TooManyGrantTypes`, `DuplicateGrantType`, and
`InvalidAnonymousPreAuthorizedAccess`. The new exact regression must cover
these without changing protocol tests or production behavior.

## Gates and limitations

Run focused/default/minimal/all-feature tests, strict Clippy/docs/format,
public API and dependency diffs, existing OID4VCI conformance/redaction tests,
direct WASM/Android/iOS package checks, all three authoritative Nix target
derivations, workspace Nix tests, factory/source contracts, and the expanded
mutation suite. OID4VCI is already named by each authoritative target
derivation. Target results are compilation evidence only, not runtime, device,
packaging, FFI, binding, or React/React Native support.

## Evidence commands

Source enumeration found 171 enum variants/constants and one wildcard-free
171-arm match. `wc` measured the source/function/crate surfaces. A Git diff
confirmed no OID4VCI source delta between the preview candidate and exact base;
the provenance-only substitution produced the digest above. Implementation,
public API, target, Nix, factory readiness, and OpenSpec validation remain
deliberately unrun planning evidence.

## Open questions and blockers

No semantic blocker remains. Implementation remains procedurally blocked until
the exact planning golden is added and independently reviewed, the
planning-only commit is signed with DCO, and the durable preimplementation
receipt passes.
