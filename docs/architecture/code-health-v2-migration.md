# Code-health v2 migration evidence

Issue [#275](https://github.com/hyperledger-identus/sdk-rust/issues/275)
replaces the v1 Python Rust-boundary scanner with the unpublished `syn`-based
classifier specified by ADR 0126. This report compares both classifiers over
the exact implementation revision
`18394fc38cc085445979ae5e123fe9454ab22bae` before adopting the v2 baseline.

## Result

| Measure | v1 Python | v2 `syn` | Delta |
| --- | ---: | ---: | ---: |
| Production source files | 143 | 143 | 0 |
| External test files | 73 | 73 | 0 |
| Files inherited as inline test | 7 | 7 | 0 |
| Inline-test authored nonblank lines | 4,162 | 4,162 | 0 |
| Production authored nonblank lines | 29,969 | 29,969 | 0 |

Every file has the same authored nonblank production/test population under
both classifiers. The source fingerprint is
`e1434bfc52689f57cdf6601c8fc158cae6d675122efa885882ef08a9bb424f6b`.

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

The v1 implementation was loaded from planning commit
`a27f31f`, while v2 classified the same Git-tree sources at the implementation
revision. The comparison checked the complete per-file line sets after
discarding whitespace-only lines, not only aggregate totals. It also compared
production, Cargo-intrinsic external-test, and inherited inline-test file sets.

The migration preserves these invariants:

- unknown or ambiguous inclusion remains production;
- a production-reachable module and its descendants remain production;
- generated exclusions require an exact configured path and marker;
- external tests, inline tests, and production remain separate populations;
- no SDK public API, wire format, persisted format, or release artifact changes.

The v2 baseline was generated and reproduced with:

```bash
nix develop --command python3 scripts/code-health-audit.py \
  --revision 18394fc38cc085445979ae5e123fe9454ab22bae \
  --output docs/architecture/code-health-baseline.json
nix develop --command python3 scripts/code-health-audit.py --verify-baseline
```

The canonical v2 report digest is
`bcd96cc97bc15a052e575c3c4f448566dc6f0a5768fc30526b9f7ebaa7f7f6c2`.
