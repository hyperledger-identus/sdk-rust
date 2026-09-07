# Agentic SDLC

The SDK is AI-first: agents may continuously perform backlog selection,
planning, implementation, testing, review, maintenance, CI triage and gated
integration into `develop`. The project sponsor defines objectives and
boundaries; agents need no per-task or format approval inside that mandate.
Human maintainers retain product-strategy, repository-administration,
security-disclosure, publishing, `main` promotion and release authority.

## Roles

| Role | Output | Cannot authorize |
| --- | --- | --- |
| Product/manager agent | backlog selection, routine scope, sequencing, dependencies and status | changing sponsor objectives, public commitments or governance |
| Planner agent | standards inventory, operational contract, ADR and acceptance plan | waiving unresolved ambiguity, evidence or risk |
| Engineer agent | one component slice, tests/docs and a feature-branch PR | cross-repository adoption, release or `main` promotion |
| Conformance agent | fixture provenance, differential results and drift checks | standards interpretation alone |
| Review agent | independent findings with severity/evidence | fabricating evidence or dismissing unresolved security findings |
| Security agent | threat analysis, negative tests and dependency review | embargo handling or public disclosure |
| Release agent | reproducible receipt and artifact verification | publishing without protected human authority |
| Maintainer agent | backlog hygiene, CI triage and eligible `develop` merge | protected settings, human maintainer status or protection bypass |

The same model may execute several roles, but a distinct local review means a
fresh review pass or context with its findings recorded. The standing sponsor
mandate and repository roadmap are the accountable source of intent. An
agent-authored issue and OpenSpec contract become operational when they are
complete, semantically reviewed and free of blockers; separate human approval
is not required for routine work or an eligible `develop` merge.

All roles use the executable lifecycle in the
[AI Software Factory handbook](../factory/README.md). Client-specific prompts
are adapters; OpenSpec artifacts and factory commands are the shared contract.

## State machine

```text
mandate / backlog
   │
   ▼
issue-framed ──► source-audited ──► research-ready ──► constraint-ready
                                                               │
                                                               ▼
                                                        contract-ready
                                                               │
                                                               ▼
                                                        implementation
                                                               │
                                                               ▼
                                                     verified candidate
                                                               │
                                                               ▼
                                                 local review + issue-linked PR
                                                               │
                                                               ▼
                                                     green CI + develop merge
                                                               │
                                            ┌──────────────────┴───────────────┐
                                            ▼                                  ▼
                                   protected release                  next component slice
```

An agent cannot skip `research-ready`, `constraint-ready` or `contract-ready`
for behavior, public API, architecture, protocol, security or multi-step work.
Research-ready means
the current implementation, sources, reuse candidates and risks have an
explicit evidence-backed disposition with no blocker. Contract-ready means the
artifacts are complete and semantically reviewed. Neither is a human approval
state. Constraint-ready means effective promises, future targets, limitations,
consumer impact and decision authority are explicit with no blocker.
`scripts/factory check` proves structural validity only.
Downstream adoption is never part of the upstream implementation state.

## One-slice operating contract

Before editing, the agent records:

- target repository/root, default branch and exact base SHA;
- issue/component ID and owner crate;
- local/remote instructions and applicable governance;
- source repositories, SHAs and paths as read-only references;
- consumer repository HEAD/status receipts when consumer code is inspected;
- expected commands, target matrix and stop conditions.

The agent then:

1. works in a dedicated worktree outside consumer repositories;
2. audits normative sources and existing implementations;
3. records candidate adoption/rejection evidence and passes
   `scripts/factory research-ready <change>`;
4. records constraint and limitation impact and passes
   `scripts/factory constraints-ready <change>`; a proposed material outcome
   stops activation until its exact direction is recorded;
5. establishes and semantically reviews the public contract and negative cases
   before porting;
6. records provenance for every adapted file or fixture;
7. implements only the bounded slice;
8. runs proportional gates and reports unrun gates exactly;
9. produces a standalone consumer-shaped proof when needed;
10. verifies consumer HEAD/status did not change;
11. completes and records a distinct local review pass;
12. pushes the focused branch and opens a ready, issue-linked pull request
    targeting `develop`;
13. monitors required CI and merges the eligible pull request into `develop`
    when every gate is green and no blocking review remains;
14. continues to the next eligible slice without waiting for ceremonial
    approval, but stops before release, publication, `main` promotion,
    repository administration, security disclosure or consumer adoption unless
    explicitly authorized by the responsible human.

Under ADR 0081, “required CI” means the exact Ubuntu `fast` status for normal
active-development integration. Weekly/manual `slow` and sanitizer results are
retained evidence and visible pre-release debt, not per-PR merge gates. Agents
must triage failures they encounter and may not prepare a release candidate or
publish while that debt or the release-phase compiler decision is unresolved.

Before final review, the agent runs `scripts/factory ready <change>` and
`scripts/factory receipt <change>`, syncs reviewed delta specs and archives the
completed OpenSpec change through `scripts/factory archive <change>` so lossy
`MODIFIED` replacements fail before canonical mutation. Product-specific gates
remain separate evidence.

## Hard repository isolation

Unless an issue explicitly authorizes a consumer adoption change, agents treat
Oxid, midnight-identity, neoprism, Lace ID Portal and other SDKs as read-only.
They must not format, generate, stage, commit, switch branches, update
submodules, install hooks or add path dependencies in those repositories.

If consumer code changes during an upstream task, the agent stops and reports
the exact mutation. It does not hide the event with reset, clean or stash.

## Stop conditions

Stop for maintainer direction only when safe in-scope investigation cannot
resolve one of these conditions:

- competing product or standards interpretations would create materially
  different user outcomes and the roadmap or normative sources do not decide;
- the proposed API cannot avoid leaking product/chain policy or raw secret
  material;
- a donor's license/provenance or applicable legal obligation is unclear;
- a new crypto primitive or unsafe block appears necessary and specialist
  review cannot establish an acceptable bounded design;
- compatibility requires an externally committed silent wire or verification
  change, data loss or irreversible migration;
- a required security/conformance review cannot be obtained or leaves a
  blocking finding unresolved;
- the slice needs an unauthorized consumer edit to appear complete; or
- progress requires secrets, private disclosure, protected repository controls,
  publication, release or `main` promotion.

Do not stop for task wording, document format, naming, ordinary implementation
choices, test organization, reversible refactoring, CI repair or routine
tooling and dependency maintenance.

## Evidence receipt

Every code PR includes:

```text
Component and issue:
Base SHA:
Source SHAs/paths/licenses:
Normative versions:
Public/wire compatibility:
Threats and bounds:
Commands passed:
Commands not run and why:
Coverage/conformance evidence:
Local/security/docs review:
Consumer preflight HEAD/status:
Consumer final HEAD/status:
Consumer changed: no
Release/adoption follow-up:
```

Generated prose is not evidence by itself. Test output, diffs, hashes,
conformance reports and recorded review findings are.
