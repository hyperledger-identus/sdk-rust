# OID4VP redirect response boundary

`identus-oid4vp` owns deterministic protocol material for `fragment`, `query`,
`form_post`, `direct_post`, and `direct_post.jwt`. It does not execute any of
those effects. Every response begins at the same verifier-authorized signed JAR
state:

```text
signed Request Object
  │ signature + audience + time + replay + exact verifier/key policy
  ▼
ValidatedAuthorizationRequest
  │ response_mode selects exactly one signed destination
  ├─ fragment/query ──► complete inert URI
  ├─ form_post ───────► inert target + form body
  └─ direct_post(.jwt)► inert HTTP request material
                            │ caller executes exact request
                            ▼
                    status 200 + application/json + no-store
                            │ bounded duplicate-safe JSON
                            ▼
                  Complete | inert redirect candidate
```

## Destination and effect rules

Redirect modes require one exact absolute `redirect_uri`; direct-post modes
require one exact HTTPS `response_uri`. Both or the wrong member fail before
response construction. The caller's `VerifierTrustPolicy` sees the exact signed
destination and can enforce DID, PKI, Federation, attestation, or registration
redirect rules before the typed request exists.

Query and fragment construction preserves non-conflicting existing components,
rejects decoded collisions with `vp_token`, `state`, `error`,
`error_description`, or `error_uri`, and checks the complete encoded URI. Form
post retains the exact target and returns only form data; adapters own secure
HTML, CSP, origin, browser, and submission behavior.

No SDK result authorizes navigation. Native, WASM, mobile, and TypeScript
adapters must apply product privacy, consent, deep-link, origin, and session
policy before revealing or dispatching presentation-bearing material.

## Direct-post acknowledgement

`bind_endpoint_acknowledgement` consumes the response envelope so the parser is
not a detached JSON utility. The caller must invoke it only with the HTTP
response to that exact sent request. The parser accepts status 200, JSON media,
`Cache-Control: no-store`, a bounded duplicate-safe object, and an optional
absolute `redirect_uri`.

One `response_code` may occur across the redirect query and fragment. It is
retained as opaque zeroizing text. The SDK does not estimate entropy, redeem the
code, store it, infer verifier success, or authorize navigation. Ambiguous,
empty, malformed, or oversized codes fail closed.

## Adapter checklist

1. Accept the exact verifier/destination binding in `VerifierTrustPolicy`.
2. Build one response from the consuming `ValidatedAuthorizationRequest`.
3. Apply disclosure, consent, and URL-leakage policy before any effect.
4. For direct post, execute only the exposed method, target, media type, and
   body under independent DNS/TLS/SSRF/redirect/time limits.
5. Bind the exact endpoint acknowledgement by consuming the sent response.
6. Apply navigation and response-code redemption policy outside the SDK.

The full-lineage executable example is
`crates/oid4vp/tests/redirect_response.rs`; fuzz and benchmark evidence share
the existing direct-post harness so acknowledgement behavior remains part of
the same request lineage.
