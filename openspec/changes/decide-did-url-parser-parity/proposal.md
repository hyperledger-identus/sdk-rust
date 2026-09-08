## Why

ADR 0008 deliberately gave `identus-did` a bounded, dependency-free DID and
DID-URL parser. Issue #159 asks whether `did_url_parser 0.3.0` can replace
those mechanics and reduce locally owned code. A dependency decision must be
based on exact lexical, resource, allocation, target and maintenance evidence:
ordinary happy-path compatibility is insufficient for an untrusted identifier
boundary.

Preliminary inspection found adoption stop signals: the candidate trims input
for validation but stores the untrimmed serialization, accepts non-HEXDIG
percent spellings and trailing-colon identifiers, has no comparable input
limits, copies owned input, exposes unchecked mutation and contains an unsafe
relative-join path. The full attributable corpus must reproduce and classify
those observations before the decision becomes final.

## What changes

- Pin the SDK, candidate, normative and consumer-repository evidence used by
  the comparison.
- Add a reproducible differential corpus covering accepted syntax, negative
  grammar, component views, exact serialization, bounds and consumer shapes.
- Record candidate features, dependency cone, allocation behavior, unsafe
  reach, compiler/target results, tests, strict-lint behavior and maintenance.
- Decide whether to retain ADR 0008 or supersede it. A retain decision updates
  ADR 0008, the dependency portfolio and negative ledger without changing
  production Cargo/Rust code.
- Preserve an explicit reconsideration trigger for a materially corrected
  future candidate rather than permanently rejecting external reuse.

## Capabilities

### Modified capabilities

- `did-core`: require parity and resource evidence before an external grammar
  engine may replace the bounded single-pass DID/DID-URL parser.

### New capabilities

None.

## Non-goals

- No production dependency, parser rewrite, public API or error change.
- No DID method semantics, document-model change, query interpretation,
  normalization, relative-reference API or downstream repository mutation.
- No upstream issue or patch; those require a separately authorized follow-up.

## Delivery

Issue #159 owns this decision under parent #151 and DID epic #5. The issue and
this specification precede committed comparison artifacts. The final decision
requires a distinct exact-diff review, complete factory/Nix checks, hosted
Linux `fast`, DCO, policy and review before merge to `develop`.
