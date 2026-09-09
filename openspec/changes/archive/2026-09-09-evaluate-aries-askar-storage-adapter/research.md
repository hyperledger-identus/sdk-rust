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
remain private. The proof uses only public store provisioning, transaction,
fetch, insert, replace, remove, commit/rollback and close behavior.

The adapter passes all 16 shared exact-record operations against encrypted
in-memory SQLite, including insert-only conflict, replacement revision
invalidation, stale-write/stale-delete preservation, scope isolation and exact
deletion. It owns an eight-byte revision envelope and validates conditional
mutations inside Askar transactions. Three fixture tests additionally prove
collision-free scope/key names, malformed-row integrity failure, input bounds
and redacted SDK errors.

The direct and resolved host normal/build dependency cone has 192 unique
rendered package/version lines; the target-complete lock has 256 packages. The
top-level package has eight direct dependencies and unconditionally selects
`askar-crypto 0.3.7` with every key family even for the storage-only fixture.
SQLite brings SQLx, Tokio and `libsqlite3-sys` with a C build script/native
`sqlite3` link.

Host behavior passes on Rust 1.98.1. The exact graph compiles for
`aarch64-apple-ios` with the Xcode clang and for `aarch64-linux-android` with
NDK 27/API 21 clang and llvm-ar. `wasm32-unknown-unknown` fails in
`getrandom 0.2.17` before SQLite compilation. Compile receipts do not establish
mobile runtime or support.

Public and wire compatibility is unchanged because the candidate stays behind
an SDK-owned facade boundary in a nested fixture. Root Cargo manifests and
locks contain no Askar package. Rollback removes only research artifacts. Unrun
checks include persistent files, process concurrency, crash recovery,
migration, runtime mobile behavior, performance and downstream integration.
The fixture revision counter is unique only within one process lifetime and its
generic conformance value is a Rust `String`; restart-safe revision allocation,
transaction cancellation/drop behavior and durable secret zeroization remain
unproved production responsibilities.

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
above. The enabled top-level Askar modules contain no authored unsafe block;
enabled `askar-storage` source contains three. `libsqlite3-sys` compiles and
links native SQLite C. Dependency-owned unsafe/native code remains material
audit scope, not a vulnerability claim.

The supply-chain evidence is deliberately feature-aware. `cargo deny` passes
advisories, bans, licenses and sources for the selected
feature graph with duplicate/unmatched-policy warnings. `cargo audit --deny
warnings` reports RUSTSEC-2023-0071 for `rsa 0.9.10` present in the lock;
`cargo tree --target all -i rsa` returns no path, proving it is not reachable
under selected features. The raw audit gate therefore does not pass, and a
future production decision must make an explicit feature-aware policy rather
than suppress the finding silently.

Version 0.4.6 was published on 2025-10-31. Upstream 0.5.0 was tagged on
2025-12-11 and main remained active at `48a49592` through 2026-06-25. The newer
source still unconditionally couples the top-level package to broad KMS crypto,
and its constituent 0.5-era crates are not published. Maintenance is credible;
publication and component-boundary posture are not sufficient for adoption.

## Candidate decisions

| Candidate | Initial disposition | Evidence needed |
| --- | --- | --- |
| `aries-askar 0.4.6` in generic/core crates | `not-adopt` | Architecture already prohibits backend/KMS types in generic crates. |
| Exact Askar SQLite leaf adapter | `not-adopt` | Behavior passes, but 192 host lines, unsliceable KMS crypto, native-only SQLite and SDK-owned revision/CAS work make production reuse disproportionate. |
| PostgreSQL, FFI, logger and migration features | `not-adopt` for this spike | Named consumer and separate backend/FFI/recovery decisions. |
| Unpublished 0.5.0 source | `oracle` | Maintenance/reference evidence only until an immutable crate and migration/API contract are published. |

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
reconsideration trigger is a published release that feature-slices encrypted
storage from unrelated KMS/FFI, exposes atomic revision/CAS semantics, passes a
feature-aware advisory and native-target policy, and proves migration/recovery
payoff for a named consumer.

## Open questions and blockers

There is no unresolved research blocker. The technical proof is positive, but
the production decision is negative for the assessed release. Any consumer may
use the fixture as a reference outside the SDK; a future SDK adapter requires a
new issue after the reconsideration trigger rather than reopening this result.

## Evidence commands

```text
cargo +1.98.1 generate-lockfile
cargo +1.98.1 test --locked
cargo +1.98.1 clippy --locked --all-targets -- -D warnings
cargo +1.98.1 tree --locked --edges normal,build
cargo deny --manifest-path <fixture>/Cargo.toml --config deny.toml check
cargo audit --file <fixture>/Cargo.lock --deny warnings
nix develop .#bindings --command env CC_aarch64_apple_ios=<xcode-clang> cargo check --locked --target aarch64-apple-ios
nix develop .#bindings --command env CC_aarch64_linux_android=<ndk-clang> AR_aarch64_linux_android=<ndk-llvm-ar> cargo check --locked --target aarch64-linux-android
nix develop .#wasm --command cargo check --locked --target wasm32-unknown-unknown
cargo tree --locked --target all -i rsa
```

The audit command intentionally reports RUSTSEC-2023-0071 for an unreachable
lock entry, and the WASM command intentionally fails at `getrandom 0.2.17`.
Those negative commands are evidence, not passing gates.
