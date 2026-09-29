# Local review

Review date: 2026-09-29
Review scope: production implementation `ec0838fd3d1a416cbf1fabf616fe3cb59d7e93dc`
Review result: passed

## Architecture and cohesion

`ResolutionQueryFields` is one private partial-state owner for decoded query
names, typed common options, version selectors, and extension values. Its three
operations match real phases: consume one source-order parameter, project that
decoded member into typed state, and finalize the public `ResolutionOptions`.
The outer decoder retains representation derivation, empty-query behavior, raw
byte policy, parameter count, and source-order iteration. The boundary neither
creates a generic query framework nor one helper per option.

## Behavioral and resource review

- Representation parsing still precedes empty-query and query validation.
- Raw byte and parameter count limits remain in the coordinator and execute
  before per-parameter decoding at their original points.
- Name decoding, value decoding, empty/control checks, decoded-name duplicate
  detection, known-option parsing, extension retention, version conflict, and
  final construction preserve source order and static errors.
- The private owner contains the same `BTreeSet`, four optional typed values,
  and `BTreeMap`; empty queries still avoid constructing it. No new clone,
  collection, callback, dynamic dispatch, I/O, dependency, or unbounded path
  was introduced.
- Rejected query content remains absent from responses and diagnostics; the
  resolver remains uncalled for every characterized combined fault.

## Rust and maintainability review

The original 78-SLOC / cognitive-13 / cyclomatic-32 decoder signal is removed.
None of the coordinator or private-owner methods creates a replacement signal.
Ownership is explicit, moves remain single-use, error mapping is local, and
Clippy reports no warning. The unrelated existing media-negotiation signal is
left visible for a separately justified slice.

## Findings

No blocking finding remains. Further splitting would weaken cohesion by
turning closed option projection into helper-per-match-arm forwarding.
