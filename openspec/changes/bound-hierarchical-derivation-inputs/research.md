# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation at sdk-rust revision
`6f247815b7a1e6248826c2d8b4c9b9365927d6e4` is contained in
`crates/crypto/src/derivation/{path,hdkey,edhdkey,cardano_v2}.rs` and the crypto
tests. `DerivationPath::from_path` calls `split('/').collect()` before checking
the root or any axis. It therefore allocates in proportion to untrusted input
before establishing a resource ceiling. The resulting owned vector is then
parsed in a second pass.

`HDKey::init_from_seed` already accepts only 16 through 64 bytes. Its child
depth uses checked `u32` arithmetic, but accepts depths above the one-byte
BIP-32 serialized domain. `EdHDKey::init_from_seed` HMACs an arbitrary borrowed
seed, and `derive_child` uses unchecked `self.depth + 1`; that can panic in a
debug build and wrap in a release build. Both string consumers inherit the
unbounded parser. Cardano V2 private and public consumers take a typed
`DerivationPath`, so a caller can build an arbitrarily long path through the
infallible append API and trigger one cryptographic operation per axis.

Repository-wide consumer search found no sdk-rust, Apollo or NeoPRISM caller
of the Rust programmatic append API. Existing paths are short fixed vectors.
Apollo revision `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` independently
uses an eager unbounded string split and unchecked signed depth addition;
Apollo's Ed25519-BIP32 seed constructor instead requires exactly 64 bytes
because it is a different algorithm. NeoPRISM revision
`d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` has no separate hierarchical
derivation implementation or competing budget.

## Normative sources

- [BIP-32](https://github.com/bitcoin/bips/blob/c0644a054fd1568ecbfc9c2b656ad5200b16ff74/bip-0032.mediawiki)
  defines 2^31 normal and 2^31 hardened child indices, master seeds of 128 to
  512 bits, and a serialized extended-key depth of one byte. The pinned file
  revision is `c0644a054fd1568ecbfc9c2b656ad5200b16ff74` (2026-03-05) and
  states a BSD-2-Clause license for the proposal text.
- [SLIP-0010](https://github.com/satoshilabs/slips/blob/a00312491693714d9bc1b6e4cb5b2e356e6c511e/slip-0010.md)
  defines a 128-to-512-bit seed and hardened-only Ed25519 child derivation. The
  pinned Standard/Final revision is
  `a00312491693714d9bc1b6e4cb5b2e356e6c511e` (2025-04-15); the source
  repository declares CC-BY-SA-4.0. No source text or implementation is copied.
- `SDK-SEC-003` requires explicit resource ceilings at materially changed
  untrusted-input boundaries; `SDK-COMPAT-001` requires newly rejected input
  classes to be recorded; `SDK-LIM-007` keeps the inherited audit open.

The protocol/draft currency is stable: BIP-32 is deployed informational
guidance and SLIP-0010 is marked Standard/Final. Neither standard specifies a
textual path grammar or byte ceiling, so those are explicit SDK resource
policy. The standards do supply the interoperable 255 maximum non-master depth
and 16–64-byte seed envelope.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| 4,096 UTF-8 path bytes | `adopt` | Matches the crate's generic crypto-text and COSE budgets and bounds root/split/number parsing while leaving far more room than interoperable 255-axis decimal paths require. | A standards-backed consumer proves a larger textual representation and an end-to-end outer budget. |
| 255 axes and maximum derived depth 255 | `adopt` | Matches BIP-32's one-byte serialized depth and puts one deterministic ceiling on parser and cryptographic work. | A named non-BIP-32 tree needs a separate typed path/depth model. |
| 16–64-byte BIP-32 and SLIP-0010 seed constants | `adopt` | Directly matches both normative master-generation ranges and avoids hashing attacker-sized EdHDKey input. | A separate algorithm specifies a different seed domain and receives a distinct type/API. |
| Change programmatic `derive` to return `Result` | `defer` | It is a public signature break and crypto consumers can reject an oversized typed path before work. | A planned API-major change replaces panic/infallible constructors consistently. |
| Make `DerivationPath` globally impossible above 255 axes | `defer` | Private fields help, but the current infallible append method cannot enforce failure without a breaking return type or panic. | A fallible builder/path constructor is approved as a public compatibility change. |
| 32-axis wallet-policy limit | `not-adopt` | Wallet/account policies are product- and chain-specific, not a generic BIP-32 primitive constraint. | A separate wallet profile type owns a documented policy. |
| Caller-configurable limits | `not-adopt` | Policy-dependent validity complicates cross-language conformance and permits unbounded default behavior. | A distinct streaming/batch API has a named large-work consumer. |
| Replace derivation libraries | `not-adopt` | The gap is at SDK input/work boundaries; replacing `k256` or `ed25519-bip32` does not define those boundaries. | A separate ADR finds a correctness, licensing or maintenance blocker in a dependency. |

## Compatibility and dependency evidence

The constants and path introspection methods are additive. Rejection of
oversized strings, over-depth typed paths, out-of-range `EdHDKey` seeds and
depth-255 children is a public behavior change directed by issue #199 during
unpublished active development. Existing valid vectors and all in-range
behavior retain the same public/wire bytes. Errors remain the SDK-owned,
redacted `Error::DerivationFailed`; no dependency type crosses the facade.

No Cargo manifest, lockfile, feature, target, MSRV, FFI, native-code or build
script surface changes. The direct derivation edges remain `k256` 0.13.4,
`hmac` 0.12.1, `sha2` 0.10.9, `zeroize` 1.9.0 and private
`ed25519-bip32` 0.4.3. The locked resolved dependency cone remains unchanged,
including `cryptoxide` 0.6.5 behind Cardano V2. Rust 1.98 remains the etalon.

License and provenance remain Apache-2.0 in Hyperledger Identus sdk-rust; the
normative sources are cited, not copied. Apollo and NeoPRISM are read-only
compatibility evidence. Supply-chain posture is unchanged: no new package,
network-at-build step, native library or authored unsafe is introduced, and
the existing lockfile, cargo-deny and audit gates remain authoritative.

## Security, privacy and maintenance evidence

The text byte check occurs before root comparison, splitting, syntax parsing
or allocation proportional to axis count. A streaming `split('/')` loop checks
the 256th axis before parsing it. An in-budget parser allocates at most 255
axes. Each key consumer verifies that existing depth plus requested work fits
the 255-depth domain before performing HMAC or curve operations. Typed Cardano
paths are checked once before the first child operation.

Seed length is checked before HMAC. Length and depth failures do not include
seed, path, key or chain-code contents. Existing `Zeroizing`, `Zeroize` and
redacted formatting behavior remains intact. Authored unsafe stays prohibited.

Maintenance cost is three public constants, one streaming parser, small
preflight helpers and boundary tests. Release, security and target support do
not otherwise change. The source `&str`, owned source `String`, or typed path
may already have been allocated by the caller, so outer transport/allocation
budgets remain necessary.

## Rejected or deferred candidates

This change does not add xprv/xpub serialization, Base58Check, fingerprints,
RIPEMD, public BIP-32 derivation, account conventions or wallet policy. It does
not replace the Cardano V2 implementation dependency. Those are separate
feature/dependency decisions and would obscure this resource fix.

The programmatic `DerivationPath::derive` remains infallible and caller-
budgeted for source compatibility; all current cryptographic consumers enforce
the work ceiling. A future fallible-builder redesign can remove the possibility
of holding an over-limit path without changing the safe consumer outcome.
Rollback is atomic across checks, tests, spec and `SDK-LIM-007` evidence.

## Open questions and blockers

No blocker remains. #168 stays open for other inherited input surfaces, #9
stays open for Apollo parity, and a fallible path-builder redesign remains a
separate compatibility question.

## Evidence commands

- `git rev-parse HEAD` pinned sdk-rust to `6f247815`; equivalent read-only
  commands pinned Apollo to `ccee22b` and NeoPRISM to `d6ad1ec`.
- `rg -n 'DerivationPath|derive_child|deriveChild|init_from_seed|initFromSeed|depth'`
  across all three repositories located the current implementation and every
  consumer; no Rust programmatic append consumer was found.
- `cargo tree --locked -p identus-crypto --all-features --prefix none` recorded
  the direct and resolved dependency cone and exact versions.
- GitHub primary-source file and commit pages for BIP-32 and SLIP-0010 were
  retrieved on 2026-09-08; repository license APIs and source metadata were
  inspected for provenance.
- Focused crypto tests, isolated feature tests, complete Nix validation and an
  exact diff review are unrun at research readiness because implementation has
  not started. They are mandatory before the PR.
- Hosted Linux CI, DCO, policy and mergeability checks are unrun until an
  issue-linked pull request exists and remain mandatory before merge.
