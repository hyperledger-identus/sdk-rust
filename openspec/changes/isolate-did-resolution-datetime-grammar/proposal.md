# Isolate DID resolution datetime grammar

## Why

No production module exceeds the 1,000 authored-nonblank-line attention
threshold, but `validate_datetime` remains an unowned function-level signal at
71 SLOC, cognitive complexity 17, and cyclomatic complexity 40. It currently
combines bounded lexical recognition, extended-year rules, numeric projection,
Gregorian leap calculation, calendar validation, and end-of-day validation.
Issue #456 requires characterization before movement so code-health work cannot
change the already released `DidResolutionDateTime` language.

## What changes

- Bind the complete accepted/rejected datetime grammar across every public
  constructor and Serde entry point before production edits.
- Keep one private parsed representation for the five calendar/time fields and
  the year modulo 400 needed for leap-day validation.
- Separate bounded lexical recognition from calendar/time validation without
  changing exact spelling, allocation ownership, or error projection.
- Preserve ASCII, byte, `Z`, whole-second, signed/extended-year, leap-year,
  calendar-day, and `24:00:00` behavior.
- Remove the touched function signal only if the resulting private ownership is
  more cohesive than the current validator.

## Capability

### Modified capability

- `did-core`: DID resolution datetime validation has explicit private lexical
  and calendar ownership while its public value contract remains unchanged.

## Non-goals

No new datetime syntax, normalization, time-zone database, numeric timestamp,
fraction, offset, dependency, feature, public helper, wire form, metadata
field, parser framework, or performance budget.

## Delivery

Issue #456 owns planning, characterization, implementation, protected evidence,
OpenSpec archive, and metrics. Protected squash delivery requires an
implementation PR followed by a canonical-evidence closeout.
