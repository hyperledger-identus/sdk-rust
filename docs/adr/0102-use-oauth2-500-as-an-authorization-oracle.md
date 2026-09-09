# ADR 0102: use oauth2 5.0.0 as an authorization oracle

- **Status:** Accepted
- **Date:** 2026-09-09
- **Issue:** [#160](https://github.com/hyperledger-identus/sdk-rust/issues/160)
- **Decision authority:** ADR 0061 and issue #160
- **Assessed source:** [`oauth2@f3424b4b`](https://github.com/ramosbugs/oauth2-rs/tree/f3424b4b2190c83c6d031fdc71eed2351d49e0df), MIT OR Apache-2.0
- **Research:** OpenSpec change `evaluate-oauth2-authorization-code-pkce`

## Context

The SDK will need OID4VCI authorization-code and PKCE behavior. Reimplementing
closed OAuth mechanics raises conformance cost, but importing a full OAuth
client can also transfer endpoint, transport, clock, RNG, secret, resource and
error policy into domain crates. Issue #160 therefore requires an executable
spike before a production choice.

Exact oauth2 5.0.0 with defaults disabled passes Rust 1.98.1 host, WASM, iOS and
Android compile checks. It matches the RFC 7636 S256 vector and generates the
expected authorization and token requests through an injected in-memory HTTP
client. Its secret values redact Debug.

The same minimal feature choice still resolves 68 normalized host/iOS/Android
normal/build tree lines and 77 on WASM. Clock, RNG, URL/ICU, HTTP and JSON are
unconditional. Constructors accept policy-invalid endpoint/scope/verifier
inputs, invalid PKCE lengths panic, unrestricted extra parameters may duplicate
managed fields, response parsing has no resource bound, and parse errors retain
the complete raw body. A large unknown OID4VC field is silently ignored by the
basic response model.

## Decision

1. Classify oauth2 5.0.0 as an `oracle`, not a production dependency.
2. Keep the exact separately locked fixture and RFC bytes as reproducible
   authorization URL, PKCE and token-request comparison evidence.
3. Do not use its token response parser in SDK code. The unbounded input and
   raw-body parse error conflict with the SDK's resource and redaction rules.
4. Do not expose or internally normalize around oauth2/url/http/chrono types in
   `identus-oid4vci`. Existing bounded Identus types remain authoritative.
5. A future authorization engine must first define bounded request/state/PKCE
   types, response correlation, redirect and transport ports, client-auth
   policy and typed OID4VC extensions. It may then reconsider individual
   mechanics, never infer acceptance of the entire client.
6. Keep the fixture outside root locks and ordinary fast CI. Research success
   does not activate a runtime dependency, downstream use or support claim.

## Consequences

The SDK gains executable independent evidence without paying the candidate's
production cone or leaking its policy/types. A future engine may require a
small amount of owned implementation around existing SHA-256, base64 and strict
form mechanics, but that code will be bounded and directly aligned with
OID4VCI Final. The decision avoids false reuse economy: one correct PKCE hash
does not justify a broad unbounded client.

## Reconsideration and rollback

Reconsider if a released oauth2 version feature-slices authorization-code/PKCE
from clock, RNG, HTTP and response models; validates verifier grammar without
panic; supports bounded redacted response handling; and a named engine proves
multi-mechanic payoff behind Identus-owned ports and types. Rollback deletes the
fixture and research assets; no API, wire, data or consumer migration exists.
