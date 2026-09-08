# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation in the SDK pins `ed25519-bip32 0.4.3` privately for
Cardano/IOG V2 behavior at upstream signed-tag revision
`6539dc9f792174fa5c2290c9e0a23710a1e1ecef`.
The SDK facade owns redacted, zeroizing bytes, but every internal conversion
creates a short-lived upstream `XPrv`. That type implements `Debug` and
`Display` as full hexadecimal secret output and calls a local unsafe
`write_bytes` helper from `Drop`.

The named consumer evidence is Apollo parity in `identus-crypto`, followed by
NeoPRISM reduction; neither consumer needs a public upstream type. The upstream
manifest requests `cryptoxide = "0.6"`. With 0.6.5 this activates
the full default feature family even though source imports require only
constant-time comparison, Ed25519/Curve25519, SHA-512 and HMAC.

## Normative sources

- Cardano/IOG [Ed25519-BIP32 scheme V2 paper](https://input-output-hk.github.io/adrestia/static/Ed25519_BIP.pdf)
  linked by ADR 0078; protocol/draft currency remains scheme V2 as used by
  Apollo and Cardano rather than standard secp256k1 BIP-32 or SLIP-0010.
- `typed-io/rust-ed25519-bip32` signed tag `ed25519-bip32-v0.4.3` at
  `6539dc9f792174fa5c2290c9e0a23710a1e1ecef`, with license and provenance
  recorded as MIT OR Apache-2.0 from the signed source tag.
- `cryptoxide 0.6.5`, whose `ed25519` feature implies `curve25519`; `sha2` and
  `hmac` are independently required by direct imports.
- RustCrypto `zeroize 1.8.2`, Apache-2.0 OR MIT, Rust 1.60, pure Rust and
  `no_std` with default features disabled and an MSRV of Rust 1.60.

## Candidate decisions

| Candidate | Exact version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Upstream `ed25519-bip32` patch | `6539dc9` plus focused patch | `adopt` | Removes the three residual risks at their source without changing V2 mechanics or SDK API. | Upstream declines, incompatible review direction, or vectors/targets fail. |
| `zeroize` | `=1.8.2`, defaults disabled | `adopt` | Maintained volatile wipe, no allocation or derive dependency, preserves upstream Rust 1.81. | An upstream-compatible newer line preserves Rust 1.81 without resolver ambiguity. |
| Local manual volatile wipe | current source | `not-adopt` | Retains bespoke unsafe code despite a maintained exact-purpose crate. | Never while `zeroize` satisfies the target and compiler matrix. |
| Fresh SDK implementation | not applicable | `not-adopt` | Reimplements subtle cryptographic mechanics and increases assurance burden. | A separately audited implementation with lower total risk becomes available. |
| SDK-maintained fork | exact accepted upstream base | `conditional-adopt` | Creates maintenance and supply-chain ownership before upstream has considered the focused patch. | Upstream declines or remains inactive through the first publishable SDK release. |

## Compatibility and dependency evidence

The proposed formatting behavior is intentionally security-breaking for callers
that parsed `XPrv` diagnostic output, but the trait implementations remain and
the binary/key conversion API is unchanged. The SDK never exposes or consumes
those formatters. The upstream crate remains `no_std` and at its declared MSRV
of Rust 1.81. Public and wire compatibility are unchanged at the Identus facade.

`zeroize = "=1.8.2"` is exact because the 1.9 line requires Rust 1.85; an
unbounded caret requirement would make the declared upstream Rust 1.81 floor
resolver-dependent. Disabling zeroize defaults avoids `alloc`. The proposed
`cryptoxide` edge uses `default-features = false` with `ed25519`, `sha2`, and
`hmac`; `ed25519` implies the required `curve25519` module.

The direct and resolved dependency cone grows from two packages to three while
its compiled cryptoxide feature surface shrinks from every default algorithm to
the four named capability modules plus unconditional constant-time support. No
native source, build script, FFI, network, storage, chain policy or
serialization is introduced.

## Security, privacy and maintenance evidence

Redacting both safe formatting traits closes the direct diagnostic exfiltration
path without relying on downstream wrappers. `zeroize` replaces the crate's
only authored unsafe block with a maintained volatile primitive. This does not
prove physical-memory erasure or remove unsafe implementation details from
`cryptoxide`; it removes the bespoke unsafe boundary owned by this dependency.
Supply-chain evidence must be refreshed against the patched lockfile before the
contribution is opened.

The patch does not alter derivation, signing, verification, parsing or byte
conversion mechanics. Existing upstream vectors therefore remain the primary
regression oracle, supplemented by formatting tests, feature-tree inspection,
Rust 1.81, `no_std`, host and portable-target compilation.

## Rejected or deferred candidates

Removing `Display` entirely is semver-breaking at the trait surface and offers
no additional protection over stable redaction. Feature subtraction cannot be
performed by the SDK because Cargo feature unification is additive. Wrapping
the current dependency more deeply does not remove the transient upstream value
or broad compiled feature family. A fork is reserved for an evidenced upstream
failure, not used as the first response. Rollback is closing or reverting the
upstream patch while retaining SDK version 0.4.3 and its existing facade.

## Open questions and blockers

No blocker prevents preparing and validating the upstream patch. Upstream merge
and release timing are external outcomes; the SDK dependency pin will remain at
0.4.3 until a separate issue can reference an immutable released artifact.

## Evidence commands

- `gh repo view typed-io/rust-ed25519-bip32` confirmed the repository is active,
  read-only to the contributor identity, and uses `master`.
- `git log`, manifest/source inspection and GitHub issue/PR enumeration pinned
  the exact base and found no competing hardening contribution.
- `cargo info cryptoxide@0.6.5` recorded the complete feature graph.
- `cargo info zeroize@1.8.2` and `cargo info zeroize@1.9.0` established the
  compiler-floor reason for the exact 1.8.2 bound.
- Patched compile, vector, feature, target and audit evidence is recorded in
  `verification.md` after implementation.
- Unrun checks at this readiness point are the patched test suite, Rust 1.81,
  portable targets, audit, unsafe scan and exact-diff review; these are tasks,
  not inferred successes.
