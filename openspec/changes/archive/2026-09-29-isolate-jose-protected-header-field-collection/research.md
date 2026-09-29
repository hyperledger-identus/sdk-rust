# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

At protected `develop@3b5c91d384756fd0ef1ee36ef3e66f051f5950ba`,
`serde_json` bounds one complete decoded header before
`RawProtectedHeaderVisitor::visit_map` consumes entries. The visitor recognizes
seven closed names, rejects unknown names immediately, detects duplicates while
decoding each known value, requires `alg` after the map ends, rejects more than
one of `kid`/`jwk`/`x5c`, and constructs one private raw header. A later
validated constructor enforces algorithm, string, public-JWK, chain, and
untrusted-evidence policy under `JwsLimits`.

The current sequential priority is therefore input order for duplicate,
unknown, and invalid typed values; after complete member collection, missing
`alg` precedes ambiguous key-reference correlation. Existing tests cover each
error family separately but do not bind combined faults that cross these
phases.

The canonical signal is `visit_map`: 67 / 13 / 31. `header.rs` is below the
module threshold and has no module signal.

## Normative sources

Issue #470, Discussion #399, ADR 0159, the canonical `jws-compact`,
`jose-module-boundary`, `jose-error-contracts`, dependency-boundary,
code-health, and spec-driven-delivery contracts are authoritative. RFC 7515
behavior and the existing strict closed profile remain unchanged. This slice
adds no capability or dependency, so no new external-library research is
required. The prior JOSE ecosystem decision remains: engines may be evaluated
operation by operation, but public types and permissive/broad policy do not
replace this facade without a separate ADR and exact parity evidence.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Closed private member enum plus raw-field collector | `adopt` | Separates name vocabulary and accumulated invariants from Serde iteration while retaining one reviewable field owner. | Characterization requires duplicated policy, altered Serde behavior, or an equivalent hotspot. |
| Keep the visitor intact and document an exception | `not-adopt` | Correctness remains the mandatory fallback if the proposed ownership is less clear. | Stop/go review rejects the extraction. |
| One helper per JSON member | `not-adopt` | A forwarding graph would hide the complete closed grammar and move metrics mechanically. | Never as a metric-only technique. |
| Public raw-header builder or intermediate | `not-adopt` | Exposes parser mechanics and invalid partial states with no consumer need. | A separately approved public capability proves demand. |
| Adopt a broad JOSE/JWT crate | `not-adopt` | ADR 0159 found no exact bounded, strict, portable, low-coupling replacement. | Separate issue/ADR proves exact facade parity and deleted maintenance risk. |

## Rejected or deferred candidates

New algorithms, JWE/general JWT support, public parser mechanics,
certificate/attestation/federation validation, OIDC/OID4VCI policy, a broad
third-party JOSE type system, and unrelated header serialization changes are
rejected or deferred to separately approved capability or dependency work.

## Compatibility and dependency evidence

The public types, paths, constructors, serialization order, accepted/rejected
JSON, static errors, marker-to-error mapping, limits, chain bounds, retained
values, and redacted Debug forms remain unchanged. No manifest, lockfile,
feature, dependency, MSRV, unsafe, native, FFI, wire, or target change is
needed.

## Security, privacy and maintenance evidence

The same bounded Serde input, duplicate detection, closed vocabulary, typed
decoders, public-only JWK validation, chain cardinality, required `alg`, and
exclusive key-reference rules remain authoritative. The collector exists only
during parsing and owns exactly the same optional fields. It adds no copy,
unbounded collection, caller-text diagnostic, dynamic dispatch, synchronization,
ambient I/O, recursion, or trust claim.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate. If the
private member vocabulary plus raw-field collector cannot preserve the exact
first error and allocation behavior without an equivalent hotspot, production
remains unchanged and the signal receives a measured exception.

## Evidence commands

Before production edits, run the complete JOSE compact suite and add the
combined-fault matrix. Afterward run focused/workspace tests, strict
Clippy/format/docs, public/source/code-health/factory checks, portable targets,
MSRV and canonical Nix gates, distinct exact-diff review, and protected
exact-head CI.
