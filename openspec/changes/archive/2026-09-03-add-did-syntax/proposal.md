## Why

Backlog item `IDR-005` requires a chain-neutral DID Core boundary before the
PRISM, Midnight and later method ports can share identifiers safely. The
current `identus-did` crate contains method-name, version and multihash
newtypes but no validated DID or DID URL. Consumers therefore either depend on
large third-party DID stacks or repeat incomplete string checks.

GitHub issue #34 defines this bounded child of parent #5. It establishes only
the W3C DID Core lexical boundary. DID documents, method semantics,
resolution, dereferencing and downstream migrations remain separate changes.

## What Changes

- Add validated, immutable `Did` and `DidUrl` value types to `identus-did`.
- Parse W3C DID and RFC 3986 DID URL grammar in a single bounded byte pass,
  without regex, URL or external DID dependencies.
- Cache component offsets so method, method-specific identifier, DID, path,
  query and fragment accessors do not allocate.
- Enforce SDK resource limits of 2 KiB for a bare DID and 4 KiB for a DID URL.
- Provide validating `FromStr`, `TryFrom<String>` and serde boundaries with
  stable, redacted `IdentusError` mappings.
- Pin standards-derived, donor-shaped and adversarial conformance tests plus
  a release-mode parser throughput diagnostic.
- Record the ownership, parser and publication boundary in ADR 0008.

## Capabilities

### New Capabilities

- `did-core`: chain-neutral DID and DID URL lexical value objects that later
  document, resolver and method-port work can safely compose.

## Impact

- **Issue:** #34, child of #5 (`IDR-005`).
- **API:** additive types in the unpublished `identus-did` package.
- **Dependencies:** none added.
- **Consumers:** NeoPRISM, midnight-identity, Lace ID Portal and Oxid gain a
  future common lexical boundary. No downstream repository is modified here.
- **Naming:** the eventual `identus-did-core` publication remains owned by
  namespace issue #3; this slice deliberately avoids a premature rename.
- **Rollback:** revert issue #34's pull request. No released API, persisted
  format or downstream migration is involved.
