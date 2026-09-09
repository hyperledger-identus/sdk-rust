# Why

The SDK owns runtime-neutral storage ports and a reusable conformance suite, but
it has no encrypted storage adapter. Issue #162 asks whether Aries Askar can
supply that implementation without making its database, cryptography, FFI,
migration, executor or error model part of the generic SDK contract.

# What changes

- Evaluate exact published `aries-askar 0.4.6` with default features disabled
  and only the SQLite backend enabled in a separately locked research fixture.
- Adapt one exact-record `SecretStore` slice to the existing SDK-owned storage
  port and run the shared conformance suite against an encrypted in-memory
  SQLite store.
- Measure the exact dependency cone, native/unsafe reach, MSRV, target fit,
  transaction and revision semantics, diagnostics, maintenance and supply-chain
  evidence.
- Compare production adoption, upstream modularization, oracle/reference use
  and retaining a consumer-owned adapter.
- Keep every root manifest, release graph, public API, support claim and fast CI
  lane unchanged.

# Capabilities

## New capabilities

- `aries-askar-storage-spike`: owns reproducible evidence for the narrow Askar
  storage-adapter decision.

# Non-goals

- No production dependency, default backend, persisted migration, PostgreSQL,
  FFI, logger, Askar KMS API, hardware key, OS keychain or downstream mutation.
- No publication, release, custody, recovery, certification or platform support
  claim.
- No Askar type or error crosses an Identus public API.

# Delivery

Issue #162 owns the spike. Research and material constraints must be ready
before fixture implementation. A distinct exact-diff review, signed and DCO PR
to `develop`, and green hosted gates are required.
