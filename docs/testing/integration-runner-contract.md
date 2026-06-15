# SDK Rust Integration Runner Contract

This document defines how the `integration` repository should execute
`sdk-rust` once the runner is added there. It mirrors the current SDK runner
model while keeping the Rust default Docker-free.

## Runner Identity

| Field | Value |
|---|---|
| Runner key | `sdk-rust` |
| Repository | `hyperledger-identus/sdk-rust` |
| Default mode | Embedded agent and mediator mode |
| Optional mode | External Cloud Agent and Mediator services |
| Result directory | `tmp/sdk-rust` |
| Allure source | `target/sdk-rust/allure-results` |

## Version Checkout

The runner must follow the existing integration `test-runner` lifecycle:

1. Clone or reuse `hyperledger-identus/sdk-rust`.
2. If `build=false`, translate the requested release version to a git tag and
   checkout that tag.
3. If `build=true`, fetch and checkout the requested branch, tag, or commit.
4. Run Rust validation from the repository root.
5. Copy Allure-compatible result files into `tmp/sdk-rust`.

The first implementation can run directly with Cargo:

```bash
cargo test --workspace
```

When the acceptance runner lands, the integration command should become:

```bash
cargo test --workspace --features integration-runner
```

If Allure output is not produced natively yet, the runner must create minimal
Allure-compatible `*-result.json` files from Cargo test outcomes so downstream
report aggregation can continue to use the existing `tmp/<runner>` contract.

## Environment Contract

`sdk-rust` must accept the normalized environment variables used by the current
integration project and compatibility aliases used by existing SDKs.

| Variable | Required | Mode | Meaning |
|---|---|---|---|
| `SDK_RUST_MODE` | No | All | `embedded` by default, `external` for service-backed tests. |
| `AGENT_URL` | External only | External | Cloud Agent base URL, compatible with sdk-ts. |
| `MEDIATOR_OOB_URL` | External only | External | Mediator OOB invitation URL, compatible with sdk-ts. |
| `TEST_RUNNER_PRISM_AGENT_URL` | External only | External | Cloud Agent base URL alias, compatible with sdk-swift. |
| `TEST_RUNNER_MEDIATOR_OOB_URL` | External only | External | Mediator OOB invitation URL alias, compatible with sdk-swift. |
| `SDK_RUST_ALLURE_DIR` | No | All | Override for `target/sdk-rust/allure-results`. |
| `SDK_RUST_SELECTED_FLOWS` | No | All | Comma-separated flow filter for manual runs. |

Embedded mode must not require `AGENT_URL`, `MEDIATOR_OOB_URL`, Docker,
PostgreSQL, Keycloak, PRISM Node, or NeoPRISM. It runs the in-process
`identus-agent` and embedded mediator tests.

External mode may require Cloud Agent, Mediator, and later PRISM/VDR services.
The runner must fail fast with a redaction-safe error when required URLs are
missing.

## Required Flow Labels

The first `sdk-rust` runner should emit Allure labels that match the existing
matrix rows:

| Flow label | Default mode | Notes |
|---|---|---|
| `backup_restore` | Embedded | Covered by `identus-agent`. |
| `connection` | Embedded | OOB connection harness. |
| `receive_jwt_credential` | Embedded | Lightweight credential format model. |
| `receive_sdjwt_credential` | Embedded | Lightweight credential format model. |
| `receive_anoncred_credential` | Embedded | Lightweight credential format model. |
| `provide_jwt_proof` | Embedded | Holder to issuer/verifier. |
| `provide_sdjwt_proof` | Embedded | Holder to issuer/verifier. |
| `provide_anoncred_proof` | Embedded | Holder to issuer/verifier. |
| `receive_jwt_revocation_notification` | Embedded | Local revocation notification model. |
| `verify_jwt_proof` | Embedded | Peer verifier role. |
| `verify_sdjwt_proof` | Embedded | Peer verifier role. |
| `verify_anoncred_proof` | Embedded | Peer verifier role. |
| `receive_oob_jwt_credential` | Embedded | Connectionless OOB fixture. |
| `provide_oob_jwt_proof` | Embedded | Connectionless OOB fixture. |

Rows that depend on real cryptography, pack/unpack, storage encryption, ledger,
OID providers, or HTTP services must be marked skipped with a stable reason
until the corresponding crate and fixture task lands.

## Report Metadata

The runner must preserve the integration report model:

- `sdk-rust` version under `env.runners["sdk-rust"].version`.
- service versions for Cloud Agent, Mediator, and PRISM Node when external mode
  is used.
- release version and draft/final status from the integration environment.
- runner status derived from Allure result status: passed, failed, broken, or
  skipped.
- failure categories that distinguish assertion failure, missing environment,
  unsupported flow, infrastructure failure, and report generation failure.

## Promotion Gates

`sdk-rust` can claim replacement coverage for a legacy SDK row only when:

- the embedded Rust runner covers the row with deterministic tests;
- service-backed external mode is either passing or explicitly not required for
  that row;
- the equivalent wrapper row for TypeScript, Swift, or Kotlin is mapped in the
  migration matrix; and
- Allure output is aggregated by the integration dashboard without custom
  downstream handling.

## Follow-Up Work

- Add the actual `sdk-rust` runner implementation to the `integration`
  repository.
- Add an Allure result writer or adapter for Rust tests.
- Add external-service gates for Cloud Agent, Mediator, VDR, OID4VCI, and
  OpenID4VP flows as the corresponding crates land.
