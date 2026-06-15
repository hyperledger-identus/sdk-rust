# Infrastructure Fixtures

Use this directory for optional conformance fixtures that need minimal Docker,
external services, ledgers, or device adapters.

These tests must stay opt-in. The default conformance suite must be able to run
without Docker or cloud infrastructure.

Expected future layout:

```text
<spec-id>/
  cases/*.json
  README.md
```

`neoprism-vdr-adapters.json` maps NeoPRISM Cardano data sources, submitters,
resolver HTTP, and storage adapters into opt-in infrastructure gates. Its
Docker-free alternative is the PRISM operation/VDR lifecycle vector under
`fixtures/conformance/vector/did-prism/`.
