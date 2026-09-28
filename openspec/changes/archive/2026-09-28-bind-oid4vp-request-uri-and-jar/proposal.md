# Why

The first `identus-oid4vp` slice stops after classifying an HTTPS Request URI.
A headless wallet still lacks a reusable, runtime-neutral way to construct the
normative GET or POST retrieval, bind the untrusted response, and prove that a
signed Request Object is a valid JAR before later authorization processing.

# What changes

- Correct the ingress contract to accept explicit Final
  `request_uri_method=get` as well as omitted/default GET and explicit POST.
- Consume a referenced request into a bounded HTTP request description and
  bind one bounded response without adding an HTTP runtime.
- Add bounded compact-JWS parsing and cryptographic JAR verification through
  the existing `identus-jose` registry/key abstractions.
- Require the Final protected `typ`, exact outer/inner `client_id` equality,
  and exact POST `wallet_nonce` correlation when one was sent.
- Preserve unverified-to-verified state separation and static redacted errors.

# Capabilities

## New capabilities

- `oid4vp-request-uri-retrieval`: bounded GET/POST request description and
  response binding for a Request Object by reference.
- `oid4vp-request-object-jar`: bounded signed compact JAR envelope validation.

## Modified capabilities

- `oid4vp-authorization-request-invocation`: explicit `get` becomes a valid
  retrieval-method spelling required by OpenID4VP 1.0 Final.

# Non-goals

- No HTTP, DNS, redirect, TLS, decompression, retry, cache, or entropy runtime.
- No JWE, JWS JSON serialization, client-prefix trust, DID/X.509/federation or
  verifier-attestation validation, audience/clock/replay policy, DCQL,
  credential selection, consent, response mode, or response construction.
- No publication, downstream change, Midnight primitive, or product policy.

# Delivery

Issue #396 and ADR 0158 control this additive unpublished slice. Planning and
the durable preflight receipt precede code. A signed/DCO PR targets `develop`
and may merge only after required hosted gates are green.
