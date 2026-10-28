# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The current implementation in `identus-jose` already owns bounded request,
public-recipient, compact-result and
redacted error types for P-256 `ECDH-ES` with `A128GCM`. `identus-oid4vp` composes
that port for Final `direct_post.jwt`. Neither crate provides an engine, and the
existing inert fixture is not cryptographic decryptability evidence.

Consumer evidence from SDK-TS, SDK-Swift and SDK-KMP is useful for capability
discovery only. It does not define Rust DTOs, errors, engine selection or
security policy.

## Normative sources

OpenID4VP 1.0 Final sections 8.3 and 8.3.1 plus RFCs 7516, 7517, 7518,
7519 and 8725 define the protocol and JWE profile. The primary specification is
https://openid.net/specs/openid-4-verifiable-presentations-1_0.html.
Candidate provenance is the immutable crates.io release archive and checksum:
`jose-rs 0.10.0`, `no-way-jose-* 0.1.0-rc.2`, `reallyme-jose 0.4.4`,
`jwt-simple 0.15.0`, `josekit 0.10.3`, `jose4rs 0.6.0`, and `jute 0.1.2`.
The pinned source revision for a crates.io package is its exact release checksum
in the generated lock. License metadata and source provenance were inspected in
the published archive. The protocol currency is OpenID4VP Final, not an older
draft.

## Candidate decisions

| Candidate | Exact release | Decision | Reason |
| --- | --- | --- | --- |
| `jose-rs` | 0.10.0, defaults off | `oracle`; defer production | Public-recipient round-trip and host/WASM/iOS/Android compilation pass. The graph directly and transitively requires `rsa 0.10.0-rc.19`, which fails the authoritative audit with RUSTSEC-2023-0071; the broad `kryptering/rustcrypto` feature cannot remove it. |
| `no-way-jose-*` | 0.1.0-rc.2 exact pins | `not-adopt` | Prerelease; public encryption requires `EcPrivateKey`; core contains unsafe UTF-8 conversion. |
| `reallyme-jose` | 0.4.4 | `not-adopt` | Its public-recipient API is promising, but declared Rust 1.99 exceeds the SDK's Rust 1.89 MSRV. |
| `jwt-simple` | 0.15.0 | `not-adopt` | Does not provide the required direct `ECDH-ES` profile. |
| `josekit` | 0.10.3 | `not-adopt` | OpenSSL/native coupling conflicts with the portable target policy. |
| `jose4rs` | 0.6.0 | `not-adopt` | AWS-LC/Boring native backends and unsafe code conflict with the portable target policy. |
| `jute` | 0.1.2 | `not-adopt` | Declares ECDH variants but returns unsupported-operation for ECDH key derivation/wrapping. |
| Local JWE composition | repository code | `not-adopt` | Maintaining ECDH KDF, ephemeral-key generation, protected headers and AEAD composition would duplicate security-critical standards logic. |

## Executed evidence

A disposable `jose-rs 0.10.0` program encrypted with a public-only P-256 JWK,
included `kid`, used exact `ECDH-ES`/`A128GCM`, decrypted with the corresponding
private JWK, and recovered the exact plaintext. Host execution passed. Compile
checks passed for `wasm32-unknown-unknown`, `aarch64-apple-ios`, and
`aarch64-linux-android`. The dependency graph resolved about 120 packages and
the authoritative Nix audit rejected `rsa 0.10.0-rc.19` under
RUSTSEC-2023-0071. No production manifest or lockfile was changed.

## Decision

Retain ADR 0181's SDK-owned port and defer a production engine. Preserve the
passing round-trip as an external oracle and make its manifest, source,
portable compile route and expected audit disposition reproducible without
placing the candidate in the workspace graph. Do not suppress the advisory,
fork a broad JOSE facade, or implement JOSE composition locally.

## Reconsideration triggers

- `jose-rs` makes RSA and unrelated algorithm families optional and its exact
  selected graph passes the repository audit;
- stable `no-way-jose` accepts a public recipient and removes or contains its
  unsafe conversions;
- `reallyme-jose` supports Rust 1.89 or the SDK independently raises its MSRV;
- another maintained stable engine passes every gate in ADR 0192.

## Compatibility and dependency evidence

The disposable manifest selects exact `jose-rs 0.10.0` with default features
disabled and a WASM-only entropy feature. Its resolved lock is bound to SHA-256
`2e893f1a3a046b55c26c9399e761d3ab812d65c9a2bf736862526b9cb47d4638`;
any graph drift fails before execution. The candidate has Rust 1.88 MSRV and
BSD-2-Clause license, but its direct and resolved dependency cone contains about
120 packages and unrelated algorithms. Supply-chain evidence includes the
generated lock checksum, `cargo tree`, source inspection, license/provenance,
unsafe/native inspection and the authoritative RustSec audit.

Public and wire compatibility are unchanged. The facade remains SDK-owned, no
third-party type crosses it, the workspace lock is unchanged, and rollback
removes only behavior-neutral evidence.

## Security, privacy and maintenance evidence

The decision is behavior-neutral and adds no dependency. SDK-owned DTOs and
errors remain stable and redacted. The quality debt remains visible under #566
because external evidence is not a shipping adapter. A later engine can be replaced
without reshaping OID4VP or JOSE APIs. Security posture is fail-closed; release
and maintenance posture require repeating exact qualification before adoption.
The exact candidate source forbids unsafe code, but its broad resolved graph and
RSA advisory fail the production gate. Native-code candidates are rejected.

## Rejected or deferred candidates

Every non-oracle candidate in the matrix is rejected or deferred for its exact
recorded blocker. No rejection is a permanent ecosystem judgment. Each can be
reconsidered only through its listed reconsideration trigger and complete
requalification.

## Open questions and blockers

Production adoption is blocked by the absence of a stable, narrow,
public-recipient engine whose exact graph passes all gates. This does not block
the engine-neutral SDK facade or external interoperability evidence. Browser
entropy execution and downstream bundler behavior remain unrun checks for a
future adopted adapter.

## Evidence commands

The durable reproduction script creates an isolated temporary Cargo package and
runs the exact round-trip and target checks. Advisory evidence is authoritative
only through the repository Nix audit toolchain. Generated manifests, locks,
targets and ciphertext remain outside the checkout.

Exact command classes are `cargo run --locked`, `cargo check --locked --target
<triple>`, `cargo tree --locked`, and the pinned Nix `cargo audit --deny
warnings`. Unrun checks are browser execution, CSP/bundler integration,
downstream product integration and any production-adapter benchmark. The oracle
route records a diagnostic source-bound round-trip timing but sets no product
performance threshold.
