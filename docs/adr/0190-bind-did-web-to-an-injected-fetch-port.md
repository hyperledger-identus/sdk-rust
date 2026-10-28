# ADR 0190: Bind did:web to an injected fetch port

- Status: Accepted
- Date: 2026-10-28
- Decision owners: SDK-Rust DID maintainers
- Issue: [#559](https://github.com/hyperledger-identus/sdk-rust/issues/559)

## Context

`did:web` resolution combines deterministic method syntax and URL mapping with
DNS, TLS, and HTTP effects. Selecting Reqwest, a browser fetch API, redirects,
timeouts, private-network rules, or trust roots inside the method crate would
make a runtime choice part of the portable SSI domain and enlarge every
consumer's dependency and security surface. Existing Rust implementations are
valuable interoperability oracles but embed broader SSI and HTTP stacks.

## Decision

Create `identus-did-web` as a strict method core. It owns bounded method parsing,
deterministic HTTPS request derivation, a minimal boxed-future fetch port,
bounded response-envelope validation, generic DID document parsing, and exact
document-ID correlation.

The request grants only the exact URL and accepted DID JSON media types. The
response contains status, optional media type, and owned bounded bytes. The
adapter owns HTTP execution, DNS, TLS, redirects, proxy, cache, retries,
cancellation, timeouts, privacy, telemetry, and SSRF policy. The core neither
follows redirects nor accepts an effective URL. A synchronous binding function
keeps protocol validation independently reusable and testable.

The first identifier profile accepts lowercase ASCII FQDNs, a canonical
uppercase `%3A` non-default port, and conservative DID-`idchar` colon-separated
paths without percent escapes or `~`. It rejects IP literals, IDNA conversion, ambiguous escapes, empty/dot
segments, and noncanonical ports. Profile expansion requires a later decision.

## Consequences

Native, WASM, iOS, Android, server, and embedded adopters can implement one
small port using their approved runtime without coupling the method crate to an
HTTP ecosystem. Network security remains explicit where deployment context is
known. The SDK must document that syntactic validation and document-ID binding
do not establish DNS/TLS trust.

Adapters require a small amount of integration code, and the first profile is
stricter than some historical implementations. Deterministic in-memory E2E can
prove the portable contract, but real-network adapter conformance remains a
separate capability and decision.

## Rejected alternatives

- Embed Reqwest: selects native HTTP/TLS/executor policy and harms WASM/mobile portability.
- Accept a closure over raw bytes: obscures the reusable adapter contract and makes E2E weaker.
- Put retrieval in `identus-did`: couples a generic DID model to one method's effects.
- Allow arbitrary URL normalization: risks authority/path reinterpretation and weakens pinned mapping.
- Perform live-network E2E in the core: introduces nondeterminism without proving caller policy.
