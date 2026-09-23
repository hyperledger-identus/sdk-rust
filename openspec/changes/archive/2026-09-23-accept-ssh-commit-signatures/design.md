# Design

## Decision record

ADR 0135 records the accepted mechanisms, the maintainer authority for
accepting a mechanism the org-level policy does not describe, and the
divergence that remains open upstream. This design fixes the implementation
details that the ADR intentionally leaves open.

## Policy contract

`.github/contribution-policy.json` replaces the boolean `requireOpenPgp` with a
declared set:

```json
"commit": {
  "requireSignature": true,
  "signatureEnvelopes": [
    "-----BEGIN PGP SIGNATURE-----",
    "-----BEGIN SSH SIGNATURE-----"
  ]
}
```

The set is data, not code, so adding or removing a mechanism is a reviewable
one-line contract change. `requireSignature` keeps the switch explicit so a
future policy can disable the check deliberately instead of by deleting the
list. A missing, empty or non-string entry in `signatureEnvelopes` while
`requireSignature` is true is a policy defect and must fail loudly rather than
silently accepting every envelope.

Rejected alternative: keeping `requireOpenPgp` as a second boolean beside a new
one. Two switches can disagree, and the checker would need an arbitration rule
that no reviewer can verify at a glance.

## Hosted checker

`validateCommitEvidence` keeps GitHub's verification result as the trust anchor
and adds the declared-envelope test:

- `verification.verified !== true` or `verification.reason !== "valid"` reports
  an unverified commit, naming the reason GitHub returned.
- a verified commit whose `verification.signature` does not begin with a
  declared envelope reports a rejected-envelope condition that names the
  rejected envelope, not an OpenPGP failure.
- a verified commit without a signature string reports a missing-envelope
  condition.

The previous single message collapsed all three into
`GitHub OpenPGP verification failed (<reason>)`, which produced the
self-contradictory `failed (valid)` that this change fixes. The record shape
consumed from `gh api .../pulls/N/commits`, the returned `{ok, errors}` shape
and the process exit codes are unchanged, so the workflow needs no edit.

Rejected alternative: inspecting the signature bytes here. The checker is
offline and has no key material, so it cannot do better than the envelope
prefix test, and pretending otherwise would misrepresent the assurance level.

## Local commit-range verification

`validateCommitRange` drops the `gpgsig` PGP header literal and the
`verifyOpenPgp` parameter name in favour of a mechanism-neutral
`verifySignature`. Verification still shells out to `git verify-commit --raw`,
which handles OpenPGP and, with `gpg.ssh.allowedSignersFile` configured, SSH.
The check captures the first non-empty line of git's stderr and appends it to
the failure, so the missing-allowed-signers case names
`gpg.ssh.allowedSignersFile` instead of claiming a PGP failure. A failure
remains a failure: the diagnostic improves, the outcome does not.

The CLI subcommand reads `VERIFY_SIGNATURES` and keeps honoring the legacy
`VERIFY_OPENPGP` name as an alias. This is deliberate: local verification is a
strengthening control, and silently turning an existing external
`VERIFY_OPENPGP=true` into a no-op would weaken a caller's setup without
telling them. Removal trigger: drop the alias in the next policy revision once
no tracked contract or documented workflow references it.

`scripts/git-hooks/local-policy.mjs` passes the renamed parameter and is
otherwise unchanged.

## Delivery profile

`.pi/delivery-profiles.json` lists `gpg-signing` among `alwaysEnforced`
invariants. It is renamed to `commit-signature-verification` so the tracked
profile does not name a mechanism this change stops requiring. The list is
validated for presence rather than content by `scripts/factory-tools/audit-pi.mjs`
and has no other consumer, so the rename is safe and is not a behavior change.

## Tests

`scripts/tests/factory-operations.mjs` gains cases for a PGP envelope, an SSH
envelope, an unknown envelope, an unsigned commit, a commit whose verification
reason contradicts its verified flag, a verified commit without an envelope
string, a commit with no verification record but a declared `gpgsig` header, a
commit whose raw header carries an undeclared envelope, the malformed-policy
branch through the `validateSignatureProvenance` document seam, and an
exact-head mismatch. The existing PGP-accepted and unverified cases stay. Every
case asserts the returned `ok` value, and the envelope cases additionally
assert the diagnostic text, so the regression that produced `failed (valid)`
cannot return unnoticed.

The suite is executed by the required `fast` lane through
`nix/checks/factory-contract.nix` and by `./bootstrap.sh --check`. It is
deliberately not added to `scripts/factory check`, which runs inside a
synthetic fixture where the suite would execute against fixture-relative files;
changing that wiring is a separate risk with no effect on this defect.

## Documentation

- `DCO.md` stops presenting the org-level PGP-only document as this
  repository's controlling gate. It states both accepted mechanisms, records
  the divergence and links ADR 0135 and issue #338.
- `CONTRIBUTING.md` states both accepted mechanisms and adds the short SSH
  setup path: `gpg.format = ssh`, `user.signingkey`, `gpg.ssh.allowedSignersFile`,
  and registering the key on GitHub as a signing key.
- ADR 0108 keeps its numbering and intent; its point about DCO and OpenPGP
  provenance is corrected to name the accepted signature mechanisms, with a
  pointer to ADR 0135 rather than a rewrite of the ADR's decision.
- The `factory-operations` specification gains the declared-envelope contract
  and a scenario for a rejected envelope, and its existing provenance
  requirement is modified to say "accepted commit-signature provenance".

## Specification preservation

The modified requirement changes an existing canonical line, so
`archive-intent.toml` records the canonical requirement and the normalized
SHA-256 of its current text with the reason for superseding it. Archiving is
otherwise unchanged and still runs through `scripts/factory archive`.

## Trade-offs accepted

- GitHub remains the hosted trust anchor. The alternative — a tracked allowed
  signers allowlist — is a real strengthening but changes the trust model for
  both mechanisms and introduces a key-rotation and false-negative burden; it
  is deferred and recorded rather than smuggled into a diagnostic fix.
- The policy that judges a pull request still comes from that pull request's
  tree. Widening the accepted set makes that property more visible; fixing it is
  issue #339.
- The change is self-applied: its own commits are SSH-signed and are admitted
  by the contract it introduces. That is disclosed in the pull request and in
  the ADR rather than hidden by re-signing with a mechanism the change makes
  unnecessary.
