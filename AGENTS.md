# SDK Rust — agent instructions

This repository is the chain-agnostic Rust SDK for the
[Identus](https://github.com/hyperledger-identus) ecosystem. The project sponsor
sets product objectives and protected boundaries. Within that mandate, agents
have standing authority to select, specify, implement, review and integrate
bounded, issue-linked work into `develop` under the evidence- and CI-gated
policy. They do not need per-task, format, push or merge approval.

Before changing files, read:

- `GOVERNANCE.md` and `CONTRIBUTING.md`;
- `docs/architecture/sdk-rust-blueprint.md`;
- `docs/adr/0001-bootstrap-branch-selection.md`;
- `docs/adr/0003-delegate-develop-integration.md`;
- `docs/adr/0004-establish-standing-agent-authority.md`;
- `docs/governance/agentic-sdlc.md`;
- `docs/governance/constraints-and-limitations.md` and the indexed constraints
  relevant to the change;
- `docs/factory/README.md`;
- the issue or OpenSpec change and any crate-local `AGENTS.md` in scope.

## Branch and repository boundaries

- `develop` is the active integration branch. Start focused branches and
  dedicated worktrees from its current protected tip.
- `main` is intentionally reserved and minimal. Do not target, populate or
  release from it until maintainers accept a separate activation decision.
- The `yet-another-seed` tree at `662f8d7` is the selected `develop` baseline.
  Existing implemented crates are foundations to stabilize, not immutable API
  commitments. Empty placeholder crates are not roadmap or release promises.
- Treat Oxid, midnight-identity, neoprism, Portal and other SDK repositories as
  read-only unless a separate adoption issue explicitly authorizes changes.
- Do not add `midnight-*`, `compact-runtime`, chain clients or product
  repositories to a generic SDK crate.
- Port later donor components one bounded, contracted slice at a time and
  record source SHA, path, license, transformation and conformance evidence.
- Do not expose raw secret bytes through errors, debug, serialization or FFI.
- Every pull request must have a corresponding repository issue. Create the
  issue before the pull request when no suitable issue exists.
- After implementation and a distinct local review pass, humans and agents may
  push focused branches and open ready pull requests targeting `develop`.
- Humans and agents may merge a non-draft, mergeable pull request into
  `develop` after every required CI gate is green and no blocking review or
  thread remains. Never bypass a required check or branch protection.
- Do not push directly to `develop` or `main`. Do not publish, release, promote
  to `main`, disclose a vulnerability or change repository settings without
  explicit human maintainer authority.

## Standing autonomy mandate

- Agents may choose and prioritize routine backlog work, create the required
  issue and OpenSpec contract, and make reversible product and technical
  decisions within the recorded roadmap and architecture boundaries.
- Issues, OpenSpec changes, ADRs, receipts and pull-request templates are
  coordination records and quality gates, not human approval queues.
- Do not pause to ask for approval of naming, formatting, task decomposition,
  routine implementation details, tests, documentation, refactoring, CI fixes
  or reversible tooling and dependency maintenance.
- Escalate only for a material product ambiguity not resolved by the roadmap;
  a change to strategy, governance, licensing or public commitments; secret or
  credential use; private vulnerability handling; an irreversible external
  action; release, publication, `main` promotion or repository administration;
  or unresolved security, privacy, cryptographic, compatibility or data-loss
  risk.
- A distinct local or specialist review remains evidence. An agent may obtain
  it from a fresh agent context; no human approval is implied unless the work
  crosses a protected boundary above.

## Factory workflow

- Create or select an OpenSpec change before qualifying implementation work.
- Run `scripts/factory doctor` before editing and record the exact base SHA.
- Read proposal, research, constraints, specs, design and tasks; clear semantic
  blockers first.
- Complete proportionate build-versus-adopt research and run
  `scripts/factory research-ready <change>` before implementation. New
  foundational, protocol, cryptography/security, storage or FFI work requires
  the full evidence matrix; routine work may record a justified
  `not-applicable` decision.
- Run `scripts/factory constraints-ready <change>` before implementation. A
  material consumer, product, compatibility, security, license,
  certification, budget or release outcome requires an exact durable decision
  reference. Research and reversible non-activating preparation may continue
  while it is proposed; activation may not.
- Commit the planning-only OpenSpec contract, then run `scripts/factory
  preflight <change> --issue N --write`. Do not begin implementation until the
  durable exact-base/exact-head receipt passes.
- Implement one task at a time and check it immediately after verification.
- Run `scripts/factory check` throughout draft work.
- Run `scripts/factory ready <change>` and `receipt <change>` before final
  review, then sync specs and archive the completed change.
- Archive completed changes through `scripts/factory archive <change>` so
  partial `MODIFIED` requirements fail before canonical specifications change.
- Complete and record a distinct local review pass.
- Open a signed, DCO-bearing PR targeting `develop` with the issue reference
  and evidence receipt.
- Monitor every required check. Merge into `develop` only when the pull request
  satisfies the repository integration policy; otherwise diagnose or stop.

Do not represent structural OpenSpec validation as semantic, security or
conformance approval.

## Development gates

A Nix flake devshell supplies the reproducible maintainer environment. Plain
stable Cargo remains a supported consumer path.

```bash
nix develop -c cargo build --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo doc --workspace --no-deps
nix flake check
```

Run the issue-specific MSRV, WASM/mobile, minimal-feature, fuzz, conformance,
dependency and public-API gates in addition. `nix flake check` is mandatory
after code, build or dependency changes. Report every unrun or failing command
exactly; never infer one gate from another.

All repository-facing commits require both DCO and a verified signature:

```bash
git commit -S -s -m "type(scope): concise summary"
```

## Code comments

- Default to no comments unless a block is genuinely non-obvious.
- Explain why, not what, and keep comments short.
- Never use source comments to narrate work to the user.
- Do not alter unrelated comments.
- Shell snippets may use concise `#` comments where needed for copyability.
