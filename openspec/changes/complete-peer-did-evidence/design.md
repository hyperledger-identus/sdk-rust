# Design

## Donor transformation record

The donor ledger names the exact `describe`/`it` selectors, the bare service
IDs present at the pinned revision, and both sides of the chosen disposition:
the adapted positive packet omits the incompatible ID while the existing
invalid-service-ID vector proves the selected fragment-only rule. The donor
behavior remains compatibility evidence, not normative authority.

## Adapter fuzz boundary

A deterministic PRNG produces bounded structured inputs and generated-view
mutations. Cases enter the same TypeScript ingress and projection functions
used by the public provider. The harness accepts only a frozen successful
value or a stable redacted adapter error, verifies generated owners are freed
exactly once, and never records input data. A fixed seed and case budget keep
the route reproducible.

The existing Rust `peer_did` libFuzzer target and ASAN receipt remain in the
core declaration only. The TypeScript declaration names only the new adapter
selector and command.

## Native differential runner

The Rust test continues to deserialize the shared packet with unknown fields
denied. Positive numalgo-2 cases compare:

- exact DID;
- authentication and key-agreement references;
- service type, URI, routing keys, and accept values.

Applicable negative cases call the matching native constructor/parser and
compare stable Rust error codes. Host-only structured ingress cases are listed
in an explicit applicability table and asserted as TypeScript-only instead of
silently skipped or credited to Rust.

## Evidence integrity

Changing the packet is unnecessary: it already contains the expected fields
and error identities. Registry evidence revisions move only after the new
selectors pass. Generated quality documentation is refreshed from the
canonical declaration, and all source revisions remain immutable.
