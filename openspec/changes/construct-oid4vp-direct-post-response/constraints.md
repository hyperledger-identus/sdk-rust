# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/487
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 keep dependencies inward and effects at the
edge. SDK-SEC-003 requires finite untrusted work. SDK-LIM-007 requires explicit
input and output bounds. The protocol, error, target, naming, unsafe-code and
release constraints remain effective. ADRs 0165 and 0168 authorize the private
DCQL engine and composed direct-post request lineage.

## Introduced or changed constraints

- Response construction consumes one `ValidatedAuthorizationRequest`; it does
  not accept independent URI, state, query-id, or cardinality assertions.
- Optional request `state` is bounded, retained in redacted owned storage, and
  returned exactly when present.
- Every response key must match exactly one validated DCQL credential-query
  identifier. Unknown and duplicate keys fail closed.
- Without credential sets every query result is present. With credential sets,
  every required set has at least one fully present option; optional sets
  impose no coverage requirement. Every present result is non-empty, and
  `multiple=false` has exactly one presentation.
- Presentation values are only Final JSON strings or objects. Values, counts,
  JSON work, `vp_token`, and final form bytes have public positive limits.
- Form output is deterministic UTF-8 `application/x-www-form-urlencoded` with
  `vp_token` first and optional `state` second.
- Public errors and `Debug` output expose static categories, variants, counts,
  and lengths only; no URI, state, query id, credential, or presentation bytes.
- The slice adds no dependency, unsafe code, native code, HTTP, executor,
  filesystem, clock, randomness, private key, signing, encryption, persistence,
  logging, or global mutable state.

## Introduced or changed limitations

- Only OpenID4VP 1.0 Final plain `direct_post` success responses are supported.
- The result is an inert request envelope; caller adapters retain DNS, address,
  TLS, redirect, timeout, retry, SSRF, and HTTP execution policy.
- The SDK does not prove presentation authenticity, holder binding, consent,
  verifier trust, replay defense, or successful delivery.
- `direct_post.jwt`, JWE, SIOPv2, DC API, HAIP and protocol error responses are
  separate decisions.
- Rust types and errors are canonical; language facades are adapter work.

## Consumer and product impact

Headless wallets gain a portable final assembly step that cannot detach a
response from validated request/DCQL lineage. Existing callers remain unchanged
because the crate is unpublished and the new API is additive. No consumer
repository is mutated by this issue.

## Activation and rollback

Activation requires issue #487 preflight, strict OpenSpec validation, official
and negative vectors, property/fuzz/benchmark declarations, native and portable
target evidence, dependency evidence, exact-head Claude review or immediate
Codex CLI fallback, protected green CI, and an issue-linked PR. Before
publication rollback deletes the additive response surface and retained private
metadata; no persistent data migration exists.

## Evidence

Evidence authorities are issue #487, ADR 0179, OpenID4VP 1.0 Final sections
8.1/8.2, RFC 6749, ADRs 0165/0168, IDR-024, OpenSpec/factory receipts, the
quality declaration, independent CLI review, and protected CI.
