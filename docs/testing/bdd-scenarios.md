# BDD Scenario Inventory

This inventory groups the BDD scenarios found across the local
`hyperledger-identus` repositories and the current GitHub repository list. Source
copies are in `cloud-agent`, `sdk-ts`, and `sdk-kmp`; `docs` mirrors those
feature files. The `integration` repository does not add SSI protocol feature
files, but it defines the cross-component release compatibility matrix and SDK
runner contract that `sdk-rust` must eventually join. Temporary shallow scans of
`identity-portal`, `infrastructure`, and `.github` did not add `.feature` files.

## Sources

| Repository | Source path | Notes |
|---|---|---|
| `cloud-agent` | `tests/integration-tests/src/test/resources/features` | Server-side BDD suite with DID, VDR, credential, OID4VCI, multitenancy, schema, health, metrics, verification API, and verification policy coverage. |
| `sdk-ts` | `integration-tests/e2e-tests/features` | Portable edge-agent E2E suite for connection, issuance, proof, revocation, backup/restore, connectionless flows, and mediator pickup. |
| `sdk-kmp` | `tests/end-to-end/src/test/resources/features` | KMP copy of the portable edge-agent E2E suite, currently without the sdk-ts mediator pickup feature. |
| `integration` | `README.md`, `src/test-runner`, `src/runner`, `tests` | Cross-component release runner, SDK capability matrix, service environment contract, Allure aggregation, report metadata, Slack failure notification, draft/final release cleanup, and weekly/manual compatibility flows. |
| `docs` | `cloud-agent/.../features`, `sdk-ts/.../features` | Mirrored documentation copies; not counted as independent behavior. |

## Capability Map

| Capability | Source features | Representative scenarios | Rust target |
|---|---|---|---|
| Agent connection | `cloud-agent/connection`, `sdk-ts/create_connection`, `sdk-kmp/create_connection` | Establish connection; invitation metadata label/goal handling. | `identus-agent` lightweight invitation/request/response flow. |
| JWT issuance | `cloud-agent/credential/jwt/issuance*`, `sdk-ts/receive_jwt_credential`, `sdk-kmp/receive_jwt_credential` | Published/unpublished PRISM DID issuance, assertion keys, schema validation, one/many credentials, connectionless offer. | Agent acceptance tests model offer/accept/issue without Docker. |
| SD-JWT issuance | `cloud-agent/credential/sdjwt/issuance*`, `sdk-ts/receive_sdjwt_credential`, `sdk-kmp/receive_sdjwt_credential` | Holder binding, selective-disclosure credential content, connectionless issuance. | Same issue/accept/receive harness using `CredentialFormat::SdJwt`. |
| AnonCreds issuance | `cloud-agent/credential/anoncred/issuance*`, `sdk-ts/receive_anoncred_credential`, `sdk-kmp/receive_anoncred_credential` | Schema definition, credential definition, offer/accept/issue. | Same issue/accept/receive harness using `CredentialFormat::AnonCred`. |
| Present proof | `cloud-agent/credential/*/present_proof`, `sdk-ts/provide_*_proof`, `sdk-kmp/provide_*_proof` | Holder presents JWT, SD-JWT, or AnonCreds proof to issuer or verifier; wrong request is rejected or fails. | Agent proof request/presentation/verification tests, including wrong claim marker. |
| Peer verification | `sdk-ts/verify_*_credential`, `sdk-kmp/verify_*_credential` | Verifier Edge Agent asks Holder to prove JWT, SD-JWT, or AnonCreds credential. | Peer verifier role in `identus-agent`. |
| Revocation | `cloud-agent/credential/jwt/revocation`, `sdk-ts/revoke_jwt_credential`, `sdk-kmp/revoke_jwt_credential` | Issuer revokes; holder receives notification; revoked proof verifies false. | Revocation notification over embedded mediator and false proof result. |
| Backup and restore | `sdk-ts/backup`, `sdk-kmp/backup` | Restore with correct seed, reject wrong seed, preserved credentials and DIDs remain functional. | `BackupSnapshot` and restore tests in `identus-agent`. |
| Mediator pickup | `sdk-ts/mediator_message_pickup` | Restored wallet receives messages queued while disconnected. | `EmbeddedMediator` queues by agent id and supports restored-agent pickup. |
| Connectionless exchange | `cloud-agent/credential/*`, `sdk-ts/connectionless_*`, `sdk-kmp/connectionless_*` | OOB credential offer and proof request without persistent connection. | Future agent fixture over the same message queue using OOB message variants. |
| DID lifecycle | `cloud-agent/did/*` | Create, publish, list, update, deactivate PRISM DID; key-purpose validation. | Future `identus-did` fixtures; no Docker in memory mode, ledger scenarios behind optional driver. |
| VDR lifecycle | `cloud-agent/vdr/*` | Create, resolve, share, update, deactivate memory/database/ledger VDR entries. | Future `identus-trust`/`identus-did` VDR ports with memory driver first. |
| OID4VCI | `cloud-agent/oid4vci/*` | Manage issuer, manage credential configuration, issue JWT credential via authorization code flow. | Future `identus-openid4vc` state-machine fixtures; HTTP facade optional. |
| Verification API and policies | `cloud-agent/verificationapi`, `cloud-agent/verificationpolicies` | Verification checks, unsupported checks, policy CRUD. | Future credential verifier fixtures and policy model. |
| Multitenancy and system | `cloud-agent/multitenancy`, `cloud-agent/system` | Wallet CRUD, health endpoint, metrics endpoint. | Future service-level crates; not part of lightweight embedded agent harness. |
| Release compatibility matrix | `integration/README.md`, `integration/src/runner/environment.ts` | Test cloud-agent, mediator, prism-node, and SDK version combinations for component, release, weekly, and manual flows. | Add `sdk-rust` as a runner and define Rust core plus wrapper compatibility gates. |
| SDK runner contract | `integration/src/test-runner/*` | Runners clone/build selected SDK version, pass Cloud Agent and mediator URLs, run tests, and move Allure results into `tmp/<runner>`. | Provide a Rust acceptance runner with `AGENT_URL`, `MEDIATOR_OOB_URL`, optional embedded mode, and Allure-compatible output. |
| Reporting and notification | `integration/src/runner/report.ts`, `integration/tests` | Aggregate Allure results, emit release metadata, clean up draft releases, and notify Slack on failed/broken test results or report-generation exceptions. | Make `sdk-rust` acceptance tests produce stable metadata and failure categories that integration dashboards can consume. |

## First Rust Coverage

`identus-agent` implements the first Docker-free acceptance layer:

- `Agent` can act as issuer, holder, verifier, or peer.
- `EmbeddedMediator` queues envelopes by recipient id and supports offline
  delivery to a restored agent with the same id.
- Credential issuance is modeled as offer, acceptance, issuance, and holder
  storage for JWT, SD-JWT, and AnonCreds format families.
- Present-proof is modeled as request, holder presentation, verifier result.
- Connectionless OOB credential offers and presentation requests are modeled as
  invitation attachments that can be accepted without creating a persistent
  connection.
- Revocation notifications update holder state and make later proof
  verification fail.
- Backup/restore preserves credentials, connections, peer DIDs, and PRISM DIDs
  and rejects wrong seeds.

This is not production DIDComm or credential verification. It is a deterministic
acceptance-test scaffold that lets the Rust SDK capture the existing BDD intent
before Docker-backed Cloud Agent, ledger, database, and external OID provider
fixtures are introduced.
