# Align generic DID bounds with PRISM long-form identifiers

## Why

The generic `identus-did` lexical ceiling is 2,048 bytes, but the pinned PRISM
method specification permits create operations whose base64url-encoded state
is much larger. Rejecting those identifiers would make the planned
`identus-prism-did` crate non-conformant before method validation begins.
Removing or casually widening all related limits would instead weaken the
SDK's denial-of-service posture.

## What changes

- Raise the generic bare-DID ceiling to 131,072 bytes and both DID URL and
  absolute URI ceilings to 139,264 bytes. This preserves the invariant that
  converting any valid DID URL into the URI used by document IDs cannot fail
  or construct an out-of-policy value.
- Keep the standalone method-name ceiling at 2,042 bytes; it no longer derives
  from the enclosing DID budget.
- Keep the 65,536-byte resolution-cache key retention ceiling. A valid request
  whose normalized key exceeds it bypasses caching observably under either
  cache backend failure mode.
- Keep generic DID-document and resolution-result JSON ingress limits
  unchanged. PRISM projection and serialization receive method-specific
  budgets in issue #521.
- Update native, WASM, UniFFI, fuzz, cross-language, adapter, and architecture
  evidence atomically.

## Capabilities

### Modified capabilities

- `did-core`: bounded lexical parsing, standalone method-name policy, cache
  identity behavior, adapter limits, and portable boundary evidence.

## Non-goals

No PRISM protobuf, CRUD, state reduction, document projection, Cardano/VDR,
HTTP routing, generic JSON-ingress widening, or crypto text-codec change lands
in this slice.

## Delivery

Issue #524 owns this prerequisite and blocks #521 and #523. The change is
intentionally small because the public resource policy is a distinct rollback
boundary.
