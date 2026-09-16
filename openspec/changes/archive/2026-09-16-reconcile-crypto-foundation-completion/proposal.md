# Reconcile cryptography foundation completion

## Why

Apollo parity closed with zero capability gaps and every bounded JOSE child of
epic #8 is complete, but canonical row IDR-004 remains `in_progress`. Issue
#286 requires one exact audit that distinguishes delivered chain-neutral
behavior from open hardening, release, bindings, adoption, and downstream
lifecycle work.

## What changes

- Add a durable completion report mapping every IDR-004 outcome and evidence
  class to immutable revisions, limitations, and owning follow-ups.
- Mark IDR-004 delivered only if the report proves its functional and
  conformance acceptance criteria without borrowing release/adoption claims.
- Reconcile the roadmap and B08 blueprint state with the completed JOSE child
  issues.
- After merge, update Apollo parity discussion #178 and close or truthfully
  narrow epic #8 using the merged report.

## Capabilities

### Modified capabilities

- `ssi-upstream-program`: defines the evidence boundary for an IDR-004
  delivered state while retaining separate release and downstream gates.

## Non-goals

- No new primitive, algorithm, dependency, API, vector, fuzz target, binding,
  package, publication, consumer mutation, Apollo lifecycle, or support tier.
- No closure of #298, #299, #163, IDR-011, or NeoPRISM PR #324 by assertion.
- No change to `main` or release authority.

## Delivery

Issue #286 owns this documentation/governance slice from protected
`develop@fb2cc64ebe09dc382de9d903a6136f7aa8132c47`.
