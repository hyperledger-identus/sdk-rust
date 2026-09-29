# Code-health v2 migration evidence

Issue [#275](https://github.com/hyperledger-identus/sdk-rust/issues/275)
replaces the v1 Python Rust-boundary scanner with the unpublished `syn`-based
classifier specified by ADR 0126. This report compares both classifiers over
the durable pre-change `develop` source revision
`5dff6f38c861b858dd62dc8310246f7d485d3e92` before adopting the v2 baseline.
Classifier identity, protocol, locked dependencies, and exact projection bind
the reviewed implementation separately from that historical source snapshot.

The final classifier request uses protocol v2. Python derives exact library and
binary target roots from Cargo manifests at the audited Git revision; this
replaced the reviewed-but-insufficient function-name heuristic. The durable
source population and projection remain unchanged by that protocol refinement.

## Result

| Measure | v1 Python | v2 `syn` | Delta |
| --- | ---: | ---: | ---: |
| Production source files | 135 | 135 | 0 |
| External test files | 73 | 73 | 0 |
| Files inherited as inline test | 7 | 7 | 0 |
| Inline-test authored nonblank lines | 3,990 | 3,990 | 0 |
| Production authored nonblank lines | 29,161 | 29,161 | 0 |

Every file has the same authored nonblank production/test population under
both classifiers. The current protected baseline source fingerprint is
`36cdda263f4d52ecdcea930fbf7c5125104b4ebcdff3fea3f74b5c832cbe0707`.
The v2 per-file population projection digest is
`1197fc9803b853694e3873db4eaad2b585148d2887c7a5a1e4c2cf27a3dc81c0`.

The sole representation delta is intentional: v1 included blank lines in a
file inherited wholly as inline test, while v2 reports only lines containing
authored non-whitespace. That removes 252 blank-only line numbers without
changing any reported population count:

| Inherited test file | Blank-only lines omitted by v2 |
| --- | ---: |
| `crates/conformance/src/guard/boundary.rs` | 48 |
| `crates/conformance/src/guard/dep_graph.rs` | 28 |
| `crates/conformance/src/guard/mod.rs` | 29 |
| `crates/conformance/src/guard/naming.rs` | 13 |
| `crates/conformance/src/guard/unsafe_policy.rs` | 26 |
| `crates/did-resolver-http/src/tests.rs` | 59 |
| `crates/wallet-conformance/src/tests.rs` | 49 |
| **Total** | **252** |

This is consistent with the contract: `authored_nonblank_lines` measures
review surface, and a line can leave production only when every authored
non-whitespace byte belongs to a proven test-only span.

## Method and invariants

The v1 implementation was loaded from planning commit `a27f31f`, while v2
classified the same Git-tree sources with the reviewed classifier.
The comparison checked the complete per-file line sets after discarding
whitespace-only lines, not only aggregate totals. It also compared production,
Cargo-intrinsic external-test, and inherited inline-test file sets.

The migration preserves these invariants:

- unknown or ambiguous inclusion remains production;
- a production-reachable module and its descendants remain production;
- generated exclusions require an exact configured path and marker;
- external tests, inline tests, and production remain separate populations;
- no SDK public API, wire format, persisted format, or release artifact changes.

The config-pinned durable source baseline is reproduced with:

```bash
nix develop --command python3 scripts/code-health-audit.py --verify-baseline
```

The canonical v2 report digest is
`d0dac39a629448e48a83f034036b01272264e88c8b9cc2ecef4e6fdea6a5f036`.
The baseline was most recently refreshed from protected
`develop@629aeb46f75be9c5f6b22aa0649e04f951f420de` after issue #473 isolated
DID HTTP resolution-query field collection behind one private owner. No
production module exceeds 1,000 authored nonblank lines, and neither the former
`decode_resolution_options` hotspot nor a replacement collector has a governed
function signal. The unrelated `negotiate` media-routing signal remains
visible. The historical v1/v2 migration method above remains
unchanged; these three values bind the current canonical source population and
report.
