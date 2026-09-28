# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-28
Source retrieval date: 2026-09-28
Research blockers: none

## Problem and existing implementation

The existing 1,616-line file has 1,445 authored nonblank lines, one public
JSON-depth ceiling, and 23 public limit structs. Each struct owns validation,
accessors, and usually a `Default` implementation with inline numeric values.
There are no tests or independent transport/parser mechanics in the file;
characterization lives with the public consumers. The problem is mixed
lifecycle ownership and audit concentration, not missing resource policy.

The public crate root explicitly re-exports all 23 types and the depth
ceiling. Consumers construct these types, call their accessors, and rely on
the existing defaults and static `CredentialOfferError` variants. Therefore a
safe refactor must preserve the crate-root surface and must not turn public
policy into a new generic abstraction.

## Normative sources

Issue #404 and Discussion #399 direct this decomposition. Closed issue #168,
the input-resource inventory, `SDK-SEC-003`, and `SDK-LIM-007` retain authority
over the meaning and external obligations of resource limits. ADR 0115 and the
code-health specification require semantic decomposition and reject arbitrary
file splitting, forwarding layers, and metric gaming. Existing OID4VCI
OpenSpec capabilities and immutable error goldens own behavior.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Central numeric policy plus four lifecycle modules | `adopt` | Keeps all effective defaults auditable in one place while aligning type mechanics with independent protocol change axes. | A lifecycle requires independently versioned policy or the public surface is redesigned under separate authority. |
| Split types into modules with inline numbers | `not-adopt` | Reduces file size but distributes security policy, contrary to issue #404. | Never without explicit policy-ownership authority. |
| Keep the aggregate file | `not-adopt` | Preserves centrality but leaves four unrelated change axes in one review hotspot. | The protocol lifecycles converge into one normative object family. |
| Generate types/defaults with a macro | `not-adopt` | Hides validation and public API details, complicating review for little reuse. | Repeated semantics, not syntax alone, become a proven maintenance defect. |
| Introduce a generic runtime limit map | `not-adopt` | Weakens typed construction, adds lookups/failure modes, and changes API semantics. | A separately specified dynamic-policy product requirement appears. |

## Rejected or deferred candidates

Inline numeric policy in lifecycle modules, an unchanged aggregate file,
macro-generated public types, and a dynamic limit map are rejected for this
slice. Changing effective values, consolidating currently equal roles,
renaming public types, or redesigning constructor validation is deferred to a
separately authorized compatibility or security decision.

## Compatibility and dependency evidence

`limits.rs` remains the only parent visible to `lib.rs`; private re-exports
preserve every `identus_oid4vci::*Limits` and
`MAX_CONFIGURABLE_JSON_DEPTH` path. Types retain names, fields, derives,
constructors, accessors, `Default` behavior, and composition. No manifest,
feature, dependency, lockfile, MSRV, target, error golden, or release surface
changes. Moving a Rust item between private source modules does not change its
public type identity when the crate-root re-export is unchanged.

## Security, privacy and maintenance evidence

A private `policy` module will be the sole definition site for every numeric
default and the public maximum JSON depth. Lifecycle modules reference named
constants and retain exact constructor validation. No new parser, allocation,
secret, unsafe code, native code, or dynamic configuration is introduced.
Review can audit numeric policy independently from public type mechanics and
can review each protocol lifecycle without scanning unrelated types.

## Open questions and blockers

There are no blockers. The implementation must not infer equality merely from
compilation: review must compare every base/head public signature, default
field value, validation predicate, and error variant. Any numeric, public API,
diagnostic, manifest, or consumer change blocks this slice.

## Evidence commands

Planning inspected revision `3ff99f833bd0c49aa0c34ec94debd835080932ab`,
issue #404, historical issue #168, all declarations/defaults in `limits.rs`,
crate-root exports, consumer construction sites, the input-resource inventory,
and code-health policy. Before implementation run
`cargo test -p identus-oid4vci --all-features`. Afterward run the same suite,
strict Clippy/format, immutable error and factory checks, code-health report
verification, relevant native Nix gates, and protected exact-head CI. These
implementation commands are unrun at planning time.
