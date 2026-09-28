# Decompose OID4VCI bounded JSON parsing

## Why

`crates/oid4vci/src/json.rs` owns 2,083 authored nonblank lines spanning a
shared bounded JSON scanner and independently changing Credential Offer,
metadata, token/nonce, and credential-response grammars. The audit in
Discussion #399 and issue #403 identify this as the highest-value production
module decomposition candidate. The current single file obscures ownership
without making the protocol parsers reusable.

## What changes

- Keep one private scanner and root-object ingress contract for node/depth,
  duplicate-name, complete-input, redaction, and iterative traversal rules.
- Move protocol-object field records and parsing methods into four private,
  cohesive modules: offer/grants, metadata, token/nonce, and credential
  responses.
- Preserve every public API, accepted/rejected wire shape, limit source,
  error variant and precedence, zeroizing owner, dependency, and target.
- Prove equivalence through the existing characterization/negative suite,
  code-health evidence, strict Rust gates, and protected CI.

## Capability

### Added capability

- `oid4vci-bounded-json-ingress`: records the private bounded ingress and
  protocol-parser ownership contract that the existing public types depend on.

## Non-goals

This change does not alter OID4VCI behavior or resource limits, replace the
scanner, introduce a generic JSON framework, share parsers with OID4VP, expose
parser internals, add dependencies, or refactor `limits.rs`.

## Delivery

Issue #403 owns this focused slice. Planning and exact-head preflight precede
all moves. Existing parser tests run before the move and again afterward; the
final code-health report must remove `json.rs` from the over-1,000-line list
without disguising the same mixed responsibility behind forwarding wrappers.
