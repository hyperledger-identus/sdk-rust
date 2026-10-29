# DID candidate readiness packet

The `identus-did` train is technically qualified but is not yet approved or
published. This page is the durable engineer-review packet for
`identus-did 0.1.0-rc.1` and `identus-did-resolver-http 0.1.0-rc.1`.

Current state: **pending an exact-protected-revision slow run and independent
approval**.

The machine-readable source is the
[readiness manifest](https://github.com/hyperledger-identus/sdk-rust/blob/develop/docs/release/did-candidate-readiness.toml).

## Candidate identity

| Field | Value |
| --- | --- |
| Train | `did` |
| Candidate | `identus-did-0.1.0-rc.1` |
| Packages | `identus-did`, then `identus-did-resolver-http` |
| Protected evidence base | `34420235103fe9dc599aa061047525c459d422cf` |
| Staged lock SHA-256 | `f7535a6819c9b9e4474c95b427b54646d46ccac51703c680086ef75c790fba4f` |
| `identus-did` API SHA-256 | `5c985cd2d3f26c07230ace1086d7a178662a4f583d0086574ac9baa6a00be639` |
| Resolver API SHA-256 | `9c432f0d0c3d5db92da3aa88aed05164aa2adf483c11be84b9239c9973179df6` |
| Candidate archive SHA-256 | `7736e445ffce327fae19a55c7b8b7adf38887f93f56aba4a9a804d18d765fb68` / `90366988dd00a4730d398431531d0066eb109ff48a1fcc8f0aed4d9673678480` |
| CycloneDX SHA-256 | `d0e0976fa6d96918d59202cd173c4162459f19b253120c41fce3058e5c59d109` / `fbaa3985eacf2c89649799c1a33effeb2b521aa21c5ec92ff70444a6afb126d6` |
| Reserved future tag | `identus-did-v0.1.0-rc.1` |
| Canonical source state | `0.0.0`, `publish = false` |
| Exact release subject | Pending the protected merge revision from [#388](https://github.com/hyperledger-identus/sdk-rust/issues/388) |

The pending merge identity is deliberate: this report cannot name its own
future squash commit. The final GitHub receipt must bind that protected merge
SHA to the complete slow run and independent approval.

## Closed evidence

| Evidence class | Status | Receipt |
| --- | --- | --- |
| Independent train identity and deterministic archives | Complete | [#382](https://github.com/hyperledger-identus/sdk-rust/issues/382), ADR 0153 |
| Public API origins and normalized CycloneDX | Complete | [PR #584](https://github.com/hyperledger-identus/sdk-rust/pull/584), [factory receipt](https://github.com/hyperledger-identus/sdk-rust/pull/584#issuecomment-6493993716), ADR 0154 |
| Engineer handbook and source-adoption guidance | Complete | [#386](https://github.com/hyperledger-identus/sdk-rust/issues/386) |
| Linux/macOS primary + MSRV and package-specific portable targets | Complete | [#387](https://github.com/hyperledger-identus/sdk-rust/issues/387), ADR 0155 |
| Exact protected-revision slow evidence | Pending | [#388](https://github.com/hyperledger-identus/sdk-rust/issues/388) |
| Independent exact-receipt approval | Pending | [#388](https://github.com/hyperledger-identus/sdk-rust/issues/388) |

The retained historical failed weekly run,
[run 40260649503](https://github.com/hyperledger-identus/sdk-rust/actions/runs/40260649503),
failed in `slow (macos-latest)` after RustSec classified build-time-only
`basic-toml 0.1.10` as unmaintained. [Issue #564](https://github.com/hyperledger-identus/sdk-rust/issues/564)
and ADR 0191 introduced a narrow, exact-path, expiring acceptance without
waiving vulnerabilities or unrelated advisories. That fix makes a new run
actionable; it does not retroactively turn the failed run green.

## Downstream E2E and QoS evidence

NeoPRISM remains the owner of Cardano observation, persistence, service
runtime, and production network selection. Its SDK adoption branch and parity
harness provide independent consumer-shaped evidence:

- [SDK adoption PR #324](https://github.com/hyperledger-identus/neoprism/pull/324)
  consumes immutable SDK source through compatibility facades.
- [Parity issue #325](https://github.com/hyperledger-identus/neoprism/issues/325)
  defines the comparison contract.
- [Harness PR #327](https://github.com/hyperledger-identus/neoprism/pull/327)
  provides isolated published-versus-candidate execution.
- [Repeated final-candidate receipt](https://github.com/hyperledger-identus/neoprism/issues/325#issuecomment-6441653199)
  records byte-identical canonical manifests at fixed Preprod slot
  `10,723,321`.
- [Latest composition receipt](https://github.com/hyperledger-identus/neoprism/issues/325#issuecomment-6487638362)
  records zero missing, extra, duplicate, or changed operations; zero pending
  rows and restarts; and diagnostic duration, cursor-rate, CPU, RAM, and process
  measurements.

These runs prove bounded compatibility for the stated images, revisions,
network boundary, and harness. They are not a claim that SDK-Rust owns Cardano
integration, that every chain state was sampled, or that diagnostic QoS ratios
are production thresholds.

## Reproduce the repository evidence

From an artifact-free checkout at the candidate revision:

```console
nix develop --command python3 scripts/check-release-candidates.py .
nix develop --command python3 scripts/check-did-final-readiness.py .
nix build .#docs-site --no-link
scripts/factory check
```

The complete Linux/macOS slow matrix runs in the protected `slow` workflow and
must bind the same protected revision. Local execution cannot substitute for
the required hosted macOS receipt or independent human approval.

## Explicit non-claims

This packet does not claim crates.io publication, stable API compatibility,
certification, vulnerability-free status, browser/device runtime support,
production readiness, or approval. The HTTP adapter remains host-only;
portable target rows are compile checks for `identus-did` only. Publication is
a later protected decision.
