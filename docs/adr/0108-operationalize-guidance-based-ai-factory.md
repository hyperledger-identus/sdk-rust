# ADR 0108: operationalize a guidance-based AI software factory

- **Status:** Accepted for implementation
- **Date:** 2026-09-09
- **Decision authority:** standing product mandate and issue #243
- **Related work:** ADRs 0003, 0004 and 0081
- **Reference snapshot:** `MediaNoxLabs/oxid@6b2320d7456cef439779a006740c561c8a4bf6d7`

## Context

The repository already uses issues, OpenSpec, Nix, local review and gated pull
requests, but the operational handoffs are mostly prose. The Obsidian `factory`
vault and Oxid contain useful patterns for bounded agents, recovery and evidence.
They describe a different product and are guidance, not an instruction to copy
versions, topology or release policy.

## Decision

1. Keep OpenSpec as the executable implementation contract. Commit and validate
   it before implementation, then retain an exact-base/exact-head preflight
   receipt with the active change.
2. Use one issue, one focused branch and one mutating managed worktree per slice.
3. Pin the worker runtime through the existing Nix lock. The initial runtime is
   Node 24 and the lock's Pi 0.84.2; tracked settings do not select a personal
   model, provider or credentials.
4. Retain the active-development CI model: one required Ubuntu `fast` lane and
   weekly/manual slow evidence. A deterministic planner records risk routing but
   does not create more per-PR compiler builds.
5. Enforce conventional scoped metadata, issue identity, DCO and OpenPGP both
   locally and from GitHub's hosted commit evidence.
6. Store closed-schema exact-head metrics privately under the Git common
   directory. Only a bounded aggregate and one hidden payload may be published.
7. Treat configuration mutations, worktree removal, remote publication and
   merge as explicit authority-bearing operations.
8. Run a separate issue-backed Pi canary after this factory reaches `develop`.
   Tune the harness only from observed friction in a follow-up contract.

## Consequences

- Agents receive a reproducible entry point and durable implementation boundary.
- Repository policy remains authoritative over vault notes and donor defaults.
- The fast path does not gain MSRV, nightly or cross-platform per-PR lanes.
- Pi policy may preserve unrelated user configuration, but authentication is
  never read, copied or written by repository tooling.
- The first version is intentionally conservative and may be tuned after the
  canary rather than speculatively expanded.

## Rejected alternatives

- Copying Oxid wholesale would import product-specific assumptions and version
  changes without SDK evidence.
- Treating vault notes as enforced upgrades would bypass dependency and Nix
  review.
- Running three Rust versions per PR would contradict ADR 0081's temporary
  active-development policy.
- Keeping pre-implementation readiness only in chat history would not be
  auditable after agent or process recovery.

## Rollback

Revert the factory commit. Existing Cargo, Nix, OpenSpec and `develop` delivery
remain usable; no product data, release state or `main` history is migrated.
