# Release readiness

The target under review is `0.1.0-rc.1` for `identus-derive`, `identus-core`,
and `identus-crypto`. Final `0.1.0` is not the current decision.

## M3 gate status

| Gate | Status | Evidence owner |
| --- | --- | --- |
| Reproducible isolated candidate | Prepared; protected publication activated | [#266](https://github.com/hyperledger-identus/sdk-rust/issues/266), ADRs 0113/0134 |
| Functional cryptography foundation | Complete within its recorded limitations | [#286](https://github.com/hyperledger-identus/sdk-rust/issues/286) |
| Public architecture/release handbook | Required evidence; deployment receipt on issue | [#324](https://github.com/hyperledger-identus/sdk-rust/issues/324) |
| Durable crate names and first-publish bootstrap | Open blocker | [#3](https://github.com/hyperledger-identus/sdk-rust/issues/3) |
| Release compiler/support matrix | Selected; exact candidate evidence pending | [#325](https://github.com/hyperledger-identus/sdk-rust/issues/325), ADR 0133 |
| Independent downstream canary | Downstream-owned; does not change or specialize the SDK | External adoption evidence |
| Natural weekly slow evidence | Complete at `19d0362038c3f2af6898624ea04347e3cd4648f7` | [run 35555024293](https://github.com/hyperledger-identus/sdk-rust/actions/runs/35555024293) |
| Exact candidate slow receipt | Complete for the current frozen pre-activation candidate; required again if the approved release SHA changes | [#326](https://github.com/hyperledger-identus/sdk-rust/issues/326) |
| Two-person release approval | Not granted | [#326](https://github.com/hyperledger-identus/sdk-rust/issues/326) |

## Approval questions

Engineers reviewing M3 should answer:

1. Are the three crate responsibilities cohesive and appropriately separated?
2. Is the public API small enough for an experimental SemVer commitment?
3. Does the compiler/target promise match consumer needs and maintenance cost?
4. Is the selected dependency and unsafe/native cone acceptable?
5. Can a downstream adopt or reject the candidate without tribal knowledge?
6. Is any release blocker absent from the evidence table?

The approval record is [issue #326](https://github.com/hyperledger-identus/sdk-rust/issues/326).
Approval must bind an exact source revision and candidate receipt. It cannot be
inferred from this page or from a green documentation deployment.
