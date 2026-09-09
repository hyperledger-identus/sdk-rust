## Context

The existing factory owns OpenSpec semantics. This change adds the operational
control loop around it without importing Oxid's product topology or changing
SDK integration authority.

## Decisions

### Keep one portable control plane

`scripts/factory` remains the semantic facade and gains preflight, runtime
audit, target-plan, metrics and worktree commands. `bootstrap.sh` only enters
the pinned Nix environment, configures local controls or launches Pi; it does
not duplicate policy.

### Make pre-implementation state durable

A production-ready dev-loop begins from one repository issue and a clean
issue branch. After the OpenSpec change is committed, `factory preflight`
validates research, constraints, strict structure, branch/issue identity and
the absence of implementation-path changes relative to the recorded base. It
writes a receipt inside the change. Final readiness validates that receipt
instead of trusting session memory.

### Preserve the SDK branch model

Issue branches may use `<type>/issue-N` or Codex's namespaced
`codex/<type>/issue-N` form and target only `develop`. Exact-head green
issue-linked PRs may be merged by an authorized agent under ADRs 0003/0004.
No milestone train or `main` flow is introduced.

### Keep routing simple during active development

The target planner derives an immutable classification from base/head paths
and risk. The required PR context remains the existing Linux `fast` lane.
It recommends weekly/manual slow evidence for toolchain, Nix, security,
bindings and release-sensitive changes; unknown diff fails closed. This gives
the factory an auditable plan without multiplying PR builds.

### Store metrics privately and publish only bounded aggregates

One JSON record is keyed by repository, issue, optional PR and exact head
under the Git common directory. A renderer produces one human summary and a
single hidden canonical payload. Publication updates only the authenticated
agent's matching comment. Unknown counters are `null`.

### Tune after a real canary

The tracked policy starts conservative. After merge, one bounded SDK slice is
delivered through `./bootstrap.sh --pi`; its time, retries, tool calls, token
coverage and disk use determine the next focused harness issue. The vault's
versions are not copied merely because they exist.

## Risks and mitigations

- Local hooks can be bypassed: hosted checks repeat mandatory invariants.
- Pi packages can drift: exact declarations and runtime audit fail closed.
- Metrics can leak context: closed schema, allowlist, size bound, private
  storage and forbidden-field scan.
- Worktree cleanup can destroy work: mutation requires exact path/head and
  refuses dirty, locked, current, primary or ambiguous candidates.
- A large factory port can overfit Oxid: every adopted behavior maps to an SDK
  requirement; product-specific topology is rejected.

## Rollback

Revert the factory PR. The prior `scripts/factory`, fast workflow and plain
Cargo path remain recoverable because no repository setting or consumer state
is mutated by the commit.
