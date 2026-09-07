## Context

Issue #151 requested a crate-by-crate reuse decision before more SSI protocol
growth. During the assessment the sponsor also requested a durable negative
ledger, reconsideration of the fixed Rust 1.85 MSRV and a factory mechanism
that forces equivalent research before future greenfield specifications.

The existing factory validates OpenSpec structure and completed tasks but has
no machine-visible pre-implementation research state. The same agent may move
from an incomplete proposal directly to code.

## Goals / Non-Goals

**Goals:**

- record evidence-backed adopt/spike/oracle/retain/reject dispositions;
- keep external engines behind high-cohesion Identus facades;
- define a measurable near-current MSRV policy without silently changing the
  current operational promise;
- add a proportional, offline and client-neutral pre-implementation gate; and
- create independently executable follow-up issues.

**Non-Goals:**

- adding or updating a Rust runtime dependency;
- changing the effective Rust 1.85 support policy in this PR;
- implementing a protocol, binding, storage adapter or downstream adoption;
- claiming security audit, conformance, runtime target support or publication.

## Decisions

### Use a narrow-engine, owned-facade rule

Focused crates may implement closed algorithms and grammars. Identus retains
public types, limits, redaction, lifecycle/security states and injected policy
ports. This is more reversible than framework convergence and safer than
continuing demonstrably incomplete bespoke crypto.

### Record negative decisions separately

The not-adopted ledger makes rejection reasons and reconsideration triggers
discoverable to future agents. It avoids repeated research and avoids turning a
point-in-time rejection into undocumented dogma.

### Use stable minus three, reviewed rather than automatic

An 18-week release distance is near-current enough for modern crates and still
gives consumers a qualification window. Quarterly review is mandatory, but
Cargo/Nix policy changes require an explicit PR and target evidence. This
avoids basing compatibility on incomplete or inaccurate average MSRV metadata.

### Add `research.md` outside the OpenSpec schema

The pinned OpenSpec `spec-driven` schema does not model research. A repository-
owned adjacent artifact and offline checker add the gate without forking the
tool or weakening its structural contract. The checker validates minimum
shape; semantic review remains a planner/reviewer responsibility.

### Separate draft structure from implementation readiness

`scripts/factory check` accepts `Research status: draft` so agents can iterate.
`scripts/factory research-ready <change>` requires `ready` and no blockers.
The apply adapters invoke this command before implementation. Final `ready`
continues to prove completed tasks and the full factory contract.

## Risks / Trade-offs

- **Research becomes formulaic** → require evidence categories and semantic
  review, while explicitly stating that structural success is not approval.
- **Routine work gains overhead** → allow concise, justified
  `not-applicable` records under the routine class.
- **Generated OpenSpec adapters overwrite local additions** → keep the
  canonical rule in `AGENTS.md`, factory scripts and governance docs; adapter
  text is defense in depth.
- **Rolling MSRV surprises consumers** → require a focused PR, target matrix,
  release notes and time-bounded consumer exceptions.
- **Dependency types leak through convenience APIs** → integration issues
  require private dependencies, public API review and rollback evidence.
- **Point-in-time upstream evidence goes stale** → pin retrieval date and SHA
  and give every decision a reconsideration trigger.

## Migration Plan

1. Merge this architecture and factory change without runtime dependencies.
2. Execute correctness-first `bip39` and `bip32` issues independently.
3. Implement the Rust 1.95 transition issue and verify all supported surfaces.
4. Execute multiformat/URI/form integration issues and bounded spikes.
5. Refresh the portfolio when a reconsideration trigger or quarterly MSRV
   review occurs.

Rollback reverts the factory/ADR PR; no public SDK or downstream data changes.
Individual future dependency PRs remain separately reversible.

## Open Questions

None block this change. Paid ISO text, mobile storage runtime behavior and FFI
ABI design remain explicit blockers inside their focused spike issues rather
than unresolved assumptions here.
