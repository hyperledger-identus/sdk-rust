# Research

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-23
Source retrieval date: 2026-09-23
Research blockers: none

## Problem and existing implementation

The current implementation of contribution provenance lives in three tracked
files. `.github/contribution-policy.json` declares the invariants, including
`"requireOpenPgp": true`. `scripts/ci/contribution-policy.mjs` consumes them in
`validateCommitEvidence`, which requires `verification.verified === true`,
`verification.reason === "valid"` and a signature string beginning with
`-----BEGIN PGP SIGNATURE-----`; the local path in `validateCommitRange` uses
the literal commit header `gpgsig -----BEGIN PGP SIGNATURE-----` and an
`OpenPGP`-named verification flag. `scripts/git-hooks/local-policy.mjs` calls
that range check from its `pre-push` hook.

The hosted path is the only one that decides mergeability. The workflow
`.github/workflows/pull-request-policy.yml` reads the pull request's commits
through `gh api .../pulls/N/commits`, passes each record's `verification`
object to the checker, and the checker never inspects the signature bytes
itself. The hosted trust anchor is therefore GitHub's verification result, and
the envelope test is a mechanism filter layered on top of it.

Consumer evidence. The consumers of this contract are the hosted
`pull-request-policy` gate, the repository-owned pre-push hook, and every
contributor or agent who must produce an admissible commit. The observed
consumer outcome on PR #337 is a maintainer who is blocked from a mergeable
pull request by a mechanism requirement he does not satisfy, together with a
diagnostic that does not identify the actual cause.

Observed behavior on PR #337: commit
`e93fe45a714652fcbab1076dc843da2f96b5dddb` is reported as verified and valid
with an SSH envelope, and the gate fails it with
`GitHub OpenPGP verification failed (valid)`. A locally reproduced control
confirms the two mechanisms are equally verifiable by git when the identity is
declared: an SSH-signed commit verifies as `%G?` = `G` with
`gpg.ssh.allowedSignersFile` configured, and the same commit is reported by
GitHub as `verified: true`, `reason: "valid"`.

## Normative sources

- Hyperledger Identus DCO and PGP policy, retrieved 2026-09-23:
  <https://github.com/hyperledger-identus/.github/blob/main/DCO.md>. It states
  that contributions must be "signed with a PGP key". This is the source of the
  current divergence; it is an org-level public commitment and is not modified
  by this change.
- GitHub documentation, "About commit signature verification", retrieved
  2026-09-23: <https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification>.
  It defines Git's four signature states and records that GitHub marks X.509,
  OpenPGP and SSH signatures, and that a signature is "Verified" only when the
  signing key is registered on the account.
- GitHub REST API commits reference, retrieved 2026-09-23:
  <https://docs.github.com/en/rest/commits/commits>. It defines the
  `verification` object consumed by the hosted checker: `verified`, `reason`
  and the raw `signature` (the envelope) and `payload` of the signed claim.
- Git documentation, `gpg.format` and `gpg.ssh.allowedSignersFile`, retrieved
  2026-09-23: <https://git-scm.com/docs/git-config>. `gpg.format` selects
  `openpgp`, `x509` or `ssh`; SSH verification through `git verify-commit`
  requires an allowed-signers file.
- OpenSSH signature format, `PROTOCOL.sshsig`, revision as shipped with
  OpenSSH 9.x, retrieved 2026-09-23:
  <https://github.com/openssh/openssh-portable/blob/master/PROTOCOL.sshsig>.
  The SSHSIG format is specified there and in the IETF draft
  `draft-miller-sshsig-01`, which is expired and not a standards-track
  specification.

## Candidate decisions

- `retain-local`: keep the repository-owned checker, keep GitHub's verification
  result as the hosted trust anchor, and replace the single hard-coded PGP
  envelope with a declared set of accepted envelopes in the tracked policy.
  Chosen. It is the smallest change that removes the divergence, it preserves
  the existing fail-closed structure and record shape, and it keeps the
  decision in a tracked, reviewable contract.
- `adopt` a dedicated signature-verification library for PGP and SSHSIG to
  verify envelopes in CI instead of trusting GitHub: rejected for this change.
  It would add a runtime dependency cone, a key-distribution problem and a
  second source of truth for "which keys are trusted", without changing the
  mergeability decision that GitHub already makes.
- `not-adopt` X.509 or Sigstore/cosign identities in this change: rejected.
  Neither is currently registered or required by the project, and adding either
  would widen the mechanism set far beyond the reported defect.
- `not-applicable` for the crypto-primitive candidates: no cryptographic
  primitive, curve, parameter set, provider or key-generation path is
  introduced or changed here. The change selects which externally produced
  signature envelopes the repository accepts; it implements no cryptography.

## Compatibility and dependency evidence

Exact tool versions. The reproduced evidence above was produced with Node
v24.19.0, git 2.54.0 and OpenSpec 1.13.1, all from the pinned Nix devshell. No
optional build feature, Cargo feature or runtime feature is added, removed or
re-gated by this change; the checker keeps using the Node standard library
only.

The change edits Node scripts, one JSON contract, one Markdown policy pair, one
OpenSpec specification and one ADR. It adds no dependency, so the direct and
resolved dependency cone is unchanged; `package.json` and `package-lock.json`
are untouched, and the change introduces no Rust dependency at all. No `unsafe`
code and no native code path is added, removed or re-enabled; the workspace
crate graph, FFI surface and bindings are untouched.

MSRV is unaffected: the workspace MSRV stays at the pinned
`SDK-COMPAT-002` value, and the Node runtime requirement stays the pinned
`nix/checks` Node version used by the existing tests. No supported Rust target
matrix entry changes.

There is no public API, facade or wire-format change: the modified surface is
internal CI policy, not a Rust public item, not a serialized message and not a
protocol exchange. The JSON contract is a configuration file consumed only by
the checker, whose record shape and exit codes are preserved.

License and provenance: no third-party source is copied. The consulted
documents are specifications and project policy, cited above; the change is
authored in-repository under the existing Apache-2.0 project license.

## Security, privacy and maintenance evidence

Threat model. The invariant is that every commit entering `develop` carries a
signature that GitHub attributes to a registered key of the pushing identity,
plus an exact `Signed-off-by` trailer. For both accepted mechanisms the hosted
trust anchor is the same GitHub verification result, so accepting SSH does not
weaken the hosted check: the account must still register the SSH key as a
signing key, and the commit must verify against it. The SSH mechanism uses
ed25519 signing keys in current practice, which is at least as strong as the
OpenPGP key types in use.

Residual risk. The hosted check still trusts GitHub's verification boolean
rather than independently validating the envelope against a checked-in set of
trusted keys, so a compromised or mistaken GitHub verification result is not
independently caught. This limitation already exists for OpenPGP and is
unchanged by this decision; enforcing an allowed-signers allowlist is deferred,
not silently adopted.

Privacy. No secret, private key, passphrase or credential is read, stored or
transmitted. The checker consumes only public commit metadata and the public
signature envelope that GitHub already exposes through its API.

Supply-chain and maintenance posture. No new package, action, tool or registry
credential is introduced; no third-party signing service becomes a release or
merge dependency. Maintenance cost moves in the cheaper direction: maintainers
no longer need to own a PGP key to satisfy a merge gate, and the accepted set is
a one-line declaration in a tracked file whose change requires review.

Rollback. Reverting the change restores the previous OpenPGP-only envelope
filter and the previous diagnostics. Nothing to roll back is stateful: no data
migration runs, no credential rotates, no published artifact or consumer
contract depends on the accepted envelope set, and commits merged while the
change was effective remain valid under the reverted revision because that
revision still accepts OpenPGP.

Release and reconsideration. The change affects only how contribution
provenance is validated, so no release artifact, published version or
distribution channel changes. Reconsideration trigger: revisit this decision if
the org-level policy requires PGP only, if an accepted envelope is found
forgeable under a realistic threat model, or if issue #339 shows the policy
cannot be evaluated independently of the pull request that changes it.

## Rejected or deferred candidates

- Allowed-signers allowlist enforcement (`gpg.ssh.allowedSignersFile` plus a
  tracked trust file for hosted evaluation): deferred. It is a real
  strengthening, but it changes the trust anchor for both mechanisms, requires
  a key-distribution and rotation process, and would make a legitimate
  contributor's unlisted key a new false-negative failure mode. Recorded as a
  follow-up candidate rather than merged into this defect fix.
- Base-versus-head policy evaluation: deferred to issue #339. A pull request
  currently widens or narrows the policy that judges it because the workflow
  checks out `refs/pull/<N>/merge`. This change is necessarily self-applied for
  the same reason, which is recorded as its own known limitation.
- Replacing the checker with a hosted required check or an organization-level
  ruleset: rejected. It would move a repository-owned, offline and testable
  contract into an opaque external setting.
- Requiring both PGP and SSH signatures, or pinning to a single mechanism:
  rejected. It contradicts the reported need and removes contributor choice
  without adding assurance, since both mechanisms share the trust anchor.

## Open questions and blockers

None blocking. The org-level divergence remains a recorded, maintainer-owned
limitation: this repository will accept a mechanism its parent organization's
published DCO document does not describe. That divergence is disclosed in the
pull request and in ADR 0135, and raising it upstream is out of scope here
because `hyperledger-identus/.github` is read-only for this change.

## Evidence commands

Run from the change worktree at the recorded base revision:

- `scripts/factory doctor` and `git rev-parse HEAD` to record the exact base.
- `openspec validate accept-ssh-commit-signatures --type change --strict`.
- `scripts/check-research-readiness.py . --change accept-ssh-commit-signatures --require-ready`.
- `scripts/check-constraints.py . --change accept-ssh-commit-signatures --require-ready`.
- `scripts/factory check`.
- `node --test scripts/tests/factory-operations.mjs`.
- `scripts/tests/factory-contract.sh` for the synthetic fixture contract.
- `nix build .#checks.x86_64-linux.factory-contract` and the required `fast`
  lane checks after the pull request is opened.
- The exact PR #337 hosted failure evidence already collected here:
  `gh pr checks 337`, and
  `gh api --paginate repos/hyperledger-identus/sdk-rust/pulls/337/commits`
  filtered to `commit.verification`.

Deliberately unrun at planning time, and reported rather than inferred:
`nix flake check` (no code, build or dependency change is present yet) and the
cryptography fuzz and benchmark campaigns, which no artifact of this change can
affect. The `fast` nix check that executes
`node --test scripts/tests/factory-operations.mjs` is not runnable at planning
time because the receipt, not the test suite, is the planning artifact.
