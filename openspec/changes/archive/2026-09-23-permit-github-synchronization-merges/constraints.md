# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/342
Constraint blockers: none

## Existing entries affected

`SDK-DELIVERY-001` remains effective. This change corrects the machine
enforcement of its contribution-provenance gate so a supported GitHub branch
synchronization does not require a protection bypass. Its issue linkage,
specification, distinct review and green-CI requirements do not change.

`SDK-REPO-001` remains effective: `develop` is the protected integration branch
and direct pushes remain prohibited. Release and publication constraints remain
unchanged.

## Introduced or changed constraints

Ordinary authored commits continue to require a conventional subject, exact
DCO trailer and accepted verified signature. A hosted synchronization merge is
not treated as an authored commit only when its parent graph, base/head refs,
canonical subject, GitHub committer identity and verified signature match the
recorded structural contract. Signature validation is never skipped.

Local range validation does not receive GitHub pull-request event context and
therefore grants no synchronization exemption.

## Introduced or changed limitations

The hosted classification trusts GitHub's verification result and platform
committer identity. It does not independently attest GitHub or reproduce the
merge tree. Pull-request review therefore remains responsible for the complete
resulting diff. Issue #339 separately addresses the fact that a pull request
currently evaluates policy code from its own tree.

## Consumer and product impact

Contributors and agents can use GitHub's supported **Update branch** merge path
without rewriting a platform-generated commit. Rust consumers, downstream
repositories, crate APIs and release archives are unchanged.

## Activation and rollback

Activation requires the issue-linked planning receipt, structural and negative
tests, updated policy documentation, distinct local review, green required CI
and a protected merge into `develop`. The sponsor direction is recorded by
issue #342 and ADR 0136. Rollback is a revert of the policy change.

## Evidence

PR #340 and workflow job `107197071068` are the exact false-negative fixture.
GitHub documents the Update branch operation, and CNCF DCO2 source at immutable
revision `14a6f0096a594f736ab368e3985c07ada7bf5154` confirms that the canonical
DCO gate already exempts merge commits.
