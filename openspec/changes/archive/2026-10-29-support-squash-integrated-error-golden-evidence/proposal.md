# Support squash-integrated error-golden evidence

## Why

PR #577 passed its feature-branch factory gate, then the protected delivery
path squash-merged it as `78bd1c01a83e014e0cc85be27075bf7634902199`.
The immutable-golden checker requires every planning commit to remain an
ancestor of `HEAD`; that is impossible after a squash merge and makes current
`develop` fail `scripts/factory doctor` despite unchanged fixture bytes.

## What changes

- Distinguish active/ancestry-preserving evidence from an archived contract
  integrated through the repository's protected squash path.
- Freeze the complete preimplementation receipt bytes as part of every golden
  binding, not only selected receipt fields.
- For archived squash-integrated evidence, require the fixed base to remain an
  ancestor of `HEAD`, require the receipt and planning/stable golden to resolve
  byte-for-byte from `HEAD` and match their immutable hashes, and validate a
  reachable contract commit when one exists.
- Keep active changes and ancestry-preserving history on the stronger existing
  base-to-contract-to-head proof.
- Add mutation tests for protected squash integration and forged receipt,
  fixture, base, and reachable-contract failures.
- Document the two evidence states in the factory knowledge base.

## Capability

### Modified capability

- `factory-operations`: preserve the planning-before-implementation invariant
  after the normal protected squash integration without relying on an
  eventually deletable feature-branch ref.

Pending quality declaration `factory.error-golden.quality.v1`, owned by issue
#578, will route deterministic mutation coverage and record fuzz, benchmark,
and external differential evidence as reviewed not applicable for this closed
offline repository validator.

## Non-goals

No public SDK, JOSE/JWE, SIOPv2, wire, dependency, workflow, merge-method,
branch-protection, release, publication, or downstream change.

## Delivery

Issue #578 owns this behavior-neutral factory repair and unblocks #543.
