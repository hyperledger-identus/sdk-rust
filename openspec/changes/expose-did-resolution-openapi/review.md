# Exact-diff local review

Review status: completed
Review date: 2026-09-09
Implementation head: 978e17b
Specification parent: 094b074d625bca2cd469ae29b38652324f78e94b
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-implementation diff, issue #205,
ADR 0092, the delta contract, both feature graphs, the emitted OpenAPI shape
and every changed manifest, production source and test. It reconciled the
document with ADRs 0090/0091, the fixed Axum handler, the pinned W3C DID
Resolution GET binding, OpenAPI 3.1.0 and NeoPRISM's read-only donor shape.

## Findings

1. **Architecture and cohesion — accepted.** The API and Utoipa dependency are
   isolated to the host-only HTTP adapter. `identus-did` retains no OpenAPI or
   HTTP edge, and the document adds no server, middleware, chain or product
   behavior.
2. **Runtime and compatibility — accepted.** The feature is disabled by
   default. The router, handlers and wire projection are untouched. The only
   public addition is one feature-gated document function on an unpublished
   `0.0.0` crate.
3. **Protocol accuracy — accepted after correction.** The document contains
   only mount-relative `GET /{did}`; required DID path, optional `Accept`
   header, four common query options, bounded extension semantics, all three
   success representations, implemented status outcomes and `Vary: Accept`.
   Review corrected the path description from DID URL to DID and made the
   `Accept` parameter explicit.
4. **Schema honesty — accepted.** The DID document requires only `id` and stays
   extensible. The resolution envelope requires its three W3C members, permits
   a null document and leaves metadata open. It does not claim a method-specific
   vocabulary.
5. **Resource and privacy safety — accepted.** All advertised byte/member
   ceilings come from runtime constants. Document construction consumes no
   request or resolver data, performs no I/O and cannot disclose DID, secret or
   PII content.
6. **Dependency decision — accepted with recorded correction.** Exact Utoipa
   5.5.0 is optional and declares Rust 1.75. The published model fails to
   compile with all features off because it references a macro-gated internal
   alias. Defaults remain disabled and only `macros` is enabled as a compile
   prerequisite; SDK code invokes no Utoipa macros. This adds `utoipa` and
   `utoipa-gen`, while their remaining resolved dependencies already existed.
7. **Graph and unsafe posture — accepted.** Default adapter and DID Core graphs
   contain neither Utoipa package. Registry-source inspection found no unsafe,
   FFI, native link or network runtime in Utoipa. Workspace unsafe guards and
   strict focused Clippy pass.
8. **Test quality — accepted.** Deterministic serialized-structure coverage
   prevents route, method, parameter, status, media, response-header and limit
   drift while the original twenty router tests pass both without and with the
   feature.

## Residual limitations

- Unknown method-specific query options are described but cannot be enumerated
  in a fixed OpenAPI parameter list.
- Hosts own mount-prefix composition, document publication and any UI.
- The optional public type deliberately couples feature users to Utoipa 5.x
  while the SDK remains unpublished and experimental.
- Utoipa's macro package remains in the opt-in compile graph until the upstream
  no-default-features defect is fixed or another typed model wins review.
- Direct Nix, nextest and cargo-deny executables are unavailable in this shell;
  hosted fast and scheduled slow gates remain authoritative.

## Review decision

The slice is accurate, bounded, optional and independently reversible. No
unresolved correctness, architecture, protocol, dependency, security,
privacy or compatibility blocker remains for hosted review.
