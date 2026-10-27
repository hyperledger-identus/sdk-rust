# Design

## Crate rings

`identus-prism-did` depends inward on generic DID and crypto crates. It exposes
method semantics but no ledger or transport. The proposed modules are:

- `syntax`: canonical and long-form `did:prism` values;
- `wire`: bounded scanner, private prost messages and exact-byte envelopes;
- `operation`: validated create, update and deactivate domain operations;
- `state`: deterministic history and atomic transition reducer;
- `document`: projection into `identus-did` domain values;
- `resolver`: local long-form resolver and method binding;
- `error` and `limits`: redacted failures and the finite public profile.

Each production module should stay below the repository attention threshold;
wire scanning, generated messages and domain conversion remain separate files.

## Exact-wire envelope

Parsing first scans protobuf keys without allocation. It rejects malformed
varints, unsupported wire types, duplicate `signed_with`, `signature` or
`operation` fields, and multiple recognized operation oneof tags. Unknown
fields are skipped within the same total byte budget.

An owned parsed envelope stores the original bytes and validated byte ranges
for the signature, operation and operation payload. Public accessors return
borrowed slices. Private prost decoding supplies typed fields, but re-encoding
is never used for inbound signature verification, operation hashes or DID
suffix derivation.

Construction takes typed candidates, revalidates every public enum payload at
the enclosing service or operation boundary, produces deterministic protobuf
bytes, and returns unsigned bytes for an external signer. The final envelope
constructor validates the key identifier and signature size before encoding.
The crate never owns a secret key or random generator.

## Method identifiers

A canonical short DID is `did:prism:` plus a lowercase 64-hex SHA-256 digest.
A long form appends `:` and unpadded base64url of a create operation. Decoding
uses a private bounded base64url path rather than the crypto text codec. The
decoded operation must be a valid create operation, and its first digest must
equal the short-form suffix. Construction also proves that the initial state
projects through the generic DID document limits, so every accepted long form
is locally resolvable without HTTP or a ledger lookup.

## Domain validation

IDs use a bounded ASCII fragment validator rather than regex. Service type
accepts the draft's single value or canonical non-empty JSON array grammar.
Endpoint text is bounded by both character and byte limits, rejects duplicate
JSON names, maps into `identus-did::ServiceEndpoint`, and requires generic URI
validation. Contexts are unique absolute URIs and cannot repeat the SDK-added
DID Core context. Validated URI text is preserved exactly so projection and
history replay cannot rewrite signed protocol values.

Variant-specific constructors (`ServiceType::one` and `ServiceEndpoint::uri`)
reject wire text for any other representation; polymorphic wire parsing stays
explicit in `parse_wire`.

Supported public key curves are secp256k1, Ed25519 and X25519 through
`identus-crypto`. Master and VDR operation keys must be secp256k1. Unknown key
usage or curve values fail closed. Master, revocation and VDR keys are retained
in protocol state but omitted from DID document verification methods.

## Deterministic state

State keeps insertion-ordered vectors of at most 50 active keys and services.
Removed values are dropped while their identifiers move to compact ordered
tombstone sets, bounded to 4,096 identifiers per kind; ledger position,
wall-clock time and transaction metadata remain owned by the outer history
adapter. Linear active lookup plus logarithmic tombstone lookup keeps replay
finite and auditable. Removed identifiers cannot be reused.

An update validates its previous-operation hash and active master signature,
clones the state, applies every action in wire order, validates final active
limits and the active-master invariant, then commits once. Any failure returns
the original state unchanged. Active counts ignore tombstones. Deactivation
marks every active key/service removed and prevents later valid updates.

## Projection and ordering

Projection preserves insertion order. Verification methods and relationship
references use the canonical or requested long DID consistently. JWK values
come from `identus-crypto`; services and contexts use generic DID domain types.
The DID Core context is first and is not duplicated. Deactivated resolution
uses the generic standards-shaped result and method metadata.

Create and update reduction construct the generic document before accepting a
candidate, so aggregate JSON budgets across keys and services cannot leave
method state that local resolution is unable to project.

Native projection has a 32 MiB method-owned output budget. Its preflight counts
one document controller, three controller occurrences per projected method and
one per service, then adds three complete 96,000-byte operation budgets for
contexts, keys and services accumulated across separate published updates. The
supported maximum long-form shape remains below the 32 MiB ceiling. This does not widen generic
hostile JSON re-ingress.

## Extension boundary

The exact-wire operation discriminator recognizes protocol-version and VDR
tags. Their payload bytes remain bounded and opaque. Issue #522 can decode VDR
payloads without duplicating the envelope scanner or exposing prost structs.

## Verification

Evidence includes pinned NeoPRISM and historical node vectors, specification
boundary cases, executable differential wire vectors, duplicate-wire attacks, rollback,
retired-ID churn, deterministic ordering, property tests, fuzz targets,
WASM/native checks and directional benchmarks. No performance number becomes
a CI threshold without a separate stability decision.
