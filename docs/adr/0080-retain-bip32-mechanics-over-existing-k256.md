# ADR 0080: retain narrow BIP-32 mechanics over existing k256

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#153](https://github.com/hyperledger-identus/sdk-rust/issues/153)
- **Research:** [OpenSpec research](../../openspec/changes/correct-bip32-scalar-validation/research.md)
- **Supersedes:** ADR 0061's approved `bip32 0.5.3` candidate disposition

## Context

The SDK's hardened-only `HDKey` has concrete correctness gaps: it reduces an
out-of-range child tweak instead of rejecting it, does not validate master
scalar or seed length, and can overflow depth. ADR 0061 selected exact
`bip32 0.5.3` as a likely replacement before its low-level semantics were
tested against the rare BIP-32 failure edges.

Focused source review found that the candidate's k256 backend parses `IL` as a
`NonZeroScalar`. It therefore rejects `IL = 0`, while BIP-32 permits zero when
the resulting child remains nonzero. The candidate also includes mandatory
Base58Check and RIPEMD dependencies used by extended-key serialization, which
the SDK does not expose. Its high-level constructor supports only 16, 32 and
64-byte seeds instead of every normative length from 16 through 64 bytes.

## Decision

Do not adopt `bip32 0.5.3` for `HDKey`. Correct the implementation by reusing
the exact scalar parsing and arithmetic already provided by `k256 0.13`:

1. Accept every seed length from 16 through 64 bytes and reject others.
2. Validate the master HMAC left half with `Scalar::from_repr` plus an explicit
   nonzero check.
3. Parse child `IL` with `Scalar::from_repr`, never modular reduction.
4. Permit zero `IL`, add it to the validated parent scalar, and reject a zero
   result or `IL >= n`.
5. Use checked depth arithmetic and the existing stable redacted error.
6. Preserve the hardened-only `HDKey` facade, Apollo vectors, raw-field
   compatibility and zeroizing ownership.
7. Keep xprv/xpub, public/non-hardened derivation, Base58Check, RIPEMD, network,
   account and wallet policy out of this change.

## Consequences

- Standards defects are corrected without adding dependencies or public types.
- The SDK retains a small amount of BIP-32 orchestration, but curve-sensitive
  scalar validation and arithmetic remain delegated to RustCrypto.
- Inputs outside the normative seed range that previously succeeded now return
  the existing fallible derivation error.
- A future crate adoption requires a new ADR and the durable ledger trigger:
  correct zero-tweak semantics, all 16..=64-byte seeds, narrow feature slicing,
  facade isolation, and complete SDK target/security evidence.

## Alternatives rejected

Adopting `bip32 0.5.3` plus a local zero-tweak workaround adds unused
serialization coupling without eliminating the critical adapter logic. Its
high-level API narrows valid seeds and broadens domain behavior. `coins-bip32`
and broad Bitcoin suites add more model, serde, runtime or native coupling.
Continuing modular reduction violates the normative algorithm, while manual
curve arithmetic duplicates a primitive already supplied by `k256`.

## Verification and rollback

Acceptance requires official vectors 1–4, Apollo parity, every seed-length
boundary, synthetic zero/order master and child cases, zero-tweak success,
zero-result and depth-overflow rejection, redacted diagnostics, zeroizing and
public-API evidence, Rust 1.85/1.98, supported portable targets, deny, audit,
complete Nix/factory gates and a distinct correctness/security review.

Rollback reverts issue #153's focused PR. No persisted representation or
downstream migration is involved.
