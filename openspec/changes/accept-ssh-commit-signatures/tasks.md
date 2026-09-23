# Tasks

## 1. Planning and decision record

- [x] 1.1 Create issue #338 with the reported failure, the evidence and the
      recorded limitation, and issue #339 for the deferred base-policy
      hardening.
- [x] 1.2 Record the accepted-mechanism decision, the authority and the
      org-level divergence in ADR 0135 and this OpenSpec change.
- [ ] 1.3 Pass research-readiness, constraint-readiness and strict OpenSpec
      validation, commit the planning-only contract, and write the
      exact-base/exact-head preflight receipt.

## 2. Policy and checker implementation

- [ ] 2.1 Replace `requireOpenPgp` with `requireSignature` and the declared
      `signatureEnvelopes` set in `.github/contribution-policy.json`, and fail
      loudly on a missing or malformed set.
- [ ] 2.2 Accept either declared envelope in `validateCommitEvidence` and split
      the unverified, missing-envelope and rejected-envelope diagnostics.
- [ ] 2.3 Make `validateCommitRange` signature-generic, rename the CLI flag to
      `VERIFY_SIGNATURES` with the legacy `VERIFY_OPENPGP` alias, and make a
      local verification failure name git's reported cause.
- [ ] 2.4 Pass the renamed parameter from the pre-push hook and rename the
      `gpg-signing` delivery-profile invariant.

## 3. Tests and documentation

- [ ] 3.1 Add PGP-accepted, SSH-accepted, unknown-envelope, unsigned,
      unverified-with-valid-reason and exact-head-mismatch cases to
      `scripts/tests/factory-operations.mjs`.
- [ ] 3.2 Align `DCO.md` and `CONTRIBUTING.md` with the accepted mechanisms,
      including the SSH setup path and the divergence statement.
- [ ] 3.3 Correct the provenance wording in ADR 0108 and in the
      `factory-operations` specification, and record the canonical
      requirement supersession in `archive-intent.toml`.

## 4. Evidence and delivery

- [ ] 4.1 Run the contribution-policy and factory-contract suites, strict
      OpenSpec validation, `scripts/factory check`, actionlint and the
      available lint checks; report every unrun gate exactly.
- [ ] 4.2 Complete the factory readiness receipt and archive the change through
      `scripts/factory archive`.
- [ ] 4.3 Complete a distinct local review pass in a fresh context and resolve
      blocking findings.
- [ ] 4.4 Open the signed, DCO-bearing pull request against `develop` with the
      exact head, the evidence and the disclosed limitation.
- [ ] 4.5 Monitor required CI to green, then stop for explicit maintainer
      approval before merge because of the org-level divergence.
