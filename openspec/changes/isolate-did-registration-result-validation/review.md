# Exact-diff review

## Scope reviewed

- Base: `f7a0d58b534a0af2a5cd135fa2bab71f77195991`
- Implementation: `b95db35942a33678068b1005ef2530958bb36819`
- Production diff: `crates/did/src/registration/result.rs`
- Characterization diff: `crates/did/tests/did_registration.rs`
- Planning and evidence: this OpenSpec change

## Architecture and cohesion

`DidRegistrationResult::new` remains the sole public construction boundary.
The private validator borrows exactly its four validation inputs and owns one
ordered lifecycle decision. Job-method correlation, exhaustive state/job
dispatch, substantive terminal/action/wait invariants, and document-metadata
policy now have distinct reasons to change without becoming public or moving
registration policy into a generic framework.

The variant methods are not helper-per-conditional movement: each owns a
complete public error projection and all predicates intentionally collapsed
behind it. Request correlation, request construction, public-data ingestion,
and registrar execution remain in their existing owners.

Finding: no blocking architecture or cohesion issue.

## Behavioral and security review

The exact diff retains job-method checking before state inspection; the same
exhaustive `(state, job)` match; finished handle, identity, and public-document
order; combined action and wait predicates; state success before metadata;
metadata failures collapsed to `MethodOrDidMismatch`; and final cloned
extension validation. The characterization proves exact errors when multiple
phases fail and covers every state/job-presence shape.

All inputs remain borrowed validated SDK values or pre-existing bounded
collections/documents. Diagnostics remain static and redact caller-controlled
values. No new panic, recursion, I/O, callback, synchronization, secret
retention, or unbounded work path was introduced.

Finding: no blocking behavior, resource, privacy, or security issue.

## Rust review

One field-only borrowed struct makes the validation context explicit. Methods
return ordinary `Result` values, exhaustive enum matching remains visible, and
lifetimes cannot escape the call. `Option::as_deref` removes an unnecessary
boxed-document layer without cloning, and the existing extension clone occurs
in the same final phase. Every private method remains below configured health
thresholds.

Finding: no blocking Rust correctness or maintainability issue.

## Residual limitations

- Action and wait deliberately retain coarse errors that hide which combined
  predicate failed; changing that is a public error-contract decision.
- Document-metadata validation deliberately maps every resolution-shaped
  failure to registration `MethodOrDidMismatch` before separately applying
  registration extension policy.
- The explicit characterization corpus is verbose by design so first-error
  evidence remains readable without a new test DSL.
- Canonical code-health evidence must be rebound after protected squash merge.

## Decision

Local review passed. The exact diff is suitable for protected implementation
delivery, followed by a distinct canonical-evidence closeout.
