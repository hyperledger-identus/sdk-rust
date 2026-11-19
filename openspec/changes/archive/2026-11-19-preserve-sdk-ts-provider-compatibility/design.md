# Design

## Type boundary

`DidProvider` retains only DID, DID URL, and DID Document parsing.
`PeerDidProvider extends DidProvider` owns the six peer operations. Query
providers follow the same split so peer-capable factories do not lose peer
typing while core-only effects remain assignable.

## Initialization boundary

Target entry points dynamically import generated artifacts only from the
peer-WASM factory. `selectDidProvider({kind: "injected", provider})` returns the
same provider without invoking that factory. The peer selection calls the
existing retryable initializer exactly once. No catch path changes selection.

## React and CommonJS

React keeps one context and lifecycle boundary typed to `DidProvider`, so an
old core-only provider remains injectable. Direct target factories return the
peer extension for consumers that need peer methods. The CommonJS bridge
forwards the same selection function and contains no provider semantics.

## Evidence

Compile fixtures prove core assignability and strong peer types. Runtime tests
prove initializer non-entry, terminal rejection, exact injected identity,
CommonJS forwarding, React injection, declaration snapshots, browser vectors,
Node vectors, and SSR/hydration. Isolated target receipts remain the final
local binding gate.
