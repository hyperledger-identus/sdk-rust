# Isolate OID4VCI Authorization Request assembly

## Why

The module-decomposition milestone removed every production module above 1,000
authored nonblank lines. The Authorization Request transition retains an
unowned 60 SLOC / cognitive 7 / cyclomatic 18 signal. It combines endpoint and
existing-query preparation, Authorization Details construction, optional
issuer-state projection, checked sizing, and exact form rendering. Issue #467
requires characterization before movement because their error order and wire
bytes are externally observable security behavior.

## What changes

- Bind endpoint-query, Authorization Details, and final-size precedence before
  production edits.
- Keep `try_into_authorization_request` as the unchanged consuming public entry
  point.
- Introduce one private request-assembly owner with semantic preparation,
  checked sizing, and rendering phases while retaining existing form/JSON
  helpers.
- Preserve endpoint query bytes, managed parameter order, conditional
  `locations`, issuer state, strict collision rules, all limits, checked
  arithmetic, one exact allocation, zeroizing ownership, and static errors.
- Remove the touched signal only if the owner remains more cohesive and easier
  to review than the current method.

## Capability

### Modified capability

- `oid4vci-authorization-request`: exact request assembly gains explicit
  private ownership with unchanged outward protocol behavior.

## Non-goals

No scope/resource/PAR/JAR field, browser or HTTP execution, callback behavior,
response handling, client authentication, trust policy, public helper,
dependency, feature, wire, error, or resource-budget change.

## Delivery

Issue #467 owns planning, characterization, implementation, protected evidence,
OpenSpec archive, and metrics. Protected squash delivery requires an
implementation PR followed by a canonical-evidence closeout.
