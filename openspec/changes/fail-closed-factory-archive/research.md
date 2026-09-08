# Factory archive postcondition research

Research class: routine
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current `archive_change` function performs readiness and preservation
preflights, calls `openspec archive <change> --yes`, runs the broad factory
check, and then prints a success message. It does not verify that OpenSpec
removed the active change or created an archive. During the completed
`bound-core-url-input` lifecycle, OpenSpec detected already-synchronized
requirements, printed that no files changed, left the active directory in
place, and nevertheless returned zero. The wrapper then printed
`factory: change archived safely`.

The existing hermetic `factory-contract.sh` fixture proves that a lossy
modified requirement fails before OpenSpec is invoked. Its fake `openspec`
returns zero without mutation, but no preflight-safe test exercises the
postcondition. `check-openspec-archive.py` intentionally validates preservation
intent before mutation; it is not a state-transition checker.

## Normative sources

- The canonical `ai-software-factory` specification requires guarded archive
  preflight and validation of post-archive state.
- `docs/factory/README.md` describes archive as readiness, preservation,
  pinned OpenSpec mutation, then validation of the resulting store.
- `docs/governance/agentic-sdlc.md` makes the wrapper the required autonomous
  archive path.
- POSIX process exit status is evidence only of the child command's chosen
  status; application-specific completion requires application state checks.

No external protocol, dependency, draft or donor repository is authoritative
for this internal shell-facade contract. The pinned OpenSpec implementation is
the mutator, while the repository specification owns the success condition.

## Candidate decisions

1. Trust OpenSpec exit zero: `not-adopt`, because the reproduced command can
   return zero without the required state transition.
2. Parse OpenSpec human-readable output: `not-adopt`, because wording, color,
   localization and version changes are not a stable machine contract.
3. Verify the deterministic active and dated archive paths: `adopt`, because
   this directly observes the repository state promised by the wrapper.
4. Discover any new archive directory by before/after enumeration:
   `not-adopt`, because it admits an unrelated concurrent archive and weakens
   identity; the pinned command's current contract uses the local ISO date and
   change name.

## Compatibility and dependency evidence

The proposed check uses Bash 3.2-compatible conditionals, `date +%F`, and
ordinary filesystem predicates already available on supported developer and CI
hosts. No array maps, GNU-only flags, network, Cargo package, Nix input, native
code or FFI are added. The expected destination is captured before invoking
OpenSpec as `openspec/changes/archive/YYYY-MM-DD-<change>` and a pre-existing
destination fails before mutation.

The success path retains the existing command syntax and message. The behavior
change is limited to false-success and collision cases, which become nonzero
failures. A run spanning local midnight may fail conservatively if OpenSpec
chooses a different date after the destination was captured; retrying is safe
because the wrapper will inspect the new day's deterministic destination.

## Security, privacy and maintenance evidence

The change processes a change name already constrained by active-directory
existence and by the OpenSpec preservation checker. All paths are quoted. It
does not evaluate OpenSpec output or introduce secret-bearing input. The
postcondition checks mandatory artifacts as regular files and the capability
delta as a directory, preventing an incomplete move from receiving a success
receipt. The final general factory check remains necessary to validate the
canonical and archive stores semantically.

Hermetic tests will make the fake OpenSpec implement both zero-exit/no-op and a
real move. They will prove nonzero/no-success output and intact active state in
the no-op case, pre-mutation rejection on destination collision, and complete
artifact preservation plus success output for the real transition.

## Rejected or deferred candidates

Output parsing and loose archive discovery are rejected above. Adding a new
OpenSpec wrapper dependency or replacing the upstream archive operation is
deferred because a narrow fail-closed postcondition fixes the observed defect.
Transactional rollback of partial upstream mutation is also deferred: this
change detects and stops on incomplete state but cannot safely infer how to
reverse arbitrary canonical changes. Such a failure remains visible for manual
or separately specified recovery.

## Open questions and blockers

No blocker remains. The finite reconsideration triggers are an OpenSpec archive
naming-contract change, support for a host without ISO-date `date`, evidence of
legitimate concurrent archive operations in one worktree, or a partial-mutation
failure that warrants a transaction/recovery design.

## Evidence commands

Repository inspection covered `scripts/factory`,
`scripts/tests/factory-contract.sh`, `scripts/check-openspec-archive.py`, the
canonical factory specification and factory/governance documentation. The
observed no-op was retained in issue #194 and the preceding delivery receipt.

Implementation evidence will run the hermetic factory contract, shell lint,
strict OpenSpec validation, the repository factory gate and complete Nix checks.
Hosted Linux CI is intentionally unrun at research readiness and remains the
merge gate.
