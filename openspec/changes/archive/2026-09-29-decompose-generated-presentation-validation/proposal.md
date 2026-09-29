# Decompose generated-presentation validation

## Why

After #444 isolated disclosure-plan validation, `GeneratedPresentation::new`
remains a concentrated validation signal at 57 SLOC, cognitive complexity 16,
and cyclomatic complexity 23. It combines successful construction with
collection, request, byte-budget, binding, format, duplicate, and coverage
invariants. Issue #450 requires characterization before movement so the code-
health metric does not replace a readable aggregate transition with fragments.

## What changes

- Bind the current exact validation and error-priority matrix before production
  edits.
- Keep the public constructor as the only construction entry point.
- Give deterministic cross-artifact validation one private borrowed owner with
  semantic phases.
- Preserve plan/artifact ownership, caller ordering, bounds, errors, redaction,
  and successful retained values.
- Remove the touched function signal only if the private boundary remains more
  cohesive and reviewable than the current constructor.

## Capability

### Modified capability

- `presentation-module-ownership`: generated presentation construction and
  deterministic artifact validation have explicit private ownership and
  unchanged outward behavior.

## Non-goals

No new presentation format, proof generation, DCQL/OID4VP behavior, artifact
encoding, wallet policy, public helper type, generic validation framework,
dependency, feature, wire form, lifecycle, persistence, or runtime behavior.

## Delivery

Issue #450 owns planning, characterization, implementation, protected evidence,
OpenSpec archive, and metrics. Protected squash delivery requires an
implementation PR followed by a canonical-evidence closeout.

