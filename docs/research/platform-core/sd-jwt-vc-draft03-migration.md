# SD-JWT VC draft-03 migration boundary

Issue [#572](https://github.com/hyperledger-identus/sdk-rust/issues/572)
qualifies one explicit input-only compatibility path for released SDK-TS
`vc+sd-jwt` credentials. It does not make draft-03 canonical and does not
change new SDK-Rust issuance from draft-22 `dc+sd-jwt`.

## Exact evidence

| Source | Frozen identity | What it establishes |
| --- | --- | --- |
| SDK-TS | `@hyperledger/identus-sdk` 8.1.4, Git `4bf86ebf69d5e96616a148e4c973f831f95fa38e` | The released consumer routes `vc+sd-jwt`, uses package 0.7.x, and applies millisecond date heuristics. |
| Legacy engine | `@sd-jwt/sd-jwt-vc` 0.7.2, Git `d3cc53b66be9ef40ede9f86a931b7a43f55dbd3a`, npm integrity recorded in the checked-in receipt | The consumer engine points to draft-03, emits protected `vc+sd-jwt`, and owns status retrieval. It is an oracle, not a Rust dependency. |
| Legacy profile | `draft-ietf-oauth-sd-jwt-vc-03`, SHA-256 `9599225eb8732c0ef3f9a6524589e637f9272ba742da185c43ecfd272b81bf5a` | Historical wire/profile meaning. |
| Canonical profile | `draft-ietf-oauth-sd-jwt-vc-22`, SHA-256 `da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124` | Current SDK-Rust profile authority and `dc+sd-jwt` identity. |

No donor source or fixture is copied. The portable packet contains clean-room
claim objects and normalized result categories. The immutable observation
receipt is stored under
`crates/sd-jwt-vc/tests/fixtures/legacy-draft03-v1/`.

## Supported transition

```text
CredentialEnvelope(format = vc+sd-jwt)
                  │ explicit adapter only
                  ▼
           UnverifiedSdJwt
                  │ issuer signature
                  ▼
        DisclosureVerifiedSdJwt
                  │ exact typ = vc+sd-jwt
                  │ common claims + placement
                  │ optional issuer-key evidence
                  ▼
        LegacyDraft03SdJwtVc
                  │
                  └── canonical_reissuance_required = true

        no conversion to Draft22SdJwtVc
```

The accepted common subset is `iss`, `vct`, optional `nbf`, `exp`, `iat`,
`sub`, and `cnf`, using the bounded RFC 9901 engine and existing profile
limits. `status`, `aka_vcts`, `vct#integrity`, Type Metadata, automatic
fallback, protected-header rewriting, legacy issuance, and millisecond
conversion are rejected or absent. Numeric dates retain JWT seconds semantics;
values whose absolute magnitude is at least `1_000_000_000_000` are rejected
for `nbf`, `exp`, and `iat` as ambiguous millisecond-style input; they are
never divided or normalized. Canonical draft-22 NumericDate behavior is not
changed by this migration-only guard.

## Operational contract

- Callers must select `LegacyVcSdJwtDraft03Adapter`; canonical parsing and
  validation continue to reject the legacy identity. Typed legacy stages
  preserve exact envelope provenance, so a generic compact parse cannot be
  substituted after the envelope check.
- Success proves signature, disclosure processing, and the structural common
  subset. It does not prove issuer trust, time acceptance, status, schema,
  storage safety, or product acceptance.
- Receipts and errors are value-free. Metrics may contain only the capability
  ID, stable outcome/stage categories, bounded counts, and whether canonical
  reissuance is required.
- The adapter performs no network, clock, storage, DID resolution, trust, UI,
  or status-list effect.

## Rollback and removal

Before publication, rollback deletes the additive adapter and its evidence.
After publication, a compatible release may disable/remove the explicit path
only through a consumer-visible change. Removal requires measured migration
of SDK-TS and every supported stored credential population to issuer-reissued
`dc+sd-jwt`; elapsed time alone is not evidence.

This slice does not modify SDK-TS or claim downstream adoption. A later
language-adapter issue may translate SDK-TS DTOs and errors without making
them normative Rust domain shapes.
