# ADR 0110: define reusable-module extraction criteria

- **Status:** Accepted for implementation
- **Date:** 2026-09-10
- **Decision authority:** sponsor-directed issue
  [#253](https://github.com/hyperledger-identus/sdk-rust/issues/253)
- **Downstream authorization:** NeoPRISM issue
  [#321](https://github.com/hyperledger-identus/neoprism/issues/321)
- **Related work:** ADR 0061, SDK issues #5, #9, #10 and #20, and discussion
  [#252](https://github.com/hyperledger-identus/sdk-rust/discussions/252)
- **Assessed revisions:** sdk-rust
  `78e0656f860c0e569df2edb25d087c42561c51a5`; NeoPRISM main
  `d4608fe3d662ebaf425c4a6380f6d241cffe74c3`; NeoPRISM beta
  `faead035915b1f51bec20f76fa17a9c404b71acd`

## Context

NeoPRISM contains Rust implementations and fixtures that helped shape the SDK,
while sdk-rust now contains generic crypto, DID, and DID Resolution HTTP
components that NeoPRISM can consume. The next milestone must move knowledge in
both directions without making sdk-rust depend on a consumer, copying donor
package boundaries, or deleting NeoPRISM behavior before its replacement is
proven.

The existing source matrix classifies repositories and broad surfaces, but the
word "reusable" remains underspecified. A source file can look generic while
still carrying a donor's public types, runtime, release cycle, error behavior,
unbounded inputs, or method-specific policy. A weighted score would let a
strong property compensate for a failed trust or ownership boundary.

The existing NeoPRISM `codex/sdk-rust-beta` branch is a useful architecture
spike: it keeps `identus-apollo` as a compatibility facade and replaces direct
crypto implementations with `identus-crypto` pinned to sdk-rust
`9ff2fa87337d3f25d67f38486c3a4d0a656bd9d6`. It removes more source than it
adds, but it has no hosted CI evidence and its SDK pin predates current
`develop`. It proves feasibility, not adoption completion.

## Decision

### Reusability is a set of hard gates

A candidate qualifies for `extract` only when every gate below passes. A
failure cannot be offset by a score, popularity, file size, or another
strength.

| Gate | Required evidence |
| --- | --- |
| Ownership | The candidate owns a named standards- or domain-defined capability used beyond one chain or product. PRISM/Cardano/Midnight operations, product policy, custody, consent, trust, UI, deployment, and concrete product storage remain downstream. |
| Cohesion | It has one primary responsibility, one dominant reason to change, and a shared invariant. It is independently useful and testable; it is not a `utils`, `common`, or umbrella dumping ground. |
| Orthogonality | Consumers can select the capability without unrelated protocols, runtimes, chains, or storage. A split must reduce coupling; types that share an invariant and change, test, and release together may remain together. |
| Dependency direction | It depends only on lower generic SDK layers, minimal ports, and separately accepted narrow engines. No cycle, upward edge, consumer repository, moving Git reference, or cross-repository path dependency is allowed. |
| Public boundary | Identus-owned types, stable redacted errors, lifecycle states, input limits, and secret ownership define public Rust, wire, WASM, UniFFI, and persistence boundaries. Donor and dependency types do not leak without a separate public-API ADR. |
| Effect isolation | Deterministic core behavior does not own network, filesystem, database, executor, clock, or entropy implementations. Effects enter through minimal ports; optional concrete adapters live in separately selectable crates. |
| Safety and trust | Untrusted bytes, elements, nesting, expansion, work, and time are bounded before expensive work where possible. Parsed, validated, verified, and trusted states remain distinct. Secrets are redacted and zeroized or represented by handles. Authored unsafe Rust requires a dedicated safety ADR. |
| Portability and cone | Minimal and enabled features, direct and resolved dependency cones, MSRV/effective compiler policy, targets, reachable unsafe/native code, and supply-chain evidence are explicit. A host build alone is insufficient. |
| Provenance and conformance | Donor repository, exact SHA, paths, file history, license, transformation, normative versions, and fixture/vector provenance are recorded. Normative standards outrank donor behavior; compatibility differences are explicit. |
| Consumer evidence | Two independent consumer-shaped uses exercise only the public minimal surface. A foundational standards primitive may document an exception naming two credible usage paths; neither form is represented as completed downstream adoption. |
| Release independence | The module is independently versionable, reviewable, and reversible. Its compatibility and migration boundary is explicit, and a downstream can pin one immutable SDK candidate without consuming unrelated work. |

### Candidate boundaries follow capabilities, not donor files

The extraction unit is the smallest independently useful capability that owns
a coherent invariant and change axis. NeoPRISM crate, module, and file
boundaries are evidence, not the target architecture. Small generic-looking
helpers such as pagination, uniqueness, or source locations stay local or use
the standard library unless a named SDK capability and independent consumer
contract justify them.

### Every candidate receives one source disposition

Each exact-revision assessment assigns one current disposition:

- `extract`: a coherent generic Rust source unit passes every hard gate;
- `adapt`: reusable behavior exists, but its API, dependencies, effects, or
  lifecycle states must be redesigned at the SDK boundary;
- `conformance-only`: behavior or fixtures provide compatibility evidence, but
  the source or public model is not reusable;
- `remain-downstream`: chain, ledger, node, runtime, concrete storage, product,
  custody, consent, trust, UI, or deployment ownership stays in NeoPRISM; or
- `reject`: no cohesive SDK responsibility or the provenance, license,
  security, maintenance, dependency, or consumer-value gate fails.

The source disposition is not the delivery action. If sdk-rust already owns an
equivalent capability, the action is comparison and downstream adoption, not a
second SDK implementation.

### The assessment record is mandatory

Before implementation, a candidate issue and OpenSpec research record use the
[reusable-module assessment template](../architecture/reusable-module-assessment-template.md)
and contain:

1. donor repository, exact revision, paths, file history, license, and
   transformation;
2. named SDK capability, owner crate/layer, primary responsibility, invariant,
   and change axis;
3. pass/fail evidence for every hard gate and the resulting disposition;
4. minimal and enabled dependency/feature graphs and public API ownership;
5. effect, policy, target, bounds, error, secret, trust-state, unsafe, native,
   and supply-chain evidence;
6. normative-source and donor fixture/vector mapping;
7. two independent consumer-shaped proofs or the foundational exception;
8. public/wire compatibility, migration, immutable pin, rollback, and exact
   downstream deletion conditions; and
9. commands passed, commands not run, and blockers.

### Delivery is SDK-first and downstream-second

For behavior missing from sdk-rust:

1. inventory and classify the candidate at an immutable NeoPRISM revision;
2. open one focused sdk-rust issue and complete its OpenSpec contract;
3. implement the SDK-owned API, record file/fixture provenance, and pass its
   component, conformance, security, dependency, and target evidence;
4. merge the reviewed issue-linked PR to `develop`, producing an immutable SDK
   revision; then
5. use a separately authorized NeoPRISM issue to pin that revision, adapt one
   surface, pass historical fixtures and relevant workspace gates, and remove
   only the duplicate implementation covered by the receipt.

An unmerged SDK branch, moving branch reference, or local path cannot authorize
downstream deletion. The NeoPRISM compatibility facade remains until the
pinned replacement is proven and preserves an explicit rollback path.

## Initial NeoPRISM disposition

| Surface | Disposition and immediate milestone action |
| --- | --- |
| `lib/apollo` | `adapt`; refresh the existing thin facade against current `identus-crypto`, retain only evidenced compatibility DTOs, and extract only a demonstrated SDK gap or reusable fixture |
| `lib/did-core` | `adapt`; produce a compile- and fixture-driven map to `identus-did` before proposing any source movement |
| `lib/did-resolver-http` | Existing source classification remains `extract`, but the action is comparison with the already implemented `identus-did-resolver-http`; port only missing normative behavior or fixtures |
| `lib/did-prism` | Method/operation/protobuf behavior remains downstream; a standard-neutral primitive requires its own passing assessment |
| PRISM indexer, ledger, submitter, `lib/node-storage`, and node binaries | `remain-downstream` because they own Cardano integration, node orchestration, concrete databases, migrations, executors, or deployment |
| Historical generic tests/vectors | `conformance-only` with exact provenance unless their implementation independently passes all gates |
| Generic-looking helpers without a named capability | `reject` as standalone SDK modules |

## Consequences

- sdk-rust grows by cohesive capabilities, not by donor directory structure.
- NeoPRISM can migrate incrementally through thin compatibility facades and
  immutable SDK pins.
- A candidate may require more design work even when its algorithm is correct,
  because public ownership, bounds, targets, and dependency shape are part of
  reusability.
- Some duplicated code remains temporarily by design until downstream evidence
  passes; this is a safer migration boundary rather than incomplete extraction.
- The criteria can reject trivial generic helpers and large framework slices
  without preventing their fixtures from serving as conformance evidence.
- This ADR does not publish crates, create a release promise, deprecate Apollo,
  or prove any downstream adoption.

## Alternatives rejected

### Copy every generic-looking NeoPRISM module

This preserves accidental donor APIs and turns source organization into SDK
architecture. It also imports hidden runtime, policy, and maintenance coupling.

### Use a weighted reusability score

A score can hide a failed security, ownership, or dependency-direction gate.
Mandatory gates keep non-negotiable boundaries visible.

### Build one shared NeoPRISM compatibility crate in sdk-rust

An umbrella named after the donor would move coupling upstream and give
unrelated capabilities one release cycle. Thin compatibility belongs in the
consumer; reusable capabilities belong in their domain crates.

### Perform an atomic cross-repository migration

Git cannot give two repositories atomic review or rollback. Independent SDK
delivery followed by a pinned consumer PR makes failure and ownership clear.

## Verification and rollback

This decision changes documentation and the canonical architecture constraint,
not executable code. Factory, OpenSpec, constraint, source-link, text, and
exact-diff gates verify its repository shape. A distinct semantic review checks
that every criterion is testable and consistent with ADR 0061 and the SDK
layering rules.

Revert this ADR and its constraint/spec changes to roll back the architecture
policy. Future extraction and NeoPRISM adoption PRs remain independently
revertible and must state their own rollback. Reversion never requires deleting
or rewriting donor history.
