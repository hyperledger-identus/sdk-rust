# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-28
Source retrieval date: 2026-09-28
Research blockers: none

## Problem and existing implementation

The existing source file is 49,313 bytes and 1,345 authored nonblank lines.
It defines three bounded string values, nine standard resolution error kinds,
an RFC 9457-style problem object, two operation metadata wrappers, document
metadata plus builder, resolution and dereferencing result envelopes, bounded
open dereferencing content, extension validation, datetime/media-type syntax,
and raw duplicate-name/depth/node/member/key-budget preflight.

`identus-did` keeps the module private and explicitly re-exports its public
items from `lib.rs`. The file contains no resolver trait, network operation,
cache, synchronization primitive, or executor binding. Resolver orchestration
uses only the crate-private `standard_resolution_failure` helper. Therefore the
safe seam is a private-module move behind the unchanged crate root, not a new
public namespace or abstraction.

## Normative sources

Issue #405 and Discussion #399 direct the decomposition. Issue #50 explicitly
retains duplicate-safe concurrent fills and defers single-flight until multiple
consumers prove a shared need. Existing DID Resolution and dereferencing tests,
the `did-core`, `did-resolution-http`, public-error, input-resource, and
code-health specifications, immutable error evidence, ADR 0115, and the current
crate root own behavior. The implementation comments pin the relevant W3C DID
Resolution Candidate Recommendation Draft; this routine refactor does not
reinterpret or update that protocol baseline.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private facade plus six responsibility modules | `adopt` | Separates value syntax, operation metadata, document metadata, resolution envelopes, dereferencing envelopes, and raw-wire preflight while preserving one public surface. | A public submodule or independently versioned package becomes a product requirement. |
| Split only at arbitrary line counts | `not-adopt` | Hides the hotspot without creating semantic ownership and violates ADR 0115. | Never without a semantic boundary. |
| Introduce a generic result-envelope framework | `not-adopt` | Resolution and dereferencing have different state invariants and content models; generic indirection would weaken reviewability. | A third proven envelope shares the full invariant, not just syntax. |
| Move wire validation into `wire_json.rs` | `not-adopt` | The scanner is shared, but DID-specific budgets and error projection belong to the resolution boundary. | Multiple domain modules require the same complete policy and error mapping. |
| Add a datetime or media-type dependency | `not-adopt` | This slice changes ownership only; dependency adoption requires separate compatibility and dependency-cone research. | A dedicated issue proves better protocol fidelity and portable-target fit. |
| Implement single-flight while splitting | `not-adopt` | It changes cancellation and concurrency semantics and is explicitly deferred to #50. | Issue #50's multi-consumer trigger is met. |

## Rejected or deferred candidates

Arbitrary file splitting, a generic envelope framework, moving DID-specific
policy into the shared scanner, new parsing dependencies, and single-flight
coordination are rejected for this slice. Protocol, public API, resource-limit,
error, cache, and runtime changes remain deferred to separately authorized
issues with their own compatibility evidence.

## Compatibility and dependency evidence

The crate root currently exports 21 resolution names: nine data/model types,
three scalar types, and nine limit constants. Those exports remain unchanged.
All public names, derives, method signatures, const qualifications, serialized
member names, nullable handling, default/empty behavior, validation ordering,
error projections, iterative hostile-JSON cleanup, and standard failure
construction remain owned by the same crate. No manifest, dependency, feature,
lockfile, MSRV, unsafe-code, FFI, or target change is needed.

Exact-head inspection found no duplicated serde attribute, documentation
sentence, or validation call in the current merged base. No incidental cleanup
is therefore part of this slice; existing state-matrix and error-stability
tests bind the ownership-only result.

## Security, privacy and maintenance evidence

Untrusted bytes still pass through the same size check, duplicate-name scanner,
depth/node/member/live-key budgets, serde conversion, semantic validation, and
redacted `Error::InvalidResolution` mapping in the same order. Rejection guards
and iterative JSON cleanup remain adjacent to the types whose hostile values
they own. No new allocation, recursion, secret, logging, I/O, blocking,
synchronization, or panic path is introduced.

Private siblings communicate through narrow `pub(super)` validation methods;
they do not expose fields or create public forwarding APIs. Each module has one
normative change axis, and the facade only declares and explicitly re-exports
items. No descendant may exceed the 1,000-line attention threshold.

## Open questions and blockers

There are no planning blockers. The move must not create cyclic ownership or
make shared internals crate-wide. Any public API, error, wire, budget,
validation-order, cache/cancellation, manifest, or target drift blocks this
slice. If clean sibling boundaries require a behavioral redesign, the redesign
is deferred rather than hidden in this refactor.

## Evidence commands

Planning inspected `b92d2ac3576de1b00fd3a7e391ec0d5e8286061b`, issues #405
and #50, `resolution.rs`, `lib.rs`, DID resolution/dereferencing/cache/query
tests, code-health policy, and canonical specifications. Before implementation
run `cargo test -p identus-did --all-features`. Afterward run the same suite,
strict crate/workspace Clippy and format, public/error/source/factory contracts,
code-health report verification, portable target and relevant native Nix gates,
and protected exact-head CI. These implementation commands are unrun at
planning time.
