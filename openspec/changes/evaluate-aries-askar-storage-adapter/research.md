# Aries Askar storage-adapter research

Research class: storage-ffi
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

The current implementation already owns generic exact-record and list-capable storage ports plus a
backend-neutral conformance suite. It does not implement encrypted persistence.
The first question is whether an external encrypted store can meet the SDK's
compare-and-swap, revision, namespace, error and async boundaries without
exporting its models or selecting a product database.

Aries Askar provides encrypted profiles and records over SQLite/PostgreSQL, KMS
cryptography, transactions, migrations and FFI. The public record API has
insert/fetch/replace/remove and transaction sessions, but returned entries do
not expose a revision. An adapter must therefore own revision representation
and atomic validation rather than pretending candidate rows satisfy the SDK
contract directly.

## Normative sources

- Published candidate: [`aries-askar 0.4.6`](https://crates.io/crates/aries-askar/0.4.6),
  tag `v0.4.6` at `e18075d0eacfb51fecf0b38c1684ac7af01591fa`,
  crates.io archive SHA-256
  `17f26462dffcad927db37fc36ef2c49741b222f81aa9f624935c1989d69f72b7`.
- License: MIT OR Apache-2.0; declared MSRV Rust 1.81.
- Upstream maintenance comparison: `v0.5.0` at
  `43d6f464aa992e9568549f06a52356317c1514ca` was released on 2025-12-11,
  and main was active on the retrieval date. No `aries-askar 0.5.0` crate is
  available from crates.io, so it is not an executable dependency candidate.
- The 0.4.6 top-level crate unconditionally depends on `askar-crypto` with
  `all_keys`, `any_key`, Argon2, crypto-box and std features. Disabling defaults
  removes PostgreSQL, FFI, logger and migration but cannot isolate storage from
  this KMS cryptography surface.

## Compatibility and dependency evidence

The consumer shape is an SDK-owned `SecretStore` implementation tested through
`identus-wallet-conformance`. Candidate `Store`/`Session`/`Entry`/`Error` values
remain private. The proof will use only public store provisioning, transaction,
fetch, insert, replace, remove, commit/rollback and close behavior.

The final record will report the direct and resolved dependency cone. Public
and wire compatibility remains unchanged because the candidate stays behind an
SDK-owned facade boundary in a nested fixture. Rollback removes only research
artifacts. Named target checks and all unrun checks are listed with exact
commands rather than inferred.

Required mismatches to adjudicate:

- Askar exposes no row revision, so the adapter must store a revision with the
  value and check it under a transaction lock.
- Askar has categories and names rather than separate typed SDK scopes and
  keys; the adapter must encode a collision-free private name.
- Askar errors can contain backend messages and causes; they must be lowered to
  closed redacted SDK errors.
- Askar futures and native SQLite are runtime/backend choices; they cannot
  redefine the executor-neutral port or portable core support.
- Exact deletion of a missing row and transaction/cancellation behavior need
  explicit evidence rather than inference from method names.

## Security, privacy and maintenance evidence

The store encrypts category, name and value data using profile keys, but that
does not establish OS key protection, backup, recovery, side-channel safety or
custody. A fixed ephemeral test key proves behavior only. Caller values and
candidate diagnostics must not appear in errors or Debug output.

The license and provenance are pinned to the immutable tag and crate checksum
above. The published source search reaches substantial authored and dependency-owned
unsafe/native code, including SQLite and cryptographic implementations. This is
expected audit scope, not a vulnerability claim. The final report must record
the locked normal/build cone, licenses, advisories, duplicate versions, build
scripts, native links and target results with exact commands.

## Candidate decisions

| Candidate | Initial disposition | Evidence needed |
| --- | --- | --- |
| `aries-askar 0.4.6` in generic/core crates | `not-adopt` | Architecture already prohibits backend/KMS types in generic crates. |
| Exact Askar SQLite leaf adapter | `spike` | Shared exact-store conformance, atomic revision proof, redaction and cone/target evidence. |
| PostgreSQL, FFI, logger and migration features | `not-adopt` for this spike | Named consumer and separate backend/FFI/recovery decisions. |
| Unpublished 0.5.0 source | `oracle` | A published immutable crate and migration/API evidence. |

## Additional policy sources

- SDK wallet storage port and conformance specifications in this repository.
- SQLite transaction/locking behavior as implemented by the exact candidate
  graph; the spike makes no broader database standard claim.
- OWF Askar 0.4.6 public API, package manifests and storage design documentation
  at the immutable source revision above.

No external framework is normative for the SDK contract. Passing candidate
behavior is implementation evidence; the Identus specification remains the
authority. Protocol or draft currency is not applicable to this storage proof.

## Rejected or deferred candidates

Production adoption, KMS key storage, key generation, list ports, pagination,
PostgreSQL, migrations, FFI, logging, hardware keys, file persistence,
multi-process behavior, crash recovery, performance claims and downstream
integration are deferred. Each requires a separate bounded decision. The
reconsideration trigger for 0.5.0 is a published immutable crate with migration
and API evidence.

## Open questions and blockers

There is no research-readiness blocker. The fixture must determine whether the
public transaction API provides sufficient atomicity for SDK-owned revisions
and whether the minimized published graph passes current target and supply-chain
gates. A negative answer is a valid final result.

## Evidence commands

```text
cargo +1.98.1 generate-lockfile
cargo +1.98.1 test --locked
cargo +1.98.1 clippy --locked --all-targets -- -D warnings
cargo +1.98.1 tree --locked --edges normal,build
cargo deny --manifest-path <fixture>/Cargo.toml --config deny.toml check
cargo audit --file <fixture>/Cargo.lock --deny warnings
nix develop .#bindings --command cargo check --locked --target aarch64-apple-ios
nix develop .#bindings --command cargo check --locked --target aarch64-linux-android
```

WASM is expected to be incompatible with SQLite and will be recorded as a
designed backend boundary, not silently omitted.
