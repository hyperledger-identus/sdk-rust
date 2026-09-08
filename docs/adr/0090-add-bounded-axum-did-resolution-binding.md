# ADR 0090: add a bounded Axum DID Resolution HTTP binding

- **Status:** Accepted for implementation
- **Date:** 2026-09-08
- **Issue:** [#201](https://github.com/hyperledger-identus/sdk-rust/issues/201)
- **Parent:** [#10](https://github.com/hyperledger-identus/sdk-rust/issues/10)
- **Decision authority:** B06 and the implemented transport-neutral resolver port

## Context

`identus-did` owns the chain-neutral W3C resolution values and resolver port.
It deliberately owns no HTTP framework. Without a reusable outer adapter,
each resolver host must reproduce negotiation, status mapping and response
projection. NeoPRISM contains an Apache-2.0 Axum binding worth extracting, but
its `HashSet`-based `Accept` handling ignores RFC quality/specificity, its query
options are ignored, and its caller-supplied route can panic during router
construction.

The SDK needs a small host adapter now, not a full resolver service framework.

## Decision

Create unpublished outer-boundary crate `identus-did-resolver-http`. Export a
fixed-route, state-closed Axum router constructor over
`Arc<dyn DidResolver>`, plus supported media type and resource-bound constants.
Consumers nest the router at their chosen external prefix.

Adopt exact `axum 0.8.9` with defaults disabled and `json` only. Adopt exact
`headers-accept 0.3.0` privately for media-range precedence and selection,
wrapped by SDK-owned aggregate 8 KiB/32-range checks and strict RFC 9110
quality/parameter preflight. Do not expose its `mediatype` values or errors.
Missing/wildcard negotiation defaults to `application/did`; supported document
representations are `application/did` and `application/json`; the full result
uses `application/did-resolution`.

Reject invalid paths/DIDs, malformed negotiation and every non-empty query as
bounded W3C errors before invoking the resolver. Map all nine standard error
kinds and deactivation to the pinned HTTP binding statuses. Project a document
only when its result metadata content type matches the negotiated type.
Failure and deactivated responses always contain the complete resolution
result. Every response varies on `Accept`.

Keep Tokio runtime, listener/protocol features, HTTP client, middleware,
timeouts, concurrency, authentication, authorization, rate limiting, OpenAPI,
query option decoding, dereferencing and registration outside this slice.

## Evidence

The normative baselines are W3C DID Resolution Candidate Recommendation Draft
source revision `71a50058090417f9947b9f13985fc8b561a4ad59` and RFC 9110.
The donor was inspected at NeoPRISM revision
`d4608fe3d662ebaf425c4a6380f6d241cffe74c3`; its source and tests match the
locally assessed snapshot hashes recorded in the OpenSpec research.

`axum 0.8.9` is MIT, declares Rust 1.80 and can omit server/runtime features.
`headers-accept 0.3.0` is MIT, uses `http` 1.x, `headers-core 0.3` and
`mediatype 0.21`, forbids unsafe, and correctly handles quoted commas,
specificity, order and wildcards. Its quality parser is intentionally
permissive for malformed values, so it is accepted only behind the strict SDK
preflight. The integrated normal cone adds 17 package names and no server
runtime, Hyper, socket, TLS or native code; Tokio and its macro are test-only.
Scoped unsafe is reachable in Axum's established `matchit` route-tree and
`sync_wrapper` pin/exclusive-access internals, while Axum, `headers-accept`,
`mediatype` and SDK code add none. Exact lockfile advisory, license and source
gates pass; full repository gates remain required before merge.

## Consequences

- DID Core remains runtime and framework neutral.
- Hosts can reuse one deterministic W3C-shaped adapter without inheriting a
  server runtime or product deployment policy.
- The SDK owns a small strict/bounded facade around upstream negotiation rather
  than reimplementing media-range precedence.
- Query options and OpenAPI remain explicit backlog, and silent partial
  compliance is avoided.
- The new crate is host-only, experimental and unpublished; portable/release/
  certification and downstream-adoption claims do not change.

## Alternatives rejected

### Copy the donor unchanged

Its unbounded/permissive negotiation, ignored options and dynamic route are
incompatible with SDK security and conformance rules.

### Put Axum in `identus-did`

That reverses dependency direction and couples every DID consumer to one host
transport framework.

### Implement the entire `Accept` algorithm locally

RFC precedence and quoted parameter mechanics are closed, reusable behavior.
The narrow maintained dependency plus an SDK policy preflight has lower risk
than a new full parser.

### Adopt `accept-header` or `negotiator`

`accept-header 0.2.3` has a stale `http` 0.2 cone, raw-value diagnostics and
incomplete semantics. `negotiator 0.1.0` is promising but immature and broader
than this three-representation consumer requires.

## Verification and rollback

Deterministic in-memory routing tests must cover default, wildcard, quality,
specificity, repeated and malformed headers; every standard status;
deactivation; invalid path/DID/query; exact resolver options; projection
mismatch; and redaction. Full format, factory, Rust 1.98, dependency/security,
Nix and hosted gates must pass. Reverting the focused PR removes the adapter
and its graph/inventory entries without data migration.
