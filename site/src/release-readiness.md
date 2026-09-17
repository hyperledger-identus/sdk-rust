# Release readiness

The target under review is `0.1.0-rc.1` for `identus-derive`, `identus-core`,
and `identus-crypto`. Final `0.1.0` is not the current decision.

## M3 gate status

| Gate | Status | Evidence owner |
| --- | --- | --- |
| Reproducible isolated candidate | Prepared; publication prohibited | [#266](https://github.com/hyperledger-identus/sdk-rust/issues/266), ADR 0113 |
| Functional cryptography foundation | Complete within its recorded limitations | [#286](https://github.com/hyperledger-identus/sdk-rust/issues/286) |
| Public architecture/release handbook | Required evidence; deployment receipt on issue | [#324](https://github.com/hyperledger-identus/sdk-rust/issues/324) |
| Durable crate names and trusted publishing | Open blocker | [#3](https://github.com/hyperledger-identus/sdk-rust/issues/3) |
| Release compiler/support matrix | Open blocker | [#325](https://github.com/hyperledger-identus/sdk-rust/issues/325) |
| Midnight Identity minimal consumer | Open blocker | [midnight-identity#81](https://github.com/MediaNoxLabs/midnight-identity/issues/81) |
| Natural weekly slow evidence | Awaiting first healthy scheduled run | [#276](https://github.com/hyperledger-identus/sdk-rust/issues/276) |
| Exact candidate slow receipt | Required after candidate freeze | [#326](https://github.com/hyperledger-identus/sdk-rust/issues/326) |
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
