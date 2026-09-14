# ADR 0119: use crate-local protocol-cohesive OID4VCI error contracts

- **Status:** Accepted for the OID4VCI slice
- **Date:** 2026-09-15
- **Decision authority:** umbrella issue
  [#271](https://github.com/hyperledger-identus/sdk-rust/issues/271) and
  delivery issue [#277](https://github.com/hyperledger-identus/sdk-rust/issues/277)
- **Applies to:** `identus-oid4vci` only
- **Related decisions:** ADR 0110, ADR 0116, ADR 0117, and ADR 0118
- **Assessed revision:** sdk-rust
  `6217384f85ff72003a7b94482bf7879f4587be02`

## Context

`CredentialOfferError` has 171 fieldless variants and 171 public stable
error-code constants. Its 860-line public const bridge maps every variant once
to code, kind, and message without a wildcard. `Display` delegates through
that bridge. All errors use the `oid4vci` capability and one static message for
both local and public display, while kind varies: 167 variants are
`InvalidInput` and four are `Unsupported`.

The current behavior is correct and already has one decision site. The
maintenance problem is that offer transport, offer semantics, issuer metadata,
authorization-server metadata, token exchange, credential/nonce HTTP, and
issuance lifecycle responsibilities occupy one 857-arm match. Reviewers cannot
inspect one protocol responsibility without traversing the entire catalogue.

ADRs 0116 through 0118 establish crate ownership, exhaustive routing, and
immutable pre-refactor characterization. They are evidence rather than a
workspace-wide abstraction. OID4VCI needs the same irreducible private shape as
JOSE—code, varying kind, and one message—but its catalogue boundaries and
protocol/wire constraints remain owned by `identus-oid4vci`.

## Decision

1. `identus-oid4vci` owns a private compile-time `ErrorContract` containing
   exactly `ErrorCode`, `ErrorKind`, and one `&'static str` message.
2. The uniform `oid4vci` capability remains centralized as `CAPABILITY`; the
   one message continues to feed both local `Display` and the public
   `IdentusError` projection.
3. Six private catalogue modules own offer transport/JSON (12), offer
   semantics/grants (21), issuer/authorization-server metadata (39), token
   request/response/errors (34), credential/nonce/HTTP (36), and
   deferred/immediate issuance (29) records.
4. The group boundaries remain exact and ordered: `InvalidLimits` through
   `UnsafeReferenceUri`; `InvalidSemanticLimits` through
   `TransactionCodeDescriptionTooLarge`; `InvalidMetadataLimits` through
   `TokenEndpointRequired`; `InvalidTransactionCodeInputLimits` through
   `TokenErrorUriTooLarge`; `InvalidCredentialErrorResponseLimits` through
   `CredentialRequestBodyTooLarge`; and
   `InvalidDeferredCredentialRequestLimits` through
   `CredentialResponseExceedsProofCount`.
5. One private macro invocation adjacent to `CredentialOfferError` generates
   only an exhaustive wildcard-free router and a test-only inventory. It does
   not generate any public declaration, error constant, wire model,
   serialization, or binding.
6. The public enum, discriminant order, derives, `#[non_exhaustive]`, public
   constants and paths, `CAPABILITY`, `From`, `Display`, `Error`, and
   `pub const fn to_identus_error` remain explicit and unchanged. All contract
   lookup and conversion remains const.
7. A planning-only 171-row golden captured from the assessed revision pins
   constant identity/visibility, code, kind, capability, exact local/public/full
   display, and source behavior. Its exact-base candidate is 59,380 bytes over
   175 LF lines with SHA-256
   `2c9e03381744b11902eb8b8bbc08934374781fad99d6561573cfcd62490bcd39`.
   The stable test copy must be byte-identical and bound to the immutable
   preimplementation Git receipt.
8. The existing data-driven checker, mutation suite, factory fixture, and Nix
   source contract gain one bounded OID4VCI binding. They retain fail-closed
   schema, provenance, hash, active/archive, root, regular-file, and symlink
   checks; implementation is expected to grow the mutation suite from 76 to
   101 cases.
9. Existing typed OID4VCI wire errors, their Serde behavior, protocol parsing,
   limits, transports, metadata, token, credential, nonce, deferred/immediate
   issuance, JOSE integration, features, dependencies, and lockfile do not
   change. Issues #7 and #168 remain behaviorally untouched.
10. No shared/public error framework, new crate, retryability or metadata
    field, localization, FFI, unsafe/native code, consumer change, runtime
    target promise, or downstream adoption is introduced.

## Consequences

- Each of the 171 variants retains one behavioral decision. The mapping-site
  count remains one and the wildcard-default count remains zero; claiming
  either deduplication or wildcard removal would be false.
- No catalogue owns more than 39 records, and the public bridge becomes a
  small delegator while protocol-owned records are reviewable independently.
- The router deliberately remains an explicit complete variant-to-record map.
  Its literals live in responsibility-owned catalogues rather than in the
  public conversion function.
- Total production lines may increase. Evidence must report physical and
  nonblank movement for `error.rs`, the relevant error-contract source, and
  the complete crate instead of treating line count as a success proxy.
- A future variant cannot select a default record: compilation requires an
  explicit router entry and the golden inventory requires an explicit row.
- Reusing the same three private fields as JOSE does not create a shared type
  or mandate identical grouping in another crate.

## Alternatives

Keeping the 857-arm match preserves behavior but retains the measured review
hotspot. Reusing credentials' five-field record repeats invariant capability
and equivalent local/public text; presentations' two-field record cannot
express the four `Unsupported` variants. A shared crate/trait, public
generator, procedural macro, or build-time schema introduces coupling and a
new release/tooling axis. Generating the golden after refactoring allows the
new source to bless its own drift. All are rejected for this slice.

## Verification and rollback

Verification requires the exact 171-row regression, all 171 public constants,
ordered discriminants, 167/4 kind distribution, const-use and source-free
tests, exact public API and manifest/feature/dependency/lock comparisons,
current OID4VCI tests, strict Clippy/docs/format, direct and authoritative
WASM/Android/iOS compile checks, factory/Nix mutation evidence, and honest
before/after maintainability metrics. Target compilation does not prove
runtime, device, packaging, FFI, or binding support. A distinct
architecture/API/security review must find no drift.

Rollback restores the explicit match and removes only private catalogue and
test/factory evidence. No persisted data, wire schema, release, downstream
adoption, or migration exists in either direction.
