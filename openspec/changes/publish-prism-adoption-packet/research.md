# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

At protected `develop` revision
`c763c3bd0020f9c726eb08f390629a1b353e5e01`, the three portable PRISM crates
are present and issues #521 and #522 are closed. The crate graph is
`identus-prism-indexer -> identus-prism-vdr -> identus-prism-did -> generic
DID/crypto`. `crates/prism-indexer/tests/indexing.rs` proves the core behavior,
and `docs/architecture/prism-vdr-indexer-quality-baseline.md` records target,
benchmark, property, fuzz, and slow-lane evidence. However, the mdBook has no
PRISM pages or diagram and IDR-040 remains `specified` under closed issue #493.

NeoPRISM PR #324 at `ffaeafe9af35dc420c446b5f99d708cd2f9fe9db`
has green checks but is intentionally open and behind `main`. Its independent
parity harness PR #327 at
`dd356b34a920034086c22632d081983a4804bef1` is also green and open. Issue #325
records two final-candidate Preprod runs at fixed slot 10,723,321 whose
baseline/candidate canonical manifests are byte-identical with SHA-256
`1b4fd7cbaaf46fcb23e3c9878facb8a28b1b861374177b866fbb6d88bd39419f`.
These are downstream compatibility evidence, not SDK-owned effects or
permission to mutate NeoPRISM.

## Normative sources

- ADR 0174 defines peer/PRISM ownership and dependency direction.
- ADR 0175 defines bounded generic DID handling for PRISM long form.
- ADR 0176 defines the in-tree portable PRISM method package.
- ADR 0177 defines the synchronous atomic VDR/indexer core.
- Archived changes `add-portable-prism-did-method` and
  `add-prism-vdr-indexing` are the implementation contracts.
- The pinned PRISM specification and NeoPRISM donor revisions in those
  contracts remain normative/compatibility sources; this documentation slice
  does not reinterpret them.
- NeoPRISM #324/#325/#327 provide immutable consumer-shaped and live Preprod
  evidence.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Keep only low-level tests and ADRs | `not-adopt` | Forces adopters to reconstruct the flow and ownership boundary. |
| Copy NeoPRISM service/runtime examples | `not-adopt` | Would import Cardano, SQL, HTTP, and deployment ownership into the SDK. |
| Add a production example with demo storage types | `not-adopt` | Risks presenting test adapters as supported runtime infrastructure. |
| Add one explicit in-memory E2E integration test | `adopt` | Executable in normal CI, readable, deterministic, and honest about effect ownership. |
| Add mdBook crate pages plus tracked DOT diagram | `adopt` | Reuses the existing reproducible documentation system and link checks. |
| Mark all of IDR-040 delivered | `not-adopt` | `did:key`, `did:jwk`, and `did:web` remain separately unplanned. |
| Mark delivered profiles explicitly and keep the portfolio in progress | `adopt` | Reflects PRISM/peer evidence without overstating the portfolio. |

## Compatibility and dependency evidence

The slice adds no production dependency, feature, public Rust API, wire or
persisted format. The E2E test uses the existing crate test dependency graph.
The site uses the established mdBook/Graphviz/Nix path. The roadmap change is
governance evidence only. Rollback removes the pages, diagram, test, and status
reconciliation without affecting consumers.

## Security, privacy and maintenance evidence

The E2E scenario uses deterministic non-production test keys, never prints
private key bytes, performs no network or filesystem effect, and exercises the
same bounded public constructors and ports as adopters. Documentation labels
test adapters, source-only crates, portability receipts, and downstream live
evidence precisely. Links prefer repository-owned durable sources and exact
GitHub evidence.

## Quality evidence

Required evidence is the focused E2E test, all-target compilation, documentation
build and offline link check, factory/OpenSpec validation, workspace/Nix gates,
and independent Claude-or-Codex CLI review. The existing quality baseline and
NeoPRISM live receipts are cited, not rerun or reclassified.

## Rejected or deferred candidates

Cardano adapters, SQL stores, HTTP services, a sample node, consumer mutation,
additional DID methods, publication, and hard QoS thresholds remain deferred
to their owning issues/repositories.

## Open questions and blockers

None. The open NeoPRISM PRs remain downstream work and do not block an honest
SDK adoption packet.

## Evidence commands

Planning uses GitHub issue/PR inspection, repository source inspection,
`scripts/factory doctor`, research/constraint readiness, strict OpenSpec
validation, and issue preflight. Implementation adds focused tests, docs-site
build/link validation, workspace gates, Nix checks, and CLI review.
