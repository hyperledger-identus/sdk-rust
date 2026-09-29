# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

At protected `develop@1a3aca558734cd317860862a7a91284cf5991b18`, the
public consuming method first obtains expected state and selected Authorization
Server metadata, parses the complete bounded query, compares state, applies the
advertised RFC 9207 issuer rule, then selects exactly one success/error branch
and validates its field grammar before constructing a redacted owned outcome.

The private `parse_query` rejects empty/full-callback shapes and encoded size
before allocating, applies the parameter-count bound during one split scan,
strict-form-decodes bounded names, rejects decoded duplicates, selects the
role-specific value ceiling, decodes the value, retains six known roles in
zeroizing storage, and validates/discards unknown unique values. Query failures
therefore precede state, issuer, branch, and value-grammar failures. State
precedes issuer; both precede success/error shape. Success code grammar and
error code/description/URI grammar are evaluated only inside the selected
exclusive branch.

Existing tests cover valid success/error outcomes, exact RFC 9207 behavior,
state and issuer rejection, ambiguous branches, duplicate/malformed queries,
independent limits, role grammars, and static redacted diagnostics. They do not
bind one compact combined-fault matrix across all four phases before
decomposition.

The canonical signals are:

- `AuthorizationRequest::try_into_authorization_response`: 78 / 21 / 25;
- `parse_query`: 75 / 16 / 25.

The 465-line module is below the attention threshold, and no production module
exceeds 1,000 authored nonblank lines.

## Normative sources

Issue #464, Discussion #399, ADR 0143, the archived issue #356 research and
verification, the canonical Authorization Response/error specifications, and
the code-health, input-resource, dependency-boundary, and spec-driven-delivery
contracts are authoritative. The original Final OpenID4VCI, RFC 6749, RFC 9207,
and RFC 9700 decisions remain unchanged. No new external protocol or crate
research is required because this slice changes no grammar, policy, dependency,
wire format, or interoperability decision.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Private bounded query decoder plus private request-bound correlator | `adopt` | The two owners align with untrusted parsing versus authority transition and can preserve exact phase order. | Characterization requires public exposure, duplicated grammar, extra allocation, or reordered failures. |
| Keep both functions intact and document exceptions | `not-adopt` | Correctness takes priority and remains the fallback if owners obscure the protocol sequence. | Stop/go review rejects the decomposition. |
| One helper per field or conditional | `not-adopt` | Mechanical forwarding would displace metrics without semantic ownership. | Never as a metric-only technique. |
| Replace strict-form decoding with an OAuth crate | `not-adopt` | ADR 0143 already rejects the broad dependency cone for this bounded exact parser. | A separate ADR proves exact behavior and multi-capability payoff. |
| Add response modes, fields, or trust policy | `not-adopt` | Product/protocol growth is outside this refactor. | Separately approved capability evidence requires it. |

## Compatibility and dependency evidence

The public method, types, paths, limits, exact errors, error order, response
values, request consumption, zeroizing ownership, issuer evidence, debug
redaction, and Final-facing behavior remain unchanged. No manifest, lockfile,
MSRV, unsafe, native, FFI, serialization, wire, feature, or target change is
needed.

## Security, privacy and maintenance evidence

The query decoder retains the same encoded and decoded bounds, single pass,
decoded duplicate set, role-specific ceilings, and zeroizing values. The
correlator owns the consumed request and parsed fields and preserves
parse-before-correlation and correlation-before-outcome authority. No caller
value enters diagnostics. No callback, trait object, executor, synchronization,
hashing, ambient I/O, or allocation class is added.

## Rejected or deferred candidates

Full callback URIs, fragment/form-post/JARM modes, state persistence, browser or
HTTP execution, code exchange, issuer-policy expansion, new fields, external
OAuth dependencies, and performance thresholds are rejected or deferred.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate. If two
private owners cannot preserve exact query-before-state-before-issuer-before-
branch/grammar precedence without duplicating policy or adding allocations,
production remains unchanged and the signals receive measured exceptions.

## Evidence commands

Planning inspected the exact protected base, issue #464, issue #356 evidence,
current implementation and complete focused tests, canonical report, manifests,
and governing specs. Before production edits, run the focused Authorization
Response suite and add the combined-fault matrix. Afterward run focused/
workspace tests, strict Clippy/format/docs, public/source/code-health/factory
checks, portable targets, relevant Nix gates, distinct exact-diff review, and
protected exact-head CI.
