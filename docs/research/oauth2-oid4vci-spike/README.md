# oauth2 5.0.0 OID4VCI spike

This unpublished, separately locked consumer fixture is executable evidence for
[ADR 0102](../../adr/0102-use-oauth2-500-as-an-authorization-oracle.md). It
proves selected authorization-code and PKCE mechanics and preserves the
candidate's policy mismatches. It is not a production dependency or support
claim.

Run the host evidence from the repository root:

```text
nix develop --command ./scripts/check-oauth2-oid4vci-spike.sh
```

The script performs no network request. Cross-target compile receipts are run
explicitly in the repository's `wasm` and `bindings` Nix shells, as recorded in
the archived OpenSpec verification.
