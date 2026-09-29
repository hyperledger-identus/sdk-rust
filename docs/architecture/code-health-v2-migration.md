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
`aff2b1239416166e84c0d405c056943ae56d7f9b49a9033f32717d8a5ea16b49`.
The v2 per-file population projection digest is
`2757e495e1d40accbae481bdb00dba835584f6a9de01c0c4fed2d24b4f9deb80`.

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
`f922df6c36ad6340e82d7a90f982a70d5958d9952b1d30c43905887f7e60a5eb`.
The baseline was most recently refreshed from protected
`develop@a33b48ead4bb1c37d63f26775c0f47c77564b4fe` after issue #444 isolated
presentation disclosure validation behind a private borrowed owner. No
production module exceeds 1,000 authored nonblank lines, and
`PresentationDisclosurePlan::new` no longer has a governed function signal.
The historical v1/v2 migration method above remains unchanged; these three
values bind the current canonical source population and report.
