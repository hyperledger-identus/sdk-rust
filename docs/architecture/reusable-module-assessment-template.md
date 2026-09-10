# Reusable module assessment

Use one copy of this template in the focused issue or OpenSpec research record
for every consumer/donor candidate. ADR 0110 defines the normative meanings.
A candidate qualifies for `extract` only when every hard gate passes; do not
calculate a compensating score.

## Identity and ownership

- Candidate:
- Named SDK capability:
- Proposed owner crate and layer:
- Primary responsibility:
- Shared invariant:
- Dominant reason to change:
- Independent consumers or credible usage paths:
- Donor repository and exact SHA:
- Donor paths and relevant file history:
- License and file/fixture provenance:
- Normative source and exact version/date:

## Hard gates

Use `pass`, `fail`, or `blocked`; attach evidence for every result.

| Gate | Result | Evidence |
| --- | --- | --- |
| Generic SDK ownership; no chain/product policy |  |  |
| One cohesive responsibility and change axis |  |  |
| Orthogonal minimal feature surface |  |  |
| Downward-only dependencies; no cycle/consumer edge |  |  |
| Identus-owned public types, states, errors, and bounds |  |  |
| Pure core and separately selectable effect adapters |  |  |
| Bounded untrusted inputs and explicit trust states |  |  |
| Redacted/zeroized secret ownership |  |  |
| Effective compiler, target, unsafe, and native posture |  |  |
| Direct and resolved minimal/enabled dependency cones |  |  |
| Supply-chain, maintenance, and release posture |  |  |
| Exact provenance and normative/conformance mapping |  |  |
| Two consumer-shaped proofs or foundational exception |  |  |
| Independent versioning, migration, and rollback |  |  |

Any `fail` or `blocked` result prevents `extract` in the current slice.

## Qualitative boundary review

- Why is this the smallest independently useful capability?
- Which types share an invariant and must change/test/release together?
- Which optional behavior has a different change axis and belongs in an
  adapter crate or feature?
- What unrelated packages disappear from the minimal consumer graph?
- Why is this not a generic helper, donor compatibility, or umbrella crate?

## Disposition

- Source disposition: `extract` | `adapt` | `conformance-only` |
  `remain-downstream` | `reject`
- Reason:
- SDK delivery action:
- Objective reconsideration trigger:

## Compatibility and evidence plan

- Public API compatibility:
- Wire/persistence compatibility:
- Historical fixtures and expected differences:
- Minimal and enabled feature commands:
- Host and target commands:
- Security/conformance/supply-chain commands:
- Commands intentionally not run and why:
- Consumer-shaped proof A:
- Consumer-shaped proof B or foundational exception:

## Two-repository delivery receipt

- sdk-rust issue/OpenSpec/PR:
- Immutable merged sdk-rust revision:
- Separately authorized downstream issue/PR:
- Exact downstream dependency pin:
- Compatibility facade retained or removed:
- Duplicate implementation eligible for deletion:
- Rollback revision and procedure:
- Unresolved blockers:
