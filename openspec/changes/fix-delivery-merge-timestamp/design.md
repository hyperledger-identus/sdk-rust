# Design

## Context

GitHub emits a valid UTC timestamp without fractional seconds. JavaScript's
canonical serializer always emits milliseconds, so textual round-trip equality
is the wrong validation rule.

## Decision

Match a closed UTC RFC 3339 grammar with four-digit year, fixed-width calendar
and time fields, optional one-to-nine-digit fractional seconds, and uppercase
`Z`. Parse the value, reject non-finite results, then compare its UTC calendar
and whole-second components with the captured fields. This accepts precision
differences without accepting normalization of an invalid date.

The receipt retains the exact hosted string rather than rewriting it. Existing
post-merge head, base, commit, signature, message, required-check, and immutable
receipt checks remain unchanged.

## Tests

Exercise the live second-precision value, millisecond and nanosecond fractions,
and reject invalid leap/day/hour values, offsets, missing/lowercase zone,
excess fraction digits, whitespace, and trailing content. Re-run the existing
merge state-machine and receipt tests.

## Rollback

Revert the parser and tests. Existing receipts remain valid data, but GitHub
seconds-form recovery will fail closed until another compatible parser lands.
