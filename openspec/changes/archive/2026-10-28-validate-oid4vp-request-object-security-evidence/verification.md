# Verification receipt

Verification status: passed locally
Verification date: 2026-10-28
Base: develop@d33321e1dd6ae47cad934d34e6ba87b366f82190
Planning commit: d201379d9300d6852620d69d7104fd31f80a0239
Preimplementation receipt: 873838a14cbe0684434e26d06e450240040919a1
Reviewed implementation head: 7309ae46866c981c85443f3da738bfa67069e5d5
Pull request: pending

## Protocol and state-machine receipt

- Request Object security validation consumes cryptographically verified state
  and is the only route to typed DCQL and authorization-request successors.
- The security policy requires one exact discovery audience, an integral `iat`,
  bounded freshness and clock skew, and optionally enforces `exp`, `nbf`, and
  atomic accept-once `jti` replay state.
- Exact-boundary tests cover the expiration cutoff, future and stale issuance,
  not-yet-valid requests, unavailable replay storage, replay rejection, and
  redacted error contracts.
- A compile-fail doctest proves that raw `VerifiedRequestObject` state cannot
  bypass the security transition. Public evidence accessors retain only the
  documented nonsensitive algorithm, header, client, nonce, payload, audience,
  and temporal lineage.

## Test and E2E receipt

- The focused request-security suite passed eight tests, including 96
  generated boundary cases and a mutex-backed stateful accept-once replay
  double.
- `identus-oid4vp` all-target/all-feature tests passed, including ordinary and
  encrypted direct-post clean-room E2E paths.
- Complete workspace all-feature, no-default-feature, and strict-Clippy gates
  passed. OID4VP rustdoc passed, including the compile-fail routing gate.
- The request-security fuzz target passed its deterministic replay and a
  300-second Linux soak: 507,495 executions in 301 seconds with 638 MB peak
  RSS. The final normalized seed corpus is part of the isolated factory
  fixture.
- The request-security benchmark completed 21 samples of 20 iterations:
  ordinary validation was approximately 487,160 ns and the valid boundary
  fixture approximately 7,520,991 ns. These values are local baselines, not a
  support or performance guarantee.

## Target and factory receipt

- Locked compile evidence passed for `wasm32-unknown-unknown`,
  `aarch64-apple-ios`, and `aarch64-linux-android`; these are compile claims,
  not linker, packaging, runtime, or FFI support claims.
- `scripts/factory check` passed all 116 governance, traceability, OpenSpec,
  quality, vector, and fixture checks.
- Exact-head `nix flake check --print-build-logs` passed the complete native
  matrix, including Rust 1.98.1 primary lanes, Rust 1.89.0 compatibility
  evidence, strict Clippy, documentation, portable builds, and 1,085/1,085
  workspace nextest tests.
- The quality registry records request-security property, fuzz, benchmark, and
  differential evidence as satisfied. The unrelated existing crypto fuzz
  obligation remains required and pending under issue #525 because its natural
  evidence is stale; this change does not suppress or weaken it.

## Review receipt

- Claude CLI was unavailable because of its session limit, so the standing
  factory fallback used Codex CLI for a fresh exact-diff review.
- The first review identified three medium findings: permissive extra audience
  recipients, the expiration cutoff, and overstated property/replay evidence.
  Follow-up passes identified canonical-spec/doctest command gaps and unsafe
  rollback wording. Each finding was corrected and rechecked.
- The final Codex CLI review at reviewed implementation head reported no
  blocker, high, or medium findings.

## Exclusions

No verifier-key trust store, key authorization, durable replay implementation,
network transport, discovery retrieval, system clock adapter, consent/UI,
credential verification, production deployment, publication, certification,
or downstream repository mutation was implemented or inferred. Verifier-key
trust remains owned by issue #545.
