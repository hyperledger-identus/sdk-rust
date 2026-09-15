## Why

`SDK-SEC-003` requires explicit resource limits for new input boundaries, but
the inherited repository still carries the broad `SDK-LIM-007` statement that
its audit is incomplete. Several focused slices added strong limits without a
single repository-wide map. One remaining concrete gap joins arbitrary
caller-supplied BIP-39 word strings and normalizes an arbitrary passphrase
before the SDK applies any bound.

## What changes

- Add one machine-readable, offline-validated inventory covering every
  implemented runtime package and each distinct input-boundary family.
- Classify boundaries as SDK-enforced, fixed-shape/no-input,
  caller-budgeted work, or outer-preallocation obligations, with exact limits,
  evidence, consumer impact, and review triggers.
- Reject oversized or structurally impossible BIP-39 inputs before joining,
  normalization, dependency parsing, or PBKDF2 work.
- Bound standalone public JWK extension count, nesting, nodes and aggregate
  key/string bytes before the crypto facade retains them.
- Inventory the bounded DID method registry and separate portable DID cache
  policy from caller-budgeted cache/clock adapter work.
- Dismantle every rejected native JWK extension tree iteratively, including
  validation errors that precede the extension budget walk.
- Narrow `SDK-LIM-007` from an incomplete audit to the unavoidable and named
  outer-allocation and caller-work obligations the SDK cannot enforce at its
  typed API boundary.
- Add mutation tests and factory integration so implemented packages, evidence,
  dispositions, and limits cannot silently disappear.

## Capabilities

### New capabilities

- `sdk-input-resource-governance`: exhaustive implemented-package inventory,
  deterministic validation, residual obligations, and review rules.

### Modified capabilities

- `crypto`: bounds BIP-39 word/count/passphrase work and retained JWK
  extensions without changing valid standard/KMP derivation results or the
  public error-enum shape.

## Non-goals

- No arbitrary cap on caller-owned hash, HMAC, signature-message, generic
  storage-record, or async-port work.
- No transport server, decompressor, serde framework, timeout scheduler, or
  concrete wallet adapter is introduced.
- No claim that a typed SDK limit prevents a caller, FFI runtime, web runtime,
  HTTP framework, or deserializer from allocating the input first.
- No public release, support-tier, protocol, wire, persistence, product, chain,
  or consumer migration.

## Delivery

Issue [#168](https://github.com/hyperledger-identus/sdk-rust/issues/168) owns
the zero-stack-depth change. Integration requires the planning receipt, focused
negative/boundary tests, a distinct security/architecture review, full local
Nix/factory gates, and protected green CI.
