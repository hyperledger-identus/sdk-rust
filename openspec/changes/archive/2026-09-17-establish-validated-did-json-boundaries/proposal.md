# Establish validated DID JSON boundaries

## Why

Issue #315 demonstrated that `OneOrMany::try_many` can reject 129 public
`ContextEntry` values before DID JSON validation, then recursively destroy an
attacker-controlled 32,768-level `serde_json::Value`. The path exists because
the generic representation type applies DID-specific cardinality policy while
`ContextEntry::Object` publicly accepts an arbitrary recursive map. The partial
constructor guards prepared for #297 cannot intercept that earlier generic
rejection.

## What changes

- Make `OneOrMany<T>` responsible only for preserving a non-empty scalar or
  array representation; validated DID owners enforce the 128-item ceiling.
- Replace the raw context-object variant payload with an opaque public
  `ContextObject` whose fallible construction validates the existing DID JSON
  budgets before a value enters the domain model.
- Add one crate-private rejection guard and iterative JSON dismantler at owned
  recursive-input boundaries across document, resolution/dereferencing,
  query-option, and registration construction.
- Preserve every accepted wire shape, limit, error class, dependency, and
  high-level behavior while documenting the intentional pre-release source
  migration.
- Add hostile-depth, policy-boundary, wire round-trip, redaction, and API-
  closure evidence before narrowing only the native cleanup clause of
  `SDK-LIM-007`.

## Capabilities

### Modified capabilities

- `did-core`: context objects are validated domain values and DID owners, not
  generic wire helpers, enforce domain collection limits.
- `sdk-input-resource-governance`: native DID rejection cleanup becomes
  iterative after SDK ownership while outer allocation remains caller-owned.

## Impact

- Affected crate: `identus-did`.
- Public source compatibility: direct
  `ContextEntry::Object(BTreeMap<String, Value>)` construction migrates to
  `ContextObject::new(map)?` followed by `ContextEntry::Object(object)`.
  `OneOrMany::try_many` no longer rejects a non-empty collection solely because
  it exceeds a DID-specific limit; enclosing DID construction still rejects it.
- Wire compatibility: accepted JSON-LD scalar, array, URI, and object forms are
  unchanged.
- Dependencies, features, lockfile, Rust 1.98.1 etalon, and dependency
  direction: unchanged.

## Non-goals

- No new DID method, JSON-LD processor, context semantics, accepted resource
  ceiling, dependency, product behavior, or downstream repository mutation.
- No claim that caller, transport, generic Serde, FFI, or JavaScript allocation
  before the typed SDK boundary is bounded.
- No custom public `Drop`, unsafe code, intentional leak, or finite-stack
  workaround.
