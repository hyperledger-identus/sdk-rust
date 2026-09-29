# Exact-diff review

## Scope reviewed

- Base: `8f229759eeb2d2b8a0be8a1949035f6a7533df06`
- Implementation: `0c9c6d11ddaf6dbcc831e20e6bc0526d9331ac06`
- Production diff: `crates/did/src/resolution/value.rs`
- Characterization diff: `crates/did/tests/did_resolution.rs`
- Planning and evidence: this OpenSpec change

## Architecture and cohesion

`DateTime` remains the public validated string newtype. The new private
`ParsedDateTime` owns only fixed lexical fields and the year modulo 400 needed
for calendar validation. Lexical parsing, extended-year grammar, two-digit
projection, and calendar policy now have distinct reasons to change without
creating public abstractions or moving generic calendar policy elsewhere.

Finding: no blocking architecture or cohesion issue.

## Behavioral and security review

The exact diff preserves the minimum and maximum length, ASCII-only bound,
terminal `Z`, signed/extended four-or-more-digit year grammar, fixed separator
positions, numeric field widths, Gregorian leap-year rules, month/day bounds,
normal time range, and exact `24:00:00Z` exception. All rejection paths retain
the static `invalid datetime` message, so caller-controlled text does not enter
diagnostics.

The parser performs bounded slicing only after length and ASCII checks. Exact
slice patterns and checked digit predicates precede arithmetic. No panic, I/O,
callback, synchronization, secret handling, or unbounded work path was added.

Finding: no blocking behavior, resource, privacy, or security issue.

## Rust review

The private parsed value makes the lexical-to-semantic boundary explicit and
avoids cloning or allocation. `split_year` centralizes the only variable-width
field, while `parse_two_digits` owns fixed-width numeric projection. Exhaustive
array matching rejects malformed tails without indexing chains, and every
private function remains below configured health thresholds.

Finding: no blocking Rust correctness or maintainability issue.

## Residual limitations

- The type intentionally validates the repository's existing RFC3339-like
  grammar rather than adopting a broader date-time crate or normalization
  policy.
- Gregorian validity is projected modulo 400; the original year spelling is
  retained unchanged in the public value.
- Canonical code-health evidence must be rebound after protected squash merge.

## Decision

Local review passed. The exact diff is suitable for protected implementation
delivery, followed by a distinct canonical-evidence closeout.
