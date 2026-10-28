# identus-oid4vp

`identus-oid4vp` is the runtime-neutral, chain-agnostic OpenID for Verifiable
Presentations 1.0 protocol crate. Its current narrow profile supports:

- bounded `openid4vp:` by-reference invocation parsing;
- effect-free GET/POST Request URI envelopes and bounded response binding;
- compact signed Request Object verification through `identus-jose`;
- discovery-bound audience, injected freshness, and atomic replay evidence;
- strict bounded DCQL validation and selection behind SDK-owned types;
- composed `vp_token` / `direct_post` request routing;
- deterministic bounded plain `direct_post` success and protocol-error response
  construction; and
- Final `direct_post.jwt` metadata selection plus encrypted-response construction
  through an injected SDK-owned JWE capability port.

## Direct-post flow

```text
openid4vp: invocation
        │
        ▼
runtime adapter executes bounded Request URI envelope
        │
        ▼
signed JAR ──verify──▶ VerifiedRequestObject
                              │
                              ▼
                  audience + injected clock + atomic replay guard
                              │
                              ▼
                  RequestObjectSecurityEvidence
                              │
                              ▼
                  ValidatedAuthorizationRequest
                              │
                              ├─ validated HTTPS response_uri
                              ├─ bounded optional state
                              └─ private DCQL coverage/cardinality rules
                                             │
                 ┌─ caller-selected bounded presentations ─▶ DirectPostResponse
validated request┤
                 ├─ presentations + JWE port ──────────────▶ EncryptedDirectPostResponse
                 └─ explicit typed protocol error ─────────▶ DirectPostErrorResponse
                                      both: inert POST + form body
```

Both response types are inert transport material. Revealing a target and body
does not authorize delivery. A platform adapter still owns DNS and IP
policy, TLS, redirects, decompression, timeouts, retries, SSRF defenses and HTTP
outcome handling. Applications still own verifier trust, consent, credential
and presentation verification, presentation generation, and the durable atomic
replay implementation behind the SDK port.

## Supported response shapes

The response body contains a deterministic form-encoded `vp_token` JSON object
and returns the exact signed `state` when one was present. Query keys follow
validated DCQL query order. Each value is a non-empty array of bounded JSON
strings or objects. Required credential-set alternatives and each query's
`multiple` flag are checked before serialization.

The mutually exclusive error body begins with a closed RFC 6749/OpenID4VP Final
error code, optionally includes bounded developer-oriented description and URI
reference values, and returns exact request `state` when present. Those optional
values are untrusted and redacted from diagnostics. The SDK never maps an
internal `Oid4vpError` to a wire error: callers remain responsible for selecting
the protocol outcome only after applying End-User interaction, consent and
privacy policy. In particular, an error URI is validated as inert syntax and is
never resolved by this crate.

For `direct_post.jwt`, verified `client_metadata` must select exactly one public
P-256 `ECDH-ES` recipient; the first profile supports `A128GCM` only. The SDK
constructs the exact response JSON, calls the injected `JweEncrypter` once, and
accepts only a bounded five-segment compact JWE whose protected header repeats
the selected `kid`. The resulting form contains only `response`. SDK-Rust does
not yet bundle a production JWE engine, so an application or future adapter
must implement the port. SIOPv2, Digital Credentials API, HAIP, network I/O and
credential-format proof creation remain separate capabilities.

The normative boundary and decisions are recorded in
[`ADR 0179`](../../docs/adr/0179-construct-bounded-oid4vp-direct-post-responses.md),
[`ADR 0180`](../../docs/adr/0180-construct-explicit-oid4vp-protocol-errors.md),
[`ADR 0181`](../../docs/adr/0181-use-an-sdk-owned-jwe-port-before-adopting-an-engine.md),
[`ADR 0183`](../../docs/adr/0183-require-oid4vp-request-object-security-evidence-before-routing.md),
with executable lineage in the versioned
[`plain direct-post vector packet`](../../docs/conformance/fixtures/oid4vp-direct-post/v1/vectors.json)
and
[`encrypted-response packet`](../../docs/conformance/fixtures/oid4vp-encrypted-direct-post/v1/vectors.json).

The exact Final capability status and remaining focused owners are maintained
in the machine-checked
[`OpenID4VP wallet-core capability report`](../../docs/conformance/oid4vp-final-wallet-core.md).
The report is traceability evidence rather than a certification claim.
