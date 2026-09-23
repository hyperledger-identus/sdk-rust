# Developer Certificate of Origin and commit signatures

All contributions follow the canonical
[Hyperledger Identus DCO and PGP policy](https://github.com/hyperledger-identus/.github/blob/main/DCO.md)
for the Developer Certificate of Origin. This repository additionally accepts
GitHub-verified SSH commit signatures; see [Accepted signatures](#accepted-signatures).

Every authored commit must include a `Signed-off-by` trailer matching its
author and a cryptographic signature GitHub can verify:

```bash
git commit -S -s -m "type: concise summary"
```

The sign-off certifies the Developer Certificate of Origin 1.1; it is not a
substitute for the cryptographic signature. Pull requests with a missing DCO or
unverified commit are not mergeable. Do not rewrite a shared branch merely for
cosmetic history; coordinate required signature/DCO repairs with maintainers.

## Accepted signatures

The tracked contribution policy declares the accepted signature envelopes and
currently accepts GitHub-verifiable OpenPGP or SSH commits. A commit passes only
when GitHub reports it as verified and valid and its envelope is declared, so
the signing key must be registered on the pushing GitHub account, either as an
OpenPGP key or as a signing key.

SSH signing setup for this repository:

```bash
git config gpg.format ssh
git config user.signingkey ~/.ssh/id_ed25519.pub
git config commit.gpgsign true
git config gpg.ssh.allowedSignersFile ~/.ssh/allowed_signers
```

`allowed_signers` is needed only for local verification, for example
`git log --show-signature` or the repository pre-push hook, and each line is
`<email> <key-type> <public-key>`.

## Org-level divergence

The organization DCO document names PGP as the required mechanism. This
repository accepts SSH signatures as well, under the maintainer decision
recorded in
[ADR 0135](docs/adr/0135-accept-ssh-commit-signatures.md) and issue
[#338](https://github.com/hyperledger-identus/sdk-rust/issues/338). The
divergence is deliberate and is reported rather than hidden; reconciling it
upstream belongs to the organization repository, which this repository does not
modify.

## GitHub branch synchronization

GitHub's **Update branch** merge path creates platform metadata whose subject
and trailers are not contributor-controlled. Hosted CI recognizes that record
only when its verified signature, `web-flow` committer identity, canonical
base-into-head subject, two-parent graph and reproduced conflict-free merge
tree match the pull-request event. It then omits the authored subject and DCO
checks while retaining signature validation.

This is not a general merge exemption. Ordinary web-editor commits, local
merges, unverified merges and incomplete or unexpected hosted records retain
all authored-commit requirements. See
[ADR 0136](docs/adr/0136-permit-verified-github-synchronization-merges.md) and
issue [#342](https://github.com/hyperledger-identus/sdk-rust/issues/342).
