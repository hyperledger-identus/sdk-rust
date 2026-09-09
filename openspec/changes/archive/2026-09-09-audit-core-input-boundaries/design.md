# Design

## Context

`identus-core` is deliberately small enough for the first post-merge canary.
The implementation already has the desired semantics; the gap is consolidated
inventory and executable negative evidence for generated numeric serde paths.

## Decisions

### Inventory by public input surface

Add a repository document that lists each public core type/trait/alias,
external construction or deserialization path, retained representation,
intrinsic bound, outer obligation and exact evidence reference. Static-only and
no-input surfaces are explicitly classified rather than omitted.

### Test existing numeric behavior

Extend the core time tests to deserialize JSON values just outside the intended
shape: `-1`, `1.5`, and `18446744073709551616`. Both serialized time newtypes
must reject all three. This is regression evidence, not a custom parser or wire
change. Existing maximum-`u64` acceptance should also be proved.

### Keep the limitation honest

Update only the evidence/enforcement wording of `SDK-LIM-007` and the human
constraint guide. The entry stays effective and repository-scoped.

### Run Pi as a bounded worker

After the planning commit passes preflight, invoke Pi via
`./bootstrap.sh --pi` with only the read/edit/bash tools needed for this issue.
Record version, duration, outcome and repository-disk friction without storing
the prompt, transcript, credentials, session or provider billing data.

## Compatibility and security

There is no public or wire behavior change. Tests lock existing serde and `u64`
behavior. No runtime input, credential or prompt enters tracked evidence. The
URL allocation caveat and all non-core audit gaps remain explicit.

## Validation

Run focused core tests and Clippy, factory checks/readiness, exact-diff review
and target routing. Full/slow checks are chosen by the exact diff and remain
weekly/manual unless the router or risk evidence requires them.

## Rollback

Revert the inventory, tests, spec addition and narrowed evidence wording as one
change. Do not remove or weaken the repository-wide limitation.
