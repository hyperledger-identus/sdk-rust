# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-27
Source retrieval date: 2026-10-27
Research blockers: none

## Problem and existing implementation

The current implementation already owns verified JAR lineage, bounded request JSON, exact
`client_id`, response mode, HTTPS response URI, nonce/state, DCQL validation,
presentation cardinality, deterministic JSON/form encoding and redacted
errors. It does not retain `client_metadata`, accept `direct_post.jwt`, or
provide JWE. `identus-jose` deliberately exposes only bounded JWS Compact and
signature capabilities under ADR 0159.

The implementation gap is therefore a protocol-owned encrypted response flow
and a narrow cryptographic capability boundary, not a reason to import a JOSE
library's complete DTO, policy, error, or algorithm surface.

Consumer evidence from SDK-TS, SDK-Swift and SDK-KMP is useful for capability
discovery but is not normative for Rust DTOs or errors; no consumer repository
is mutated by this slice.

## Normative sources

OpenID for Verifiable Presentations 1.0 Final, published 2025-07-09 and
retrieved with current errata on 2026-10-27, is authoritative:
https://openid.net/specs/openid-4-verifiable-presentations-1_0.html.
Sections 5.1, 8.1, 8.3 and 8.3.1 require an unsigned encrypted JWT whose
plaintext contains the Authorization Response members. The Wallet selects a
public JWK from verifier metadata; the JWK must carry `alg`, the JWE `alg` must
match it, `kid` is copied when present, and `enc` comes from
`encrypted_response_enc_values_supported` with `A128GCM` as the permitted
default. The form body contains exactly one `response` parameter. RFC 7516,
7517, 7518, 7519 and 8725 govern compact JWE, keys, algorithms, JWT and safe
algorithm handling.

This supersedes older-draft assumptions that `direct_post.jwt` means signed
JARM. No draft behavior is silently preserved.

## Candidate decisions

All candidates were tested outside the workspace lock on Rust 1.98.1, with
their relevant default features disabled, against host,
`wasm32-unknown-unknown`, `aarch64-apple-ios`, and
`aarch64-linux-android`. WASM requires an explicit target-only
`getrandom 0.4/wasm_js` feature because both viable engines generate ephemeral
ECDH keys and AEAD nonces.

The pinned source revision for each crates.io candidate is its exact immutable
release checksum in the disposable Cargo lock. Exact version and feature
selection is recorded in each disposable manifest. License and provenance were
read from the published package metadata and source archive. The candidate
MSRV was compared with the repository's Rust 1.98.1 toolchain. `cargo tree`
measured the direct and resolved dependency cone rather than inferring it from
top-level manifests.

| Candidate | Exact release | Result | Evidence and reason |
| --- | --- | --- | --- |
| `jose-rs` | 0.10.0, defaults off | `oracle` | Compiles on all targets, Rust 1.88, BSD-2-Clause, `forbid(unsafe_code)`, bounded token/JWK APIs, public-JWK ECDH-ES and A128GCM. It adds about 80 packages absent from the workspace and pulls unrelated RSA, DSA, Ed448, ML-DSA/ML-KEM, CBC/PBES2 and prerelease crypto families through `kryptering`; this violates the narrow dependency goal. |
| `no-way-jose-core` + ECDH-ES + AES-GCM | 0.1.0-rc.2 exact pins | `not-adopt` | Compiles on all targets and adds about 35 packages, but all three packages are prerelease; core uses four unsafe UTF-8 conversions; and the ECDH encrypting key type requires `EcPrivateKey`, so a wallet cannot encrypt from the verifier public JWK without an upstream API correction. |
| `jwt-simple` | 0.15.0, pure Rust | `not-adopt` | Stable and portable, but supports ECDH-ES only with AES key wrap, not the Final example/required exact `ECDH-ES` public-key seam. Its JWT claims model also owns more policy than this protocol needs. |
| `josekit` | 0.10.3 | `not-adopt` | Mature JOSE coverage, but OpenSSL/native coupling and absent WASM evidence conflict with the supported portable targets. |
| local JWE implementation | repository code | `not-adopt` | Reimplementing ECDH KDF, ephemeral-key generation, protected headers and AEAD composition expands security-critical maintenance and contradicts the reuse decision. |
| SDK-owned JWE capability port | repository facade | `retain-local` | Keeps protocol DTOs and policy independent from an engine while allowing a separately qualified adapter to provide cryptography. |

The local qualification manifests and locks remain disposable under repository
`tmp/`; they are not product dependencies or release evidence. The old local
`cargo audit` executable cannot parse the advisory database's CVSS 4 records,
so authoritative advisory evidence remains the repository's pinned Nix audit
gate rather than a false clean result.

## Decision

Keep `identus-jose` as the SDK-owned facade and add only a bounded JWE
capability port. The request contains an SDK-owned P-256 ECDH-ES public key,
optional bounded key ID, exact `A128GCM` selection, bounded plaintext and
maximum compact-output size. The result is an opaque bounded compact JWE. The
port exposes no third-party type or error and diagnostics reveal no key,
plaintext, ephemeral key, ciphertext, state, target or presentation.

`identus-oid4vp` owns verifier metadata, supported-algorithm policy, DCQL
lineage and Final payload/form semantics. It invokes the port once only after
all structural and resource checks pass. A later `identus-adapters-jose` or
equivalent private implementation may wrap a qualified engine without changing
the protocol API.

## Compatibility and dependency evidence

The new surface is additive. Plain `direct_post` behavior and wire output are
unchanged, no persisted representation migrates, and no third-party public type
or error enters either SDK crate. The candidate builds used exact releases,
disabled default features where supported, Rust 1.98.1, and the host, WASM,
iOS and Android target matrix. The production workspace lock is unchanged.

Supply-chain evidence includes exact package checksums, license metadata,
unsafe and native-code inspection, dependency-cone comparison and the attempted
advisory scan. Public API and wire compatibility remain additive because the
facade exposes only SDK-owned types and plain response behavior is unchanged.

## Security, privacy and maintenance evidence

Only public P-256 recipient material is accepted; private JWK members are
rejected. Selection fails on zero or multiple eligible keys. `use` must be
`enc` when present; `key_ops`, when present, must permit the applicable
encryption/key-agreement operation. `alg` is exactly `ECDH-ES`; `enc` is exactly
`A128GCM` in this first profile. Unknown algorithms are not forwarded.

The port is effectful only for cryptographic randomness and computation. It
does not perform network, trust, storage, clock, logging or policy work. The
protocol layer validates five compact segments, bounded base64url grammar and
overall output length but does not decrypt its own result in production.

## E2E and knowledge evidence

The versioned E2E packet covers invocation → Request URI response binding → JAR
verification → Final metadata/key selection → DCQL response selection → JWE
port call → compact envelope validation → exact response-only form
construction. A test engine records the exact SDK request and returns bounded
inert compact material. This proves the protocol-to-adapter contract, but it
does not prove independent cryptographic decryptability. Issue #538 owns that
qualification before a production engine can be adopted. The knowledge page
explains the draft-to-Final correction, component responsibilities, dependency
decision, adapter contract, diagrams, limits, failure classes and
reconsideration gates.

The dependency decision deliberately leaves cryptographic implementation out of
the protocol slice. This avoids locally maintaining ECDH KDF, nonce generation,
protected-header integrity and AEAD composition while retaining one bounded,
auditable interface for a future engine. Public and private JWK members,
plaintext, ciphertext, state and presentation values remain absent from errors
and diagnostics.

## Rejected or deferred candidates

`no-way-jose`, `jwt-simple`, `josekit`, a local JWE implementation and direct
production adoption of `jose-rs` are rejected for this slice for the exact
reasons in the candidate matrix. `jose-rs` remains a researched candidate, not
executed interoperability evidence. A production adapter and independent
decryptability oracle are deferred to issue #538, where an engine must meet the
narrow dependency, public-recipient, unsafe, portability and maintenance
contract.

## Open questions and blockers

No research blocker remains. The production engine is intentionally unresolved
and is not required to specify, test, or release the SDK-owned capability port.
A future adapter issue must repeat exact-version qualification and cannot infer
adoption from this research.

Rollback is deletion of the additive port and protocol transition before any
published compatibility commitment; no storage or existing wire contract needs
migration.

## Evidence commands

The disposable qualification workspaces ran these command classes for each
candidate and retained their manifests and locks below repository `tmp/`:

```text
cargo +1.98.1 check --locked
cargo +1.98.1 check --locked --target wasm32-unknown-unknown
cargo +1.98.1 check --locked --target aarch64-apple-ios
cargo +1.98.1 check --locked --target aarch64-linux-android
cargo tree --locked --edges normal --prefix none
```

The installed unpinned `cargo audit` could not parse current CVSS 4 advisory
data, so it is not recorded as successful evidence. The repository's pinned
Nix audit remains authoritative.

Exact commands above were executed for the candidate matrix. Unrun checks are
browser entropy behavior, CSP/bundler execution, a production-engine audit in
the workspace, and downstream consumer integration; each belongs to a future
adapter or adoption slice rather than this engine-neutral contract.

## Reconsideration triggers

Reassess when `no-way-jose` publishes a stable release with public-recipient
ECDH encryption and reviewed unsafe removal/containment, or when `jose-rs`
offers narrow algorithm features that remove unrelated families. A different
stable engine may qualify if it passes the same exact gates. No automatic
upgrade or public DTO migration is authorized.
