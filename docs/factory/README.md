# AI Software Factory

The SDK-Rust factory turns human intent into bounded, reviewable changes that
LLM agents can implement. OpenSpec is the planning source of truth, GitHub is
the collaboration and approval surface, and Nix supplies reproducible tools and
gates. Agents produce artifacts and evidence; accountable humans approve scope,
security decisions, merges and releases.

## Delivery flow

```text
human intent / issue / Discussion
               │
               ▼
      explore and source audit
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
 implementation ──► verification ──► readiness receipt
                                             │
                                             ▼
                               sync specs and archive change
                                             │
                                             ▼
                             human-reviewed PR into develop
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
- `specs/<capability>/spec.md` with testable normative scenarios;
- `design.md` recording implementation decisions and trade-offs;
- `tasks.md` with ordered, parseable checkboxes;
- a semantic review with zero uncleared blockers.

## Factory commands

Run the repository facade directly, through `just`, or as a Nix app:

```bash
./scripts/factory doctor
./scripts/factory status
./scripts/factory validate <change>
./scripts/factory check
./scripts/factory ready <change>
./scripts/factory receipt <change>

just factory-check
nix run .#factory -- check
```

| Command | Contract |
| --- | --- |
| `doctor` | checks repository contracts, OpenSpec health and local branch ancestry |
| `status` | shows Git identity and active OpenSpec changes |
| `validate` | runs strict structural validation for one change or the whole store |
| `check` | runs the CI-safe structural and OpenSpec gates; incomplete draft tasks are allowed |
| `ready` | requires the named active change and every task to be complete |
| `receipt` | runs readiness, then prints immutable branch/head/base identifiers |

The receipt proves only the factory contract. Rust, target, conformance,
security and release gates must be attached separately and truthfully.

## Human gates

An agent must stop for human direction before:

- accepting or expanding product scope;
- choosing an unresolved standards/profile interpretation;
- waiving a security, privacy, compatibility or provenance finding;
- changing governance, protected settings or publishing ownership;
- merging, publishing, disclosing a vulnerability or mutating a downstream;
- accessing a secret or identity not explicitly supplied for the task.

Separate agent contexts can plan, implement, test or review, but they do not
create independent human approval. The pull request records the accountable
maintainer and required specialist reviews.

## Definition of ready

A change is ready for final review when:

1. its OpenSpec artifacts pass structural and semantic review;
2. every implementation task is checked and maps to a requirement;
3. `scripts/factory ready <change>` passes;
4. focused and repository-wide gates are recorded exactly;
5. provenance, threats, bounds and compatibility are addressed;
6. consumer repositories remain unchanged unless separately authorized;
7. current capability specs are synced and the completed change is archived;
8. the signed, DCO-bearing PR targets `develop`.

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
