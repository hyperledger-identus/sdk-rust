# Add portable did:web core

## Why

SDK-Rust owns generic DID documents and deterministic static DID methods, but
cannot resolve the common `did:web` method without making a product select an
HTTP stack. The method combines pure identifier-to-URL transformation with an
untrusted network retrieval. Those concerns need an explicit boundary so the
portable SDK remains usable on native, WASM, iOS, and Android targets while
runtime adapters retain DNS, TLS, redirect, proxy, cache, cancellation, retry,
and SSRF policy.

## What changes

- Add an independently selectable `identus-did-web` crate.
- Parse a strict ASCII FQDN, optional canonical `%3A<port>`, and colon-delimited
  path profile under finite identifier and segment limits.
- Derive the exact HTTPS `did.json` URL without performing I/O.
- Expose a minimal asynchronous fetch port carrying one bounded GET request and
  returning status, media type, and bytes.
- Bind a successful response under finite header/body limits, parse it with
  `identus-did`, and require exact DID document ID equality.
- Add normative, negative, property, fuzz, benchmark, portable-target,
  documentation, and deterministic in-memory E2E evidence.

## Capabilities

### Added capabilities

- `did-web-method`: bounded parsing, URL derivation, injected retrieval, and
  exact document binding for the approved `did:web` profile.

## Non-goals

No concrete HTTP client, DNS resolver, TLS policy, redirect following, proxy,
cache, retry, executor, timer, SSRF decision, IP-host profile, IDNA conversion,
registration/update/deactivation API, trust policy, persistence, binding,
consumer migration, or downstream repository mutation.

## Delivery

Issue #559 owns this SDK-only slice under ADRs 0187 and 0190. The change is
additive and unreleased. Pending quality declaration `did.web.profile.quality.v1`
and change-ledger ID `did.web.profile.v1` bind the evidence required before
activation.
