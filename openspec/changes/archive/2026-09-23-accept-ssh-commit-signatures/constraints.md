# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/338
Constraint blockers: none

## Existing entries affected

`SDK-DELIVERY-001` is the closest existing entry: qualifying work is
issue-linked, spec-driven, distinctly reviewed and CI-gated. This change alters
one mechanism of that gate and therefore its machine enforcement, without
changing the entry's statement, scope or state, so
`docs/governance/sdk-constraints.toml` is not edited. `SDK-SEC-002` continues to
govern secret handling and is reinforced rather than relaxed: no private key,
passphrase or credential is read, stored or transmitted by the checker, and the
local verification failure path reports a missing trust file instead of
degrading silently.

`SDK-REPO-001` keeps `develop` as the only integration target. `SDK-LIM-001`,
`SDK-LIM-002`, `SDK-LIM-003` and `SDK-LIM-005` continue to keep public API,
FFI, platform certification and product behavior outside this slice. No release
entry changes: no published artifact, version or distribution channel is
affected.

## Introduced or changed constraints

The accepted commit-signature mechanisms become a declared set in the tracked
contribution policy rather than a single hard-coded envelope. Every commit
reaching `develop` still requires a GitHub-verified signature, an exact
`Signed-off-by` trailer, a Conventional Commit subject and an issue-backed
branch; an unsigned commit, a commit GitHub does not verify, and a verified
commit whose envelope is not declared all fail closed.

The hosted trust anchor is unchanged: the checker continues to consume GitHub's
`verification.verified` and `verification.reason`, so a signature is accepted
only when GitHub attributes it to a key registered on the pushing account. This
widens the accepted mechanisms to GitHub-verifiable OpenPGP and SSH envelopes
and widens nothing else.

The local pre-push hook keeps cryptographically verifying the outgoing commit
range through `git verify-commit`, and must now report the cause of a local
verification failure — including a missing `gpg.ssh.allowedSignersFile` —
instead of a mechanism-specific message.

## Introduced or changed limitations

This repository accepts a mechanism that the org-level Hyperledger Identus DCO
and PGP policy does not describe. That divergence is a public commitment owned
in `hyperledger-identus/.github`, which this change does not modify, and it
requires a maintainer decision to accept here and a separate upstream
discussion to reconcile.

The hosted check trusts GitHub's verification result for both mechanisms
rather than independently validating envelopes against a tracked set of trusted
keys. Enforcing an allowed-signers allowlist is deferred and recorded, so a
mistaken or compromised GitHub verification result remains undetected by
repository-owned code.

A pull request is judged by the policy in its own tree, because the workflow
checks out `refs/pull/<N>/merge`. This change is necessarily self-applied under
that property and makes it more visible rather than weaker; hardening it is
issue #339 and is out of scope here.

## Consumer and product impact

The consumers are contributors, agents and the two enforcement surfaces: the
hosted `pull-request-policy` workflow and the repository-owned pre-push hook.
They gain a mechanism-agnostic contract, an accurate diagnostic and a usable
path for maintainers who hold only an SSH signing key. Rust consumers, SDK
users, downstream repositories and product behavior are unaffected: no crate
API, dependency, binding, protocol or artifact changes.

## Activation and rollback

Activation requires the issue-bound preflight receipt, the planning-only
contract commit, the implemented checker, policy, hook, tests and documentation,
strict OpenSpec validation, `scripts/factory check`, the contribution-policy
and factory-contract test suites, actionlint, a signed DCO-bearing pull request
with an exact-head local review, and green required CI. Merge stops for explicit
maintainer approval because of the org-level divergence.

Rollback is a revert of the pull request. It restores the previous
OpenPGP-only envelope filter, the previous diagnostic text and the previous
local verification message, with no data migration, no published artifact to
withdraw and no consumer contract to renegotiate. Commits already merged remain
valid under both revisions because both accept OpenPGP.

## Evidence

Issue #338 carries the reported hosted failure on PR #337, the verified SSH
commit record and the maintainer direction to accept SSH signatures. Issue #339
tracks the deferred base-policy hardening and issue #326 remains the release
coordinator that consumes this gate. ADR 0135 records the decision, its
authority and its divergence from the org-level policy.
