# Exact-diff review

## Scope reviewed

- Base: `3de0a6eac2b12c02ec80f9d0e17bded26648d506`
- Implementation: `f4599a8fabd9c3567801d5c2f982f267c9758302`
- Production diff: `crates/did/src/dereference.rs`
- Characterization diff: `crates/did/tests/did_url_dereferencing.rs`
- Planning and evidence: this OpenSpec change

## Architecture and cohesion

`PreparedRequest` remains the private immutable handoff to the generic
dereferencer. The new private builder owns exactly the mutable state formerly
held as constructor locals. Its phases correspond to stable reasons to change:
caller option projection, complete query application, typed resolution versus
resource parameter projection, and final cross-field validation. It neither
creates a public abstraction nor mixes resolver execution, fragment/service
projection, relative-reference resolution, or method-specific policy into
preparation.

Finding: no blocking architecture or cohesion issue.

## Behavioral and security review

The exact diff preserves accept/extension clone order, verification-
relationship collision priority, complete `BTreeMap` parse before application,
decoded-name traversal, every existing typed validator, `ResolutionOptions`
construction before cross-field checks, and selector-check order. The
characterization combines invalid phases and proves the resolver remains
untouched. Successful recording-resolver coverage proves retained values reach
the same one-call boundary.

Failures remain the private `InvalidDidUrl`/`InvalidOptions` enum and project to
static W3C errors. No caller-controlled DID, parameter, selector, or extension
value enters diagnostics. No new panic, I/O, callback, synchronization, or
unbounded work path was introduced.

Finding: no blocking behavior, resource, privacy, or security issue.

## Rust review

Moving the former locals into one private owned struct makes the mutation
boundary explicit. The fieldless private enum makes resolution-parameter
dispatch exhaustive without a panic path or stringly unreachable branch.
Ownership transfer avoids new cloning; fallible typed construction and early
returns remain idiomatic. All private phase functions stay below configured
health thresholds.

Finding: no blocking Rust correctness or maintainability issue.

## Residual limitations

- Query parameters intentionally remain collected into a `BTreeMap`; decoded-
  name sorting and duplicate rejection are established security behavior, not
  source-order semantics.
- The unchanged `dereference_services` function retains its pre-existing
  cyclomatic attention signal at 66 / 8 / 20. Service projection is outside
  request preparation and is not claimed as an improvement.
- Canonical code-health evidence must be rebound after protected squash merge.

## Decision

Local review passed. The exact diff is suitable for protected implementation
delivery, followed by a distinct canonical-evidence closeout.
