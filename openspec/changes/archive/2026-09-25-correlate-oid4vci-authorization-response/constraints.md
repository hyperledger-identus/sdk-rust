# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/356
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001`/`002`, `SDK-SEC-001` through `003`, `SDK-COMPAT-001` through
`005`, `SDK-DELIVERY-001`, and `SDK-LIM-001`, `005`, `006`, `007`, `009`
remain effective. B09, issue #356, ADR 0083, ADR 0102, ADR 0142,
OpenID4VCI Final, RFC 6749, RFC 9207 and RFC 9700 direct this bounded generic
protocol transition.

## Introduced or changed constraints

- Owner remains `identus-oid4vci`; no wallet, chain, browser, OS or transport
  behavior enters the crate.
- Input is one already-routed query component, not a full callback URI, HTTP
  response, form-post body, fragment, JARM object or browser event.
- Exact state comparison precedes usable outcomes.
- The expected `iss` is always the selected Authorization Server metadata
  issuer. Advertised support requires exact presence/match; false or omitted
  support rejects a present issuer.
- Success retains the #354 lineage plus one bounded code. Error drops request
  secrets and retains only bounded explicitly untrusted fields.
- Positive bounds cover encoded query bytes, field count, decoded names/values
  and every retained role. Unknown fields are validated and discarded.
- New diagnostics append after every existing live error row.

## Introduced or changed limitations

No effective limitation is removed. The response parser does not validate
callback routing/registration, response-mode negotiation, fragments,
form-post, JARM/JAR, PAR, browser listeners, HTTP/DNS/TLS, entropy, durable
state invalidation, code exchange, client authentication, DPoP, token handling,
authorization/trust, retry/recovery, persistence, product policy,
Midnight/Cardano/PRISM logic or credential formats. `NotAdvertised` is not
mix-up protection and later owners must not represent it as such.

## Consumer and product impact

Consumers gain one headless query correlation transition and truthful issuer
evidence. Adapters retain callback routing and transport authority. No stored
data, migration, product, chain, format, release, certification or support
promise changes.

## Activation and rollback

Activation requires a committed OpenSpec contract and immutable preflight
receipt before code, exact-diff review, canonical archive, signed+DCO commits,
green hosted `fast` evidence and merge to `develop`. Rollback removes the
additive response states/parser, RFC 9207 metadata projection, private grammar
factoring, diagnostics and evidence without changing #354 request behavior.

## Evidence

Normative success/error examples, query-form octets, default and delegated
servers, metadata flag true/false/omitted/type confusion, exact state/issuer,
duplicate/ambiguity/malformed cases, exact/one-over limits, ownership,
redaction, predecessor regressions, factory gates, full Rust gates and exact
hosted CI are mandatory.
