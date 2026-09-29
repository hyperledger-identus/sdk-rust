# A1 compatibility evidence source audit

**Date:** 2026-09-30

**Scope:** planning evidence for milestone 5 and issue
[#504](https://github.com/hyperledger-identus/sdk-rust/issues/504)

**Decision:** implementation-ready source inventory; not a conformance,
support, dependency-adoption, or consumer-migration claim

## Audit method

The audit used immutable GitHub trees/content at the accepted revisions rather
than dirty local donor worktrees. Representative fixture and test paths were
inspected for origin, portability, authority, and reuse value. File counts and
test agreement were not treated as correctness evidence. Official standards,
accepted Identus decisions, and canonical SDK-Rust contracts take precedence.

## Sources and dispositions

| Source | Immutable identity | Candidate evidence | Authority and A1 use | Limits |
| --- | --- | --- | --- | --- |
| [W3C DID Core 1.0](https://www.w3.org/TR/did-core/) | W3C Recommendation, 2022-07-19 | DID/DID URL grammar, data model, conformance basis | `normative`; cite exact sections and derive/publicly redistribute bounded seed cases | DID Core does not define method-specific peer or Prism behavior |
| SDK-Rust | `1a638af6cd5985dc3a85f1e4b23b379158da7eb4` | `crates/did/tests/did_syntax.rs`, public error codes, byte limits, serde equivalence | canonical current Rust behavior; seed implementation and regression selectors | existing tests are not automatically normative and are not yet language-neutral packets |
| [SDK-TS 8.1.4](https://github.com/hyperledger-identus/sdk-ts/tree/4bf86ebf69d5e96616a148e4c973f831f95fa38e) | `4bf86ebf69d5e96616a148e4c973f831f95fa38e` | DID/DID URL parser tests, DID fixtures, peer DID cases, BIP vectors, credential fixtures, DIDComm/application flows, E2E features | `consumer-regression`; use selected generic DID cases to discover migration requirements | TypeScript DTOs/errors are not canonical; peer/Prism/protocol cases are later milestones |
| [SDK-Swift](https://github.com/hyperledger-identus/sdk-swift/tree/ebbfdb666a3b0c9905dd443589e03ccf2ae8087b) | `ebbfdb666a3b0c9905dd443589e03ccf2ae8087b` | DID/DID URL, peer DID, SD-JWT, AnonCreds, DIDComm and E2E tests | `consumer-regression`; compare the first generic DID packet and later expose deviations | Apple wrapper shapes, errors, storage, and runtimes remain platform-owned |
| [SDK-KMP](https://github.com/hyperledger-identus/sdk-kmp/tree/5a8fda770bbbca84192979b2c8e6090ee94790e3) | `5a8fda770bbbca84192979b2c8e6090ee94790e3` | copied DID parser cases, BIP vectors, DIDComm/application tests, older E2E features | `consumer-regression`; compatibility-only cross-check after TS/Swift | oldest SDK; missing or old behavior is not a target contract |
| [Apollo](https://github.com/hyperledger-identus/apollo/tree/ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c) | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | crypto and BIP behavior already normalized into 22 stable vector records | existing classified oracle/consumer vectors; reference current IDs without copying payloads | no A1 DID seed; historical module/API names are provenance only |
| NeoPRISM generic DID evidence | `8becb225132efb1d9302b2c5f6ed4d87b84e8685` | generic DID documents, relationships and resolution envelopes; current repository also has crypto/DID tests | `consumer-regression` and differential oracle for later document/resolution packets | Prism method, Cardano, node, storage and HTTP policy remain downstream |

The five donor repositories are Apache-2.0 at the assessed revisions. That
permits reuse but does not eliminate attribution, exact-revision, authorship,
redistribution, or authority fields in the vector record.

## Representative observations

### Generic DID syntax is the right seed

SDK-TS `DIDParser.test.ts`, SDK-Swift `DIDParserTests.swift`, and SDK-KMP
`DIDParserTest.kt` repeat the same three positive and five negative examples.
This is valuable compatibility evidence and a warning: copied cases are not
three independent authorities. SDK-Rust already has a stronger bounded suite
covering ASCII grammar, percent escapes, DID URLs, exact byte limits, serde,
allocation behavior, and redacted stable errors.

The first packet should therefore combine:

- normative DID/DID URL grammar cases derived from W3C DID Core;
- accepted SDK-Rust boundary and redaction cases classified as Identus or
  implementation contracts as appropriate; and
- pinned donor examples classified as consumer regressions.

It must not force donor error strings such as `Invalid DID string` into the
Rust error model. Issue #505 maps canonical `did.invalid_did` and
`did.invalid_did_url` outcomes to temporary language shapes.

### Method and protocol fixtures stay deferred

SDK-TS carries useful peer DID creation/resolution values and explicitly cites
a historical key-ID clarification. SDK-Swift produces a different serialized
service ordering/shape in its peer creation test. That is a deviation to
resolve under A2, not a value to bless inside A1. The current peer DID
standardization proposal also distinguishes current numalgo 4 from historical
numalgo 2 behavior, reinforcing the need for a separate version decision.

SD-JWT, AnonCreds, DIDComm, and application-protocol tests are inventory inputs
for A3/A4/A6. Their current library/draft/service coupling makes them poor seed
data for the shared infrastructure.

### Existing Rust records should be referenced, not rewritten

Apollo parity already owns stable vector IDs, source revisions, transformations,
selectors, coverage and limitations. OID4VC and error golden records also have
their own integrity gates. A1 must permit external record references and a
controlled migration path; a new umbrella file must not copy every existing
payload or replace domain-specific validators.

## Gaps A1 must close

| Gap | Owner | Required closure |
| --- | --- | --- |
| No cross-language vector/provenance schema | #420 | versioned catalog, hashes, authority/license rules, offline validator, mutation tests, DID seed |
| No canonical Rust-to-language mapping record | #505 | DTO/error mapping schema, compatibility window/deprecation/rollback, validator, DID seed mappings |
| Consumer ledger is empty and cannot reference shared evidence | #422 | schema v2, stable cross-references, renderer, mutation tests, no fake runtime change required |
| Four quality classes are named but not routable | #501 | exact declaration schema, evidence/not-applicable rules, risk/target routing, DID seed declaration |
| A1 ownership and closure were implicit | #504 | machine delivery graph, GitHub sub-issues/dependencies, combined validator receipt |
| First consumer proof could start too early | #492 | remain blocked by all four contracts; later consume the exact packet through a versioned TS adapter |

## Proposed DID seed inventory

The seed is deliberately small enough for review while covering distinct
failure classes:

1. valid bare DID with simple and colon-separated method-specific IDs;
2. valid percent escapes with case preservation;
3. valid DID URL path, query, fragment, empty query, and empty fragment;
4. invalid scheme, method, empty ID, malformed escape, Unicode/space, and DID
   containing URL delimiters;
5. exact DID and DID URL byte boundaries;
6. canonical stable Rust error code and redaction expectation; and
7. the pinned donor examples as separately classified consumer regressions.

Peer DID, DID document JSON, resolution, registration, protobuf, network, and
cryptographic operations are explicitly outside this first packet.

## Conclusion

A1 can proceed without selecting a new Rust crate or changing a consumer. The
evidence exists, but it needs four small contracts and stable cross-references.
The dependency order is #420/#505, then #501/#422 closure, then combined #504
validation. Only after that should #492 implement the SDK-TS DID canary.
