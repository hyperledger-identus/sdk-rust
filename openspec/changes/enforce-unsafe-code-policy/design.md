## Context

Every current package already opts into `[workspace.lints]` through
`[lints] workspace = true`, while the root only denies warnings. Six library
roots independently forbid unsafe code. Those attributes protect their library
crate but do not establish uniform policy for integration tests, examples,
benches, binaries or build scripts, and duplication would drift as the monorepo
grows.

Cargo 1.98.1 accepts `unsafe_code = "forbid"` under
`[workspace.lints.rust]` and forwards it as `-F unsafe-code` to opted-in package
targets. A disposable experiment verified rejection for library, binary,
integration-test, example, benchmark, build-script and proc-macro targets.

## Goals and non-goals

Goals are one fail-closed policy source, complete member inheritance, target
class negative evidence, future-member drift detection, explicit exception
governance and honest retirement of `SDK-LIM-008`. Non-goals are scanning
dependency internals, approving unsafe code, refactoring dependencies that use
unsafe, changing CI cadence, or claiming that a lint proves semantic safety.

## Decisions

### Use workspace `forbid`, not duplicated crate attributes or `deny`

Add `unsafe_code = "forbid"` beside `warnings = "deny"` in the root workspace
lint table. `forbid` is intentionally non-overridable by nested source
attributes. Existing crate-root forbids remain harmless defense in depth; this
change does not create churn by removing them.

### Prove both configuration edges

The compiler rule is effective only when the root declares exact `forbid` and
each package opts into workspace lints. A new `guard/unsafe_policy.rs` test
parses the root plus every manifest returned by the shared crate-manifest
walker and fails with the package/path when either edge is absent or changed.
The current exception set is empty.

### Use disposable compile-fail workspaces as behavioral evidence

A verification-only conformance test creates isolated dependency-free Cargo
workspaces under the process temporary directory, runs the repository Cargo in
offline mode and requires each selected build to fail specifically with the
unsafe-code lint. It covers library, binary, integration-test, example,
benchmark, build-script and proc-macro implementation targets, plus a safe
proc macro whose expansion emits an unsafe block in a consumer.

The fixture source exists only as string data in test code, so the SDK itself
contains no compiled unsafe block. Every temporary path is process-scoped and
removed through an RAII guard. Failure output is bounded before assertion
diagnostics. The probe does not depend on a registry or network.

### Keep exceptions explicit and intentionally non-generic

No exception is approved and no dormant allowlist weakens this delivery. A
future exception requires a new issue and dedicated safety ADR naming:

- exact crate, target and module/function scope;
- why safe Rust or an external audited dependency is insufficient;
- every safety invariant and how tests/Miri/sanitizers or review prove it;
- owner, specialist reviewer, security and maintenance cost;
- target/MSRV/FFI/secret implications, activation and rollback;
- an indexed exception record linked to `SDK-SEC-001` and an atomic update to
  the conformance guard.

Because Cargo workspace lint inheritance cannot be selectively overridden, an
approved implementation must replace inheritance only for the named package,
mirror all unrelated workspace lint levels locally, use `deny(unsafe_code)` at
the package boundary and permit only the named source scope. The guard must
reject every unregistered deviation and any broader `allow`. This deliberate
future change is reviewable rather than silently pre-authorized here.

### Treat current gates as enforcement, not dependency certification

The root `rust-build --workspace --all-targets`, feature builds, Clippy and
nextest lanes compile supported first-party targets with inherited lints.
Dependencies are passed with Cargo's cap-lints behavior and remain governed by
dependency research, audits and facade review. The policy claim is therefore
limited to first-party SDK source.

## Risks and mitigations

- A package can omit lint inheritance: the manifest guard enumerates every
  workspace crate and fails closed.
- A Cargo behavior regression can make configuration inspection misleading:
  behavioral compile-fail probes run in the normal test/Nix path.
- Recursive Cargo tests can become flaky: fixtures are dependency-free,
  offline, process-scoped, separately targeted and share no build directory.
- Compiler output text can drift: assertions require non-zero status and the
  stable lint identifier rather than a complete snapshot.
- An exception could weaken other lints: its future guard must require exact
  mirrored unrelated levels and reject unregistered scope.

## Rollout and rollback

Commit this specification and ADR first. Then add the root lint and guard,
prove positive and negative cases on Rust 1.98.1, run full compatible Nix and
hosted Linux gates, and retire `SDK-LIM-008` in the same PR. Reverting the PR
restores the disclosed limitation and prior manual-review posture; it changes
no runtime data or consumer API.
