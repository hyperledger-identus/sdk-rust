# SDK-TS DID canary quality receipt

- Candidate revision: `c37a35d7344c312211d0395c18f2866875ac9b7f`
- Recorded: `2026-11-10`
- Host: `aarch64-darwin`
- Scope: private DID and DID URL TypeScript facade over `identus-wasm-did`

## Property and strict-type evidence

Command:

```text
nix develop path:.#typescript --command ./scripts/check-sdk-ts-did-target.sh all
```

The command passed at the candidate revision. It reproduced the web and Node
WASM artifacts twice with assembly SHA-256
`ed7c98a4bb7808582be56fd7f9365014c63500699f379f1f1be06e8e0b6730ff`,
passed the one-way dependency boundary, strict TypeScript compilation, 13
provider and public-API tests, all 22 browser vectors, Node ESM and CommonJS,
React server rendering and browser hydration. React Native was recorded as
inapplicable to the WASM canary.

## Fuzz evidence

Command:

```text
nix develop path:.#fuzz --command ./scripts/fuzz-did.sh soak all
```

Both targets used cargo-fuzz 0.13.2 with AddressSanitizer,
`max_len=139264`, `timeout=5`, `rss_limit_mb=1024`, and a 300-second target
budget.

| Target | Executions | Coverage/features | Peak RSS | Result |
| --- | ---: | ---: | ---: | --- |
| `did` | 69,376,205 | 188 / 380 | 525 MiB | passed |
| `did_url` | 56,697,346 | 463 / 1,089 | 714 MiB | passed |

No crash, timeout, sanitizer finding, or failure artifact was produced. Both
targets reached the declared maximum input size.

## Measurement evidence

Command:

```text
nix develop path:.#typescript --command ./scripts/check-sdk-ts-did-target.sh measurements
```

The directional Node 24.19.0 measurement on Apple arm64 recorded 55,095 web
artifact bytes, 49,489 Node artifact bytes, 0.124083 ms initialization, 101
measured parses after 21 warmups, ordinary parse p50/p95 of 0.003500/0.005791
ms, near-limit parse p50/p95 of 0.403375/0.444583 ms, and 53,854,208 bytes
process RSS. These values are measurements, not accepted support or regression
budgets.

## Differential evidence

Command:

```text
nix develop path:.#typescript --command ./scripts/check-sdk-ts-did-target.sh differential
```

The provider-neutral assertions passed all 22 canonical `did.syntax.v1`
vectors through the applicable browser, Node, and React web surfaces, including
exact values, stable redacted codes, and absent-versus-empty DID URL
components. The test-only legacy provider proves rollback selection mechanics;
it does not claim that the external SDK-TS consumer has adopted the facade.

## Rust wrapper coverage

Command:

```text
nix develop path:. --command cargo llvm-cov -p identus-wasm-did --summary-only
```

All three wrapper tests passed. The report recorded 95.40% region, 91.30%
function, and 94.12% line coverage for `crates/wasm-did/src/lib.rs`.

## Limitations

This receipt does not prove external SDK-TS adoption, a default-route switch,
npm publication, accepted performance budgets, React Native delivery, or
production support. Those remain separate post-merge acceptance gates under
issue #492.
