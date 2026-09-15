# Complete hosted input-boundary review

## Why

Hosted review of PR #296 found that the completed package-level audit omitted
three distinct families: the bounded DID method registry, DID resolution cache
policy versus its injected cache/clock adapters, and non-recursive destruction
of rejected native JWK extension trees. These findings must be resolved before
issue #168 can retire broad incomplete-audit disclosure.

## What changes

- Add separate inventory rows for the DID method registry, portable cache
  policy, and caller-budgeted cache/clock adapters.
- Guard every native JWK constructor exit so rejected recursive JSON is
  dismantled iteratively, including errors that precede budget validation.
- Add hostile-depth regression coverage and re-run the complete candidate,
  factory, Nix, hosted-CI, and review gates.

## Capabilities

### New requirements

- `crypto`: native JWK rejection cleanup is iterative and redaction-safe.
- `sdk-input-resource-governance`: distinct registry, cache-policy, and
  injected-adapter ownership families remain independently visible.

## Non-goals

- No new DID cache, clock, registry, JSON, transport, or cryptographic API.
- No change to accepted JWK budgets, wire forms, dependency cone, or stable
  public errors.
- No concrete cache-adapter QoS policy or downstream migration.

## Delivery

Issue [#168](https://github.com/hyperledger-identus/sdk-rust/issues/168) and PR
[#296](https://github.com/hyperledger-identus/sdk-rust/pull/296) own this review
correction. Integration requires all three hosted review threads resolved and
the protected exact-head checks green.
