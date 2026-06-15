# Interop Fixtures

Use this directory for fixtures that compare `sdk-rust` behavior against
reference implementations, partner SDKs, or standards conformance suites.

`wrapper-api-parity.json` is the first generated parity inventory for existing
TypeScript, Swift, and Kotlin SDK public exports. It is allowed to cite legacy
source names because it is migration evidence, not a new public Rust API.
Regenerate or check it with `node tools/generate-wrapper-api-parity.mjs --write`
or `node tools/generate-wrapper-api-parity.mjs --check`.

Expected future layout:

```text
<spec-id>/
  cases/*.json
  README.md
```
