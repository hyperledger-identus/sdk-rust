# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

Issues #382, #384, #386, and #387 delivered the train identity, deterministic
candidate assembly, API and CycloneDX evidence, public handbook, and staged
compiler/target matrix. The canonical manifests remain unpublished at version
`0.0.0`; isolated release-shaped archives use `0.1.0-rc.1`. The existing
release-candidate specification already requires exact source and staged-lock
identity, bounded receipts, and honest per-package target limitations.

PR #584 merged the additive-only DID API-origin rebaseline as protected
`develop@34420235103fe9dc599aa061047525c459d422cf`. Its closed receipt records
13 additions, zero removals, the refreshed staged-lock identity, deterministic
archives and SBOMs, primary/MSRV evidence, full Nix evidence, and an independent
Codex review. This packet consumes those exact identities; it does not infer
them from a mutable branch.

The latest natural weekly run at
`5da04223c1c4e029cb368870877e56d699d40388` failed only in the macOS slow job
when RustSec classified build-time-only `basic-toml 0.1.10` as unmaintained.
Issue #564 and ADR 0191 subsequently added an exact-path, exact-version,
machine-expiring acceptance for that warning while preserving authoritative
audit of every vulnerability and unrelated advisory. A new complete run is
therefore required; the failed run is provenance, not passing evidence.

NeoPRISM PR #324 proves immutable-Git adoption of SDK crypto, DID syntax,
resolution projection, and HTTP bindings. NeoPRISM issue #325 and PR #327
carry repeated fixed-boundary Preprod semantic parity and QoS receipts. Those
are downstream compatibility evidence only: they do not prove registry
availability, SDK target support, or production promotion.

## Normative sources

The repository sources of truth are coordinator #381, delivery issue #388,
ADRs 0132/0153/0154/0155/0191, the closed train descriptor and staged lock,
the canonical release-candidate and documentation specifications, and the
archived prerequisite review/verification receipts. W3C DID behavior is not
reinterpreted. NeoPRISM receipts are compatibility evidence under that
consumer's own architecture and acceptance contract.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Durable repository readiness report plus issue/discussion receipt | `adopt` | Keeps release facts reviewable and linkable while GitHub comments carry live approval state. | Release evidence becomes generated from signed attestations. |
| Reuse the September green slow run | `not-adopt` | It predates material candidate and audit changes and is not the frozen revision. | Never; exact-SHA evidence is required. |
| Treat the failed October run as passing because its receipt job succeeded | `not-adopt` | One required macOS job failed; an aggregate artifact cannot waive a red lane. | Never; all required lanes must succeed. |
| Dispatch one complete slow run after the evidence-only PR merges | `adopt` when explicitly authorized | Issue #388 and the user's autonomous delivery request authorize bounded final-M5 verification; it avoids waiting a week while preserving exact-SHA identity. | A fresh natural schedule covers the same exact revision first. |
| Fold PRISM method/VDR/indexer crates into M5 | `not-adopt` | M5 deliberately qualifies only the generic DID and HTTP packages. | A separate portable-PRISM release-train issue and ADR. |
| Mutate NeoPRISM to refresh evidence | `not-adopt` | Downstream PRs already carry immutable receipts and remain independently governed. | A NeoPRISM adoption issue explicitly owns a new revision. |

## Sources and provenance

- SDK-Rust issues #381, #382, #384, #386, #387, #388, #564, and #582;
  PR #584 and its retained factory metric receipt.
- ADRs 0132, 0153, 0154, 0155, and 0191.
- `docs/release/release-trains.toml`, `did-candidate.toml`, and the staged lock.
- Archived OpenSpec review/verification records for the prerequisite slices.
- Natural slow run https://github.com/hyperledger-identus/sdk-rust/actions/runs/40260649503.
- NeoPRISM PRs #324/#327 and issue #325, all read-only in this slice.

All new text is repository-owned Apache-2.0 material. No donor source or
fixture enters the repository.

## Compatibility and dependency evidence

The slice changes static documentation, one deterministic report validator,
and its tests. It adds no Rust, Cargo, Nix, browser, runtime, wire, persisted-
data, public API, feature, compiler, or target behavior. The validator uses the
repository's existing Python toolchain and reads bounded tracked text only.
Rollback removes the report, navigation, validator, and tests without changing
the candidate archives or any consumer.

## Evidence matrix

- **Identity:** exact protected merge SHA, staged-lock digest, train descriptor,
  package order, version, and archive checksums.
- **API and supply chain:** committed public-API origins, normalized CycloneDX,
  license/source/advisory policy, and explicit risk-acceptance boundary.
- **Targets:** Linux/macOS primary and MSRV lanes; portable compile-only lanes
  only for `identus-did`; HTTP adapter remains host-only.
- **Documentation:** reproducible mdBook, diagrams, offline links, crate
  responsibilities, source-evaluation guidance, and limitations.
- **Downstream E2E:** immutable NeoPRISM source pin plus repeated Preprod
  operation parity and diagnostic CPU/RAM/duration receipts.
- **Review:** Codex or Claude exact-head review followed by independent human
  approval of the final immutable receipt.
- **Commands still unrun:** candidate qualification and local factory/site
  checks after preflight; complete hosted slow evidence after merge.

## Security, privacy and maintenance evidence

No secret, private key, credential, personal data, or private disclosure is
used. The report must not echo environment variables or raw runner logs.
Security evidence may reference the bounded ADR 0191 acceptance but must not
claim vulnerability-free software. Publication remains separately protected
and administrator-owned.

The report is static, credential-free, analytics-free, and contains no secret
or personal input. Its machine guard rejects known release overclaims and
forces explicit pending state. Maintenance remains with the release train and
documentation owners; immutable links and exact identities prevent an old
green receipt from silently becoming current evidence.

## Rejected or deferred candidates

Reusing old/failed evidence, changing candidate scope, activating publication,
mutating NeoPRISM, publishing PRISM crates, and weakening the advisory gate are
rejected or deferred as recorded in the candidate table.

## Open questions and blockers

No research blocker remains. Completion is conditional on a fresh complete
slow run and independent approval; those are delivery gates, not facts to
predeclare.

## Evidence commands

Planning runs factory doctor, research/constraint readiness, strict OpenSpec
validation, and exact issue preflight. Implementation runs the report mutation
tests, DID candidate checks, source-distribution checks, site build/link checks,
factory/OpenSpec gates, independent CLI review, and protected exact-head CI.
The merged revision then receives the complete hosted slow run.
