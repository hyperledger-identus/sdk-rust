# Design

## Context

`identus-sd-jwt` owns stable RFC 9901 mechanics and deliberately has no VC
profile. Draft-22 adds a canonical protected type, required and constrained
claims, issuer-key-discovery choices, Type Metadata, status-list vocabulary,
and profile validation. These semantics change on a different lifecycle from
the RFC base and therefore need a separate crate and capability version.

## Goals / Non-Goals

**Goals:**

- implement exact draft-22 compact-profile issuance and validation;
- reuse all RFC 9901, JOSE, digest, signature and key-binding mechanics;
- make syntactic, cryptographic, issuer-evidence, metadata, status-evidence and
  product-trust states impossible to confuse;
- validate hostile profile inputs under explicit finite bounds and static
  diagnostics;
- integrate through generic credential and OID4VP capability seams without
  importing protocol or language DTOs;
- preserve a deterministic migration trigger for a later draft/RFC.

**Non-Goals:**

- HTTP, redirect/cache/decompression policy, DID resolution, X.509 path
  building, JWK-set retrieval, trust-anchor selection or issuer authorization;
- status-list retrieval/decompression, credential acceptance policy, replay,
  storage implementation, consent, rendering/SVG execution, telemetry or UI;
- legacy `vc+sd-jwt`, JWS JSON Serialization, HAIP/mdoc, FFI, publication,
  certification or downstream edits.

## Decisions

### Separate profile crate with inward dependencies

```text
identus-sd-jwt-vc
  ├── identus-sd-jwt         RFC 9901 staged mechanics
  ├── identus-credentials    envelope/verifier/status vocabulary
  └── identus-oid4vp         format capability port (dev/test adapter only)
```

The production crate depends on `identus-sd-jwt` and
`identus-credentials`; OID4VCI/OID4VP are dev-only E2E dependencies so
protocol crates do not become credential-format foundations. The conformance
rulebook classifies the profile beside other credential-semantics crates.

### Minimal reusable hooks in the RFC base

The base gains a profile-neutral issuance option for a caller-validated JWS
protected type and read-only accessors for bounded issuer-verification
evidence: the validated protected header, the exact public verification key,
and whether a caller-supplied claim path was disclosed. The header/key pair is
captured only after signature verification and carried unchanged through
disclosure processing. This lets a profile correlate an inline `x5c` reference
with the key a caller actually used without treating the certificate chain as
trusted. The existing `issue` function remains behavior-compatible and emits
no type. Raw compact/signature bytes, mutable claims, salts, digests and private
disclosure indexes remain private.

This is narrower and less coupled than making the base know about draft-22 or
reparsing compact bytes in the profile.

### Consuming profile state transition

```text
DisclosureVerifiedSdJwt
  + ProfileValidationInputs
  --validate_draft22-->
Draft22SdJwtVc
  + optional caller evidence
  --> Draft22VerificationReceipt
```

The profile consumes a disclosure-verified base value. It requires exact
protected `typ`, validates claims and their disclosure placement, then owns
the base value so later selection and key binding remain available without
reparsing. A receipt reports only booleans/counts and evidence classes; it does
not report claim values, credential type, issuer, metadata URL or status.

Issuer signature validity is inherited base evidence. Issuer-key discovery,
Type Metadata and status are optional separately typed caller inputs. Passing
their structural checks records only that supplied evidence correlates; it
does not establish trust, issuer authorization, freshness or acceptability.

### Claim contract

`vct` is a required bounded, non-empty, case-sensitive collision-resistant
name represented by a string. The SDK preserves exact spelling and accepts a
bounded absolute URI, dotted namespace or UUID-shaped name; ecosystem
semantics and issuer authorization remain caller policy.
`aka_vcts`, when present, is non-empty, bounded, exact-unique and cannot
contain `vct`.

`iss`, `nbf`, `exp`, `cnf`, `vct`, `vct#integrity`, `aka_vcts`, and `status`
must be present only as clear root claims, never disclosures. `sub` and `iat`
may be disclosed. Numeric dates must be finite JSON numbers in seconds; the
profile does not accept the SDK-TS millisecond heuristic. A known `nbf > exp`
interval fails structurally, while current-time acceptance remains policy.

If key binding is required by caller input, `cnf` must exist and carry a
bounded supported confirmation method. This records a confirmation method
only; holder-proof verification, key selection and trust remain separate
caller-owned steps.

### Issuer discovery inputs

The closed structural choices are:

- inline X.509 evidence, reporting that the caller validated the exact
  protected `x5c` chain and extracted the exact public verification key. The
  profile compares both against the bounded header/key pair retained by the
  verified base value; or
- JWT VC Issuer Metadata with exact `iss` correlation and exactly one of
  bounded inline `jwks` or HTTPS `jwks_uri` evidence. Both forms retain the
  resolved keys and require the exact issuer-verification key to be present.

The crate does not parse certificates, retrieve URLs, choose a `kid`, validate
JWK cryptography, or decide that the mechanism is allowed for an issuer. Those
belong to injected adapters/policy. Unknown future mechanisms require an
explicit versioned extension rather than an untyped success boolean.

### Type Metadata inputs

The first slice validates caller-supplied metadata needed to protect credential
semantics: exact `vct`, optional bounded `extends` chain, exact-unique claim
paths with member, array-index and `null` wildcard segments, `mandatory`, and
`sd` values (`always`, `allowed`, `never`). It validates
rules against the complete issuer view when available and the presented view
without treating an undisclosed mandatory claim as proof of issuer failure.

Display, locale, image and SVG data remain bounded opaque metadata and are not
rendered or executed. Remote retrieval and integrity verification occur before
the caller constructs validated metadata evidence. Extension traversal has an
explicit aggregate byte/depth/count/work ceiling, independently measures
retained claim paths rather than trusting a caller byte count, and rejects
cycles.

The generic verification adapter preserves the stage of optional evidence
failures: issuer discovery, Type Metadata and status correlation fail their
respective issuer-key, schema and status stages after signature proof remains
passed.

### Status vocabulary and evidence

The profile recognizes the draft-22 `status` object and the `status_list`
mechanism, whose token format must be JWT. It maps the bounded URI/index pair
into generic `CredentialStatusBinding` values. A caller may attach
`CredentialStatusEvidence`; exact binding correlation is checked, but status
meaning, freshness and accept/reject policy remain outside the profile.

Unknown status mechanisms are preserved only as unsupported profile evidence;
they are never silently accepted as checked.

### Public default limits

`SdJwtVcLimits` composes the RFC 9901 limits and adds conservative profile
ceilings:

| Boundary | Default |
| --- | ---: |
| credential type bytes | 2,048 |
| additional types | 16 |
| issuer identifier / metadata URI bytes | 2,048 |
| issuer metadata JSON bytes | 131,072 |
| JWK-set keys | 64 |
| Type Metadata JSON bytes | 262,144 |
| type-extension depth | 8 |
| claim metadata entries | 256 |
| claim-path segments | 32 |
| status JSON bytes | 16,384 |
| retained profile output bytes | 524,288 |
| aggregate profile work units | 500,000 |

Complete output remains within the generic credential-envelope and RFC base
ceilings. Cheap raw-size/count checks precede JSON traversal, URI parsing,
path evaluation and retained allocation.

### Static error taxonomy

`SdJwtVcError` is non-exhaustive with stable `sd_jwt_vc.*` codes for invalid
limits, protected type, profile claims, type, additional types, claim
placement, numeric date, key binding requirement, issuer evidence, metadata,
metadata cycle/rule, status, evidence mismatch, resource limit, base failure
and internal invariant. Formatting is static and no external error source is
exposed.

### Draft watch and migration

The source manifest records draft name, date, canonical URL and text hash.
Tests bind constants and vectors to that manifest. A different IETF revision,
IESG publication, media/protected-type change, claim-rule change, or source
hash mismatch triggers a new issue, ADR review and capability version. Cargo
dependency updates cannot move the profile implicitly.

### Evidence and decomposition

Production modules remain responsibility-shaped: `claims`, `limits`,
`issuer_evidence`, `type_metadata`, `status`, `profile`, and `error`. Tests
separate conformance, properties, rejections, protocol E2E and migration
evidence. This cohesive security-sensitive slice may exceed twelve files, but
each production module stays under the 1,000-authored-nonblank-line attention
threshold and evidence lands atomically with the profile.

## Risks / Trade-offs

- **Draft churn:** exact pinning creates explicit migration work. This is
  preferable to silently changing credential meaning.
- **Local profile maintenance:** no candidate passes every SDK gate. Frozen
  oracles, source-bound vectors and replaceable private mechanics reduce risk.
- **Evidence booleans can be over-read:** states and receipts name structural
  correlation rather than trust/acceptance, and product decisions remain
  impossible to construct from a raw boolean alone.
- **Type Metadata breadth:** rendering is useful but unsafe and orthogonal.
  This slice validates credential semantics while keeping rendering opaque.
- **Legacy interop:** a released consumer uses `vc+sd-jwt`. Issue #572 makes
  migration explicit without contaminating canonical output.

