# OID4VC reuse evidence completion research

Research class: protocol
Research status: ready
Decision date: 2026-09-26
Source retrieval date: 2026-09-26
Research blockers: none

## Problem and existing implementation

The current implementation and consumer evidence are unchanged from archived
issue #391 research. PR #392 supplied ADR 0156, a private-facade spike, target
and supply-chain evidence, but not the complete capability matrix or measured
payoff requested by the issue body.

## Normative sources

[OpenID4VCI 1.0 Final](https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html)
and [OpenID4VP 1.0 Final](https://openid.net/specs/openid-4-verifiable-presentations-1_0.html)
remain the protocol/draft currency. Candidate provenance remains exact
`siros-dcql 0.3.0` at revision `d4eeff53`, Impierce `e9d99d21`, Spruce
`e5f29b85`, Affinidi `66120dd7`, Equs `0ad382ce`, and Credibil `288e2dcb` as
linked in the archived research and portfolio.

## Candidate decisions

| Candidate/seam | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Existing OID4VCI | `develop@cd4d4ec` | `retain-local` | Bounded Final behavior is already delivered. | A candidate proves complete lower-risk parity. |
| Full OID4VC frameworks | exact revisions above | `oracle` | Useful breadth but excess coupling/MSRV/release cost. | A narrow published core passes all SDK gates. |
| SIROS DCQL | 0.3.0 / `d4eeff53` | `conditional-adopt` | Cohesive engine with bounded-facade mismatch. | Named OID4VP consumer and complete facade/runtime evidence. |
| Equs/Credibil | exact revisions above | `not-adopt` | Cone or public reproducibility fails. | Cohesive reproducible releases exist. |

## Compatibility and dependency evidence

The exact SIROS direct dependency cone is serde plus serde_json; the resolved
dependency cone is 12 registry packages. Comparing package names with the root
lock shows one prospective incremental production name, `siros-dcql`; the
unpublished fixture package is not a dependency. Version and feature selection
remain exact and separately locked. Public and wire compatibility, facade
boundary, rollback, MSRV, and target evidence remain as ADR 0156 records.

Preliminary exact measurements found 1,421 candidate production source lines,
1,254 upstream test lines, and 43 pre-test adapter lines. One clean Rust 1.98.1
host `cargo check` took 7.83 seconds real time and 290,275,328 bytes maximum
process RSS on the current aarch64-darwin host. These are diagnostics, not
portable performance or guaranteed code deletion.

## Security, privacy and maintenance evidence

No unsafe or native instrumentation is introduced. Candidate collection paths
allocate query models, per-query matches, selected claims, and credential
combinations, so production callers must bound query structure, credential
inventory, selected values, and combination count before/at the facade. The
existing byte limit and redacted category error remain necessary but are not a
complete production resource contract. Supply-chain, license, provenance,
release, maintenance, advisory, and security posture evidence remains pinned
by the exact lock and ADR 0156.

## Rejected or deferred candidates

No full framework is newly adopted. Unsafe allocator instrumentation, network
interoperability, browser/device runtime, signing, encryption, and presentation
construction remain deferred because they would expand this evidence-only
follow-up. Allocation count is recorded as structurally assessed rather than
fabricated.

## Open questions and blockers

There is no research blocker. The final matrix must make absent or partial
capabilities explicit, and the report must distinguish observed facts from
inference. Production reuse remains separately blocked as ADR 0156 states.

## Evidence commands

Exact commands include `wc -l` over candidate source/tests, lock-name `comm`,
`/usr/bin/time -l` with a fresh target directory, locked fixture tests and
Clippy, Rust 1.89 compilation, Rust 1.98.1 WASM/iOS/Android compilation,
`cargo deny`, `cargo audit`, source unsafe/native scan, factory checks, and the
root workspace suite. Unrun browser/mobile runtime and interoperability checks
remain explicit.
