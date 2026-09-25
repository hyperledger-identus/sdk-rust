# OpenID4VCI 1.0 Final wallet-core conformance report

**SDK revision:** issue #376 candidate based on
`develop@149a35e62fc54f91dc99a496feb884d5b2f92cce`; exact merge revision pending

**Normative baseline:** OpenID for Verifiable Credential Issuance 1.0 Final,
published 2025-09-16.

**Matrix:**
[`oid4vci-final-wallet-core.csv`](oid4vci-final-wallet-core.csv)

## Verdict

M4 and issue #7 can close after issue #376 merges at an exact reviewed
revision. The bounded SDK wallet core has coherent structural coverage across
Final sections 4 through 9 and relevant metadata, and the last required
cross-consumer row now has clean-room executable evidence.

The historical Oxid/Lace ID Portal fixture bytes remain reference-only. The
repository-authored suite proves generic wallet-side expressibility and known
legacy rejection without copying ambiguous or chain-specific material. Its
bounded assessment is
[`oid4vci-generic-interoperability-vectors.md`](oid4vci-generic-interoperability-vectors.md).
No additional runtime feature is justified by this closeout alone.

## Coverage summary

| Status | Rows | Meaning |
| --- | ---: | --- |
| Implemented | 15 | Bounded public surface, canonical contract and executable tests exist. |
| Partial | 5 | A deliberate subset exists with explicit unsupported behavior. |
| Unsupported | 4 | The behavior is adjacent or outside the current wallet-core profile. |
| Missing | 0 | No required M4 wallet-core row lacks focused evidence. |

The statuses are traceability labels, not compliance scores. `implemented`
does not mean that HTTP origin, TLS, trust, token validity, credential-format
verification, storage, product policy or official certification is provided.

## Section findings

- **Section 4:** bounded by-value/reference Credential Offers, core semantics
  and both grants are implemented. Fetching and the issuer-facing Credential
  Offer Endpoint response are outside the wallet core.
- **Section 5:** one Authorization Details/PKCE S256 GET profile and correlated
  query responses are implemented. Scope, PAR, alternate response modes,
  browser and callback routing are not.
- **Section 6:** both request constructors and request-bound Token Endpoint
  response transitions exist. The Authorization Code flow also correlates
  returned Authorization Details; later pre-authorized Credential Request
  policy remains outside this narrow binder.
- **Section 7:** optional Nonce Request/Response syntax and HTTP metadata are
  implemented without transport, DPoP or nonce lifecycle policy.
- **Section 8:** unencrypted JWT-proof request and immediate/deferred/error
  classification are implemented. Other proof types, credential formats,
  verification, trust, storage and RFC 6750 challenge handling remain outside.
- **Section 9:** request construction, issued/pending/error classification,
  transaction correlation and authority-preserving continuation are
  implemented. Polling effects and interval policy remain downstream.
- **Sections 10 and 11:** encryption and notifications are explicitly outside
  the M4 profile.
- **Section 12:** bounded partial Issuer and Authorization Server metadata are
  implemented; Client Metadata, discovery, signed metadata and complete
  display/format semantics are not.

## Issue #7 reconciliation

| Original acceptance item | Current evidence | Disposition |
| --- | --- | --- |
| Wire types and bounded validation | Forty bounded deliveries, canonical specs and crate tests | Delivered for the recorded wallet-core subset |
| Deep-link grammar | `CredentialOfferRequest` by-value/reference transport tests | Delivered |
| Extension point with test double | Unknown JSON members and format summaries remain bounded/lossless; chain formats stay downstream | Reinterpreted as data extension points; no generic chain-plugin trait is justified |
| Import cross-implementation vectors | Portal-derived bytes remain unsafe to copy; #376 supplies clean-room equivalents with hashes and public-API execution | Delivered as qualified repository-authored evidence, not donor byte parity |
| Both app teams sign off | Immutable Oxid ADR/tests and Portal profile sources are mapped to the SDK; no current human or live-product approval is claimed | Reconciled as bounded design evidence; downstream adoption remains separate |
| Published `identus-oid4vci` 0.1 | Publication is protected release work and outside M4 | Deferred; no release claim |

## Consumer fixture disposition

The inspected Oxid/Lace evidence remains reference-only:

- MediaNoxLabs/oxid evidence revision
  `e9ecfe5df27c0790dac40776e1d0a37e5f99f907` is Apache-2.0 and snapshots
  consumer behavior and source hashes.
- input-output-hk/lace-id-portal profile revision
  `25499870f84d77173c46e4af3021311decfb840b` pins Portal baseline
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`, profile source
  `76e8edf394a4cb37ca822037272d543c68f25f71`, fourteen fixture hashes and Oxid
  evidence.
- The exact Portal source revision contains no explicit repository license,
  and several vectors assert the product-specific `midnight_cbor_phase1`
  format.
- Issue #376 adds nine independently authored Apache-2.0 generic fixtures with
  a closed SHA-256 provenance manifest and public-API tests.

No consumer fixture bytes were copied. Consumer hashes, paths and revisions
remain design evidence; the executable suite is SDK-authored.

## Assurance boundary

The matrix checker proves a closed schema, status semantics, repository path
existence and focused ownership for required gaps. The Rust/factory gates prove
the checked revision behaves as its tests specify. Neither proves official
OpenID certification, production interoperability, transport provenance,
issuer trust, credential correctness, privacy compliance or fitness for a
particular wallet.

## Read-only repository receipt

- Oxid inspected at
  `183664aeca500c25d6d27a22fa402b4d40c649d3`; four pre-existing status entries;
  no mutation.
- Lace ID Portal inspected at
  `d284b85bfcbb4a7e5d2200837703419c145c60f5`; one pre-existing status entry;
  no mutation.
