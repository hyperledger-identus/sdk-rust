# Design

## Representation and policy

`OneOrMany<T>` remains the wire-cardinality abstraction. `try_many` rejects
only the structurally invalid empty array. DID owners call the existing
`validate_collection` policy before return: document context and controller,
service types, and every existing DID collection retain the 128-item ceiling.
This prevents a generic container from needing destruction knowledge about
arbitrary `T`.

## Validated context object

`ContextEntry::Object` carries `ContextObject`, not a raw map. `ContextObject`
has private storage, a fallible `new`/`TryFrom` boundary, borrowed and consuming
accessors, transparent map serialization, validated deserialization, and
redacted diagnostics. Construction validates the existing depth, node,
property-name, property-count, array-cardinality, and string limits. No
unchecked or mutable map escape hatch is exposed.

Private Serde wire staging may own an untrusted map only until conversion. A
rejection guard is armed before validation; failure destructures the map and
drains all arrays/objects with the sole iterative JSON worklist. Success yields
the exact allocation. Consequently normal destruction after construction is
bounded by the domain invariant.

## Complete native rejection family

The same private guard protects every audited public fallible boundary that
can own recursive JSON:

- verification methods, services, context objects, and document builder;
- resolution errors, operation/document/content metadata, and dereferenced
  content;
- resolution/dereferencing options and builders; and
- registration public data.

Small private adapters destructure candidates and enqueue JSON roots. They do
not duplicate traversal. The compiler exposes a newly added owned field because
the adapters use exhaustive destructuring.

## Compatibility and evidence

Accepted `ContextObject` serialization is the exact underlying map. Existing
document JSON round trips remain unchanged. Tests construct hostile depth
iteratively and prove:

1. `ContextObject::new` rejects and dismantles 32,768 levels stack-safely;
2. a non-empty 129-entry `OneOrMany<ContextEntry>` is structurally representable
   but `DidDocumentBuilder::build` rejects it with `TooManyItems`;
3. every other native fallible family cleans both JSON-validation and earlier
   semantic failures iteratively; and
4. compile-fail/public-API evidence prevents a raw recursive map from directly
   inhabiting `ContextEntry`.

## Rollback

Restore the raw context payload, generic cardinality check, former constructor
behavior, and `SDK-LIM-007` native cleanup disclosure atomically. Accepted wire
or persisted data requires no migration.
