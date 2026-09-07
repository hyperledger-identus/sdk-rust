# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/177
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` remains effective: the implementation adds a reusable named
cryptographic algorithm but no Cardano ledger, address, wallet or product
policy. `SDK-COMPAT-002` and `SDK-COMPAT-004` remain Rust 1.85.0 and primary
Rust 1.98.1; the dependency declares Rust 1.81 and passed both relevant probes.

`SDK-SEC-001` remains effective because no unsafe block is added to SDK source.
The transitive dependency's reachable unsafe is reviewed and bounded by ADR
0078 rather than represented as safe Rust. `SDK-SEC-002` governs the new
secret-bearing type and requires the explicit export plus redaction tests.
`SDK-DELIVERY-001` is satisfied by issue #177 and this change lifecycle.

## Introduced or changed constraints

No repository-wide constraint or support-policy value changes. The focused
decision permits `ed25519-bip32 0.4.3` and SDK-locked `cryptoxide 0.6.5` in the
`identus-crypto` implementation graph only while the SDK-owned facade,
redaction, zeroization, exact version/vector evidence, target matrix and
rollback contract remain true. It does not permit dependency types in public
API or authorize other `cryptoxide` use. The candidate's activation of all
cryptoxide default features is an explicit temporary limitation and must be
included in the upstream-hardening follow-up.

## Introduced or changed limitations

The default crypto graph gains reachable unsafe inside two third-party crates.
The direct crate can format its own `XPrv` as secret hex and uses manual unsafe
zeroing; the SDK prevents access to those surfaces but cannot claim to have
remediated upstream code. Compile checks on WASM, Android and iOS do not prove
runtime, FFI, packaging, memory-erasure or platform certification behavior.

The raw `expose_secret_bytes` operation deliberately transfers a copy to the
caller, who then owns its protection and erasure. No FFI or serialization
surface is introduced. Upstream hardening and the temporary-fork fallback are
tracked separately and do not block the bounded implementation.

## Consumer and product impact

Default `identus-crypto` consumers compile two additional pure-Rust packages
and receive additive Cardano V2 derivation types. Minimal consumers can disable
default features or select only existing capabilities. Existing `EdHDKey`,
wire formats and persistence remain unchanged. Downstream Apollo and NeoPRISM
migrations are not performed or claimed by this change.

## Activation and rollback

The bounded dependency and API activate only when the issue #177 PR merges to
`develop`. Revert that single PR to remove the types, feature and dependency;
no migration or persisted value must be rewritten. Any future public exposure,
new use of `cryptoxide`, FFI, or relaxation of redaction/zeroization requires a
new material decision.

## Evidence

The research record contains exact provenance, source diff, dependency cone,
MSRV/primary and supported-target compile results, unsafe/native scan and
RustSec output. Implementation tasks require Apollo/upstream vectors, feature
graph checks, redaction/zeroization tests, public-API evidence, full factory
validation and distinct security review before merge.
