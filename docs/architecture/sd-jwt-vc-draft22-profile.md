# SD-JWT VC draft-22 profile

`identus-sd-jwt-vc` assigns credential-profile meaning to the bounded RFC 9901
mechanics in `identus-sd-jwt`. It is pinned to
`draft-ietf-oauth-sd-jwt-vc-22` dated 2026-10-27 and emits only canonical
`dc+sd-jwt` credentials (`application/dc+sd-jwt`).

```text
opaque OID4VCI credential
          │
          ▼
CredentialEnvelope(format = dc+sd-jwt)
          │ exact bounded payload
          ▼
UnverifiedSdJwt ── issuer signature ──► IssuerVerifiedSdJwt
          │
          └── disclosure graph ───────► DisclosureVerifiedSdJwt
                                               │
                                               ├─ exact protected typ
                                               ├─ profile claims/placement
                                               ├─ optional issuer evidence
                                               ├─ optional Type Metadata
                                               └─ optional status evidence
                                                        │
                                                        ▼
                                               Draft22SdJwtVc
                                                        │
                              OID4VP format/DCQL ────────┤
                                                        ▼
                               disclosure selection + holder binding
```

The state transitions deliberately separate syntax, issuer signature,
disclosure processing, profile validation, optional structural evidence and
product acceptance. A caller cannot represent retrieved metadata, a validated
certificate chain, or a status-list observation as trust merely by supplying
it to the profile.

## Ownership boundary

The crate owns exact draft-22 profile claims, non-selective placement,
canonical protected/envelope types, finite Type Metadata inheritance,
status-list reference mapping and redaction-safe receipts. It has no HTTP
client, certificate parser, trust store, DID resolver, status-list decoder,
renderer, storage, clock/replay policy, UI, or language DTO compatibility.
Structural status correlation therefore remains `NotChecked` in the generic
verification report; only a caller-owned status policy may turn the observed
value into an acceptance decision.
Issuer-key resolution likewise remains `NotChecked` when the generic adapter
was given only a verification key. It advances to `Passed` only when optional
issuer discovery evidence also correlates the authenticated `iss` and exact
verification key.
Type Metadata claim paths preserve draft-22 member, non-negative array-index
and `null` wildcard segments. Generic verification reports attribute optional
evidence failures to issuer-key, schema, or status stages without erasing an
already successful signature proof.
For both inline and URI-discovered JWKS metadata, callers supply the bounded
resolved key set and the profile requires the exact verified issuer key to be
present. A URI alone is never evidence of key authorization.

The production dependency direction is inward:

```text
identus-sd-jwt-vc ──► identus-sd-jwt ──► identus-jose
         │                    │
         └────────► identus-credentials
```

OID4VCI and OID4VP are dev/test dependencies only. The executable offline
journey proves their generic seams without making protocol crates foundations
of credential semantics.

## Migration and draft watch

The implementation binds the draft name, publication date, canonical source
URL and SHA-256 digest in code and conformance tests. A new draft/RFC,
protected/media type change, claim-rule change, or source mismatch requires a
new issue and compatibility decision. The released SDK-TS `vc+sd-jwt` spelling
is migration evidence only; canonical output never uses it. Issue #572 adds a
separately named input-only adapter for the draft-03/draft-22 common subset:

```text
vc+sd-jwt envelope ─► issuer verification ─► disclosure processing
       └──────── explicit LegacyVcSdJwtDraft03Adapter ────────┐
                                                              ▼
                                               legacy migration state
                                         canonical reissuance required
```

The adapter preserves exact envelope provenance through typed legacy stages;
a generic compact parse cannot enter its validator. It rejects status,
additional types, type-integrity metadata, Type Metadata, automatic fallback,
header rewriting, legacy issuance, and `nbf`/`exp`/`iat` values whose absolute
magnitude is at least `1_000_000_000_000`, without date-unit conversion. It
cannot convert into `Draft22SdJwtVc`. Exact source and
removal evidence is recorded in the
[draft-03 migration report](../research/platform-core/sd-jwt-vc-draft03-migration.md).
