# did:jwk quality baseline

Issue [#558](https://github.com/hyperledger-identus/sdk-rust/issues/558)
owns this source-bound baseline for `did.jwk.profile.v1`. Results are
directional development evidence, not portable release SLAs.

## Directional benchmark

Command:

```text
cargo bench -p identus-did-jwk --bench directional
```

Environment: Rust 1.98 etalon on Apple Silicon macOS, 2026-10-28. Each result
is the median of 101 samples after 500 warm-up iterations; every measured
sample contains 500 iterations.

| Operation | Median ns/iteration |
| --- | ---: |
| create Ed25519 DID | 5,775 |
| parse Ed25519 DID | 6,016 |
| resolve local document | 1,475 |
| reject oversized identifier | 2 |

Compare order of magnitude and early rejection behavior on the same host
rather than treating nanoseconds as a cross-host promise.

## Quality routes

- Conformance: pinned P-256 and X25519 method examples with immutable source,
  revision, retrieval date, license, and expected relationship counts.
- Property: deterministic Ed25519/X25519 public material round-trips through
  create and parse across 64 seed classes.
- Parser security: alternate ordering and whitespace preserve the exact DID;
  nested duplicates, private members, `key_ops`, malformed carriers, weak or
  invalid key material, incompatible explicit `use`, parser-depth drift, and
  unprojectable extension collections fail closed.
- E2E: an opt-in generic registry resolves, serializes, reparses, and checks the
  JWK and JSON/JSON-LD context behavior without I/O or global registration.
- Fuzz: the carrier, duplicate-safe JSON, typed JWK, key validation, and local
  projection path receives deterministic smoke and a 300-second ASan route.

Canonical commands and freshness state belong in
`quality-evidence-declarations.toml`; this file explains the stable evidence
contract and receives measured figures after the benchmark run.

## Stop-boundary evidence

The package manifest is constrained to `base64`, `serde`, `serde_json`, and
the existing `identus-core`, `identus-crypto`, and `identus-did` layers. A
depth-one `cargo tree --locked -p identus-did-jwk --edges normal,build` audit
must match that boundary. The public API inventory must contain only the
profile limits, redacted error taxonomy, `KeyUse`, `DidJwk`, and the opt-in
`DidJwkResolver`; public generation, signing, verification, transport,
storage, runtime, binding, registrar, JWK Set, or private-key types fail the
slice audit.

The repository diff is additionally constrained to the new package and its
workspace registration, conformance/fuzz evidence, OpenSpec and ADR records,
architecture and site documentation, and generated policy registries. This
proves that the slice does not mutate a consumer, binding, runtime, release,
publication, network-resolution, persistence, or custody surface.
