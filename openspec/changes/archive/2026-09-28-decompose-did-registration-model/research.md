# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

The existing source defines bounded public extension data, six opaque
identifier types, secret ownership policy, document operations, action/job
models, five request variants, four lifecycle states, result validation, and
the runtime-neutral registrar port. It is 1,280 physical lines and 1,163
authored nonblank lines.

`identus-did` keeps the module private and explicitly re-exports its public
items from `lib.rs`. It contains no adapter dispatch, persistence, custody,
network transport, synchronization, or executor implementation. Therefore the
safe seam is a private-module move behind the unchanged crate root.

## Normative sources

Issue #406 and Discussion #399 direct the decomposition. Existing DID
registration tests, `did-core`, public-error, input-resource, and code-health
specifications, immutable error evidence, ADR 0115, and the current crate root
own behavior. The module deliberately defines an internal chain-neutral Rust
profile rather than the experimental DIF JSON/HTTP representation; this
routine refactor does not reinterpret that boundary.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private facade plus public-data, request, result, and port modules | `adopt` | Separates four independent change axes while preserving one public vocabulary. | A public namespace or independently versioned package becomes a product requirement. |
| Split only at arbitrary line counts | `not-adopt` | Hides the hotspot without semantic ownership and violates ADR 0115. | Never without a semantic boundary. |
| Export child modules | `not-adopt` | Creates new supported paths without consumer need. | Consumers demonstrate a stable public namespace requirement. |
| Introduce a generic lifecycle framework | `not-adopt` | Weakens reviewability and invents abstraction without another equivalent lifecycle. | A second domain proves the complete shared invariant. |
| Adopt a registration dependency | `not-adopt` | This internal safety profile and adapter boundary are already SDK-owned; ownership-only work needs no dependency. | Dedicated protocol/adoption research proves compatibility and portable-target fit. |

## Compatibility and dependency evidence

The crate root exports 31 registration names and five public limit constants;
those exports remain unchanged. Public names, derives, signatures, constness,
identifier and JSON limits, validation order, error projection, redaction,
iterative cleanup, and registrar semantics remain owned by the same crate. No
manifest, dependency, feature, lockfile, MSRV, unsafe-code, FFI, or target
change is needed.

## Security, privacy and maintenance evidence

Untrusted JSON still passes through the same byte, property, string, item,
depth, node, reserved-name, and private-material checks. Rejection guards and
iterative cleanup remain adjacent to public data. Opaque identifiers, actions,
jobs, requests, results, and failures preserve redacted `Debug` behavior. No
new recursion, allocation policy, secret, logging, I/O, blocking,
synchronization, or panic path is introduced.

Private siblings communicate through narrow `pub(super)` validators; they do
not expose fields or create public forwarding APIs. Each module has one
normative change axis, and the facade only declares and explicitly re-exports
items. No descendant may exceed the attention threshold.

## Rejected or deferred candidates

Arbitrary splitting, public child modules, a generic lifecycle framework, and
a new dependency are rejected. Protocol, adapter, persistence, custody,
cancellation, finality, error, resource-limit, and runtime changes remain
deferred to separately authorized issues.

## Open questions and blockers

There are no planning blockers. Any public API, error, wire, budget,
validation-order, cleanup, adapter/cancellation, manifest, or target drift
blocks this slice. If clean sibling boundaries require behavioral redesign,
the redesign is deferred rather than hidden here.

## Evidence commands

Planning inspected `5c469f7cd5c4ae7b54c54ee5090aa3382a41fbd6`, issue #406,
Discussion #399, `registration.rs`, `lib.rs`, DID registration/registry tests,
code-health policy, and canonical specifications. Before implementation run
`cargo test -p identus-did --all-features`. Afterward run the same suite,
strict crate/workspace Clippy and format, public/error/source/factory contracts,
code-health verification, portable targets, relevant native Nix gates, and
protected exact-head CI. These implementation commands are unrun at planning
time.
