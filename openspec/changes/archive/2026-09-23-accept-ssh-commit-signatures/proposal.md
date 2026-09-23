# Accept SSH commit signatures in the contribution gate

## Why

The hosted `pull-request-policy` gate rejects a commit that GitHub has already
verified. On PR #337 the head commit is reported by GitHub as
`verified: true`, `reason: "valid"`, with the envelope
`-----BEGIN SSH SIGNATURE-----`, and the gate fails it with
`GitHub OpenPGP verification failed (valid)` because
`scripts/ci/contribution-policy.mjs` hard-codes the OpenPGP envelope and
`.github/contribution-policy.json` sets `requireOpenPgp: true`.

The repository documents a weaker requirement than the machine gate enforces.
`CONTRIBUTING.md` requires "a cryptographic signature that GitHub can verify",
which an SSH signature satisfies, while `DCO.md` defers to the canonical
Hyperledger Identus DCO and PGP policy, which requires a PGP key. A maintainer
who has registered an SSH signing key with GitHub but has no usable OpenPGP
key is therefore unable to open a mergeable pull request, and the gate reports
the failure with a misleading reason.

## What changes

- Replace the `requireOpenPgp` switch with an explicit, declared set of
  accepted commit-signature envelopes in the tracked contribution policy.
- Accept GitHub-verified OpenPGP and SSH envelopes; continue to reject unsigned
  commits, commits GitHub does not verify, and verified commits whose envelope
  is not declared.
- Separate the two failure modes in the diagnostics, so an unverified commit
  and a verified-but-unaccepted envelope no longer collapse into one message
  that reads as a contradiction.
- Make the local commit-range check and the pre-push hook mechanism-agnostic
  and make a local verification failure name its cause, including the missing
  `gpg.ssh.allowedSignersFile` case.
- Align `DCO.md`, `CONTRIBUTING.md`, ADR 0108, the delivery profile and the
  `factory-operations` specification with the accepted mechanisms, and record
  the decision and its divergence from the org-level policy in ADR 0135.

## Capabilities

### Modified capabilities

- `factory-operations`: contribution provenance accepts the declared signature
  envelopes, declares them in the tracked policy and fails closed with an
  actionable diagnostic.

## Non-goals

No change to `hyperledger-identus/.github`; the org DCO/PGP policy is a public
commitment owned outside this repository. No allowed-signers allowlist
enforcement, since both accepted mechanisms continue to rely on GitHub's
verification result as the hosted trust anchor. No change to DCO trailer
requirements, branch grammar, review rules or merge authority. No repair of
PR #337, which unblocks itself once this lands. No hardening of the workflow's
base-versus-head policy evaluation, which is tracked separately as issue #339.

## Delivery

Issue #338 owns this change and its evidence. The follow-up base-policy
hardening is issue #339. The pull request must reach green required CI and then
stop for explicit maintainer approval, because accepting SSH diverges from an
org-level public commitment.
