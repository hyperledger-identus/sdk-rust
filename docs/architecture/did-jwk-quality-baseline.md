# did:jwk quality baseline

Issue [#558](https://github.com/hyperledger-identus/sdk-rust/issues/558)
owns this source-bound baseline for `did.jwk.profile.v1`. Results are
directional development evidence, not portable release SLAs.

## Directional benchmark

Command:

```text
cargo bench -p identus-did-jwk --bench directional
```

The benchmark measures deterministic Ed25519 creation, parsing, local document
projection, and early oversized-carrier rejection. Record same-host medians
after the implementation gates complete; compare order of magnitude and early
rejection behavior rather than treating nanoseconds as a cross-host promise.

## Quality routes

- Conformance: pinned P-256 and X25519 method examples with immutable source,
  revision, retrieval date, license, and expected relationship counts.
- Property: deterministic Ed25519/X25519 public material round-trips through
  create and parse across 64 seed classes.
- Parser security: alternate ordering and whitespace preserve the exact DID;
  nested duplicates, private members, `key_ops`, malformed carriers, weak or
  invalid key material, and incompatible explicit `use` fail closed.
- E2E: an opt-in generic registry resolves, serializes, reparses, and checks the
  JWK and JSON/JSON-LD context behavior without I/O or global registration.
- Fuzz: the carrier, duplicate-safe JSON, typed JWK, key validation, and local
  projection path receives deterministic smoke and a 300-second ASan route.

Canonical commands and freshness state belong in
`quality-evidence-declarations.toml`; this file explains the stable evidence
contract and receives measured figures after the benchmark run.
