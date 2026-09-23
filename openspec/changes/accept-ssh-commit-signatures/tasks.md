# Tasks

## 1. Planning and decision record

- [x] 1.1 Create issue #338 with the reported failure, the evidence and the
      recorded limitation, and issue #339 for the deferred base-policy
      hardening.
- [x] 1.2 Record the accepted-mechanism decision, the authority and the
      org-level divergence in ADR 0135 and this OpenSpec change.
- [x] 1.3 Pass research-readiness, constraint-readiness and strict OpenSpec
      validation, commit the planning-only contract, and write the
      exact-base/exact-head preflight receipt.

## 2. Policy and checker implementation

- [x] 2.1 Replace `requireOpenPgp` with `requireSignature` and the declared
      `signatureEnvelopes` set in `.github/contribution-policy.json`, and fail
      loudly on a missing or malformed set.
- [x] 2.2 Accept either declared envelope in `validateCommitEvidence` and split
      the unverified, missing-envelope and rejected-envelope diagnostics.
- [x] 2.3 Make `validateCommitRange` signature-generic, rename the CLI flag to
      `VERIFY_SIGNATURES` with the legacy `VERIFY_OPENPGP` alias, and make a
      local verification failure name git's reported cause.
- [x] 2.4 Pass the renamed parameter from the pre-push hook and rename the
      `gpg-signing` delivery-profile invariant.

## 3. Tests and documentation

- [x] 3.1 Add PGP-accepted, SSH-accepted, unknown-envelope, unsigned,
      unverified-with-valid-reason and exact-head-mismatch cases to
      `scripts/tests/factory-operations.mjs`.
- [x] 3.2 Align `DCO.md` and `CONTRIBUTING.md` with the accepted mechanisms,
      including the SSH setup path and the divergence statement.
- [x] 3.3 Correct the provenance wording in ADR 0108 and in the
      `factory-operations` specification, and record the canonical
      requirement supersession in `archive-intent.toml`.

## 4. Evidence and delivery

- [x] 4.1 Run the contribution-policy and factory-contract suites, strict
      OpenSpec validation, `scripts/factory check`, and the `factory-contract`
      and `lint-text` nix checks; report every unrun gate exactly.
- [ ] 4.2 Complete a distinct local review pass in a fresh context and resolve
      blocking findings.
- [ ] 4.3 Complete the factory readiness receipt and archive the change through
      `scripts/factory archive`.

## Delivery boundary

Opening the pull request, monitoring the required checks and the maintainer
approval that follows are delivery events, not implementation tasks. The pull
request targets `develop` with the issue reference, the exact head, the
validation evidence and the disclosed limitation, and it stops for explicit
maintainer approval because accepting SSH diverges from the org-level policy.
The follow-up base-policy hardening stays in issue #339.
