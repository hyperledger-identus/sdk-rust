# Why

The post-merge acceptance audit for issue #391 found that ADR 0156 and the
executable SIROS spike answered the two product decisions, but the requested
full capability matrix and quantified deletion/compile/resource payoff were
not recorded. The issue must remain open until that evidence is explicit.

# What changes

- Add a seam-by-seam OID4VCI/OID4VP capability matrix for the SDK and viable
  Rust candidates.
- Extend the isolated SIROS fixture with clean-room Final-spec behavior vectors.
- Record incremental package names, source/deletion proxy, cold compile time
  and RSS, resource/allocation characteristics, target evidence, and rollback.
- Amend ADR 0156 only if the completed evidence changes its decision.

# Capabilities

## New capabilities

- `oid4vc-reuse-evidence-completion`: closes the measurable evidence contract
  for issue #391 without activating production reuse.

# Non-goals

- No production dependency, API, protocol implementation, workflow, release,
  support claim, or downstream change.
- No claim that source lines equal future net deletion or that compile checks
  prove runtime support.

# Delivery

Issue #391 remains the authority. Specification and readiness precede fixture
or report edits; reviewed signed/DCO delivery targets `develop`.
