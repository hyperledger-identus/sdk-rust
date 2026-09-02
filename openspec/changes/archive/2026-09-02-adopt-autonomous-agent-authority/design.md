## Context

The repository already has deterministic structural checks, mandatory issue
linkage, a distinct local review, signed DCO commits, protected pull requests
and required CI. The remaining universal scope-acceptance step is an authorship
gate rather than a risk gate: the same bounded contract is blocked when an
agent writes it and unblocked when a human approves its format.

The project sponsor has directed the factory to minimize friction so agents can
operate continuously. The implementation must broaden routine authority without
implicitly granting release, disclosure, secret, repository-administration or
externally binding authority.

## Goals / Non-Goals

**Goals:**

- make standing routine authority explicit across every active policy surface;
- let agents originate scope and reversible decisions within the roadmap;
- preserve objective contract, review, CI and branch-protection gates;
- define a small set of material, consequence-based escalation boundaries;
- keep the state model portable across agent clients and runtimes.

**Non-Goals:**

- a hosted 24/7 scheduler or agent process;
- autonomous publication, releases, `main` promotion or repository settings;
- secret access, vulnerability disclosure or legal-risk acceptance;
- direct pushes, protection bypass or waiver of unresolved findings;
- live GitHub ruleset mutation or downstream repository changes.

## Decisions

### 1. The roadmap is a standing mandate

The sponsor does not need to accept every derived issue or contract. Agents may
select backlog work, decompose roadmap outcomes and make reversible details
concrete. They record assumptions and decisions so later agents and humans can
inspect or supersede them.

### 2. Contract readiness replaces contract approval

Qualifying work still needs proposal, delta requirements, design, tasks and
semantic review before implementation. The state is `contract-ready` when those
artifacts are structurally valid, semantically coherent and have no unresolved
blocker. The author's identity and a separate approval token are irrelevant.

### 3. Evidence gates remain mandatory

Issue linkage, local review, specialist review where risk requires it, signed
DCO commits, required CI and normal protected merge remain stop/go gates. An
agent may repair a failed gate and retry; it does not need permission merely
because the gate was initially red.

### 4. Escalation follows consequence and unresolved risk

Agents proceed through routine ambiguity with documented assumptions and
reversible choices. They escalate only when safe investigation cannot resolve a
material product fork or when the work reaches protected strategy, governance,
licensing, secret, disclosure, irreversible external, release, `main`, settings
or unresolved-risk authority.

### 5. Continuous operation is runtime-independent

Repository policy must support a long-running agent or scheduler, but it does
not select or install one. Pi, Codex, another MCP-capable agent or a future
orchestrator can all use the same issues, OpenSpec lifecycle and gates.

## Risks / Trade-offs

- **An agent chooses a poor reversible design** → require durable assumptions,
  ADRs where applicable, local review, tests and easy rollback.
- **Scope drifts beyond the roadmap** → require issue and contract boundaries;
  material outcome changes trigger escalation.
- **Agent review shares blind spots with implementation** → use a distinct
  context and require specialist review for risk classes; unresolved findings
  block integration.
- **Continuous operation amplifies a CI mistake** → required checks and branch
  protection remain absolute; no bypass is delegated.
- **The word autonomy is mistaken for release authority** → enumerate protected
  actions consistently in controlling documents and templates.

## Migration Plan

1. Record the standing mandate in ADR 0004 and the current OpenSpec contracts.
2. Replace active approval-state wording in governance, contributor, agent and
   factory documents.
3. Replace mandatory human-owner fields with the mandate or roadmap source.
4. Run structural, policy, documentation and full Nix validation; perform a
   distinct contradiction-focused local review.
5. Deliver through an issue-linked PR to `develop` and merge only after every
   required CI gate is green.

Rollback is a revert on `develop`. ADR 0003 then remains the controlling
delegation and per-scope human acceptance becomes required again.
