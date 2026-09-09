# Design

## Boundary

The oracle is a nested Cargo workspace under `docs/research/` with its own lock.
It depends on the current `identus-did` path and exact published
`identity_did 1.5.1`. Candidate types and errors never leave the fixture.

## Corpus model

Each case owns an identifier, input and normative expectation. The harness
records independent Identus and IOTA accept/reject results and checks one of:

- `intersection`: both must accept and preserve exact spelling;
- `shared-rejection`: both must reject;
- `documented-divergence`: the expected result is pinned per implementation
  and accompanied by a W3C/RFC/SDK-limit or representation reason.

The bounded fixed corpus covers method grammar, method-specific identifiers,
parameters, path/query/fragment, percent escapes, delimiter errors, Unicode,
control bytes and exact SDK byte ceilings. It does not use candidate errors as
the SDK error taxonomy.

## Isolation and drift

A manually invoked script runs the locked harness and strict Clippy, proves `identity_did` is
absent from the root manifest/lock, and asserts the exact candidate release.
The fixture lock and case expectations make upstream and resolver drift visible.

## Decision rule

The completed evidence retains the fixture as manual reference only: it finds
seven useful differences but lacks independent parser diversity, has a broad
advisory-bearing graph and fails the SDK WASM compile. It is not a CI oracle.
Production adoption is excluded.

## Risks

- Oracle agreement can be mistaken for normative truth. Every divergence is
  classified against W3C DID Core 1.0 or RFC 3986.
- A broad candidate graph can slow CI. The fixture stays outside root and is
  manually invoked only.
- Error strings can leak caller content. The harness records booleans/classes,
  never candidate diagnostic text.
- Target compilation can be overrepresented as support. Compile receipts are
  explicitly non-runtime, non-support evidence.

## Rollback

Remove the nested fixture, script and oracle report/ADR update. Root packages,
public APIs, serialized data and downstream consumers require no migration.
