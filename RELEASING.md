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

## First protected crypto train

The first approved train is exactly `identus-derive`, `identus-core`, and
`identus-crypto` at `0.1.0-rc.1`. Its immutable tag is
`crypto-v0.1.0-rc.1`; its only publication workflow is
`.github/workflows/publish-crates.yml`; and its protected GitHub environment is
`crates-io`.

The release manager performs these steps from the exact protected `develop`
revision approved on issue #326:

1. Confirm the release PR has an independent maintainer approval, required CI
   is green, the complete slow receipt is current for the exact revision, all
   three names remain available, and environment `crates-io` requires the
   `identus-maintainers` team with self-review prevention and administrator
   bypass disabled.
2. For the first upload only, confirm environment secret `CARGO_PUBLISH` is a
   short-lived least-privilege crates.io token. Do not place it in ordinary
   repository secrets, a local Cargo credential file, an issue, or a log.
3. Create an annotated signed tag and verify it locally before pushing:

   ```console
   git tag -s crypto-v0.1.0-rc.1 <approved-full-sha> \
     -m "SDK-Rust crypto train 0.1.0-rc.1"
   git verify-tag crypto-v0.1.0-rc.1
   git push origin refs/tags/crypto-v0.1.0-rc.1
   ```

4. Dispatch the workflow from protected branch `develop`, binding the tag and
   full SHA explicitly:

   ```console
   gh workflow run publish-crates.yml \
     --repo hyperledger-identus/sdk-rust \
     --ref develop \
     -f release_tag=crypto-v0.1.0-rc.1 \
     -f expected_sha=<approved-full-sha> \
     -f authentication=bootstrap-token
   ```

5. A maintainer other than the initiating release manager reviews and approves
   the protected environment deployment. The workflow rebuilds evidence and
   rebinds the credential-bearing checkout to the signed tag and exact SHA,
   then publishes derive, core, and crypto. Never approve a run whose tag, SHA,
   candidate receipt, or requested authentication class differs.
6. Verify all three crates.io versions, checksums, owners, docs.rs pages,
   provenance attestations, publication receipt, and GitHub prerelease. Attach
   immutable links to issues #3 and #326.
7. In each crate's crates.io settings, configure trusted publishing for GitHub
   organization `hyperledger-identus`, repository `sdk-rust`, workflow
   `publish-crates.yml`, and environment `crates-io`. Revoke the bootstrap
   token and remove `CARGO_PUBLISH` from the environment.

Every later train uses the same dispatch shape with
`authentication=trusted-publishing`. The workflow exchanges GitHub OIDC for a
short-lived registry token and fails closed; it never falls back to
`bootstrap-token` or `CARGO_PUBLISH`.

If a publish job fails after verification, use **Re-run failed jobs** only for
the same workflow run, tag, and expected SHA. The run-scoped candidate artifact
is intentionally stable across attempts. Existing package versions are reused
only when their registry checksums match, and an existing GitHub release is
resumed only when its tag, prerelease metadata, notes, and asset digests match.
Any identity or digest disagreement is an incident: stop instead of rerunning
with another tag, SHA, artifact, or authentication class.

## Crate ownership and recovery

All published crates must remain recoverable by the Hyperledger Identus
maintainer organization rather than one person's account. The post-publication
receipt records crates.io owners for each name. The assigned release manager
operates the train; the independent environment approver verifies immutable
identity and evidence; the canonical Identus maintainer team owns succession,
owner recovery, and ordinary yank decisions; and the security response team
coordinates security yanks/advisories.

Loss of an individual account must not prevent owner rotation. A crate owner or
publisher change requires a maintainer-reviewed release/governance record. A
yank never deletes the version or its evidence, and unyank/corrected-version
decisions are recorded on the release issue.

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
