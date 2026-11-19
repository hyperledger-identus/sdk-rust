# Design

## Capability split

`identus-prism-did` remains the sole portable semantic owner.
`identus-wasm-did` exposes a narrow generated value boundary. The private
TypeScript package owns strict structural ingress, immutable projection,
provider composition, observation, cancellation, and target loaders. Injected
effects own platform work. No TypeScript function parses protobuf, derives a
suffix, verifies a signature, reduces state, or projects a document.

ADR 0201 records the exact outer-boundary composition edge from the private
WASM artifact to the portable PRISM method crate. The exception is closed to
that source-target pair and does not broaden generic or native dependencies.

## Generated binding v5

The binding adds owned views for a parsed/created PRISM identifier and reduced
state. Views carry only copied strings: requested/canonical DID, form,
normalized document when active, deactivation state, and stable operation/hash
metadata required for correlation. Getters are closed and ownership is freed
exactly once.

Explicit functions parse an identifier, create a long form from strict JSON,
locally resolve long-form initial state, and reduce exact signed-operation
history. History JSON contains only a closed ordered array of unpadded
base64url signed-operation bytes. Rust decodes, bounds, verifies, correlates,
reduces, and projects it atomically.

## Public-material creation input

The TypeScript input uses readonly closed values: key identifier, one explicit
usage, one supported curve, unpadded base64url public bytes; service identifier,
one-or-many type, URI/object/array endpoint; and ordered contexts. It has no
private key, signing callback, entropy source, derivation path, custody handle,
or generic operation-kind switch.

TypeScript performs iterative structural and aggregate preflight before closed
serialization. Rust constructs canonical crypto values and PRISM builders and
revalidates the complete create operation. The result returns long and
canonical DIDs plus the locally projected document, never raw private state.

## Portable history effect

`PrismDidHistoryEffect` receives an immutable validated request containing the
requested/canonical DID and optional `AbortSignal`. It returns a closed ordered
array of exact signed-operation base64url strings. Canonical resolution starts
with a signed create. Long-form resolution starts from its embedded unsigned
self-certifying create and accepts only subsequent signed updates or
deactivation.

The adapter bounds the host result before serialization, then calls Rust once.
Rust rejects a mismatched create, repeated create, broken previous hash,
invalid authorizing key/signature, action failure, post-deactivation operation,
or resource excess. The host cannot inject a replacement document or DID.

## Immutable projection and errors

The facade returns frozen `PrismDidValue` and `PrismDidStateValue` values that
reuse `DidValue` and `DidDocumentValue`. Active/deactivated state is a closed
discriminated union. Unknown generated fields, unsafe numbers, API versions,
forms, state kinds, or error categories fail closed after cleanup.

Stable TypeScript errors map canonical Rust syntax/wire/operation/state
categories without caller data. Host unavailable/failure, abort, observer, and
binding-contract failures are separate. Observations contain only operation,
route, mapping ID, result, stable code, and bounded duration.

## Target behavior and rollback

Browser and Node artifacts expose identical binding v5 symbols. Node ESM/CJS,
real browser, React SSR, and hydrated client execute one neutral assertion
body. React contains no PRISM logic. React Native remains not applicable.

Selection occurs before initialization between an injected provider and the
PRISM-WASM capability. Initialization, canonical validation, host effect,
history reduction, projection, observer, and binding failures are terminal.
Rollback is a new explicit pre-initialization choice; no automatic fallback or
retry exists.

## Evidence ownership

The canonical Rust/Preprod packet supplies identifier, operation, signature,
state, document, negative, and resource cases. SDK-TS donor rows classify
applicable identifier/resolver behavior and exclude platform custody/agent
flows. NeoPRISM merged revisions supply downstream boundary and adoption
evidence only. A new TypeScript quality declaration owns adapter/target proof;
it does not relabel Rust core evidence.

## CI isolation

The fail-closed impact plan selects all TypeScript targets for PRISM core,
binding, package, vector, mapping, quality, ledger, and fixture changes. Rust
core completes first. Package, Browser, Node.js, React Web, and explicit React
Native inapplicability remain isolated sibling jobs. Terminal `fast` succeeds
only when every selected receipt is exact-head and successful.

## Cohesion and decomposition

This train is one invariant across explicit public material, canonical Rust
encoding/reduction, generated ownership, strict projection, injected effects,
and identical target behavior. Splitting those layers would leave an unusable
or unverifiable API-version boundary. Cardano/VDR/indexing, custody/signing,
native bindings, external activation, and release remain separate slices.
