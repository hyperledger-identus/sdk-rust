# Code-health v2 migration evidence

Issue [#275](https://github.com/hyperledger-identus/sdk-rust/issues/275)
replaces the v1 Python Rust-boundary scanner with the unpublished `syn`-based
classifier specified by ADR 0126. This report compares both classifiers over
the exact reviewed implementation revision
`a42835a8d0877f3a0fb6bc7fae797308d6efb38e` before adopting the v2 baseline.

## Result

| Measure | v1 Python | v2 `syn` | Delta |
| --- | ---: | ---: | ---: |
| Production source files | 143 | 143 | 0 |
| External test files | 73 | 73 | 0 |
| Files inherited as inline test | 7 | 7 | 0 |
| Inline-test authored nonblank lines | 4,180 | 4,180 | 0 |
| Production authored nonblank lines | 29,974 | 29,974 | 0 |

Every file has the same authored nonblank production/test population under
both classifiers. The source fingerprint is
`8116415f54ee456f1f284a4010925debac9005407b352d228d1d14d8f2c68dc6`.
The v2 per-file population projection digest is
`803571144e45c161ff77a090088459ba0add67a023a05e9e9f23dff2ac38fda6`.

The sole representation delta is intentional: v1 included blank lines in a
file inherited wholly as inline test, while v2 reports only lines containing
authored non-whitespace. That removes 252 blank-only line numbers without
changing any reported population count:

| Inherited test file | Blank-only lines omitted by v2 |
| --- | ---: |
| `crates/conformance/src/boundary.rs` | 48 |
| `crates/conformance/src/dep_graph.rs` | 28 |
| `crates/conformance/src/guard/mod.rs` | 29 |
| `crates/conformance/src/naming.rs` | 13 |
| `crates/conformance/src/unsafe_policy.rs` | 26 |
| `crates/did-resolver-http/src/tests.rs` | 59 |
| `crates/wallet-conformance/src/tests.rs` | 49 |
| **Total** | **252** |

This is consistent with the contract: `authored_nonblank_lines` measures
review surface, and a line can leave production only when every authored
non-whitespace byte belongs to a proven test-only span.

## Method and invariants

The v1 implementation was loaded from planning commit `a27f31f`, while v2
classified the same Git-tree sources at the reviewed implementation revision.
The comparison checked the complete per-file line sets after discarding
whitespace-only lines, not only aggregate totals. It also compared production,
Cargo-intrinsic external-test, and inherited inline-test file sets.

The migration preserves these invariants:

- unknown or ambiguous inclusion remains production;
- a production-reachable module and its descendants remain production;
- generated exclusions require an exact configured path and marker;
- external tests, inline tests, and production remain separate populations;
- no SDK public API, wire format, persisted format, or release artifact changes.

The v2 baseline was generated and reproduced with:

```bash
nix develop --command python3 scripts/code-health-audit.py \
  --revision a42835a8d0877f3a0fb6bc7fae797308d6efb38e \
  --output docs/architecture/code-health-baseline.json
nix develop --command python3 scripts/code-health-audit.py --verify-baseline
```

The canonical v2 report digest is
`3e88aa68b86398e260648ba8628cae02383ff7e36278307d6479e36c016c88b8`.
