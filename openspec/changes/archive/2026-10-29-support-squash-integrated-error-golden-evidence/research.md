# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

At exact base `78bd1c01a83e014e0cc85be27075bf7634902199`,
`scripts/factory doctor` fails only with:

`jwe preimplementation receipt current head must descend from the contract head`

The archived receipt identifies planning commit
`0f81f09c72b920d31c456312822ab28e50b9706c` over base
`c66d506690d3cd6f119f9ae6430b285f98115d10`. That commit is signed, changes
only the issue-owned OpenSpec paths plus ADR 0195, and contains the exact
planning golden. Protected PR #577 instead introduced the reviewed result as
single-parent squash commit `78bd1c01`; Git cannot make the source planning
commit an ancestor of that commit.

The first four historical golden bindings happen to retain their planning
commits in ancestry. The JWE binding is the first one integrated after the
factory standardized protected squash merges, so the old assumption is not a
valid repository invariant.

## Normative sources

ADRs 0003 and 0004 authorize the protected issue-linked delivery path, ADR
0127 separates integration from production promotion, ADR 0195 owns the JWE
golden, and the canonical factory-operations specification requires an exact
planning receipt before implementation. Issue #578 is the exact repair
authority. Git commit identities and the archived receipt are repository
evidence; GitHub PR #577 confirms the protected squash result but is not a
runtime dependency.

## Evidence model

The immutable contract already pins repository, issue, change, branch, base,
planning head, timestamp shape, readiness flags, exact planning golden bytes,
stable fixture bytes, and a hard-coded SHA-256. The gap is that the receipt
itself is not byte-pinned, so simply dropping head ancestry would allow its
planning-head or timestamp fields to drift undetected.

The repair adds a fixed receipt SHA-256 to each `GoldenBinding`. Active or
ancestry-preserving evidence still proves `base -> contract -> HEAD` and
recomputes the planning-only diff. Archived evidence whose contract commit is
not an ancestor instead proves:

1. fixed base is an ancestor of current `HEAD`;
2. the archived receipt, archived planning golden, and stable fixture each
   resolve from `HEAD` and match their working-tree bytes;
3. the receipt matches the binding's immutable receipt hash;
4. archived planning and stable fixture bytes match each other and the fixed
   golden hash; and
5. when the named contract object is locally reachable, it still descends from
   the base, has only allowed planning paths, and contains the same golden.

The fallback is independent of feature-branch retention. It trusts only data
already reviewed and committed in canonical protected history plus constants
in the checker; it does not infer a new planning head from current source.

## Candidate decisions

| Candidate | Decision | Reason |
| --- | --- | --- |
| Require feature branches forever | `not-adopt` | Conflicts with branch cleanup and still does not restore ancestry after squash. |
| Change protected integration to merge commits | `not-adopt` | Repository administration and merge policy are outside this repair. |
| Accept any archived receipt without ancestry | `not-adopt` | Receipt fields could drift and planning-only evidence would become unverifiable. |
| Store Git bundles or tags per contract | `not-adopt` | Adds binary/history lifecycle and external-ref maintenance disproportionate to eight-row fixtures. |
| Pin receipt bytes and validate reachable source commits opportunistically | `adopt` | Preserves exact canonical evidence, keeps the stronger proof where possible, and survives ordinary source-branch cleanup. |
| Regenerate the JWE receipt against the squash commit | `not-adopt` | Would falsely claim the behavior-bearing squash commit was planning-only. |

## Compatibility and dependency evidence

No manifest, feature, lockfile, crate, supported target, public API, wire value,
or runtime behavior changes. The checker remains Python standard-library plus
Git and introduces no dependency. Existing fixture and receipt files remain
byte-identical; only their validation rules and immutable hashes are extended.

## Security, privacy and maintenance evidence

The change alters only repository evidence validation. Public API, runtime
behavior, dependencies, fixtures, error categories and release support remain
unchanged. Exact receipt hashes strengthen all five bindings. Missing,
malformed, mutated or mismatched receipts remain fatal. A reachable but invalid
contract object remains fatal rather than silently selecting the fallback.

The checker must not depend on a remote branch, network access, a private merge
receipt, GitHub metadata, or checkout-specific trust configuration. Mutation
tests cover both evidence states and prove the fallback cannot accept a changed
receipt, base, golden, reachable non-planning commit, or uncommitted/transplanted
working-tree copy absent from `HEAD`.

Pending declaration `factory.error-golden.quality.v1` will own this on-change
property/mutation route. Coverage-guided fuzzing, performance benchmarking, and
external differential comparison are not applicable: the input domain is a
closed five-binding repository manifest, execution is dominated by bounded Git
subprocesses with no supported latency budget, and no independent
implementation defines this repository-specific evidence policy. Issue #578
owns those reviewed dispositions.

## Rejected or deferred candidates

No dependency or external service is adopted. A general v2 preflight manifest
may later carry content hashes for all OpenSpec planning files, but it is not
needed to repair the existing fixed golden bindings and would expand this
urgent slice into a repository-wide receipt migration.

## Open questions and blockers

None. The protected squash path is already the accepted repository delivery
path, and issue #578 authorizes a repository-local, reversible repair.

## Evidence commands

`scripts/factory doctor`, `python3 scripts/check-error-golden.py .`,
`python3 scripts/tests/error-golden.py`, `scripts/factory check`, focused
factory mutation tests, formatting/lint/tests, Nix checks, and a distinct
Claude or Codex CLI review.
