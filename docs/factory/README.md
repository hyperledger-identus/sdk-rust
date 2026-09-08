# AI Software Factory

The SDK-Rust factory turns the standing product mandate and backlog into
bounded, reviewable changes that LLM agents can deliver continuously. OpenSpec
is the planning source of truth, GitHub is the durable coordination surface,
and Nix supplies reproducible tools and gates. Agents may originate routine
scope and integrate issue-linked work into `develop` after local review and
green required CI. Accountable humans retain only the protected decisions
listed below.

## Delivery flow

```text
standing mandate / backlog / issue
               │
               ▼
      explore and source audit
               │
               ▼
    research-ready decision
               │
               ▼
   constraint-ready decision
               │
               ▼
 OpenSpec proposal + specs + design + tasks
               │
               ▼
 structural validation + semantic review
               │
               ▼
 focused branch/worktree from develop
               │
               ▼
 implementation ──► verification ──► local review
                                             │
                                             ▼
                               sync specs and archive change
                                             │
                                             ▼
                         issue-linked PR ──► green CI ──► develop
```

`main` is not part of this flow. Downstream adoption is a separate change in
the downstream repository after an immutable SDK candidate exists.

## Start a change

Use the pinned environment so every agent and maintainer receives the same
OpenSpec version:

```bash
nix develop
openspec new change <kebab-case-name>
openspec status --change <kebab-case-name>
```

OpenSpec is required for behavior, public API, architecture, protocol,
security, dependency-policy and multi-step changes. Typos, formatting,
non-behavioral fixes and mechanical chores may use the explicit PR exemption.

Before implementation, the change must contain:

- `.openspec.yaml` using the `spec-driven` schema;
- `proposal.md` explaining intent, scope and capabilities;
- `research.md` recording the current implementation, normative sources,
  adopt/build/reject decisions, compatibility evidence and zero blockers;
- `constraints.md` classifying constraint/limitation impact, effective versus
  target state, decision authority, consumer impact, activation and rollback;
- `specs/<capability>/spec.md` with testable normative scenarios;
- `design.md` recording implementation decisions and trade-offs;
- `tasks.md` with ordered, parseable checkboxes;
- a semantic review with zero uncleared blockers.

Run `scripts/factory research-ready <change>` before the first implementation
edit. The [research readiness contract](research-readiness.md) is lightweight
for routine changes and requires a full candidate/evidence matrix for new
protocol, foundational, cryptography/security, storage or FFI work.
Run `scripts/factory constraints-ready <change>` at the same boundary. A
material proposal remains researchable but cannot become implementation-ready
until its exact durable decision reference resolves the product outcome.

## Factory commands

Run the repository facade directly, through `just`, or as a Nix app:

```bash
./scripts/factory doctor
./scripts/factory status
./scripts/factory validate <change>
./scripts/factory check
./scripts/factory research-ready <change>
./scripts/factory constraints-ready <change>
./scripts/factory ready <change>
./scripts/factory receipt <change>
./scripts/factory archive <change>
./scripts/check-bootstrap-inventory.py
./scripts/check-ssi-upstream-backlog.py
./scripts/check-support-policy.py
./scripts/benchmark-support-policy.py --samples 20

just factory-check
nix run .#factory -- check
```

| Command | Contract |
| --- | --- |
| `doctor` | checks repository contracts, OpenSpec health and local branch ancestry |
| `status` | shows Git identity and active OpenSpec changes |
| `validate` | runs strict structural validation for one change or the whole store |
| `check` | runs the CI-safe structural and OpenSpec gates; incomplete draft tasks are allowed |
| `research-ready` | requires reviewed research, an explicit candidate disposition and zero declared research blockers before implementation |
| `constraints-ready` | requires explicit constraint/limitation impact and exact authority for material outcomes before implementation |
| `ready` | requires the named active change and every task to be complete |
| `receipt` | runs readiness, then prints immutable branch/head/base identifiers |
| `archive` | snapshots matching archives, rejects a known dated-destination collision, runs readiness and preservation preflight, archives through pinned OpenSpec, then proves exactly one new regular requested archive, mandatory artifacts and the resulting store before reporting success |

The receipt proves only the factory contract. Rust, target, conformance,
security and release gates must be attached separately and truthfully.

During the temporary active-development policy, an issue-linked PR receives
one Ubuntu `fast` status containing the factory contract, repository lint,
formatting, workspace build, strict Clippy and normal tests on Rust 1.98.1.
The exhaustive Linux/macOS flake and sanitizer campaigns remain weekly and
manually dispatchable `slow` evidence. Agents may merge on green required
`fast` evidence, but must treat any slow failure as visible debt; release or
publication is prohibited until that debt and the release-phase compiler
matrix are resolved.

OpenSpec `MODIFIED` operations replace a complete canonical requirement. The
repository checker permits the default additive path only when every existing
nonblank canonical line survives in order. An intentional rewrite or deletion
requires `archive-intent.toml` in the active change:

```toml
[[modified_requirement]]
capability = "example-capability"
requirement = "Existing requirement"
canonical_sha256 = "<normalized canonical requirement SHA-256>"
reason = "Why replacing canonical behavior is intentional"
```

The acknowledgement is bound to the exact canonical block, moves into archive
evidence and never enters the living spec. Missing, malformed, stale, duplicate
or unused entries fail. A missing-intent diagnostic prints the required
canonical hash without printing the requirement body. Use
`scripts/factory archive <change>` rather than raw `openspec archive`; the
facade checks readiness and preservation before any canonical or active-change
mutation. A zero exit from OpenSpec is not sufficient: the facade reports
success only when the active change is absent, exactly one new matching regular
archive directory exists relative to its pre-mutation snapshot, all mandatory
artifacts remain and the resulting store validates. The receipt does not infer
the completed archive from the host-local date, so timezone disagreement or a
date rollover cannot turn a valid archive into a false failure.

The SSI backlog checker validates the canonical SDK component ledger offline.
It rejects missing or duplicate rows, schema and enum drift, non-SDK ownership,
invalid issue/predecessor links and unknown source repositories. The factory
structural check runs it automatically.

The bootstrap-inventory checker validates repository-local governance
evidence, Cargo publication denial, complete package/path/layer classification
and quarantined-placeholder shape. It is offline and does not claim that
protected GitHub settings or publishing authority are active.

The support-policy checker validates the machine-readable Rust, Nix, host,
target, feature, FFI and budget contract against Cargo and the declarative Nix
gate manifest. Nix generates its Crane checks from the same manifest. The
checker fails structural CI when a compatibility claim loses its actual gate
or a toolchain/target surface drifts independently, without interpreting Cargo
semantics from Nix source text. The benchmark records 20-sample warm and
fresh-process p50/p95 diagnostics and compares PR heads to their base without
creating a compatibility budget.

## Authority gates

The project sponsor grants standing authority for routine work within the
recorded roadmap and architecture. An agent may select and prioritize a slice,
create or refine its issue and OpenSpec contract, make reversible product and
technical decisions, implement and review it, push the focused branch, open the
ready pull request, repair branch-owned CI failures and merge it into `develop`
after every required CI gate succeeds. These steps do not require formal,
format, push or merge approval.

An agent stops for human direction only before:

- choosing between materially different product outcomes that the roadmap and
  evidence do not resolve;
- changing product strategy, governance, licensing or public commitments;
- accepting unresolved security, privacy, cryptographic, compatibility,
  provenance, legal or data-loss risk;
- changing protected settings or publishing ownership;
- publishing, promoting to `main`, disclosing a vulnerability or mutating a
  downstream without a separate authorization; or
- accessing a secret or identity not explicitly supplied for the task.

The [constraint guide](../governance/constraints-and-limitations.md) makes this
boundary explicit. A future target may be researched without stopping the
factory, but it cannot become an effective consumer promise until the exact
material outcome has a durable decision reference. Routine reversible choices
remain autonomous.

Pending or failing gates, a draft state, merge conflicts and unresolved
blocking reviews stop integration until the agent resolves them; they do not
automatically require a human. No agent may bypass branch protection. The pull
request records the mandate or roadmap source and local or specialist reviews.

## Definition of ready

A change is ready for final review when:

1. `scripts/factory research-ready <change>` and `scripts/factory
   constraints-ready <change>` passed before implementation and remain
   consistent with the result;
2. its OpenSpec artifacts pass structural and semantic review;
3. every implementation task is checked and maps to a requirement;
4. `scripts/factory ready <change>` passes;
5. focused and repository-wide gates are recorded exactly;
6. provenance, threats, bounds and compatibility are addressed;
7. consumer repositories remain unchanged unless separately authorized;
8. current capability specs are synced and the completed change is archived
   through the guarded factory command;
9. a distinct local review pass has no unresolved blocker;
10. the signed, DCO-bearing PR targets `develop` and references its issue.

## Client adapters

The `.agents/skills/openspec-*` skills are generated OpenSpec adapters. Pi
chains under `.pi/` delegate a supplied change name to the same lifecycle.
Other clients may add repository-owned adapters when they call the same factory
commands and preserve these authority gates.

Do not commit personal model selection, tokens, MCP configuration, environment
files, editor state or workspace settings. User-level orchestration belongs
outside the repository.

## Evidence and troubleshooting

Use the [evidence receipt](evidence-receipt.md) in pull requests. If a gate
fails, preserve its exact command and output category; do not rewrite fixtures,
lower thresholds or skip the gate merely to obtain green CI. Structural success
does not prove semantic correctness, conformance, security or product fitness.
