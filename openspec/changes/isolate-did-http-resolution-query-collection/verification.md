# Verification evidence

## Identity

- Issue: #473
- Exact protected planning base:
  `90f1c5f0dd51ca4d49615acbb2a4e85a1a079c7e`
- Planning commit:
  `3820bbbf681120c2a9fecba150fc09777addc06a`
- Preimplementation receipt commit:
  `13b1201b10353e1976be2d4f31eb9a3e1675872e`
- Characterization commit:
  `1fc523da26dfb3666117d883b128ac72a7f7105e`
- Production implementation commit:
  `ec0838fd3d1a416cbf1fabf616fe3cb59d7e93dc`

## Behavioral compatibility

- The complete pre-change DID HTTP suite passed: 21 tests.
- The characterization combines parameter-count/malformed-tail,
  decoded-duplicate/invalid-known-value, invalid-name/version-conflict, and
  invalid-representation/oversized-malformed-query faults. Every HTTP case
  returns the same static `INVALID_OPTIONS` result without resolver invocation;
  representation derivation remains the earlier observable `INTERNAL_ERROR`.
- The post-change focused suite passes: 22 tests. Exact accepted option values,
  percent decoding, query semantics, bounds, response status/media shape,
  redaction, and resolver-call behavior remain green.

## Public, dependency, and resource compatibility

- `cargo-public-api 0.52.0` reports a byte-identical inventory against the
  committed `identus-did-resolver-http` candidate baseline: 960 bytes, digest
  `9c432f0d0c3d5db92da3aa88aed05164aa2adf483c11be84b9239c9973179df6`.
- No manifest, lockfile, feature, dependency, unsafe, native, FFI, public API,
  wire, serialization, protocol, limit, or error contract changed.
- The private owner preserves the same collections, typed values, allocation
  classes, source-order work, and empty-query fast path.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Implementation head |
| --- | ---: | ---: |
| `decode_resolution_options` SLOC / cognitive / cyclomatic | 78 / 13 / 32 | no function signal |
| Replacement owner method signals | n/a | 0 |
| `did-resolver-http/src/lib.rs` module signal | 0 | 0 |

The clean implementation report has source fingerprint
`36cdda263f4d52ecdcea930fbf7c5125104b4ebcdff3fea3f74b5c832cbe0707`,
population projection
`1197fc9803b853694e3873db4eaad2b585148d2887c7a5a1e4c2cf27a3dc81c0`,
and report digest
`044049bb2167d0447255e7596bbddfadd041d1d157b2fb882461af2d5ac69256`.
The only remaining signal in the touched module is the pre-existing
`negotiate` media-routing function.

## Local gates

- Focused DID HTTP tests: 22 passed.
- Workspace tests with all features: passed.
- Strict workspace Clippy with all targets and features: passed.
- Workspace documentation with warnings denied: passed.
- Public API comparison: identical candidate inventory.
- Source distribution, factory contract, OpenSpec, formatting, and exact-diff
  gates: passed.
- Nix WASM, Android ARM64, iOS ARM64, Rust 1.89 MSRV, and Rust 1.98 etalon
  checks: passed.

## Remaining delivery evidence

Synchronize the final protected baseline after prerequisite factory and JOSE
PRs land, refresh exact-head report identity, then require green hosted CI,
guarded merge, metrics publication, canonical baseline rebinding, archive, and
Discussion #399 closeout.
