# Exact-diff architecture, security and release review

Review status: completed
Review date: 2026-09-25
Base: develop@0fed1ec7f002fbf9ca5d7b84eaa0e7b62b068183
Implementation head: 93b1239dc1bc36ba8883824032b6090028e12630
Reviewed head: 93b1239dc1bc36ba8883824032b6090028e12630
Specification commit: c9c6aeb7edc3c14972dc196f22b78ddab8c8bd69
ADR/receipt commit: a8ffddf7c58cf48648d48ae7c1d76a1c756bce36
Unresolved blockers: none

## Findings

1. **Compatibility honesty — accepted.** The first staged DID candidate has no
   released predecessor. ADR 0154 forbids self- or workspace-`0.0.0`
   comparison, commits an exact public-API origin and records the SemVer result
   as not applicable. Execution of cargo-semver-checks is denied while its
   exact future version remains governed.
2. **Supply-chain identity — accepted.** One CycloneDX 1.5 JSON document is
   generated per ordered package. Validation binds root name, candidate
   version, root license, unique component references, a closed dependency
   graph and absence of Git sources. Malformed, missing, duplicate, oversized
   and identity-drift cases fail before atomic output publication.
3. **Reproducibility — accepted.** Cargo-cyclonedx's random scratch paths are
   normalized only in local `path+file` references. Package/version fragments
   and graph relationships remain intact. Two clean passes produced identical
   API, SBOM and crate hashes, while the crate hashes remain exactly those from
   #382.
4. **Security and authority — accepted.** The subprocess allowlist permits
   only local read/build/evidence operations. Registry publication, Git/GitHub
   mutation, credentials and first-candidate SemVer comparison remain absent.
   Evidence files are regular, non-empty and bounded; package extraction keeps
   its traversal/link/member/expansion defenses.
5. **Claim boundary — accepted.** The receipt names repository license and
   advisory authority but honestly records that no candidate-specific scan ran.
   It explicitly disclaims attestation, vulnerability-free, platform,
   registry, publication and certification conclusions.
6. **Architecture and cohesion — accepted.** Descriptor validation remains in
   the policy checker; staging, evidence generation and atomic receipt assembly
   remain in the candidate builder; negative behavior remains in the mutation
   suite. Canonical package manifests and Rust product code are unchanged.
7. **Delivery integrity — accepted.** All implementation commits have valid
   OpenPGP signatures and DCO sign-offs. The exact preimplementation receipt
   predates implementation and binds the protected base and issue.

## Decomposition decision

The base-to-implementation diff spans 17 paths and 1,783 added lines, above the
preferred line guidance. Of those, 996 lines are generated, reviewable public
API baselines and 391 lines are required OpenSpec/ADR governance. Executable
policy, builder and mutation changes are 345 added lines split across their
existing cohesive modules. Separating the two package baselines from the
generator would create unverified evidence, while splitting API and SBOM
qualification would duplicate the same staged candidate and receipt boundary.
No further functional split is warranted.

## Residual limitations

- This is candidate-only evidence; canonical DID manifests remain unpublished.
- The committed public API is an origin for later comparison, not a stable API
  guarantee.
- SBOMs are local deterministic inventory, not signed provenance or an
  independent advisory result.
- Platform evidence, crates.io availability, protected promotion, downstream
  adoption and final release remain later M5 slices.

## Review decision

The change is cohesive, fail-closed, reproducible, reversible and honest about
its compatibility and supply-chain claims. No unresolved correctness,
security, privacy, compatibility, architecture, dependency or release finding
remains.
