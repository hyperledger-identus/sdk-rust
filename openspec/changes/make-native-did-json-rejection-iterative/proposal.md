# Make native DID JSON rejection cleanup iterative

## Why

Issue #297 owns the avoidable `SDK-LIM-007` residual demonstrated after issue
#168: a direct Rust caller can pass an already-owned hostile-depth
`serde_json::Value` or map, receive the correct validation error, and then
overflow the process stack while the rejected tree is recursively destroyed.
The bounded wire-slice path is already safe, but native callers should receive
the same stack-safe rejection after ownership crosses the SDK boundary.

## What changes

- Add one crate-private generic rejection guard and iterative JSON dismantler.
- Apply it to every audited native DID constructor/build rejection path that
  can own caller-provided recursive JSON.
- Add hostile-depth regression coverage across document, resolution,
  dereferencing, query, and registration families, including failures that
  occur before resource validation.
- Remove only the native rejection-cleanup clause from `SDK-LIM-007`, the
  machine inventory, and explanatory documentation after complete evidence.

## Capabilities

### Modified capabilities

- `sdk-input-resource-governance`: native DID rejection cleanup becomes
  SDK-enforced after entry while outer allocation remains caller-owned.

### Added capabilities

- `did-core`: rejected native owned JSON is dismantled iteratively across the
  complete public constructor family.

## Non-goals

- No new accepted value, limit, public constructor, representation, error,
  dependency, wire behavior, support tier, or downstream migration.
- No attempt to prevent allocation performed before SDK entry.
- No change to caller-budgeted work or the Multihash, crypto-codec, and JOSE
  retained-input compatibility clauses still owned by `SDK-LIM-007`.

## Delivery

Issue #297 owns the focused implementation from protected
`develop@508917b0416147668b9d12949eec987f0b82946b`.
