# Verification evidence

## Identity

- Issue: #456
- Exact protected `develop` base:
  `8f229759eeb2d2b8a0be8a1949035f6a7533df06`
- Planning commit:
  `7aaa44681125beaa61e470000ee10e9440209496`
- Preimplementation receipt commit:
  `e07e7550d585a8390ccf202d4fcf33f9a4937d50`
- Protected-base synchronization:
  `f0865d134bd3a397ba1005dce003f5241f9cd6d9`
- Reviewed implementation head:
  `0c9c6d11ddaf6dbcc831e20e6bc0526d9331ac06`

## Behavioral compatibility

- The complete pre-change DID resolution suite passed: 21 tests passed and
  one release-only diagnostic was ignored.
- Characterization covers every fixed separator and digit position,
  representative structural/calendar/time failures, exact end-of-day syntax,
  an extended negative leap year, and owned, borrowed, `FromStr`, `TryFrom`,
  and Serde entry-point equivalence.
- The post-change suite passes with that characterization added: 22 tests
  passed and one release-only diagnostic was ignored.
- The private parsed value preserves signed and extended years, bounded ASCII
  grammar, Gregorian leap-year projection modulo 400, `24:00:00Z`, and the
  single static `invalid datetime` failure exposed by every public entry point.

## Public, dependency, and resource compatibility

- `cargo-public-api 0.52.0` with repository rustdoc JSON reports byte-for-byte
  identical simplified public inventories for `identus-did` at the protected
  base and reviewed implementation head: 120,106 bytes each.
- No manifest, feature, lockfile, dependency, unsafe, native, FFI,
  serialization, wire, W3C algorithm, or public error contract changed.
- Parsing remains a single bounded pass over caller-provided text. The change
  adds no allocation, collection, clone, I/O, callback, dynamic dispatch, or
  unbounded work path.
- Source-distribution verification passes for all five governed packages.

## Code-health comparison

| Signal | Protected base | Implementation head |
| --- | ---: | ---: |
| `validate_datetime` SLOC | 71 | no function signal |
| `validate_datetime` cognitive complexity | 17 | no function signal |
| `validate_datetime` cyclomatic complexity | 40 | no function signal |
| Replacement helper signals | n/a | 0 |
| `resolution/value.rs` module signal | 0 | 0 |

The improvement is one private parsed value with lexical parsing separated
from calendar validation and one cohesive extended-year parser. It is not
generated code, threshold weakening, a waiver, or a helper per conditional.
The live report for the reviewed head has source fingerprint
`1b218c5e705000136a42ee30e265d8d66a67886060114ce2f39925b96e4134de`,
population projection
`2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`,
and report digest
`9bf7917397be95fb34d466da25dcc48440093bf6465875c78b77cf054e540963`.

The implementation was squash-merged to protected
`develop@094735f0995dc2914f92e4106d11ddfa189f7399`. The canonical report was
regenerated from that immutable revision and has source fingerprint
`1b218c5e705000136a42ee30e265d8d66a67886060114ce2f39925b96e4134de`,
population projection
`2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`,
and report digest
`b03d789f82ac590e444dfbf025d0cc895afdd99d5d0450e8447c787ad53dbb90`.
The protected report has no `validate_datetime`, replacement-helper, or
`resolution/value.rs` module signal.

## Local gates

- Focused DID resolution tests: passed.
- Workspace tests with all features: passed.
- Strict workspace Clippy with all targets and features: passed.
- Workspace documentation with warnings denied: passed.
- `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios` workspace checks: passed.
- Public API comparison: identical simplified inventories.
- Source distribution, factory contract, OpenSpec, formatting, diff, and Nix
  flake evaluation gates: passed.
- Rust 1.89 MSRV and canonical Nix `rust-test` gates: passed.
- Implementation PR #457 exact-head CI and protected guarded merge: passed.
