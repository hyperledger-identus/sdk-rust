# OID4VCI Authorization Request research

Research class: protocol
Research status: ready
Decision date: 2026-09-25
Source retrieval date: 2026-09-25
Research blockers: none

## Problem and existing implementation

Issue #352 delivered `CredentialOfferWithAuthorizationRequestInput`, which
owns one selected offered configuration, a capable Authorization Server,
bounded client/redirect/state values, a validated verifier and its SDK-derived
S256 challenge. It intentionally emits no wire bytes. The remaining narrow
problem is to turn that state into one exact bounded request without importing
runtime policy or allowing endpoint query data to override managed fields.
The current implementation already contains the strict local form serializer
for Pre-Authorized Token Requests and a strict decoder for Credential Offer
invocation values; both are suitable private mechanics to factor without
changing their existing behavior. Consumer evidence remains the wallet-side
flow under parent #7 and the exact separately locked oauth2 oracle.

## Normative sources

- [OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-final.html),
  published 2025-09-16, sections 5.1, 5.1.1 and 5.1.3. A request may identify
  a Credential with `authorization_details` or scope. The
  `openid_credential` detail requires `credential_configuration_id`; when
  issuer metadata contains `authorization_servers`, `locations` must contain
  the Credential Issuer identifier. Offered `issuer_state` is returned exactly.
- [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749.html), sections 3.1 and
  4.1.1 plus Appendix B. The Authorization Endpoint may contain a form query
  that must be retained; request parameters must not repeat; code flow requires
  `response_type=code` and `client_id` and defines redirect/state semantics.
- [RFC 9396](https://www.rfc-editor.org/rfc/rfc9396.html), sections 2 and 3.
  `authorization_details` is a JSON array of typed objects serialized as one
  application/x-www-form-urlencoded request parameter.
- [RFC 7636](https://www.rfc-editor.org/rfc/rfc7636.html), section 4.3. The
  request carries `code_challenge` and the explicit `S256` method.
- [RFC 9700](https://www.rfc-editor.org/rfc/rfc9700.html), section 2.1.1. PKCE
  and transaction-bound correlation remain required; serialization alone does
  not complete CSRF, code-injection or mix-up defenses.
- ADR 0083 retains the strict local form codec because `form_urlencoded 1.2.2`
  is lossy/permissive on malformed input. ADR 0102 keeps exact `oauth2 5.0.0`
  as an oracle rather than a production dependency. ADR 0141 defines the
  predecessor and private crypto boundary.

## Candidate decisions

| Candidate | Decision | Evidence and boundary |
| --- | --- | --- |
| Emit scope from configuration metadata | `not-adopt` | The bounded metadata summary does not retain optional scope, and inventing one would be profile policy. |
| Emit both scope and Authorization Details | `not-adopt` | Final permits both but requires individual interpretation; doing so silently creates ambiguous/duplicated credential intent. |
| Emit one `openid_credential` Authorization Detail | `adopt` | It directly names the already selected configuration and needs no format-profile interpretation. |
| Always emit `locations` | `not-adopt` | Final makes it mandatory only when issuer metadata explicitly contains `authorization_servers`. |
| Conditionally emit issuer `locations` | `adopt` | It satisfies Final section 5.1.1 exactly and distinguishes explicit server delegation from the default issuer-as-server case. |
| Emit RFC 8707 `resource` | `not-adopt` | Final recommends it for the scope route; the selected Authorization Details route already uses conditional `locations`. |
| Adopt `oauth2 5.0.0` | `oracle` | ADR 0102 rejects its broad unconditional URL/clock/RNG/HTTP/JSON cone and under-bounded construction surface. |
| Adopt `form_urlencoded 1.2.2` | `oracle` | ADR 0083 rejects its permissive/lossy parser and found serializer-only payoff too small after checked sizing, ordering and secret ownership. |
| Reject all endpoint queries | `not-adopt` | RFC 6749 explicitly permits a form query and requires retaining it. |
| Preserve endpoint query without inspection | `not-adopt` | Reserved or duplicate names could override the SDK's typed request claims. |
| Strictly validate, bound and retain endpoint query | `retain-local` | The existing local form rules can reject malformed UTF-8/percent encoding, duplicate decoded names and managed-name collisions before exact retention. |
| Accept caller extra parameters | `defer` | Extension ownership and collision policy need an explicit later contract; this slice handles only the advertised endpoint query and mandatory request fields. |

## Deterministic contract

The Authorization Details JSON is minified and ordered as `type`, conditional
`locations`, then `credential_configuration_id`. The SDK appends these managed
parameters in this order after an exact retained endpoint query:

1. `response_type=code`
2. `client_id`
3. `redirect_uri`
4. `state`
5. `code_challenge`
6. `code_challenge_method=S256`
7. `authorization_details`
8. optional `issuer_state`

Application/x-www-form-urlencoded output uses ASCII alphanumeric plus `*-. _`
literal rules without the displayed space: space becomes `+`, all other bytes
become uppercase `%HH`. The existing query is preserved byte-for-byte only
after strict decoding proves bounded unique non-empty names, bounded values,
and no collision with managed, alternate-intent (`scope`, `resource`) or
request-object (`request`, `request_uri`) parameters.

## Compatibility and dependency evidence

The API and wire surface are additive in an unpublished experimental crate.
No dependency, feature, lockfile, target, unsafe/native, runtime, clock, RNG,
network or crypto change is required. The local serializer already has an
accepted ADR and exact tests in the crate; this slice factors it into a private
cohesive helper rather than adding a third-party type to the public facade.

The direct and resolved dependency cone remains exactly the current
`identus-core + identus-crypto + identus-jose` internal cone plus its already
locked external packages. No new feature is enabled. The repository MSRV is
Rust 1.89.0 and the primary/etalon compiler is Rust 1.98.1; host, WASM, iOS and
Android target claims remain governed by the existing slow/release matrix.

The exact oauth2 oracle remains release 5.0.0 at revision
`f3424b4b2190c83c6d031fdc71eed2351d49e0df`, MIT OR Apache-2.0, checksum
`51e219e79014df21a225b1860a479e2dcd7cbd9130f4defd4bd0e191ea31d67d`.
The exact rejected form candidate remains `form_urlencoded 1.2.2`, MIT OR
Apache-2.0, checksum
`cb4cb245038516f5f85277875cdaa4f7d2c9a0fa0468de06ed190163b1581fcf`.
Those exact version, feature, license and provenance records are retained from
ADRs 0083 and 0102. Their recorded cargo-deny/audit results remain
supply-chain evidence for the oracles, not permission to add either production
dependency. No new upstream or consumer source is copied.

## Security, privacy and maintenance evidence

Positive limits independently bound Authorization Details bytes, existing
endpoint-query parameter count, decoded query name bytes, decoded query value
bytes and final request URI bytes. Checked size arithmetic runs before final
allocation. Malformed percent escapes, decoded invalid UTF-8/NUL, empty names,
duplicate decoded names and reserved names fail with static diagnostics.

The request URI contains state and may contain issuer state, so it uses
zeroizing ownership, a redacted Debug surface and an explicitly sensitive
accessor. Browser history, referrer leakage, URI-handler ownership and
endpoint-query trust remain adapter/application concerns and are not hidden by
the type name.

Maintenance stays inside one small crate-private form module under the existing
ADR 0083 decision. Security posture is fail-closed and release posture is
unchanged: this unpublished additive API is not publication, certification or
production-promotion evidence. No direct unsafe block, native library, build
script or new advisory surface is introduced.

## Rejected or deferred candidates

Scope/resource, caller extensions, PAR/JAR, browser dispatch, response parsing
and correlation, code exchange, transport and downstream adoption are rejected
from this slice or deferred to focused successors. Reusing oauth2 or
form_urlencoded in production remains rejected under their accepted ADRs.

## Open questions and blockers

There is no blocker for this request-construction slice. A successor must parse
and correlate the Authorization Response with the exact selected server,
redirect and state before any code-exchange claim. PAR remains recommended by
Final but requires its own client authentication, endpoint and transport
contract.

Protocol and draft currency is OpenID4VCI 1.0 Final plus the published RFCs;
no pre-final draft behavior is admitted. The reconsideration trigger for the
form dependency is ADR 0083's strict non-lossy parity or demonstrated
multi-consumer serializer payoff. The oauth2 trigger remains ADR 0102's
released, bounded feature-sliced facade evidence. Rollback removes the
additive request type and private factoring while leaving the issue #352
predecessor and all existing wire bytes unchanged.

## Evidence commands

```text
scripts/factory research-ready construct-oid4vci-authorization-request
scripts/factory constraints-ready construct-oid4vci-authorization-request
scripts/factory preflight construct-oid4vci-authorization-request --issue 354 --write
cargo test -p identus-oid4vci --test authorization_request
cargo test -p identus-oid4vci --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps
scripts/factory check
```

External server/browser/PAR/callback interoperability, mobile URI dispatch,
HTTP, response correlation, code exchange, downstream adoption, performance,
publication and certification remain unrun and unclaimed.
