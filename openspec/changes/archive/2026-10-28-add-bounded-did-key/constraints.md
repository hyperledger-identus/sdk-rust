# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/557
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 require chain-neutral inward dependencies,
cohesive ownership, and isolated effects. SDK-SEC-001 and SDK-SEC-003 require
safe Rust and finite untrusted work. SDK-COMPAT-001 through SDK-COMPAT-005
govern stable, MSRV, target, and toolchain evidence. ADR 0187 authorizes the
separate effect-free method crate; ADR 0188 fixes its first profile.

## Introduced or changed constraints

- `identus-did-key` depends inward only on generic SDK DID/crypto/core and the
  narrow existing base58 codec; unrelated crates cannot depend on it by default.
- Only canonical base58-btc `z` identifiers and the four named public-key
  multicodecs are accepted.
- Creation accepts typed caller-owned public keys; parsing revalidates exact
  length and curve points through `identus-crypto`.
- The method-specific text is at most 128 bytes and decoded material at most
  67 bytes; checks happen before proportional decoding or projection.
- Public errors contain stable categories and optional safe limit/curve kinds,
  never identifier text or key bytes.
- Resolution is deterministic and effect-free. The crate has no private keys,
  entropy, signing, persistence, network, executor, clock, or global registry.
- Production and test dependency graphs exclude all differential oracles.

## Introduced or changed limitations

The supported draft revision and strict profile are part of unreleased
compatibility. Base64url, additional curves, implicit Ed25519-to-X25519
derivation, update/deactivate, bindings, and downstream adoption remain absent.
The crate is source-only and not a release-train promise.

## Consumer and product impact

Rust adopters gain an opt-in deterministic method over SDK-owned types. No
existing crate changes behavior and no language SDK or product is migrated.
Adapters may later map their DTOs to this Rust contract under separate issues.

## Activation and rollback

Activation requires exact #557 preflight, ADR 0188, provenance, all quality
evidence, immutable ledger/catalog entries, exact-diff CLI review, green
protected CI, and an issue-linked PR. Before publication, rollback removes the
additive crate and its unreleased evidence without migrating runtime state.

## Evidence

Evidence authorities are issue #557, ADRs 0187 and 0188, the pinned v0.9
source, attributed oracle fixtures, OpenSpec/factory receipts, quality
declarations, target/dependency gates, exact-diff review, and protected CI.
