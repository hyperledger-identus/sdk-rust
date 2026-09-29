# SDK-TS 8.1.4 capability inventory

**Snapshot:** 2026-09-29

**Source:** [`sdk-ts@4bf86eb`](https://github.com/hyperledger-identus/sdk-ts/tree/4bf86ebf69d5e96616a148e4c973f831f95fa38e)

**Published package:**
[`@hyperledger/identus-sdk@8.1.4`](https://www.npmjs.com/package/@hyperledger/identus-sdk/v/8.1.4)

**Decision:** ADR 0169; SDK-TS is the first discovery baseline, not normative
authority

**Machine-readable evidence:**
[`identus-platform-ts-capabilities.toml`](../../architecture/identus-platform-ts-capabilities.toml)

## Executive result

SDK-TS is the best current map of Identus product behavior, but it is not a
single reusable core. Its 8.1.4 public package combines portable identity
semantics, historical JavaScript implementations, embedded Rust/WASM engines,
browser and Node adapters, an agent runtime, and an npm plugin host.

The inventory identifies 24 cohesive responsibilities:

- 10 are preliminary `move-to-rust` candidates because SDK-Rust already owns or
  should own the portable semantics;
- 3 are `replace-upstream` candidates whose donor engines are evidence but not
  approved dependencies;
- 6 remain `defer` until their standard/profile, ownership, or wire contract is
  resolved; and
- 5 remain TypeScript platform responsibilities.

These counts are planning evidence, not implementation authorization. Every
deviation is resolved independently through the authority order in ADR 0169.

## Immutable evidence boundary

The report inspected the repository at
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`, the source revision associated
with package version 8.1.4 on 2026-09-29. The donor checkout was detached and
not modified. Approximate size is 216 non-test-like SDK TypeScript files, 67
executable `*.test.ts` entries, 84 TypeScript files below the SDK test tree,
and 15 Gherkin scenarios dependent on Cloud Agent or Mediator infrastructure.

File and test counts measure discovery scope only. They do not establish
coverage, correctness, or compatibility.

## Package and runtime contract

| Surface | SDK-TS evidence | Rust migration consequence |
|---|---|---|
| Root package | ESM, CommonJS, declarations | Rust-backed behavior must preserve idiomatic TypeScript DTOs and errors |
| Plugin exports | `anoncreds`, `didcomm`, `oidc`, `dif`, `oea` | Plugin host and subpath layout remain TypeScript-owned |
| Runtime | ES2022 plus DOM; Node 20/LTS guidance | Browser and Node are distinct canary targets |
| Browser policy | last two Chrome, Firefox, Safari, and Edge versions | WASM proof needs bundler and real browser evidence, not compilation alone |
| Private workspace packages | domain, protobuf, AnonCreds WASM, DIDComm WASM, JWE WASM | Private bundling does not make an engine suitable for SDK-Rust |
| Distribution | npm trusted publishing and `tsup` bundles | Rust release and npm binding trains stay separately versioned |
| License | Apache-2.0 repository | Third-party dependency licenses remain candidate-specific gates |

The root export still exposes donor-era namespaces and key types. Those names
are source aliases for migration traceability, not SDK-Rust target modules.

## Normalized capability result

| Responsibility | SDK-TS behavior | SDK-Rust direction | Important deviation decision |
|---|---|---|---|
| key operations | Ed25519, secp256k1, X25519, sign/verify/ECDH | `identus-crypto`; move portable operations | custody and raw secrets remain outside general bindings |
| HD derivation | mnemonic, seed, derivation paths | `identus-crypto`; move | accepted vectors outrank donor implementation |
| DID syntax and documents | DID/DID URL parser and document types | `identus-did`; move | DID Core semantics, not donor object shape, govern |
| peer DID | creation and resolution | defer | pin method version and adapter ownership first |
| Prism DID | long-form construction and resolution | defer to method-specific adapter decision | generic SDK-Rust must remain chain-neutral |
| DID resolution | method and HTTP resolvers | generic Rust contracts plus injected adapters | preserve Rust cancellation/cache/resource semantics |
| JWT VC | credentials, presentations, JWT utilities | `identus-credentials`; move by selected profile | pin the VC profile and JOSE contract per slice |
| SD-JWT | issuance, disclosure and presentation | replace donor dependency behind owned facade | RFC 9901 plus current VC profile; evaluate current Rust engines |
| AnonCreds v1 | issue, store, present, verify, revocation | qualify current engine behind owned facade | current AnonCreds 1.0 evidence outranks embedded fork |
| credential status | revocation and status checks | portable verification in Rust | fetch/cache/privacy policy remains injected |
| presentation exchange | definitions, requests and verification | defer | pin PE version and decide overlap with DCQL |
| OID4VCI wallet | offer through credential acquisition | continue `identus-oid4vci` | current bounded Final Rust core outranks narrower donor behavior |
| DIDComm engine | pack/unpack, routing, secret resolution | qualify current engine behind owned facade | pin DIDComm version/profile; embedded fork is not selected |
| DIDComm protocols | mediation, pickup, issue, present, OOB, basic message | split portable state from orchestration | Cloud Agent/Mediator behavior needs contract evidence |
| wallet storage contracts | repositories, models, relationships | move portable contracts to `identus-wallet` | browser database and reactive runtime stay TypeScript-owned |
| backup format | version 0.0.1 export/restore and JWE | defer | wire, encryption and cross-SDK migration evidence required |
| reusable protocol state | connections and task state | defer | isolate deterministic state machines first |
| agent runtime | jobs, events, fetch and lifecycle | retain TypeScript | application policy and scheduling are platform responsibilities |
| plugin host | registration and optional plugin exports | retain TypeScript | Rust exposes capabilities, not a TypeScript plugin framework |
| browser/Node networking | fetch, WebSocket, redirects and callbacks | retain TypeScript | Rust accepts bounded inputs and injected responses |
| npm packaging | ESM/CJS/types/subpaths | retain TypeScript | package compatibility remains a canary gate |
| DID value WASM facade | proposed opt-in DID/DID URL path | first canary | non-secret, narrow, observable, and immediately reversible |

## Dependency and engine evidence

| Donor dependency | Donor role | SDK-Rust decision |
|---|---|---|
| `@hyperledger/identus-apollo@^1.4.5` | crypto/key behavior | do not reproduce the package boundary; use existing `identus-crypto` evidence |
| `@sd-jwt/sd-jwt-vc@^0.7.1` | SD-JWT and SD-JWT VC | not selected; assess current RFC 9901/profile Rust engines |
| `input-output-hk/anoncreds-rs@08fbc3f…` | private AnonCreds WASM engine | compatibility evidence only; reassess current AnonCreds 1.0 engine |
| `elribonazo/didcomm-rust@9c4fcdf…` | private DIDComm WASM engine | compatibility evidence only; assess current engine and profile |
| RIDB packages | browser/Node persistence | retain as platform adapter evidence, not generic core dependency |
| `did-jwt@^8.0.4` | JWT issue/verify | preserve wire evidence; Rust uses its owned JOSE boundary |
| `jsonld@^9.0.0` | JSON-LD processing | capability-specific dependency decision required |
| TypeBox | runtime data shapes | retain TypeScript DTO validation where it is a facade responsibility |
| `multiformats@^13.4.2` | encoding and DID support | compare behavior through vectors; no automatic Rust dependency |

No Rust dependency is activated by this inventory.

## Test authority

| Evidence family | Preliminary authority | Required promotion evidence |
|---|---|---|
| BIP-32/BIP-39 fixtures | `normative-candidate` | original publication, exact inputs/outputs and profile scope |
| DID parsing and method unit tests | `implementation-regression` | shared vector plus governing specification clause |
| package API/build tests | `consumer-regression-candidate` | published-package and named-consumer reproduction |
| backup fixtures | `identus-contract-candidate` | cross-SDK bytes/JSON, version, encryption and migration contract |
| Cloud Agent/Mediator E2E | `identus-contract-candidate` | immutable service versions and expected wire/error behavior |
| ordinary unit tests | `implementation-regression` | evidence of public behavior before promotion |
| property, fuzz, benchmark and differential tests | `exploratory` | retained as quality evidence, never normative by themselves |

Issue [#420](https://github.com/hyperledger-identus/sdk-rust/issues/420)
owns exact fixture provenance and promotion. A passing legacy suite or coverage
percentage does not establish authority.

## Known consumers

GitHub source search found references in the Identus workshop,
`hyperledger-identus/identity-portal`, `input-output-hk/lace-kyc`, Trust0
wrappers, and third-party examples. This proves source references, not current
deployment, supported versions, or runtime behavior. Each migration slice must
name and rehearse the affected consumer before changing a default.

## Deviation playbook

For every mismatch among SDK-TS, SDK-Swift, SDK-KMP, SDK-Rust, or a candidate
library:

1. name the smallest cohesive responsibility and exact behavior in conflict;
2. pin every implementation, standard, profile, and dependency revision;
3. apply ADR 0169's authority order rather than voting among SDKs;
4. classify fixtures using ADR 0164 and preserve incompatible evidence;
5. choose `move-to-rust`, `retain-platform`, `replace-upstream`, `defer`,
   `deprecate`, or `drop` with an explicit owner;
6. quarantine safe legacy compatibility at a versioned outer facade;
7. reject unsafe legacy behavior even when a donor or consumer exhibits it;
8. record target, dependency-cone, resource, secret, wire, persistence,
   migration, observability, and rollback constraints; and
9. activate work only through a bounded issue and OpenSpec receipt.

This is how “latest SDK” remains useful without becoming normative.

## First canary recommendation

Use bounded DID and DID URL value parsing through the existing experimental
`identus-wasm-did` surface. The canary is opt-in and excludes resolution,
method operations, keys, storage, networking, and agent orchestration. Its exit
evidence must include:

- stable TypeScript DTO and error mapping;
- ESM, CommonJS, declarations, bundler, browser, and Node checks;
- shared positive, negative, boundary, and differential vectors;
- package-size and parse-latency baselines;
- one named consumer rehearsal;
- route-level observability; and
- immediate fallback to the existing TypeScript implementation.

The canary proves the migration mechanism, not retirement of SDK-TS.

## Follow-up order

1. assess current RFC 9901 and SD-JWT VC Rust engines under
   [#489](https://github.com/hyperledger-identus/sdk-rust/issues/489);
2. refresh the AnonCreds 1.0 engine/adapter decision under
   [#490](https://github.com/hyperledger-identus/sdk-rust/issues/490);
3. select a DIDComm version, engine, and protocol boundary under
   [#491](https://github.com/hyperledger-identus/sdk-rust/issues/491);
4. resolve peer/Prism adapter ownership under
   [#493](https://github.com/hyperledger-identus/sdk-rust/issues/493),
   Presentation Exchange/DCQL responsibilities under
   [#494](https://github.com/hyperledger-identus/sdk-rust/issues/494), and the
   backup wire contract under
   [#495](https://github.com/hyperledger-identus/sdk-rust/issues/495);
5. build the shared vector catalog under #420;
6. specify and execute the reversible SDK-TS DID/DID URL canary under
   [#492](https://github.com/hyperledger-identus/sdk-rust/issues/492);
7. use SDK-Swift discovery to challenge the normalized inventory; and
8. inspect SDK-KMP last for required compatibility, not target architecture.
