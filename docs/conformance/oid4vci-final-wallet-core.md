# OpenID4VCI 1.0 Final wallet-core conformance report

**SDK revision:** issue #375 candidate; exact merge revision pending

**Normative baseline:** OpenID for Verifiable Credential Issuance 1.0 Final,
published 2025-09-16.

**Matrix:**
[`oid4vci-final-wallet-core.csv`](oid4vci-final-wallet-core.csv)

## Verdict

M4 and issue #7 should remain open. The bounded SDK wallet core has broad,
coherent structural coverage across Final sections 4 through 9 and relevant
metadata. Issue #375 closes the required Pre-Authorized Code response-binding
gap, but one required exit condition is not delivered:

1. the historical Oxid/Lace ID Portal vector-suite and consumer-review
   acceptance is not safely transferable as written; issue
   [#376](https://github.com/hyperledger-identus/sdk-rust/issues/376) owns
   license clarification, generic/profile separation and durable consumer
   review.

After #376 lands, update the matrix at its exact merged revision and re-evaluate
issue #7 and M4. No additional feature is justified by this report alone.

## Coverage summary

| Status | Rows | Meaning |
| --- | ---: | --- |
| Implemented | 14 | Bounded public surface, canonical contract and executable tests exist. |
| Partial | 5 | A deliberate subset exists with explicit unsupported behavior. |
| Unsupported | 4 | The behavior is adjacent or outside the current wallet-core profile. |
| Missing | 1 | A required closeout condition has a focused open owner. |

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
| Wire types and bounded validation | Thirty-nine bounded deliveries, canonical specs and crate tests | Delivered for the recorded wallet-core subset |
| Deep-link grammar | `CredentialOfferRequest` by-value/reference transport tests | Delivered |
| Extension point with test double | Unknown JSON members and format summaries remain bounded/lossless; chain formats stay downstream | Reinterpreted as data extension points; no generic chain-plugin trait is justified |
| Import cross-implementation vectors | Exact consumer fixtures exist, but license and genericity are incomplete | Not delivered; #376 |
| Both app teams sign off | Historical interoperability evidence exists, but not current durable SDK-surface review | Not delivered; #376 |
| Published `identus-oid4vci` 0.1 | Publication is protected release work and outside M4 | Deferred; no release claim |

## Consumer fixture disposition

The inspected Oxid/Lace evidence is useful but remains reference-only:

- MediaNoxLabs/oxid evidence revision
  `e9ecfe5df27c0790dac40776e1d0a37e5f99f907` is Apache-2.0 and snapshots
  consumer behavior and source hashes.
- input-output-hk/lace-id-portal profile revision
  `25499870f84d77173c46e4af3021311decfb840b` pins Portal baseline
  `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`, profile source
  `76e8edf394a4cb37ca822037272d543c68f25f71`, fourteen fixture hashes and Oxid
  evidence.
- The exact Portal source revision contains no explicit repository license.
- Several vectors assert the product-specific `midnight_cbor_phase1` format.

No fixture bytes were copied. Hashes, paths and revisions are evidence, not an
inferred license or chain-neutral conformance suite.

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
