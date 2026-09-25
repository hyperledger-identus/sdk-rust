# Release readiness

The first `0.1.0-rc.1` train for `identus-derive`, `identus-core`, and
`identus-crypto` was published from approved source
`21cfc28321f76f1d14a2483d536d302017674a18`. Final `0.1.0` is not implied by
that prerelease.

## M3 release record

| Gate | Status | Evidence owner |
| --- | --- | --- |
| Reproducible isolated candidate | Published from the approved exact candidate | [release `v0.1.0-rc.1`](https://github.com/hyperledger-identus/sdk-rust/releases/tag/v0.1.0-rc.1), ADRs 0113/0134 |
| Functional cryptography foundation | Complete within its recorded limitations | [#286](https://github.com/hyperledger-identus/sdk-rust/issues/286) |
| Public architecture/release handbook | Deployed | [#324](https://github.com/hyperledger-identus/sdk-rust/issues/324) |
| Durable crate names and first-publish bootstrap | Published; tokenless ownership/recovery hardening remains open | [#3](https://github.com/hyperledger-identus/sdk-rust/issues/3), [#344](https://github.com/hyperledger-identus/sdk-rust/issues/344) |
| Release compiler/support matrix | Complete for the published candidate | [#325](https://github.com/hyperledger-identus/sdk-rust/issues/325), ADR 0133 |
| Independent downstream canary | Exact-source canary green; downstream registry refresh remains externally owned | [#326](https://github.com/hyperledger-identus/sdk-rust/issues/326) |
| Natural weekly slow evidence | Complete at `19d0362038c3f2af6898624ea04347e3cd4648f7` | [run 35555024293](https://github.com/hyperledger-identus/sdk-rust/actions/runs/35555024293) |
| Exact candidate slow receipt | Complete for the published source | [run 35887204030](https://github.com/hyperledger-identus/sdk-rust/actions/runs/35887204030) |
| Two-person approval and protected publication | Complete; bootstrap publication receipt retained | [run 35990705278](https://github.com/hyperledger-identus/sdk-rust/actions/runs/35990705278), [#326](https://github.com/hyperledger-identus/sdk-rust/issues/326) |

## Approval questions

Engineers reviewing later crypto trains should reuse these questions:

1. Are the three crate responsibilities cohesive and appropriately separated?
2. Is the public API small enough for an experimental SemVer commitment?
3. Does the compiler/target promise match consumer needs and maintenance cost?
4. Is the selected dependency and unsafe/native cone acceptable?
5. Can a downstream adopt or reject the candidate without tribal knowledge?
6. Is any release blocker absent from the evidence table?

The approval and publication record is [issue #326](https://github.com/hyperledger-identus/sdk-rust/issues/326).
That issue remains open for post-bootstrap hardening, not because the first
publication is unapproved. Future approval must again bind an exact source
revision and candidate receipt; it cannot be inferred from this page or from a
green documentation deployment.

## M5 DID candidate status

The separate M5 target is `0.1.0-rc.1` for `identus-did` and
`identus-did-resolver-http`. It is candidate-only and has no publication
workflow or registry claim.

| Gate | Status | Evidence owner |
| --- | --- | --- |
| Independent train identity and deterministic archives | Complete | [#382](https://github.com/hyperledger-identus/sdk-rust/issues/382), ADR 0153 |
| Public API origins and normalized CycloneDX evidence | Complete | [#384](https://github.com/hyperledger-identus/sdk-rust/issues/384), ADR 0154 |
| Engineer-facing architecture and adoption handbook | Implemented in this source revision; deployment receipt remains on the issue | [#386](https://github.com/hyperledger-identus/sdk-rust/issues/386) |
| Rust/target qualification | Open | [#387](https://github.com/hyperledger-identus/sdk-rust/issues/387) |
| Final exact-SHA candidate approval | Open; follows all prior gates | [#388](https://github.com/hyperledger-identus/sdk-rust/issues/388) |
| Trusted-publishing administration | Separate administrator-owned hardening | [#344](https://github.com/hyperledger-identus/sdk-rust/issues/344) |

Reviewers should start with the [DID candidate train](did-candidate.md). A
green site build proves documentation integrity only; it does not complete the
open target, approval, registry, tag, or publication gates.
