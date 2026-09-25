# OID4VCI Authorization Response research

Research class: protocol
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

Issue #356 starts from `AuthorizationRequest` delivered by #354. That owner
already contains the exact selected Authorization Server Metadata, CSRF state,
redirect URI, PKCE verifier/challenge and selected credential intent. This
slice parses and correlates only the default OAuth Authorization Code query
response. It does not receive a browser event, validate OS routing, exchange a
code, or perform network work.

The crate already has a strict private form codec and bounded redacted OAuth
Token Error Response grammar. Those mechanics can be shared privately without
changing existing wire behavior or admitting a new dependency.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  sections 5.2 and 5.3, requires successful and error Authorization Responses
  to follow RFC 6749.
- [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749.html), sections 3.1,
  4.1.2 and 4.1.2.1 plus Appendix B, requires query-form response parameters,
  forbids repeated response parameters, requires exact returned state when
  requested, defines success/error fields and six base Authorization Endpoint
  errors, requires ignoring unknown fields, and leaves code size undefined.
- RFC 6749 Appendix A.7-A.9 defines error, description and URI-reference
  grammar; Appendix A.11 defines code as one or more visible ASCII characters.
- [RFC 9207](https://www.rfc-editor.org/rfc/rfc9207.html), sections 2-3,
  defines form-decoded `iss`, exact simple-string comparison with the selected
  server metadata issuer, the metadata support flag and its false default.
- [RFC 9700](https://www.rfc-editor.org/rfc/rfc9700.html), sections 2.1 and
  2.1.1, requires CSRF protection and a mix-up defense for clients using
  multiple Authorization Servers. RFC 9207 is preferred; distinct redirect
  URIs are an adapter-owned alternative.
- ADR 0083 retains strict local form mechanics; ADR 0102 keeps exact `oauth2
  5.0.0` oracle-only; ADR 0142 defines the sensitive predecessor lineage.

## Candidate decisions

| Candidate | Decision | Evidence and boundary |
| --- | --- | --- |
| Accept a full callback URI | `not-adopt` | Routing, custom schemes, loopback ports and claimed HTTPS are adapter/OS policy; string-prefix validation is unsafe. |
| Accept an already-routed query component | `adopt` | Matches the request's default query response mode and keeps protocol correlation transport-neutral. |
| Accept query, fragment, form-post and JARM in one parser | `not-adopt` | These have distinct delivery and cryptographic contracts and would create ambiguous authority. |
| Require exact returned state | `adopt` | Every predecessor sent state; RFC 6749 requires its exact return and RFC 9700 requires CSRF defense. |
| Compare `iss` with the Credential Issuer | `not-adopt` | RFC 9207 identifies the Authorization Server that produced the response. |
| Derive issuer rules from selected server metadata | `adopt` | The predecessor owns exact validated server metadata and its RFC 9207 support signal. |
| Treat missing issuer support as verified | `not-adopt` | Omission defaults false and cannot truthfully prove mix-up protection. |
| Preserve `VerifiedRfc9207` versus `NotAdvertised` | `adopt` | Keeps interoperability without inventing issuer evidence. |
| Adopt `oauth2 5.0.0` | `oracle` | ADR 0102 rejects its broad unconditional cone for this narrow bounded parser. |
| Reuse private strict form and OAuth grammar | `retain-local` | Existing exact behavior is accepted, locked and regression-tested. |
| Perform Authorization Code exchange | `defer` | Token Endpoint, client authentication, request body and transport are a separate transition. |

## Selected contract

The API accepts an already-extracted query component without a leading `?`.
The caller must first route the callback to the exact registered handler. A
complete encoded-byte ceiling applies before strict form decoding. Positive
bounds cover parameter count, decoded names/values and retained roles. Empty
names, NUL, malformed UTF-8/percent escapes and decoded duplicate names fail;
unknown unique fields are validated then discarded.

Returned state is mandatory and exactly compared before a code or remote error
becomes usable. Exactly one branch is valid: success has a non-empty visible-
ASCII `code` and no error fields; error has a non-empty NQSCHAR `error`, no
code, and optional independently bounded NQSCHAR description and syntactically
valid URI-reference. Empty known parameters are treated as omitted. Six
standard errors receive a closed classification; extensions remain exact and
untrusted.

Authorization Server Metadata retains optional boolean
`authorization_response_iss_parameter_supported`, defaulting false. When true,
`iss` is required and exactly equals the selected Authorization Server issuer.
When false/omitted, `iss` must be absent. Outcomes expose only
`VerifiedRfc9207` or `NotAdvertised`; the latter is not a mix-up-defense claim.

## Compatibility and dependency evidence

Current implementation evidence is the merged #354 request lineage plus the
existing form and token-error tests. Consumer evidence is parent #7's generic
wallet flow and Oxid/Lace interoperability boundary; no downstream code is
copied or changed. The public facade boundary adds only protocol values and a
consuming state transition, never callback routing or transport.

The API is additive except for accepting and exposing one previously ignored
standard metadata flag. Existing metadata JSON, extensions, request bytes,
error prefix, public API and wire behavior remain compatible. The direct and
resolved dependency cone remains `identus-core + identus-crypto +
identus-jose`; no dependency, feature, lockfile, target, unsafe/native, source,
license or MSRV claim changes.

The exact oracle version and features remain `oauth2 5.0.0` at revision
`f3424b4b2190c83c6d031fdc71eed2351d49e0df`, MIT OR Apache-2.0, checksum
`51e219e79014df21a225b1860a479e2dcd7cbd9130f4defd4bd0e191ea31d67d`.
Its URL, transport, RNG, clock and serialization cone provides no payoff over
the accepted local bounded primitives. License and provenance are therefore
unchanged. Existing cargo-deny/advisory records remain supply-chain evidence
for the oracle, not permission to add it as a dependency.

## Security, privacy and maintenance evidence

The encoded input, parameter count, decoded names/values and every retained
role have positive ceilings. Size and count fail before unbounded work.
Duplicate comparison occurs after decoding, preventing encoded-name bypasses.

State and issuer correlation precede usable outcomes. Code, state, issuer,
description, URI and extensions use zeroizing ownership where retained and
never enter Debug or static diagnostics. Success alone retains the request for
the next code-exchange constructor; error drops verifier and request URI after
correlation. Request consumption prevents accidental double correlation
through this API but does not claim durable one-time-use enforcement.

Maintenance stays within the crate: reuse `form`, move two exact grammar
predicates into a private `oauth` module, and preserve token-error regression
tests. No runtime, unsafe block, build script or advisory surface is added.
The maintenance, release and security posture remains additive experimental
API guarded by the normal fast integration line and later slow production
evidence; this work is not publication or certification.

## Rejected or deferred candidates

Full callback URI validation, query/fragment auto-detection, form-post, JARM,
JAR, PAR, browser dispatch, state persistence/invalidation, HTTP and code
exchange are rejected from this slice or deferred to separately bounded
owners. Production adoption of `oauth2` remains rejected under ADR 0102.

## Open questions and blockers

There is no blocker for query-mode response correlation. The successor must
construct the Authorization Code Token Request without treating
`NotAdvertised` as a mix-up defense; it will need verified RFC 9207 evidence or
a separately typed BCP 240 alternative before code exchange. A future
form-post/JARM capability must be issue-first and cannot silently reuse this
query-only entry point. Protocol and draft currency is OpenID4VCI 1.0 Final
plus published RFCs; no pre-final draft rule is admitted. The reconsideration trigger
for external OAuth reuse remains ADR 0102's released bounded
feature-sliced facade with demonstrated multi-capability payoff. Rollback
removes the additive parser, metadata projection, grammar factoring and
evidence while leaving #354 behavior intact.

## Evidence commands

```text
scripts/factory research-ready correlate-oid4vci-authorization-response
scripts/factory constraints-ready correlate-oid4vci-authorization-response
scripts/factory preflight correlate-oid4vci-authorization-response --issue 356 --write
cargo test -p identus-oid4vci --test authorization_response
cargo test -p identus-oid4vci --test authorization_server_metadata
cargo test -p identus-oid4vci --test token_error_response_core
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps
scripts/factory check
scripts/factory backlog-live
```

Browser/mobile routing, form-post/JARM, HTTP, Token Endpoint interoperability,
state invalidation storage, code exchange, downstream adoption, performance,
publication and certification remain unrun and unclaimed.
