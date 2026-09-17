# Release policy

This policy applies to every published crate and repository release. It extends
the Identus ecosystem release process with Rust/crates.io controls.

## Release authority

A human maintainer is assigned release manager. At least one other maintainer
approves the release receipt. Publishing runs only in a protected GitHub
environment. Because crates.io cannot configure trusted publishing before a
crate's first release, the first publication uses an organization-controlled,
short-lived, crate-scoped token under the same two-person approval. Trusted
publishing is configured immediately after each namespace exists and is
mandatory for later releases. Local `cargo publish` remains emergency-only and
requires a governance-recorded incident review.

## Version stages

- `0.0.x`: namespace reservation or bootstrap; no usable API promise.
- `0.x`: experimental component with SemVer, migration notes and a declared
  support window.
- `1.x`: stabilized only after independent security review, published
  conformance evidence and at least two independent consumers.

Crates version independently. A signed release manifest records the exact set
of crate versions tested together and their standards/profile capabilities.

## Candidate gate

ADR 0133 selects Rust 1.89.0 as the `0.1.x` MSRV and Rust 1.98.1 as the primary
compiler for `identus-derive`, `identus-core`, and `identus-crypto`. The fast
lane stays on one Linux primary build; the release gate additionally requires
the independent MSRV, profile, and compile-target matrix. Every recorded slow
and sanitizer failure must be resolved and required repository settings must
be verified.

Before tagging, the release manager verifies:

- the component issue and required reviews are complete;
- fmt, clippy, tests, docs, the accepted compiler matrix and supported targets pass from a clean
  clone and locked dependencies;
- feature/default/minimal combinations and eligible WASM/mobile targets pass;
- conformance, negative, fuzz/property and resource-limit evidence is current;
- license, source, advisory, secret, unsafe-code and dependency policies pass;
- public API and serialized/wire compatibility diffs are reviewed;
- changelog and migration guidance describe every breaking/deprecated surface;
- `cargo package` contents and `cargo publish --dry-run` are reviewed;
- SBOM, source archive, checksums and build provenance are generated;
- downstream compatibility is demonstrated without modifying production
  consumer branches;
- no unresolved high-severity security finding exists.

## Procedure

1. Open a release issue listing crates, versions, standards capabilities,
   support dates and assigned release/security reviewers.
2. Freeze the candidate revision; allow only reviewed release blockers.
3. Run the candidate gate and attach the machine-readable release receipt.
4. During the pre-1.0 bootstrap, create a signed tag from the approved,
   protected `develop` revision using the project tag convention. Do not move
   code or tags to `main`; activating `main` requires a later ADR.
5. The protected workflow builds/packages again from the tag. The namespace-
   creating release uses the approved bootstrap token; subsequent releases use
   crates.io trusted publishing.
6. Verify crates.io ownership, package contents, docs.rs and checksums.
7. Publish the GitHub release with manifest, SBOM, provenance, conformance and
   migration links.
8. Open or update downstream adoption issues. A release does not authorize an
   automatic consumer dependency change.

## Failure and rollback

Crates.io releases are immutable. If publication is wrong:

- stop the release workflow and downstream adoption;
- yank a broken crate version when appropriate, without deleting evidence;
- publish a corrected patch or compatibility version through the full gate;
- issue a security advisory when the defect is security-relevant;
- record cause, affected versions and consumer action in the release issue.

Never retag an existing version or replace a published artifact under the same
version.

## Emergency security release

Embargoed fixes use the private Hyperledger/Identus security process. Public
release notes disclose only at the coordinated time. The security team may
limit the ordinary review group, but two-person release control and signed
provenance remain required.
