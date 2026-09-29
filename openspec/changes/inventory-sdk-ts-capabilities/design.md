# Design

## Inventory model

The change adds a dedicated SDK-TS inventory rather than expanding the broad
cross-SDK file-count survey into an unreviewable table. Each record uses a
responsibility-based stable identifier and carries:

- outcome and cohesive responsibility;
- immutable SDK-TS source paths and donor-era aliases for traceability;
- public/package, wire, persistence, and operational surfaces;
- specification/profile and direct engine basis;
- test authority, fixture paths, and contradictions;
- known consumers and compatibility risk;
- SDK-Rust state, target owner, remaining TypeScript owner, and disposition;
- security, resource, target, migration, rollback, and issue evidence.

The existing global registry remains a family-level index and points to the
SDK-specific inventory. It no longer presents legacy labels as target module
boundaries.

## Naming model

Target identifiers use a dotted responsibility taxonomy, for example:

```text
crypto.key-operations
crypto.hd-derivation
did.syntax
did.document
did.method.peer
did.method.prism
credentials.jwt-vc
credentials.sd-jwt
credentials.anoncreds-v1
presentations.exchange
openid4vc.issuance-wallet
messaging.didcomm-v2
wallet.storage
wallet.backup
agent.protocol-state
platform.typescript-plugin-host
```

The old names appear only in `source_aliases` or explanatory prose. Existing
Apollo parity manifests and KMP compatibility identifiers remain immutable
provenance; they do not define new architecture.

## Normalization model

Inventory answers three separate questions for every behavior:

1. What does SDK-TS 8.1.4 expose and what do known consumers rely on?
2. What do current standards, profiles, and authoritative vectors require?
3. What should SDK-Rust own, delegate to a private maintained engine, or leave
   to the TypeScript platform layer?

Differences become explicit gaps or compatibility records. They are not
resolved by copying the newest repository. SD-JWT base mechanics and the
SD-JWT VC profile are separate rows. AnonCreds v1 domain/protocol behavior,
cryptographic engine, VDR access, secrets, and platform packaging are separate
responsibilities. The same separation applies to DIDComm engine behavior and
agent protocol workflows.

## Test authority

The report classifies suites at a useful granularity:

- published BIP vectors can be nominated as `normative` after provenance is
  completed;
- cross-SDK backup and released Cloud Agent/Mediator behaviors can be
  nominated as `identus-contract` or `consumer-regression`;
- ordinary unit tests default to `implementation-regression`;
- coverage, benchmark, property, and differential work is `exploratory`.

Nomination is not promotion. Issue #420 owns the shared catalog and exact
fixture provenance.

## Canary selection

The provisional canary is bounded DID and DID URL value behavior through
`identus-wasm-did`. It is non-secret, narrow, existing, and reversible. The
canary issue must define an opt-in SDK-TS facade route, stable TypeScript DTO and
error mapping, ESM/CommonJS/bundler/browser/Node evidence, shared vectors,
observability, package versions, consumer rehearsal, and immediate fallback to
the current TypeScript path. It must not include DID resolution, method
operations, keys, storage, network, or agent orchestration.

## Verification

Verification is documentary and structural: immutable links, TOML parsing,
finite vocabulary validation, one disposition per record, no legacy target
identifiers, issue linkage, strict OpenSpec/factory checks, Markdown/link
checks, and a distinct semantic review. Child issues own runtime, dependency,
conformance, and consumer proof.
