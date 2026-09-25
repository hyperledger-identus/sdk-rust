# ADR 0152: use clean-room OID4VCI interoperability vectors

- **Status:** Accepted for implementation
- **Date:** 2026-09-25
- **Decision authority:** issue #376 under the accepted M4/#7 roadmap
- **Constraint impact:** material provenance closeout directed by issue #376
- **Related:** ADR 0041, ADR 0151, issues #7, #372, #375 and #376

## Context

The last missing OpenID4VCI M4 matrix row asks for generic Oxid/Portal vector
evidence. Oxid's pinned Apache-2.0 evidence is an accepted negative
compatibility gate derived from Portal behavior. The pinned Portal repository
contains detailed source and fixture hashes but no explicit license grant, and
its named profile selects `midnight_cbor_phase1`. Some exchanged fixtures are
issuer-side inputs that a wallet-core SDK should not parse.

Copying either fixture tree would therefore conflate a product gate with a
generic conformance suite, retain unresolved origin licensing, and encourage
Midnight or issuer behavior to leak into `identus-oid4vci`.

## Decision

1. Keep every pinned Oxid and Portal file reference-only. Record repositories,
   exact revisions, relevant paths, license disposition and genericity, but
   copy no source or fixture bytes.
2. Author a small Apache-2.0 fixture packet inside sdk-rust from the
   OpenID4VCI 1.0 Final wallet contract. Use visibly synthetic values and no
   consumer host, credential, token, nonce, key, DID or deployment data.
3. Cover one coherent Pre-Authorized Code positive journey plus independently
   authored negative classes for extra offer query data, null Transaction Code
   and singular Credential Response.
4. Retain only inputs consumed by public wallet-side SDK APIs. Classify
   issuer-side proof verification, authorization replay/origin policy and
   credential-format semantics as downstream or outside M4.
5. Bind every fixture to a closed manifest entry with SHA-256, normative
   section, authorship/license, transformation, expected result and public API.
   Validate the manifest and digests before executing vectors.
6. Treat pinned consumer ADRs, source and profile commits as design evidence.
   Do not represent them as current human approval, live interoperability,
   official certification or authorization to mutate a consumer.
7. When this suite and its review are green, the cross-consumer matrix row can
   become implemented and IDR-023/M4 can close with all partial, unsupported
   and downstream limitations preserved.

## Consequences

- The SDK owns legally clear, reproducible conformance inputs and can evolve
  them with its public API.
- Consumer behavior remains useful without creating a dependency or copying
  ambiguous/product-specific content.
- Exact byte parity with the historical fixture exchange is deliberately not
  claimed; behavioral mapping is explicit and reviewable.
- A future live Oxid/Portal test, app-team acceptance, release or certification
  remains separately owned and cannot be inferred from this milestone.

## Rejected alternatives

- **Copy Portal fixtures:** no explicit license at the pinned revisions.
- **Copy Oxid's Portal-derived fixtures:** downstream Apache-2.0 wrappers do
  not independently resolve the source-origin question.
- **Use a Midnight format test double:** it would create chain-profile leakage
  and add no evidence for generic protocol state.
- **Document compatibility without execution:** prose alone cannot detect
  fixture or public-API drift.
