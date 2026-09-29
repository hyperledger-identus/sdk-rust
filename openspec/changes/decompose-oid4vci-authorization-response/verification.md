# Verification evidence

## Identity

- Issue: #464
- Exact protected `develop` base:
  `1a3aca558734cd317860862a7a91284cf5991b18`
- Planning commit:
  `72dad579fb005681cc1bab75b7ff8f5e0ae6e41e`
- Preimplementation receipt commit:
  `e36551490f78e7616ff957e9fe8b68ac7acdfb6d`
- Characterization commit:
  `75d677146f0fa3f3f2a6f23da38debf18ca6a083`
- Production decomposition commit:
  `8a3fc21bda9379b9cef75ff4eefc36668ce015f2`
- Protected-base synchronization commit:
  `11f1676f046e096a8cbcfa95429c2117efedf44f`
- Reviewed source head after precedence correction:
  `0d4af96c2b3b9a96d287d7fca3993bd6af886160`

## Behavioral compatibility

- The complete pre-change Authorization Response suite passed: 10 tests.
- Preimplementation characterization binds query decoding before state,
  issuer, branch selection, and branch grammar; state before issuer; issuer
  before branch shape; branch shape before value grammar; and error code before
  description before URI grammar.
- Exact-diff review found that the first decomposition decoded a duplicate
  parameter's value before rejecting its decoded name. The correction restores
  the original name-before-value precedence and adds an invalid-value duplicate
  case to the combined-fault matrix.
- The post-change focused suite passes: 11 tests. All existing success, error,
  issuer, state, malformed-form, limit, redaction, and diagnostic tests remain
  green.

## Public, dependency, and resource compatibility

- `cargo-public-api 0.52.0` reports byte-for-byte identical simplified public
  inventories for `identus-oid4vci` at protected `develop` and the reviewed
  source head: 272,060 bytes each with digest
  `e03d241781d6d1fbe0d2335054c53a396e7b2b4eeba4ea3a6f88577388c616a4`.
- No manifest, feature, lockfile, dependency, unsafe, native, FFI,
  serialization, wire, public error, or protocol capability changed.
- The private decoder owns the same query borrow, limits, decoded-name set,
  and zeroizing response fields. The private correlator owns the same consumed
  request and decoded response. The scan remains single-pass and adds no
  collection, clone, allocation, callback, dynamic dispatch, trait bound,
  synchronization, I/O, or unbounded work path.
- Existing encoded query, parameter-count, decoded name/value, and role-specific
  ceilings remain the authoritative work and memory boundaries.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Reviewed source head |
| --- | ---: | ---: |
| `try_into_authorization_response` SLOC / cognitive / cyclomatic | 78 / 21 / 25 | no function signal |
| `parse_query` SLOC / cognitive / cyclomatic | 75 / 16 / 25 | no function signal |
| Replacement decoder/correlator method signals | n/a | 0 |
| `authorization_response.rs` module signal | 0 | 0 |

The improvement is two private owners aligned to the untrusted-input and
request-authority boundaries, not forwarding wrappers or one helper per
condition. The module remains below the 1,000-line attention threshold at 549
authored nonblank lines. The live reviewed report has source fingerprint
`e005c014c24d829626949ed9c70f927f0aac8f6b517315ecc0bafcdca9afd84d`,
population projection
`2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`,
and report digest
`93fecfaca1f83285423b9b6a4845f3c5db935b2abf34bcf33e2a1c5e49f5542d`.

Canonical evidence must be regenerated and rebound after the implementation
is squash-merged to protected `develop`.

## Local gates

- Focused Authorization Response tests: passed.
- Workspace tests with all features: passed.
- Strict workspace Clippy with all targets and features: passed.
- Workspace documentation with warnings denied: passed.
- `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios` workspace checks: passed.
- Public API comparison: identical simplified inventories.
- Source distribution, factory contract, OpenSpec, formatting, diff, and Nix
  flake evaluation gates: passed.
- Rust 1.89 MSRV Nix gate: passed.
- Canonical Rust 1.98 Nix nextest gate: passed.
