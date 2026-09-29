# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-29
Source retrieval date: 2026-09-29
Research blockers: none

## Problem and existing implementation

At protected `develop@90f1c5f0dd51ca4d49615acbb2a4e85a1a079c7e`,
`decode_resolution_options` first derives `ResolutionOptions.accept` from the
already negotiated representation. An absent or empty query constructs empty
options. A present query then follows this exact order:

1. reject raw query bytes above 8 KiB;
2. before each member, reject an index at or above 32;
3. require one raw `=` split;
4. strictly percent-decode the name under 256 decoded bytes;
5. strictly percent-decode the value under 4,096 decoded bytes;
6. reject empty/control-bearing names, control-bearing values, then duplicate
   decoded names in that short-circuit order;
7. reject query-owned `accept`, parse exact booleans and typed version fields,
   or retain an unknown name/value as a string extension;
8. reject simultaneous `versionId` and `versionTime`; and
9. construct `ResolutionOptions`, mapping any final invariant failure to
   `invalidOptions`.

Path validation and Accept negotiation occur before this function. Every
query rejection occurs before the object-safe resolver is called. The current
implementation makes one source-order scan, allocates decoded name/value
strings, clones each accepted name once for the duplicate set, and uses one
ordered extension map.

The canonical signal is `decode_resolution_options`: 78 / 13 / 32.
`did-resolver-http/src/lib.rs` is below the module threshold and has no module
signal. Existing tests bind valid exact common/extension projection,
literal-plus and percent-decoded delimiter semantics, exact raw and component
ceilings, duplicates, malformed values, representation merging, redacted
responses, and resolver non-invocation, but not a compact combined-fault matrix
across all collection and finalization phases.

## Normative sources

Issue #473, Discussion #399, the canonical `did-resolution-http`, DID query
option, input-resource, public-error, dependency-boundary, code-health, and
spec-driven-delivery contracts are authoritative. The existing W3C DID
Resolution HTTP binding behavior and URI-query semantics remain unchanged.
This slice adds no capability or dependency, so no new external-library
research is required.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| One private query-field owner plus the existing coordinator | `adopt` | Separates partial decoded-name/option state and final correlation from representation/empty-query projection while retaining one source-order owner. | Characterization requires duplicated policy, altered query behavior, or an equivalent hotspot. |
| Keep the function intact and document an exception | `not-adopt` | Correctness remains the mandatory fallback if the proposed ownership is less clear. | Stop/go review rejects the extraction. |
| One helper per query option | `not-adopt` | A forwarding graph would hide the complete closed option vocabulary and move metrics mechanically. | Never as a metric-only technique. |
| Adopt a generic URL/form decoder | `not-adopt` | Common form semantics may translate `+`, accept repetitions, or allocate before SDK ceilings. | A separate assessment proves exact strict URI-query parity and lower maintenance risk. |
| Move parsing into `identus-did` | `not-adopt` | Transport query grammar and HTTP-controlled `accept` belong to the outer adapter. | A separately approved transport-neutral query capability proves consumer demand. |

## Rejected or deferred candidates

POST, DID URL dereferencing, server/runtime, middleware, authorization, generic
form parsing, public parser mechanics, new query options, query-controlled
`accept`, and unrelated content negotiation/response projection changes are
rejected or deferred to separately approved capability work.

## Compatibility and dependency evidence

The Axum router and handler, public exports, `ResolutionOptions`, accepted and
rejected query bytes, W3C error/status projection, representation behavior,
resolver invocation count, bounds, retained values, and redacted diagnostics
remain unchanged. No manifest, lockfile, feature, dependency, MSRV, unsafe,
native, FFI, wire, allocation-class, or target change is needed.

## Security, privacy and maintenance evidence

The same raw/member/name/value ceilings, strict single percent-decoding pass,
literal-plus behavior, control and duplicate rejection, typed validators,
mutually exclusive version fields, and final validated constructor remain
authoritative. The owner exists only during request parsing and holds exactly
the current set/map/options. It adds no caller-text diagnostic, unbounded
collection, dynamic dispatch, synchronization, ambient I/O, recursion, trust
claim, or resolver call on rejection.

## Open questions and blockers

There are no planning blockers. Characterization is a stop/go gate. If one
private query owner cannot preserve the exact first error, resolver
non-invocation, and allocation behavior without an equivalent hotspot or
helper-per-option chain, production remains unchanged and the signal receives
a measured exception.

## Evidence commands

Before production edits, run the complete DID Resolution HTTP suite and add the
combined-fault/resolver-call matrix. Afterward run focused/workspace tests,
strict Clippy/format/docs, public/source/code-health/factory checks, source
distribution, portable targets, MSRV and canonical Nix gates, distinct
exact-diff review, and protected exact-head CI.
