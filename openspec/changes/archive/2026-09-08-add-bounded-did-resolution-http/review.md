# Exact-diff local review

Review status: completed
Review date: 2026-09-08
Implementation head: d31e8c395f10f9e90c1b46c9a9f0bca3e88cdb30
Specification parent: d8de711f527d585818a87effe9ff02e74238f098
Unresolved blockers: none

## Scope reviewed

The review inspected the complete `origin/develop`-to-implementation diff,
the public router surface, every negotiation and response branch, all focused
tests, the resolved normal dependency graph, the conformance rulebook and the
issue #201 specification, ADR and constraint delta. The implementation was
also reconciled with the pinned W3C DID Resolution HTTP binding, RFC 9110 and
the NeoPRISM oracle revision recorded in research.

## Findings

1. **Architecture boundary — accepted.** The new crate is an outer-boundary
   adapter over `identus-did`; the conformance rulebook rejects the reverse
   edge. DID Core remains free of Axum and transport behavior.
2. **Public surface — accepted.** The crate exports one state-closed
   constructor and five policy constants. Resolver, DID, options and result
   values remain Identus-owned. Negotiation parser types stay private.
3. **Resource and syntax policy — accepted.** Repeated `Accept` values are
   bounded by aggregate bytes and range count before parsing. Quote-aware list
   validation and strict duplicate/quality checks close the selected library's
   permissive malformed-quality behavior.
4. **Protocol projection — accepted.** Full-result and document requests
   construct distinct options; document success requires matching metadata.
   All nine standard error kinds, extension errors and deactivation have
   explicit status and body behavior.
5. **Router variance — accepted after correction.** Exact-diff review found
   that Axum-generated 404/405 fallbacks lacked the specified `Vary: Accept`.
   The fixed route now installs explicit empty 404/405 fallbacks, with a
   regression proving variance and no resolver invocation.
6. **Privacy and failure safety — accepted.** Invalid path, DID, query and
   negotiation values are never reflected. Unexpected construction or
   serialization failure returns a static envelope. Production code contains
   no `unsafe`, panic, `unwrap`, `expect`, TODO or unimplemented path.
7. **Dependency posture — accepted.** Exact Axum and negotiation releases are
   private implementation dependencies. The 17-package normal external cone
   has no Tokio, Hyper, listener, socket, TLS or native component. Scoped
   transitive unsafe in `matchit` and `sync_wrapper` is documented; SDK code
   adds none.
8. **Host/portable split — accepted.** The adapter is host-only and
   unpublished. Existing portable package selectors still pass without
   claiming that a server router supports WASM, Android or iOS.

## Residual limitations

- Query options, DID URL dereferencing, OpenAPI and publication remain under
  parent issue #10.
- The host must supply timeout, concurrency, authentication, rate limiting,
  TLS, telemetry and shutdown policy before internet exposure.
- `application/json` and missing-`Accept` default behavior are explicit SDK
  compatibility policy around the draft binding, not certification claims.
- `headers-accept` is small and does not declare an MSRV; the exact pin,
  Rust 1.98.1 integration and advisory monitoring remain required.
- Local Nix cannot execute x86_64-Linux; hosted `fast` is merge authority.

## Review decision

The implementation is cohesive, bounded and reversible. No unresolved
correctness, architecture, security, privacy, protocol, compatibility or
supply-chain blocker remains for the issue-linked hosted review.
