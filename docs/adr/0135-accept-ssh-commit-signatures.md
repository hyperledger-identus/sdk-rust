# ADR 0135: accept SSH commit signatures in the contribution gate

- **Status:** Accepted under maintainer direction
- **Date:** 2026-09-23
- **Issue:** [#338](https://github.com/hyperledger-identus/sdk-rust/issues/338)
- **Follow-up:** [#339](https://github.com/hyperledger-identus/sdk-rust/issues/339)
- **Diverges from:** the org-level Hyperledger Identus DCO and PGP policy
- **Corrects the wording of:** ADR 0108 point 5
- **Review no later than:** the next contribution-policy revision

## Context

The hosted `pull-request-policy` gate rejected a commit that GitHub had already
verified. Commit `e93fe45a714652fcbab1076dc843da2f96b5dddb` on PR #337 is
reported by GitHub as `verified: true`, `reason: "valid"`, with the signature
envelope `-----BEGIN SSH SIGNATURE-----`; the gate failed it with
`GitHub OpenPGP verification failed (valid)` because
`scripts/ci/contribution-policy.mjs` hard-coded the OpenPGP envelope and
`.github/contribution-policy.json` set `requireOpenPgp: true`.

The repository documented a weaker requirement than the gate enforced.
`CONTRIBUTING.md` required "a cryptographic signature that GitHub can verify",
which an SSH signature satisfies. `DCO.md` deferred to the canonical
Hyperledger Identus DCO and PGP policy, which requires a PGP key. A maintainer
holding only a GitHub-registered SSH signing key therefore could not open a
mergeable pull request, and the failure text did not describe the actual cause.

## Decision

1. The accepted commit-signature mechanisms become a declared set in the
   tracked contribution policy: `requireSignature` plus `signatureEnvelopes`
   containing the GitHub-verifiable OpenPGP and SSH envelopes. The set replaces
   the `requireOpenPgp` boolean.
2. Hosted provenance keeps GitHub's verification result as the trust anchor and
   fails closed on a commit GitHub does not report as verified and valid, a
   commit without a signature, and a verified commit whose envelope is not
   declared. The three conditions report distinct diagnostics.
3. Local commit-range verification and the pre-push hook become
   mechanism-agnostic, and a local verification failure names git's reported
   cause, including a missing `gpg.ssh.allowedSignersFile`.
4. The repository accepts SSH signatures without waiting for the org-level
   policy, which is owned in `hyperledger-identus/.github` and is not edited
   here. The divergence is recorded in `DCO.md` and in issue #338 for
   maintainer-visible reconciliation.
5. Enforcing a checked-in allowed-signers allowlist remains deferred, and the
   workflow property that a pull request is judged by the policy in its own
   tree is tracked as issue #339 rather than fixed here.

## Consequences

- A maintainer who signs with an SSH key can open a mergeable pull request, and
  the gate's diagnostic names the real failure mode.
- OpenPGP-signed contributions keep working unchanged, and the repository never
  requires PGP-only tooling to satisfy the merge gate.
- Assurance is unchanged in kind: both mechanisms still require a signature
  GitHub attributes to a key registered on the pushing account, and the hosted
  check still does not validate envelopes against a repository-owned trust set.
- This repository is now less strict than its parent organization's published
  DCO document. That is a deliberate maintainer decision with a named
  reconciliation path, not an unnoticed drift.
- Because the policy is read from the pull request's own tree, this change is
  self-applied: its commits are SSH-signed and are admitted by the contract
  they introduce. The pull request discloses this.
