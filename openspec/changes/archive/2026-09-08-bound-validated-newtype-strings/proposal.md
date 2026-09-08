## Why

Validated `String` newtypes currently clone borrowed input before validation,
so an inherited invalid-input path can allocate attacker-sized text before a
validator rejects it. `DidMethod` compounds that ordering with no standalone
byte ceiling and caller-controlled error detail, leaving a concrete part of
the resource-bound audit in #168 unresolved.

## What Changes

- Validate borrowed input before allocating the owned success value in
  generated validated-string `parse` and `FromStr` implementations.
- Give `DidMethod` an explicit byte ceiling that preserves every method name
  representable inside the existing maximum-size bare `Did`.
- Reject an oversized method before character traversal and make all method
  validation diagnostics independent of caller text.
- Add exact/one-over, validation-precedence, redaction, constructor-path and
  maximum-size DID Registration compatibility evidence.
- Narrow `SDK-LIM-007` with this evidence while retaining its outer transport
  and deserializer allocation limitation.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `domain-newtype-macro`: validated borrowed string parsing validates before
  allocating the owned value used on success.
- `did-core`: DID method names have an explicit compatible byte ceiling and
  validation failures retain no caller-controlled text.

## Impact

The change affects `identus-derive` generated code, `identus-did::DidMethod`,
their focused tests, the DID/newtype canonical specifications, ADR and resource
limitation evidence. Borrowed invalid inputs that previously caused a complete
clone or exceeded any bounded DID become rejected earlier; accepted DID,
registry and Registration behavior is preserved. Owned `TryFrom<String>` and
serde retain their current ownership/allocation model. No dependency, feature,
target, wire, downstream, chain or product surface changes.
