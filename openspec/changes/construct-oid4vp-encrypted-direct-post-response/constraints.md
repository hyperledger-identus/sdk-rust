# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/535
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 keep protocol policy above primitives and
effects behind ports. SDK-SEC-003 and SDK-LIM-007 require bounded untrusted work.
ADR 0159 keeps JOSE protocol-neutral and requires a new decision before a
production engine is added. Existing portability, unsafe, error, release and
consumer-ledger constraints remain effective.

## Introduced or changed constraints

- The Final `direct_post.jwt` profile is unsigned encrypted JWT, never signed
  JARM by default or compatibility inference.
- Only P-256 `ECDH-ES` plus `A128GCM` is accepted in the first profile.
- The verifier public key, algorithm, key use/operations and optional `kid` are
  validated before retained state is constructed; private JWK members fail.
- Key selection is deterministic and fails on ambiguity.
- The JWE port accepts only SDK-owned bounded values and returns only a bounded
  opaque compact value; dependency DTOs and errors cannot cross the facade.
- Request/DCQL lineage is consumed once. Target, state, query identifiers and
  payload cannot be rebound after validation.
- The encrypter is invoked at most once and only after all non-cryptographic
  checks and output upper-bound calculations pass.
- Diagnostics expose static categories and counts/lengths only.
- No production JWE dependency is adopted by this change.

## Introduced or changed limitations

- SDK-Rust supplies the safe protocol and cryptographic capability boundary,
  not a built-in JWE engine in this slice.
- Only one recipient, compact serialization, P-256 ECDH-ES and A128GCM are
  supported. X25519, P-384, key wrapping, CBC-HMAC, HPKE, compression,
  multi-recipient JWE and critical extensions are excluded.
- Compile evidence does not prove browser entropy runtime, CSP or bundler
  behavior. A real adapter must provide target-specific runtime evidence.
- Encryption provides confidentiality and integrity, not wallet authentication.
- Plain protocol errors remain caller-selected fallback; automatic SDK-error
  mapping is prohibited.

## Consumer and product impact

The change adds an opt-in Final `direct_post.jwt` construction path without
changing plain `direct_post`, persisted data, default transport behavior or
existing consumers. Callers must supply a separately implemented JWE port;
SDK-Rust does not claim a bundled production engine in this slice. Language
adapters may wrap the Rust-native types but must not reshape the core DTO or
error contract to match a donor SDK.

## Evidence

Issue #535 is the exact standing-authority decision for this additive slice.
ADR 0181 records the dependency and facade boundary. The OpenSpec scenarios,
canonical consumer-change ledger entry, cross-language vector packet and
quality-evidence declaration bind activation to focused, E2E, property, fuzz,
benchmark, differential, portable-target and protected-CI evidence.

## Activation and rollback

Activation requires issue #535 preflight, accepted ADR, strict OpenSpec,
vectors, E2E/property/fuzz/benchmark evidence, portable and dependency gates,
exact-head CLI review and protected CI. Rollback removes the additive types,
mode, transition and evidence before publication; plain response behavior is
unchanged and no persisted migration exists.

