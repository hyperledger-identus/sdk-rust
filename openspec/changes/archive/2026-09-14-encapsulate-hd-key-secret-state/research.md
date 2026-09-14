# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-14
Source retrieval date: 2026-09-14
Research blockers: none

## Problem and existing implementation

The current implementation at
`develop@21cdbde6b9c5650d073a4a61f443640363585b38` has `HDKey` and
`EdHDKey` each derive `Zeroize + ZeroizeOnDrop` and redact `Debug`, but each
still exposes its private-key and chain-code `[u8; 32]` as public fields. The
published-vector integration tests consume those fields directly. The
unpublished candidate API rendering contains the fields in every re-exported
path.

No repository consumer is needed to establish the defect: Rust array field
access copies a `Copy` value into caller ownership. Issue #69 and ADR 0022
explicitly deferred opacity; issue #269 supplies the missing pre-publication
decision. No donor source or fixture is added.

## Normative sources

- BIP-0032 repository text at the Bitcoin BIPs default branch, retrieved
  2026-09-14, remains the secp256k1 derivation and vector source:
  https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
- SLIP-0010 repository text and vectors at the SatoshiLabs SLIPs default
  branch, retrieved 2026-09-14, remain the Ed25519 derivation source:
  https://github.com/satoshilabs/slips/blob/master/slip-0010.md
- `zeroize` 1.9.0, crates.io checksum
  `e13c156562582aa81c60cb29407084cdb54c4164760106ab78e6c5b0858cf64e`,
  documents that `Zeroizing` calls `Zeroize` on drop, while its exact source
  also shows a derived `Debug` implementation. License is Apache-2.0 OR MIT,
  repository https://github.com/RustCrypto/utils.
- The Cargo SemVer compatibility guide retrieved 2026-09-14 classifies
  removing public items and closing an all-public struct as breaking and notes
  that `0.y.z` uses the left-most non-zero component as the compatibility
  boundary: https://doc.rust-lang.org/cargo/reference/semver.html

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| SDK-owned fixed secret exposure value | repository-local at the issue base | `retain-local` | Narrow semantics, redacted formatting and no extra dependency or generic custody claim. | Several SDK components require the same exact export contract and can prove a cohesive reusable abstraction. |
| `zeroize::Zeroizing<[u8; 32]>` as public return | `zeroize` 1.9.0 | `not-adopt` | It owns/erases the copy but derives byte-revealing `Debug` and exposes an implementation type. | Upstream offers a redacted wrapper appropriate for public secret APIs. |
| Borrow-only field accessor | Rust 1.98.1 | `not-adopt` | Avoids an SDK-created copy but makes raw access generic and gives no owned export lifecycle for consumers. | A concrete consumer proves callback/borrow-only integration is sufficient. |
| `secrecy` | not added | `not-adopt` | Adds a new dependency and broader generic secret API for a fixed 32-byte boundary already covered by `zeroize`. | A separate cross-crate secret-container issue demonstrates reduced total surface and target compatibility. |
| Preserve/deprecate public fields | current API | `not-adopt` | Rust cannot make field copying explicit or prevent it during deprecation. | Never for a public release; only a downstream compatibility facade may own an isolated adapter. |

## Compatibility and dependency evidence

Public and wire compatibility are deliberately separated. This intentionally
breaks Rust source compatibility with the unpublished
`0.1.0-rc.1` API rendering. Canonical packages are still version `0.0.0` and
`publish = false`; no registry artifact, tag, FFI ABI, serialized form, or
downstream adoption exists. The candidate baseline is therefore replaced in
place before release. `cargo-semver-checks` remains evidence against protected
base `8110277c24714206436ae4a3fe678bdc58a84736`, while the ADR records why field
removal is intentional.

The direct and resolved dependency cone is unchanged. `zeroize` 1.9.0 already
supports the workspace MSRV and pure-Rust native, WASM, iOS and Android target
families. Feature activation is unchanged: the new type exists only with the
existing `derivation` feature. The SDK-owned facade exposes no donor or
backend type. Public derivation metadata, algorithm results and error codes are
unchanged. Rollback is a source revert before publication.

The named exposure methods replace reads, not construction. There is no public
`from_parts` or import surface for private-key, chain-code and metadata state.
A caller retaining the original seed and supported path can reconstruct with
the existing derivation API; callers retaining only raw extended state cannot
rehydrate it in this candidate. That pre-release breaking limitation is
accepted here and any raw-state import requires separate threat analysis and
API authority.

## Security, privacy and maintenance evidence

The threat is accidental copying, logging or serialization of secret arrays by
ordinary field access. Private fields remove that ambient capability. The
explicit exposure owner has no `Clone`, `Copy`, `Display`, Serde, `Deref`,
`AsRef`, FFI annotation or data-bearing `Debug`; its named borrowed view is
tied to an owner that zeroizes on drop.

No new unsafe or native code is introduced. The existing direct dependency
already carries reviewed license, provenance, MSRV, target and supply-chain
evidence from ADR 0022. The implementation remains small and SDK-maintained.
Its maintenance posture is compile-fail API tests, runtime redaction/zeroize
tests, published vectors, public-API rendering and candidate checks. Protocol
and draft currency are unchanged because no derivation behavior changes.

The claim remains best effort: it excludes compiler copies, registers,
allocator state, swap, crash dumps, hostile hardware and caller-created copies.
It is not custody, memory locking or key isolation.

## Rejected or deferred candidates

`secrecy`, direct `Zeroizing` returns, public borrowed fields, deprecated field
compatibility, key handles, custody, HSMs, secure enclaves, serialization and
FFI are rejected or deferred for the reasons above. A future general secret
container requires its own evidence; this issue does not establish one.

## Open questions and blockers

No implementation blocker remains. The known source break is intentional,
pre-release, reversible, and directed by issue #269. Raw extended-state
rehydration remains unsupported. Publication, an import API, and consumer
migration remain separate protected work.

## Evidence commands

- `cargo metadata --no-deps --format-version 1` confirmed `zeroize ^1.9` with
  `alloc,derive` and no new feature requirement.
- Local inspection of the checksum-verified `zeroize-1.9.0` source confirmed
  `Zeroizing` ownership/drop semantics and its derived `Debug` surface.
- `rg` over `crates/crypto` and the committed API baseline identified the four
  public fields and every repository-local test use.
- `scripts/factory doctor` passed on the exact issue base.
- Unrun before implementation: focused crypto tests, feature/target checks,
  candidate generation, full Nix checks and hosted CI. They remain delivery
  evidence and are not represented as research results.

## Reconsideration triggers

- A released or separately authorized downstream facade requires a migration
  adapter.
- `zeroize` changes its guarantee, license, target or MSRV posture.
- Multiple crates prove that a shared secret-container abstraction lowers
  coupling without widening formatting, serialization or FFI exposure.
- Publication is authorized and requires a version/baseline decision.
