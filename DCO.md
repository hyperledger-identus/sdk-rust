# Developer Certificate of Origin and commit signatures

All contributions follow the canonical
[Hyperledger Identus DCO and PGP policy](https://github.com/hyperledger-identus/.github/blob/main/DCO.md).

Every commit must include a `Signed-off-by` trailer matching its author and a
cryptographic signature GitHub can verify:

```bash
git commit -S -s -m "type: concise summary"
```

The sign-off certifies the Developer Certificate of Origin 1.1; it is not a
substitute for the cryptographic signature. Pull requests with a missing DCO or
unverified commit are not mergeable. Do not rewrite a shared branch merely for
cosmetic history; coordinate required signature/DCO repairs with maintainers.
