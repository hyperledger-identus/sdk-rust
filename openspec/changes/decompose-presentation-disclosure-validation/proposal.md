# Decompose presentation disclosure validation

## Why

After the module-ownership work in #407, the public
`PresentationDisclosurePlan::new` constructor remains a concentrated validation
signal at 73 SLOC, cognitive complexity 23, and cyclomatic complexity 27. It
combines construction with collection, candidate, duplicate, claim, coverage,
and multiplicity invariants. Issue #444 requires characterization before any
movement so a metric does not replace a readable state transition with helper
fragments.

## What changes

- Bind the current exact validation and error-priority matrix before production
  edits.
- Keep the public constructor as the only construction entry point.
- Give deterministic validation one private context owner with semantic phases.
- Preserve request/candidate ownership, selected-claim ordering, query ordering,
  limits, errors, redaction, and successful retained values.
- Remove the touched function signal only if the private boundary remains more
  cohesive and reviewable than the current constructor.

## Capability

### Modified capability

- `presentation-module-ownership`: disclosure construction and deterministic
  validation have explicit private ownership and unchanged outward behavior.

## Non-goals

No new presentation format, DCQL behavior, proof generation, wallet selection
policy, OID4VP feature, public helper type, generic validation framework,
dependency, feature, wire form, lifecycle, persistence, or runtime behavior.

## Delivery

Issue #444 owns planning, characterization, implementation, protected evidence,
OpenSpec archive, and metrics. Protected squash delivery requires an
implementation PR followed by a canonical-evidence closeout.
