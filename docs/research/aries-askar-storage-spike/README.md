# Aries Askar 0.4.6 storage spike

This unpublished, separately locked fixture is executable evidence for
[ADR 0103](../../adr/0103-spike-aries-askar-as-an-isolated-storage-adapter.md).
It adapts one SDK-owned exact-record `SecretStore` surface to encrypted
in-memory SQLite and runs the shared storage conformance suite. It is not a
production dependency, database recommendation, custody design or support
claim.

Run the host evidence from the repository root:

```text
nix develop --command ./scripts/check-aries-askar-storage-spike.sh
```

The fixture enables only the candidate's `sqlite` feature. PostgreSQL, FFI,
logger, migration, persistent files and downstream repositories are untouched.
