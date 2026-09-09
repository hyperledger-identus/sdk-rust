# OAuth2 authorization-code and PKCE research

Research class: protocol
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

`identus-oid4vci` owns bounded OID4VCI 1.0 Final issuer and authorization-server
metadata, authorization-code grant hints, pre-authorized token requests and a
partial bounded token response. It has no authorization-request or
authorization-code token-exchange API. Consumer evidence is the wallet-side
authorization-code flow required by issue #160 and the OID4VCI roadmap; no
downstream integration is claimed.

The current implementation therefore needs a reuse decision before adding
those mechanics. Exact `oauth2 5.0.0` is a mature typed OAuth client, but its
URL, HTTP, secret, clock, RNG and response models are one inseparable default-
disabled runtime cone. The spike evaluates public candidate APIs, not copied
source, and leaves every Identus API and wire representation unchanged.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html),
  published 2025-09-16, sections 5.1 and 6.1. It defines the authorization
  request as RFC 6749 section 4.1.1 behavior, recommends PKCE, defines
  `authorization_details`/scope/`issuer_state`, and uses RFC 6749 section 4.1.3
  for the authorization-code token request.
- [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749.html), authorization code
  grant and token request.
- [RFC 7636](https://www.rfc-editor.org/rfc/rfc7636.html), including the
  Appendix B verifier/challenge vector used by the fixture.
- [RFC 9700](https://www.rfc-editor.org/rfc/rfc9700.html), OAuth 2.0 Security
  Best Current Practice. It requires transaction-specific binding and treats
  S256 as the current non-disclosing challenge method.
- [`oauth2` tag 5.0.0](https://github.com/ramosbugs/oauth2-rs/tree/f3424b4b2190c83c6d031fdc71eed2351d49e0df),
  release commit `f3424b4b2190c83c6d031fdc71eed2351d49e0df`, MIT OR
  Apache-2.0. The crates.io archive checksum is
  `51e219e79014df21a225b1860a479e2dcd7cbd9130f4defd4bd0e191ea31d67d`.

The release tag, not repository main at `72ce7440`, is the immutable provenance
behind version 5.0.0. The main revision is maintenance evidence only.

## Candidate decisions

| Candidate/mechanic | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| oauth2 authorization URL | 5.0.0 / `f3424b4b` | `oracle` | Correctly orders/encodes the basic flow and supports OID4VC extension strings, but endpoint/scope/extra-parameter inputs are unbounded and permit policy-invalid or duplicate reserved values. | A named engine needs the breadth and an Identus facade proves bounds, duplicate rejection, endpoint policy and smaller feature isolation. |
| oauth2 PKCE S256 | 5.0.0 / `f3424b4b` | `oracle` | Matches the RFC 7636 vector and redacts verifier Debug, but its verifier constructor accepts invalid characters, its challenge method panics on invalid length, and rand/getrandom remain in the cone. | A narrow non-panicking API validates the complete verifier grammar, accepts injected entropy and feature-slices PKCE from client/runtime models. |
| oauth2 token request | 5.0.0 / `f3424b4b` | `oracle` | The injected recording client proves exact public-client form bytes without a network stack, but request/HTTP types, implicit client-auth selection and unrestricted extension duplicates do not fit domain boundaries. | Two consumers prove shared transport/client-auth semantics behind bounded owned inputs and an injected Identus transport port. |
| oauth2 token response | 5.0.0 / `f3424b4b` | `not-adopt` | No response byte/depth/member limit; unknown OID4VC fields can be ignored; parse errors retain the complete raw response body; chrono clock support is unconditional. | A released bounded parser preserves required OID4VC extensions and guarantees redacted diagnostics without clock/runtime coupling. |
| SDK-owned future facade | current `develop` | `retain-local` | Existing bounded metadata/token states and strict errors remain authoritative; no authorization engine exists yet to justify the broad cone. | A focused production issue names consumer states, ports, limits and exact reusable mechanics. |

The crate's overall production disposition is `oracle`, not adoption. A future
engine may use the fixture and RFC bytes for comparison, but it cannot depend on
the research crate or infer that one passing mechanic admits the full client.

## Compatibility and dependency evidence

The direct and resolved dependency cone was measured with the exact locked
candidate manifest across every compile target listed below.

The fixture selects exact `oauth2 = 5.0.0` with `default-features = false` and
no optional feature. The candidate still has ten unconditional direct
dependencies: base64, chrono with `clock`/`serde`/`std`/`wasmbind`, http, rand,
serde, serde_json, serde_path_to_error, sha2, thiserror and url. WASM adds
target-specific getrandom with JavaScript support.

The separately generated lock contains 95 external packages across target
conditions. Unique normalized normal/build tree lines are 68 on host, iOS and
Android and 77 on `wasm32-unknown-unknown`. Disabling defaults removes Reqwest
and TLS, but not URL/ICU, clock, RNG, HTTP, JSON or WASM-bindgen families. The
crate declares MSRV Rust 1.65, below the SDK's exact Rust 1.98.1 etalon.

Rust 1.98.1 host tests and strict Clippy passed. Exact locked compile checks
passed for `wasm32-unknown-unknown`, `aarch64-apple-ios` and
`aarch64-linux-android` using repository Nix shells. Those are compile receipts,
not runtime or support claims.

Public and wire compatibility is unchanged because the fixture is a nested
workspace and no dependency enters root Cargo manifests or locks. No oauth2,
url, http or chrono type crosses an Identus facade boundary. Rollback deletes
the fixture, script, ADR and research spec; no migration is required.

## Security, privacy and maintenance evidence

The deterministic RFC 7636 verifier produced challenge
`E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM`. Public candidate APIs generated
the expected authorization URL and exact public-client token form through an
in-memory `SyncHttpClient`; no network, clock or RNG call occurred. Secret
newtypes redact Debug, which is good evidence.

The mismatch tests also prove why no production dependency is admitted:

- `AuthUrl` accepts an HTTP URL containing userinfo and a fragment; it provides
  syntax rather than the SDK's HTTPS/no-userinfo/no-fragment policy and has no
  byte ceiling.
- `Scope::new` accepts embedded spaces. `add_extra_param` can duplicate
  `client_id`; upstream documents conflicts with managed parameters as
  undefined. OID4VC `authorization_details`, `resource` and `issuer_state`
  remain unrestricted strings at this layer.
- `PkceCodeVerifier::new` accepts an invalid 43-character `!` value.
  `from_code_verifier_sha256` checks only byte length with `assert!` and panics
  below 43 or above 128 bytes instead of returning a bounded error.
- A one-megabyte unknown `authorization_details` field is parsed and ignored by
  the basic token response. A parse failure returns `RequestTokenError::Parse`
  containing the complete response body, including an access-token canary.
  The SDK cannot permit that dependency error across Debug/log/FFI boundaries.

The oauth2 crate's own Rust source contains no unsafe block, native link or
build script. Its resolved graph reaches dependency-owned unsafe/build/native
code, including libc/Core Foundation or Android platform time handling and,
on WASM, wasm-bindgen/js-sys through unconditional clock/RNG features. This is
not a vulnerability claim; it is material cone and audit scope.

Root-configured `cargo deny` passed advisories, bans, licenses and sources, with
only expected unmatched-root-exception and duplicate-syn warnings. `cargo
audit --deny warnings` scanned 96 lock entries and passed. Exact locks and the
registry checksum provide supply-chain evidence; no current advisory is a
future guarantee.

Version 5.0.0 was released 2025-01-21. The repository is not archived and main
was pushed through 2026-02-22, when it began separating Reqwest integration,
but no newer core oauth2 release exists on the retrieval date. The declared
MSRV policy is six months of Rust compatibility. Maintenance is credible, yet
the 5.0.0 feature topology and OID4VC semantic/resource mismatches dominate the
decision. Protocol or draft currency is OID4VCI 1.0 Final plus RFC 9700;
oauth2 remains a general RFC 6749 client rather than an OID4VCI profile
implementation.

## Rejected or deferred candidates

Production oauth2 5.0.0 is not adopted now. Default HTTP clients, URL/HTTP/
chrono public types, candidate RNG, browser launch, redirect listener,
authorization response correlation, PAR, DPoP, RAR, persistence and downstream
integration are deferred. Reimplementing PKCE casually is also rejected: a
future owned implementation must reuse existing audited SHA-256/base64 engines
or a narrower maintained crate and retain this RFC vector as oracle evidence.

## Open questions and blockers

There is no research blocker. The next production question belongs to a new
authorization-engine issue: define bounded state/PKCE secret ownership,
authorization response correlation, redirect/transport ports, typed OID4VC
authorization details and client-auth policy before selecting an engine.

## Evidence commands

The ephemeral pre-readiness probe and committed fixture use exact locked
commands equivalent to:

```text
cargo +1.98.1 generate-lockfile
cargo +1.98.1 test --locked
cargo +1.98.1 clippy --locked --all-targets -- -D warnings
cargo +1.98.1 tree --locked -p oauth2 --edges normal,build
cargo deny --manifest-path <fixture>/Cargo.toml --config deny.toml check
cargo audit --file <fixture>/Cargo.lock --deny warnings
nix develop .#wasm --command cargo check --locked --target wasm32-unknown-unknown
nix develop .#bindings --command cargo check --locked --target aarch64-apple-ios
nix develop .#bindings --command cargo check --locked --target aarch64-linux-android
rg 'unsafe|extern "C"|#\[link' <oauth2-5.0.0>/src
```

Unrun checks are browser/mobile runtime, network interoperability, downstream
builds, performance, publication, release, certification, PAR, DPoP and
authorization-response processing. No success is inferred for them.
