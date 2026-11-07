# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-11-07
Source retrieval date: 2026-11-07
Research blockers: none

## Problem and existing implementation

The current implementation already contains a 24-capability SDK-TS inventory,
an A0-A9 adoption roadmap, versioned cross-language vectors, adapter/error
mapping records, a consumer-change ledger, and quality-evidence declarations.
The source inventory is pinned to SDK-TS `8.1.4` at revision
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`, but it has no separate parity
relation/conformance-state contract. The roadmap also reports A0 as
`in-progress` and A1 as `planned` even though their owner issues are closed.

The DID readiness implementation predates the final receipt and still reports
the exact slow line as pending. The backlog similarly understates DID Core and
port delivery after the named consumer adoption merged.

## Normative sources

The normative authority remains capability-specific: selected W3C, IETF,
OpenID, DIF, and Identus profiles; accepted SDK-Rust ADRs; versioned Identus
compatibility mappings; and named consumer contracts. SDK-TS implementation
tests remain donor or consumer evidence according to their declared authority.
The controlling program sources are ADRs 0162, 0163, 0164, 0169, and 0170 and
the canonical `identus-platform-core-migration` specification.

Primary repository sources are:

- https://github.com/hyperledger-identus/sdk-rust
- https://github.com/hyperledger-identus/sdk-ts
- https://github.com/hyperledger-identus/neoprism

## Current immutable evidence

SDK-Rust protected `develop@c429ac494a6eaeb2e22c0b38a5b1d70c0eb6ff62`
passed slow run 40888621367 on attempt 1. All eleven hosted jobs passed; the
four DID candidate matrix lanes were clean and shared the frozen staged lock.
The run and receipt are bound in issue #388.

NeoPRISM PR #324 pins that exact SDK revision and merged into `main` as
`41b7f1991a2665c3cb9d0d994d861f9a32f07727`. PR #327 supplies the merged
fixed-boundary Preprod parity harness and its final candidate receipt reports
byte-identical manifests, zero pending/missing/extra/duplicate/changed
operations, and no container restarts. PR #346 records the retained boundary:
SDK-Rust owns generic crypto, DID, resolution, and HTTP behavior; NeoPRISM owns
PRISM/Cardano effects, persistence, service composition, and runtime policy.

The current SDK-TS discovery baseline remains release
`@hyperledger/identus-sdk@8.1.4` at immutable `main@4bf86ebf69d5e96616a148e4c973f831f95fa38e`.
The live default branch still resolves to that revision on 2026-11-07, so the
existing 24-capability source inventory is current and does not require a
speculative donor refresh.

## Problem with a single parity concept

Six distinct relationships are needed:

| Relation | Meaning |
| --- | --- |
| `semantic-equivalent` | Both surfaces must produce the same bounded domain/protocol result for the selected profile. |
| `compatible-adapter` | TypeScript may expose different names, DTOs, errors, async conventions, or host effects while mapping to the canonical Rust contract. |
| `platform-owned` | The responsibility is inherently TypeScript/browser/Node/npm integration; only its port boundary is shared. |
| `intentionally-different` | SDK-Rust follows a newer or safer selected profile and compatibility is explicit rather than regressing Rust to donor behavior. |
| `deferred` | Product/specification/engine evidence is insufficient to commit to parity. |
| `retired` | The donor capability has no continuing target and requires an explicit removal path. |

A relation is not a delivery state. A semantically equivalent target may still
be blocked, while a platform-owned item can be complete without Rust source.
Conformance therefore uses a separate state vocabulary: `delivered`, `partial`,
`blocked`, `excluded`, and `not-applicable`.

## Conformance evidence

Each capability needs the evidence that applies to its relation and risk:

- exact standard/profile and Identus policy decision;
- immutable Rust and SDK-TS revisions;
- shared normative, Identus-contract, and named-consumer vectors;
- DTO/error mapping and known intentional deviations;
- property, fuzz, benchmark, and differential dispositions under #501;
- target/runtime/package evidence only for selected targets;
- one named consumer or cross-SDK journey;
- observability, fallback, rollback, and deprecation state.

Platform-owned capabilities do not need fake Rust implementations. They need a
bounded port contract, security/resource ownership, and proof that TypeScript
does not duplicate portable semantics. Intentionally different capabilities
need an explicit profile migration and compatibility adapter or a documented
break, not donor-vector equality.

## Compatibility and dependency evidence

This change adopts no production dependency and changes no direct or resolved
dependency cone. SDK-TS remains exactly version 8.1.4 with its existing
features, package graph, public API, wire formats, and npm distribution. The
SDK-Rust MSRV, primary compiler, native targets, WASM, iOS, Android, and host
support claims are unchanged. No unsafe or native code is added and existing
supply-chain, license, provenance, advisory, and source-integrity evidence
continues to apply to each capability's owning implementation slice.

Public and wire compatibility is recorded rather than changed. The facade
boundary remains canonical Rust semantics below versioned TypeScript adapters
and platform effects. The matrix will identify where semantic equivalence is
required and where DTO, error, async, packaging, or host behavior is allowed to
differ.

## Security, privacy and maintenance evidence

The matrix contains only public capability metadata and immutable repository
identities. It must not store secrets, credentials, private issue bodies,
consumer data, or raw provider responses. Bounded diagnostics identify record
IDs and missing fields without printing external payloads.

Maintenance, release, and security posture remain capability-owned. A parity
classification cannot waive an advisory, unsafe/native review, resource bound,
redaction rule, target gate, or protected release decision. Protocol or draft
currency is pinned by the owning capability; an intentionally different record
must name the exact profile and migration rather than silently following SDK-TS.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| One SDK-wide parity percentage | `not-adopt` | Hides excluded/platform-owned work and makes blocked breadth look like delivered quality. |
| Identical Rust and TypeScript APIs | `not-adopt` | Imports language/runtime accidents into the canonical core. |
| Historical Apollo/Castor/Pollux/Mercury/Pluto module parity | `not-adopt` | Donor packaging is evidence, not target architecture. |
| Capability relation plus independent conformance state | `adopt` | Expresses the target and current evidence without overstating sameness. |
| Capability-by-capability activation after A1 | `adopt` | Lets DID, crypto, credentials, and later protocols progress independently and reversibly. |
| Small PR per mechanical step | `not-adopt` | Multiplies hosted-CI latency and leaves capability evidence fragmented across partial integration states. |
| Local-first 5,000-10,000 meaningful-line capability trains | `adopt` | Keeps implementation, adapters, vectors, migration, docs, and tests in one coherent rollback unit while signed checkpoints preserve reviewability. |
| Begin mdoc as the next feature | `not-adopt` | Sponsor direction prioritizes the SDK-Rust/SDK-TS roadmap and milestone definition. |

## Rejected or deferred candidates

Repository-wide coverage parity, line-count parity, historical-module parity,
and identical public API shapes are rejected. SDK-TS mutation, binding default
activation, mdoc, DIDComm/AnonCreds engine adoption, production JWE, and package
publication are deferred to their owning issues. No new crate or engine is a
candidate in this change, so license, dependency, target, unsafe/native, and
supply-chain qualification remain with those later implementation slices.

The train-size target is not line-count parity and is not an incentive to pad
diffs. Generated code, vendored sources, lockfiles, formatted evidence, and
other mechanical churn are excluded. A smaller PR remains correct when the
capability is intrinsically bounded or needs separate authority/rollback; a
larger PR requires an explicit cohesion and reviewability justification.

Reconsideration trigger: revise the relation vocabulary only when a real
capability cannot be represented without conflating target ownership and
delivery state, and revise a capability record only through reviewed immutable
profile, consumer, or implementation evidence.

## Open questions and blockers

There are no blockers to the planning/reconciliation change. Individual
capabilities retain their own blockers: production JWE #566 is upstream-
blocked; AnonCreds and DIDComm need engine qualification; platform adapters
need target decisions; and the first SDK-TS mutation remains #492.

## Sources

- SDK-Rust issues #388, #381, #5, #10, #20, #492, #502, and #594
- SDK-Rust ADRs 0162, 0163, 0164, 0169, and 0170
- NeoPRISM PRs #324, #327, #346, #348, and #350
- SDK-TS `main@4bf86ebf69d5e96616a148e4c973f831f95fa38e`
- Existing SDK-TS capability inventory and A0-A9 roadmap

## Evidence commands

Research used read-only GitHub issue, pull-request, release, milestone, branch,
and revision inspection plus repository-local inventory and validator review.
No workflow was dispatched or rerun and no consumer repository was mutated.
Exact planning commands include `gh repo view`, `gh pr view`, `gh issue view`,
`gh api repos/.../milestones`, `git rev-parse origin/develop`,
`scripts/check-platform-ts-capabilities.py`, and the factory research and
constraint readiness commands. Unrun checks at planning time are the new parity
validator/mutations, full factory suite, docs build, exact-diff review, hosted
CI, SDK-TS canary, consumer E2E, and any release or publication operation.
