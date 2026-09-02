## Why

`IDR-001` asks for an identifiable SDK repository, charter, maintainers,
contribution rules and release authority. Most repository-local documents now
exist on `develop`, but their presence is not checked as one contract. The
selected seed also carries eight metadata-only crates whose package names and
speculative dependency edges can be mistaken for implemented or publishable
capabilities.

GitHub issue #25 defines the repository-local slice before implementation.
Read-only GitHub API evidence also shows that the repository is still private,
`develop` is unprotected, no publishing environment exists and repository
security features are disabled. Those protected settings are explicitly owned
by follow-up #26; this change must not represent them as active or mutate them.

## What Changes

- Add one machine-readable bootstrap inventory covering repository governance
  evidence and every Cargo workspace package exactly once.
- Classify real foundations, verification-only code and quarantined
  placeholders without treating package existence as a roadmap or release
  commitment.
- Make every bootstrap package fail closed against accidental publication.
- Remove unused speculative dependency edges from metadata-only placeholders,
  leaving only the dependency required by their component marker.
- Add a human-readable inventory of the implemented experimental API families,
  feature surfaces, owner issues and stabilization boundaries.
- Add an offline validator plus adversarial tests and run it from the factory
  structural contract.
- Keep `IDR-001` in progress until the protected live-state work in #26 is
  completed; do not overlap crates.io namespace and trusted-publishing issue
  #3.

## Capabilities

### New Capabilities

- `sdk-governance-evidence`: governs repository-local governance evidence,
  package maturity classification, bootstrap publication denial and offline
  drift validation.

### Modified Capabilities

- `crate-ring-layout`: replaces the obsolete claim that implemented crates are
  stubs and removes speculative dependency cones from quarantined placeholders.

## Impact

This change affects Cargo metadata, metadata-only placeholder manifests,
architecture documentation, factory validation and OpenSpec. It adds no crate,
runtime behavior, wire format, public API or consumer dependency. Apollo,
NeoPRISM, midnight-identity, Lace ID Portal and Oxid remain read-only. Release,
publication, repository administration and `main` remain outside agent
authority and outside this change.
