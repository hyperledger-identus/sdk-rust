## Why

Issues #28, #30, and #32 established validated public JWK, COSE Key, and RFC
7638 thumbprint boundaries. Their deterministic vectors cover known edge cases,
but do not continuously search hostile byte combinations for parser panics,
nondeterministic encoding, representation drift, or conversion disagreement.

Issue #58 (`IDR-004d`) adds that missing generative assurance before DID, VC,
JOSE, and binding crates rely on the public-key representations. The work
reuses the independent sanitizer infrastructure introduced by #35 without
adding a production dependency or changing cryptographic behavior.

## What Changes

- Generalize the standalone fuzz package documentation/name while preserving
  the existing DID campaigns and commands.
- Add separate arbitrary-byte `public_jwk` and `public_cose` libFuzzer targets
  with accepted-value serialization, thumbprint, deterministic encoding, and
  structural-conversion invariants.
- Add original standards-shaped positive/negative corpora and dictionaries,
  including a text transport for exact binary COSE seeds.
- Add one crypto wrapper for corpus replay, deterministic smoke, and bounded
  soak modes under the existing pinned runner/toolchain.
- Add a path-scoped Ubuntu workflow with deterministic PR/push smoke,
  scheduled/manual soak, supply-chain checks, and failure-only artifacts.
- Record limits, minimization, performance evidence, compatibility, and
  residual risk.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crypto`: makes sanitizer-backed generic public JWK, COSE Key, thumbprint,
  and structural-conversion fuzzing reproducible without widening the API.

## Impact

- **Issue:** #58, child of #9 / `IDR-004`.
- **Affected surface:** independent `fuzz/` workspace, focused workflow,
  crypto fuzz wrapper/docs/corpora, canonical crypto requirements, ADR 0019.
- **Public/wire compatibility:** unchanged; targets consume existing APIs.
- **Dependencies:** no published crate or root-lock change; fuzz-only packages
  remain in the independent workspace.
- **Consumers:** Apollo, NeoPRISM, midnight-identity, Lace, and Oxid stay
  read-only.
- **Rollback:** revert the focused PR; no release, persisted data, chain,
  consumer, repository setting, or `main` branch is changed.
