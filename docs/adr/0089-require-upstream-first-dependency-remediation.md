# ADR 0089: require upstream-first dependency remediation

- **Status:** Accepted
- **Date:** 2026-09-08
- **Decision authority:** sdk-rust issue #179 and ADR 0078
- **Upstream issue:** [typed-io/rust-ed25519-bip32#8](https://github.com/typed-io/rust-ed25519-bip32/issues/8)
- **Upstream contribution:** [typed-io/rust-ed25519-bip32#9](https://github.com/typed-io/rust-ed25519-bip32/pull/9)

## Context

ADR 0078 conditionally adopts `ed25519-bip32 0.4.3` behind an SDK-owned facade
because it is the narrow maintained implementation with exact Apollo/Cardano V2
semantics. Its private formatter, manual unsafe wipe and broad `cryptoxide`
defaults are focused upstream defects. Cargo cannot subtract features selected
by an upstream manifest, while a local fork would transfer maintenance and
supply-chain ownership to the SDK before upstream had a chance to accept a
small compatible correction.

The same choice recurs whenever the SDK conditionally adopts a useful crate
with a bounded, independently fixable defect. Agents need a deterministic path
that improves the ecosystem without turning upstream response time into an
unbounded delivery queue.

## Decision

1. When a conditionally adopted dependency has a focused defect that can be
   removed without changing required SDK semantics, prepare and validate the
   minimal upstream contribution before activating an SDK-maintained fork.
2. Preserve the dependency's relevant compiler, target, license, feature and
   conformance contract. Do not bundle unrelated cleanup into the contribution.
3. Keep dependency types behind the existing Identus facade and never point the
   SDK at an unpublished branch merely because a pull request exists.
4. Treat an open or merged upstream pull request as coordination evidence, not
   an effective remediation. A separate issue must validate and pin an immutable
   released artifact.
5. If upstream declines the change, provides an incompatible result or remains
   inactive through an already accepted release trigger, a focused fork may be
   proposed with an exact base, unchanged conformance evidence, maintenance
   owner, rollback and sunset condition back to upstream.

## Application to ed25519-bip32

The contribution based on signed tag revision `6539dc9`:

- preserves `Debug` and `Display` traits but renders `[REDACTED]`;
- replaces the crate's local unsafe wipe with default-disabled
  `zeroize = "=1.8.2"`, retaining the upstream Rust 1.81 floor;
- disables `cryptoxide` defaults and selects only `ed25519`, `hmac` and `sha2`;
- changes no derivation, signing, verification, parsing or raw key conversion;
- passes the upstream vectors on Rust 1.81 and 1.98.1, its Rust 1.81 no-std
  target, and the SDK's WASM, Android and iOS compile targets.

The SDK remains on 0.4.3. A release update will measure whether the bounded
zeroize 1.8 line causes a duplicate package beside SDK zeroize 1.9 and will
rerun the full ADR 0078 evidence matrix.

## Consequences

- Remediations can benefit every consumer instead of living only in an SDK fork.
- The SDK avoids hidden Git dependencies and premature fork maintenance.
- Upstream timing cannot block unrelated SDK work; the current private facade
  and explicit residual-risk record stay truthful until a release exists.
- A fork remains available at an objective trigger, but it is a reviewed
  exception with an exit path rather than the default response.

## Rollback

Supersede this ADR if upstream-first contribution repeatedly creates a greater
security or delivery cost than a bounded fork. Closing or reverting the current
upstream contribution leaves SDK version 0.4.3 and its existing facade
unchanged; no downstream data or API migration is required.
