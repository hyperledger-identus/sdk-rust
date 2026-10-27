# identus-oid4vp

`identus-oid4vp` is the runtime-neutral, chain-agnostic OpenID for Verifiable
Presentations 1.0 protocol crate. Its current narrow profile supports:

- bounded `openid4vp:` by-reference invocation parsing;
- effect-free GET/POST Request URI envelopes and bounded response binding;
- compact signed Request Object verification through `identus-jose`;
- strict bounded DCQL validation and selection behind SDK-owned types;
- composed `vp_token` / `direct_post` request routing; and
- deterministic bounded plain `direct_post` success response construction.

## Direct-post flow

```text
openid4vp: invocation
        │
        ▼
runtime adapter executes bounded Request URI envelope
        │
        ▼
signed JAR ──verify──▶ ValidatedAuthorizationRequest
                              │
                              ├─ validated HTTPS response_uri
                              ├─ bounded optional state
                              └─ private DCQL coverage/cardinality rules
                                             │
caller-selected bounded presentation values ─┘
                                             ▼
                                    DirectPostResponse
                                    POST + form body
```

`DirectPostResponse` is inert transport material. Revealing its target and
body does not authorize delivery. A platform adapter still owns DNS and IP
policy, TLS, redirects, decompression, timeouts, retries, SSRF defenses and HTTP
outcome handling. Applications still own verifier trust, consent, credential
and presentation verification, presentation generation, replay policy and
retention.

## Supported response shape

The response body contains a deterministic form-encoded `vp_token` JSON object
and returns the exact signed `state` when one was present. Query keys follow
validated DCQL query order. Each value is a non-empty array of bounded JSON
strings or objects. Required credential-set alternatives and each query's
`multiple` flag are checked before serialization.

`direct_post.jwt`, JWE, SIOPv2, Digital Credentials API, HAIP, network I/O and
credential-format proof creation are intentionally separate capabilities.

The normative boundary and decisions are recorded in
[`ADR 0179`](../../docs/adr/0179-construct-bounded-oid4vp-direct-post-responses.md),
with executable lineage in the versioned
[`direct-post vector packet`](../../docs/conformance/fixtures/oid4vp-direct-post/v1/vectors.json).
