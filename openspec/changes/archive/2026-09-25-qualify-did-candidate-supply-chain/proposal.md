# Qualify DID candidate supply-chain evidence

## Why

#382 proves deterministic DID archives and their local dependency closure, but
M5 still lacks reviewable public-API origins and machine-readable dependency /
license evidence for the exact staged candidates. A first candidate cannot
honestly pass a backward-compatibility comparison against itself or the
unpublished workspace `0.0.0` state.

## What changes

- Pin candidate evidence tools and committed public-API baseline paths in the
  closed DID descriptor.
- Derive both API baselines from staged `0.1.0-rc.1` sources, then require exact
  agreement on normal candidate runs.
- Generate and validate one CycloneDX JSON SBOM for each candidate package.
- Bind API/SBOM digests, exact tools and an explicit first-candidate SemVer
  status into the candidate receipt.
- Preserve the two archive byte hashes delivered by #382 and reuse existing
  repository license/advisory policy without claiming a registry attestation.

## Capability

### Modified capability

- `release-candidate-trains`: adds deterministic API-origin and supply-chain
  evidence to candidate qualification.

## Non-goals

No canonical version/publish change, package upload, tag/release, workflow,
credential, repository setting, platform claim, consumer mutation, stable
SemVer promise, DID method/chain, binding release or certification.

## Delivery

Issue #384 owns this bounded M5 slice from protected
`develop@0fed1ec7f002fbf9ca5d7b84eaa0e7b62b068183`.
