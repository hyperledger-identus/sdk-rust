# Agentic SDLC

The SDK is AI-first: agents may perform planning, implementation, testing,
review and maintenance, while human maintainers retain product intent,
governance, security disclosure, merge and release authority.

## Roles

| Role | Output | Cannot authorize |
| --- | --- | --- |
| Product/manager agent | issue framing, sequencing, dependencies and status | scope expansion, release or governance change |
| Planner agent | standards inventory, contract, ADR and acceptance plan | treating a proposal as accepted |
| Engineer agent | one component slice and its tests/docs | cross-repository adoption or merge |
| Conformance agent | fixture provenance, differential results and drift checks | standards interpretation alone |
| Review agent | independent findings with severity/evidence | self-approval or dismissal of security findings |
| Security agent | threat analysis, negative tests and dependency review | embargo handling or public disclosure |
| Release agent | reproducible receipt and artifact verification | publishing without protected human approval |
| Maintainer agent | backlog hygiene and stale-evidence reports | human maintainer status |

The same model may execute several roles, but independent review means a fresh
review context and a human accountable for accepting the result.

## State machine

```text
proposed
   │
   ▼
source-audited ──► contract-approved ──► implementation
                                            │
                                            ▼
                                  verified candidate
                                            │
                                            ▼
                                  reviewed + released
                                            │
                         ┌──────────────────┴───────────────┐
                         ▼                                  ▼
                downstream adoption                next component slice
```

An agent cannot skip `contract-approved` for behavior or public API work.
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
3. proposes the public contract and negative cases before porting;
4. records provenance for every adapted file or fixture;
5. implements only the accepted slice;
6. runs proportional gates and reports unrun gates exactly;
7. produces a standalone consumer-shaped proof when needed;
8. verifies consumer HEAD/status did not change;
9. opens a draft PR with the evidence receipt;
10. stops before push, merge, release or consumer adoption unless explicitly
   authorized by the responsible human.

## Hard repository isolation

Unless an issue explicitly authorizes a consumer adoption change, agents treat
Oxid, midnight-identity, neoprism, Lace ID Portal and other SDKs as read-only.
They must not format, generate, stage, commit, switch branches, update
submodules, install hooks or add path dependencies in those repositories.

If consumer code changes during an upstream task, the agent stops and reports
the exact mutation. It does not hide the event with reset, clean or stash.

## Stop conditions

Stop for maintainer direction when:

- normative sources conflict or a profile version is ambiguous;
- the proposed API leaks product/chain policy or raw secret material;
- a donor's license/provenance is unclear;
- a new crypto primitive or unsafe block appears necessary;
- compatibility requires a silent wire or verification behavior change;
- a required security/conformance reviewer is unavailable;
- the slice needs a consumer edit to appear complete;
- repository controls, signatures or release ownership are not operational.

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
Security/docs review:
Consumer preflight HEAD/status:
Consumer final HEAD/status:
Consumer changed: no
Release/adoption follow-up:
```

Generated prose is not evidence by itself. Test output, diffs, hashes,
conformance reports and human review decisions are.
