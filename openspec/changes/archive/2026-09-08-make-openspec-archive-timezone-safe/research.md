# Timezone-safe archive receipt research

Research class: routine
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

The current `archive_change` function computes one expected destination from
the host-local `LC_ALL=C date +%F`, rejects that one path if it exists, invokes
OpenSpec, and requires that predicted path afterward. Issue #218 records direct
evidence that OpenSpec 1.2.0 successfully archived
`establish-crypto-performance-baseline` under `2026-09-08` while the host in
WITA had reached `2026-09-09`. The active change was removed, canonical specs
were updated, all mandatory artifacts survived, and validation passed, but the
wrapper returned a false failure.

The existing hermetic factory contract covers lossy canonical modification,
same-predicted-date collision, zero-exit no-op, incomplete archive and complete
archive outcomes. It does not separate the fake OpenSpec archive date from the
host date and therefore cannot reproduce the observed mismatch.

## Normative sources

- GitHub issue #218 is the durable defect and acceptance contract:
  https://github.com/hyperledger-identus/sdk-rust/issues/218
- The canonical `ai-software-factory` specification requires archive success
  to derive from repository state and preserve mandatory artifacts.
- `docs/factory/README.md` defines the guarded archive lifecycle.
- `docs/governance/agentic-sdlc.md` requires agents to use the guarded facade
  before final delivery.

No external protocol, dependency, draft, donor repository or standards text
governs this repository-local receipt. OpenSpec remains the mutator; the SDK
factory owns the success assertion.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Keep one host-local predicted path | `develop@ee7332d` | `not-adopt` | Reproduced evidence disproves the date-basis assumption. | None; the observation is definitive. |
| Predict OpenSpec's path with UTC | OpenSpec 1.2.0 observation | `not-adopt` | Fixes the observed timezone mismatch but retains a midnight rollover race and undocumented coupling. | OpenSpec exposes a stable machine destination contract. |
| Parse OpenSpec output for its path | OpenSpec 1.2.0 | `not-adopt` | Human output is not a stable machine contract. | OpenSpec adds structured output with a versioned schema. |
| Snapshot matching entries and require one new regular directory | Repository filesystem contract | `adopt` | Directly proves identity and novelty across timezone mismatch and date rollover without parsing prose. | Legitimate concurrent archive mutation in one worktree becomes supported. |
| Accept any matching archive after the call | Repository filesystem contract | `not-adopt` | A pre-existing archive could be mistaken for completion after a no-op or collision. | None. |

## Compatibility and dependency evidence

The implementation uses Bash 3.2-compatible indexed arrays, quoted filesystem
paths, portable `find` predicates already available on macOS and Linux, and the
existing fixed `YYYY-MM-DD-<change>` archive naming shape. It adds no Cargo,
Nix, native, unsafe, network or FFI dependency. Before/after membership is
compared by exact path; an older archive with the same suffix is not accepted
as the new result.

Public and wire compatibility are not applicable because this is a private
repository tool. The existing command syntax and success marker remain stable.
Rollback restores the predicted-path implementation but also restores the
known false-failure defect.

## Security, privacy and maintenance evidence

The active change name has already passed OpenSpec's kebab-case contract and
all expansions remain quoted. The new receipt accepts neither symlinks nor
multiple new matching entries, so ambiguity and path substitution fail closed.
It retains active-change removal, mandatory regular artifacts, capability
directory, canonical preservation preflight and final whole-store validation.
No secret-bearing value, external input parser, serialization surface or
consumer data is introduced.

Maintenance remains bounded to the repository's archive filename contract.
The deterministic fake OpenSpec date override covers the regression without
clock mutation or timezone assumptions.

## Rejected or deferred candidates

UTC prediction, output parsing and loose post-call lookup are rejected in the
candidate matrix. Transactional rollback of arbitrary partial OpenSpec
mutation remains deferred because this receipt can detect but cannot safely
reverse canonical changes; such evidence must stay visible for a separately
specified recovery design. Concurrent mutation of one worktree is unsupported
and deliberately fails on an ambiguous multiple-new-entry result.

## Open questions and blockers

None. Reconsider if OpenSpec publishes a stable structured destination receipt,
the archive naming contract changes, or one worktree must support concurrent
archive commands.

## Evidence commands

Research inspected `scripts/factory`, `scripts/tests/factory-contract.sh`,
`scripts/check-openspec-archive.py`, the canonical factory specification, issue
#218 and factory/governance documentation at `develop@ee7332d`.

Implementation evidence will run the hermetic factory contract, shell lint,
strict OpenSpec validation, repository factory checks and full local Nix
checks. Hosted Ubuntu fast CI is unrun at research readiness and remains the
merge gate.
