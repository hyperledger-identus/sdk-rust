# SDK constraints and limitations

This guide is the human entry point for constraints that materially shape the
SDK. The machine-readable index is
[`sdk-constraints.toml`](sdk-constraints.toml); detailed values continue to be
owned by their linked ADR, specification or policy file.

## Vocabulary

A **constraint** says what the SDK must, may or must not do. A **limitation**
says what the SDK does not currently support, test, certify or promise. A
limitation is not permission to fill the gap inside an unrelated change.

Kinds and lifecycle state answer different questions:

| Kind | Meaning |
| --- | --- |
| `hard` | Boundary that cannot be waived by routine implementation work |
| `guardrail` | Default architecture or delivery rule changed only by an explicit decision |
| `budget` | Quantified compatibility, performance, resource or time boundary |
| `limitation` | Unsupported, unverified, deferred or deliberately excluded surface |

| State | Meaning |
| --- | --- |
| `effective` | Current enforced promise or known limitation |
| `target` | Researched future intent with no current compatibility promise |
| `deferred` | Not selected for current delivery; no activation is implied |
| `prohibited` | Explicitly forbidden within the stated scope |

The words **target**, **planned**, **candidate** and **quarterly review** never
mean effective automatically.

## What is material

Use `material` when a change alters any of these outcomes:

- minimum compiler, supported target, feature surface or dependency boundary;
- public API, wire/data shape, persisted data or migration promise;
- product/chain ownership, trust boundary or consumer responsibility;
- security, privacy, cryptographic, unsafe-code or secret-handling posture;
- license/provenance obligation or certification/compliance claim;
- enforced performance, size, resource or availability budget;
- crate publication, release/LTS policy or promotion to `main`;
- irreversible external action or data-loss risk.

Use `routine` for reversible repository-local choices that do not change those
outcomes. Use `none` when the change only observes or documents facts without
changing a constraint or limitation. If evidence shows the classification is
wrong, semantic review corrects it before merge.

## Authority without ceremony

| Situation | Required action |
| --- | --- |
| Routine reversible choice | Record useful reasoning and continue under standing authority |
| Material outcome already stated exactly in an effective index entry or roadmap decision | Cite that decision in `constraints.md` and continue |
| New or ambiguous material outcome | Record it as `proposed`; research and reversible preparation may continue, but activation waits for sponsor/responsible-maintainer direction |
| Governance, release, publication, legal-risk acceptance, secrets, private disclosure or protected settings | Follow the protected human authority path |

This is an intent gate, not a document-format approval. Once the exact product
decision is recorded, agents may implement, review and merge the bounded change
through the normal CI-gated `develop` flow.

## Change artifact

Every active OpenSpec change contains `constraints.md` with:

```text
Impact class: none | routine | material
Decision status: not-required | proposed | directed
Decision reference: not-required | exact durable URL
Constraint blockers: none | explicit blockers
```

It also contains substantive sections for affected entries, introduced or
changed constraints, introduced or changed limitations, consumer/product
impact, activation/rollback and evidence.

For a material record:

- `proposed` is valid planning state but fails readiness;
- `directed` names the exact durable GitHub issue/Discussion or an already
  effective constraint ID that resolves the outcome;
- `Constraint blockers` must be `none` before implementation and integration.

## Lifecycle

```text
observed need
     │
     ▼
proposed target/limitation ──► evidence and consumer impact
     │                                      │
     │ unresolved                           ▼ exact direction
     └────────────── stop activation ◄── directed decision
                                                │
                                                ▼
                                  implementation + enforcement
                                                │
                                                ▼
                                    effective source updated
```

Activation requires all of the following:

1. focused issue and exact decision reference;
2. material `constraints.md` record;
3. affected consumers and migration stated;
4. canonical source and enforcement changed together;
5. rollback and exception behavior defined;
6. local review, required specialist review and green CI.

## Exceptions

An exception does not edit history or make the original rule disappear. It
records:

- base constraint ID;
- exact crate, target, consumer or time-bounded scope;
- owner and rationale;
- exit/review trigger;
- security and maintenance cost;
- evidence proving why the base rule cannot currently be met.

An expired or scope-less exception is a blocker, not tacit permission.

## Compiler-floor example

- `SDK-COMPAT-002` is effective: Rust 1.98.1 is the temporary workspace floor,
  enforced by Cargo, Nix and the support-policy checker.
- `SDK-COMPAT-003` is deferred: the release-candidate compiler matrix is not
  selected during unpublished active development.
- `SDK-COMPAT-004` is effective: Rust 1.98.1 is the primary stable validation
  compiler.
- `SDK-COMPAT-005` is effective: Rust 1.98.1 is also the temporary
  compatibility etalon; the nightly is isolated to sanitizer tooling.
- A release candidate or the 2026-12-08 review must use named consumer and
  target evidence to select a new matrix. The temporary policy cannot become a
  publication promise merely through elapsed time.

This distinction preserves useful research without converting it into a
surprise compatibility promise.

## Bounded-input example

- `SDK-SEC-003` is an effective forward guardrail: every new or materially
  changed untrusted-input boundary must ship with explicit resource limits.
- `SDK-LIM-007` is an effective current limitation: inherited surfaces have
  not completed a repository-wide resource-bound audit. `identus-core::Url`
  now rejects values above 8,192 UTF-8 bytes; generic crypto text rejects
  values above 4,096 bytes; and hierarchical derivation is limited to 4,096
  path bytes, 255 axes/depth, and 16–64-byte BIP-32/SLIP-0010 seeds. Callers
  must still apply an outer limit before a transport, decompressor, or
  deserializer allocates hostile input. Programmatic path construction remains
  caller-budgeted, while every current cryptographic path consumer rejects
  more than 255 axes before child work. Other inherited surfaces remain
  limited by their explicit evidence rather than by a repository-wide
  completion claim.

This pairing keeps the intended security direction enforceable without
misrepresenting incomplete inherited coverage as a proven SDK guarantee.

## Unsafe-code example

- `SDK-SEC-001` is effective project policy: unsafe Rust is prohibited unless
  a dedicated safety ADR grants a bounded exception. ADR 0087 backs that policy
  with an inherited workspace `forbid`, a member-inheritance guard and
  compile-fail probes across supported authored first-party Cargo targets. ADR
  0088 additionally makes `identus-derive` reject enumerated unsafe constructs
  and attributes in its completed direct item output before emission.
- `SDK-LIM-008` remains as a narrow compiler limitation: Rust 1.98.1 skips the
  `unsafe_code` lint for procedural-macro expansion spans that allow internal
  unsafe. Direct `identus-derive` item syntax is guarded, but external macros
  and syntax created later by nested macro expansion remain outside that check.
- The assurance is deliberately first-party: external dependency unsafe/native
  posture remains dependency-decision evidence, and the lint does not claim to
  prove logical or side-channel safety.

Any exception must be indexed separately with exact scope, invariants, owner,
specialist review, cost, exit trigger and rollback; it cannot silently weaken
the visible base rule.

## Dependency-remediation example

- `SDK-DEP-001` is effective: a focused defect in a conditionally adopted
  dependency is offered upstream before the SDK activates a maintained fork.
- An upstream issue or pull request is evidence of coordination, not an
  immutable dependency or completed remediation. The SDK remains on its
  reviewed release behind an Identus facade until a separate update issue
  validates a new released artifact.
- If upstream declines or misses an already accepted release trigger, a fork
  remains possible through a bounded decision with an exact base, owner,
  rollback and sunset condition back to upstream.

This keeps reusable fixes in the ecosystem without making upstream response
time a permanent delivery blocker or turning unpublished branches into SDK
dependencies.
