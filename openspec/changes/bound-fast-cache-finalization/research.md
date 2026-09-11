# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-11
Source retrieval date: 2026-09-11
Research blockers: none

## Problem and existing implementation

The current implementation at exact base
`71ae2fe82b65e1e2da513ad9091ed9709d10a756` runs
`.github/workflows/factory-contract.yml` on pull requests and `develop` pushes.
It installs Nix, launches the pinned Magic Nix Cache action, and requests eight
unchanged Nix checks. The action restores useful paths, but its registered post
phase uploads new store paths one by one after correctness evidence exists.

Three comparable successful pull-request runs provide the baseline. Durations
come from GitHub job/step timestamps and are exact timestamp differences.

| Run | Head | Job | Install Nix | Cache setup | Gates | Cache finalization |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| `34585602554` | `e8288a2e` | 682 | 10 | 14 | 249 | 393 |
| `34573965525` | `aab9ceed` | 694 | 10 | 12 | 174 | 478 |
| `34449567650` | `819bb145` | 738 | 10 | 14 | 424 | 273 |
| **median** | n/a | **694** | **10** | **14** | **249** | **393** |

The latest run's logs show sequential GitHub Actions cache uploads throughout
the 393-second finalizer and a non-failing unauthenticated FlakeHub annotation.
The cache is acceleration, not build, test or conformance evidence.

## Normative sources

- GitHub workflow syntax defines `jobs.<job_id>.cache-mode: read` as restore
  allowed and save denied, enforced by scoped cache tokens. Denied saves are
  informational and continue:
  https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#jobsjob_idcache-mode
- GitHub recommends cache writes only from trusted triggers. Its current
  default policy expires entries unused for seven days, provides 10 GB per
  repository, evicts least-recently-used entries and rate-limits transfers:
  https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching
- The pinned action's closed input contract supports explicit GitHub Actions
  cache, explicit FlakeHub disablement and an empty diagnostic endpoint:
  https://github.com/DeterminateSystems/magic-nix-cache-action/blob/908b263ff629f4cc17666315b7fd3ec127c6244d/action.yml
- Its official README documents graceful rate-limit degradation and `runs.post`
  publication:
  https://github.com/DeterminateSystems/magic-nix-cache-action/blob/908b263ff629f4cc17666315b7fd3ec127c6244d/README.md

The current action revision is verified commit
`908b263ff629f4cc17666315b7fd3ec127c6244d` (v14), committed 2026-05-15,
under MIT. The executed backend remains an action-owned runtime download; this
change neither upgrades nor widens that existing supply-chain boundary.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Explicit read-only GHA authority; pinned action; FlakeHub/diagnostics disabled | GitHub 2026 syntax / action `908b263f` | `adopt` | Retains restore hits, denies PR save authority and long path uploads, adds no dependency, secret or cost boundary, and degrades faults to a miss. | Exact-head post phase exceeds 30 seconds, syntax is unsupported, or cache error alters the gate result. |
| Remove the cache action from `fast` | repository-local | `conditional-adopt` | Smallest trust cone and no cache finalizer, but discards restore acceleration before a cache-free build baseline exists. | Adopt immediately if the read-only canary misses the finalizer bound. |
| `nix-community/cache-nix-action` restore-only | current main reviewed 2026-09-11 | `not-adopt` | Adds a third-party action and whole-store/database semantics without evidence it improves on native read-only authority. | Read-only Magic Nix Cache fails and a measured replacement beats cache-free operation. |
| Authenticated FlakeHub Cache | current service reviewed 2026-09-11 | `not-adopt` | Requires external service ownership, credential/OIDC authority and possible cost outside this issue. | Maintainers separately authorize the service, trust, identity and budget. |
| Current implicit write behavior | action `908b263f` | `not-adopt` | A 393-second median finalizer dominates the lane and probes an unauthenticated service. | Never without a new measurement and explicit trusted-writer design. |

## Compatibility and dependency evidence

The public and wire compatibility of every Rust crate is unchanged. Rust
1.98.1, Nix/flake inputs, Cargo features, target declarations and all eight
Nix installables remain exact. No Cargo direct or resolved dependency cone,
MSRV, native code, unsafe code or SDK facade changes. The action dependency and
revision remain the existing pinned one.

Cache entries contain Nix store artifacts under GitHub Actions cache semantics;
they do not define derivation identity. A hit restores acceleration. A miss,
eviction, denial or rate limit falls back to signed substituters and local
realization of the same graph. Weekly/manual slow workflows keep their existing
cache plumbing and release evidence role. Rollback reverts this workflow and
validator change. No consumer or target behavior changes.

## Security, privacy and maintenance evidence

The job retains `contents: read`, adds no ID token and introduces no secret.
GitHub-enforced read-only cache tokens reduce pull-request cache-poisoning
authority. Explicitly disabling FlakeHub prevents the observed unauthenticated
service probe; disabling the optional diagnostic endpoint removes action
telemetry for this required lane. `continue-on-error` keeps cache availability
non-normative, while the 20-minute job timeout leaves a hung action as a failed
required check rather than a false success.

The existing action is MIT; no new license or supply-chain component is added.
Its maintenance and release posture remains pinned-update-only. Protocol/draft
currency and FFI are not applicable to this hosted CI control. GitHub's
retention, eviction and rate limits mean a cache hit is never promised. Extra
cache storage can create cost, so this issue removes fast-lane writes rather
than changing repository cache settings or budget.

## Rejected or deferred candidates

The replacement action and authenticated FlakeHub are not adopted for the
dependency, service, identity and cost reasons in the candidate table. A fully
cache-free fast lane is the pre-authorized fallback if the candidate cannot
meet its exact-head 30-second post-phase bound. Slow-lane write optimization is
deferred to separate measured work because it is outside the PR critical path.

## Open questions and blockers

No implementation blocker remains. Hosted canary duration cannot be known
before the PR runs, so the fallback and threshold are fixed in advance rather
than interpreted after seeing the result. The 480-second median across three
future comparable runs is a throughput target, not a public SLA. Review is due
2026-12-08 or before release-candidate preparation.

## Evidence commands

- `gh api repos/hyperledger-identus/sdk-rust/actions/runs/<run>/jobs` retrieved
  exact step timestamps for runs `34585602554`, `34573965525` and
  `34449567650`.
- `gh run view 34585602554 --log` proved sequential cache uploads and the
  FlakeHub annotation.
- `gh api` verified action commit `908b263f`, action input contract and MIT
  license; official GitHub workflow/cache documentation was retrieved on
  2026-09-11.
- `scripts/factory doctor` and structural checks ran at the exact base before
  implementation; final focused, full Nix and hosted commands remain unrun and
  are tasks, not claimed successes.
