# ADR 0166: evaluate pull-request policy from the protected base

- **Status:** Accepted under issue #341
- **Date:** 2026-09-29
- **Issue:** [#341](https://github.com/hyperledger-identus/sdk-rust/issues/341)

## Context

The hosted pull-request policy executes checker code and configuration from the
pull-request merge tree. A contribution can therefore change the mechanism
that judges the same contribution. Merely selecting a base file after checkout
is insufficient because the workflow definition itself is also part of the
trust root.

## Decision

Use `pull_request_target` solely as a read-only base-context evaluator. Check
out and verify the event's exact base SHA with persisted credentials disabled.
Run only base-owned repository code. Fetch the numbered pull-request head ref
only as Git objects, verify it equals the exact event head SHA, and treat all
head content and metadata as untrusted data. Never check out, import, install,
source, build, or execute the head tree in this job.

Retain immutable third-party action pins, explicit read-only permissions, and
no secrets. Policy monotonicity between the base and proposed head remains the
separate #339 decision.

## Consequences

A pull request cannot pass by weakening its own checker, configuration, or
workflow. The event has a more sensitive base-context token, so the no-head-
execution and least-authority rules are permanent security invariants. The PR
that introduces this change cannot naturally exercise its new base workflow;
the next pull request supplies that hosted evidence.

## Verification and rollback

Mutation-resistant source tests assert the event, exact SHA bindings,
credential behavior, permissions, and absence of head checkout. Existing
policy tests, actionlint, factory validation, signed/DCO review, and hosted CI
remain required. Rollback is a normal repository revert and changes no SDK or
consumer API.
