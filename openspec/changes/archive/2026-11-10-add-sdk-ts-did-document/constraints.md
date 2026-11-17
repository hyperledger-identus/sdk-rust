# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/502
Constraint blockers: none

## Existing entries affected

- `SDK-LIM-002` remains effective: the private document facade is not a
  supported binding commitment.
- `SDK-LIM-003` remains effective: browser WASM and native targets are not
  runtime-certified.
- `SDK-LIM-009` remains effective: the existing terminal `fast` status is the
  protected integration verdict; isolated target evidence does not add a new
  required status.
- TS1 success criteria SC-01 through SC-09 remain controlling.

## Introduced or changed constraints

The slice introduces a private raw-document-to-Rust boundary, an API-versioned
owned WASM view, strict immutable TypeScript projection, shared cross-target
evidence, and fail-closed isolated target routing. The detailed limitations
below prevent those local capabilities from becoming support commitments.

## Introduced or changed limitations

Raw document input SHALL reach `DidDocument::from_json_str` before any
TypeScript JSON parse. The Rust model SHALL remain authoritative for byte,
wire, semantic, public-key, collection, and extension validation.

The private binding SHALL use API version 2, deterministic target-specific
artifacts, exact owned views, and redacted stable errors. Each generated value
or error SHALL be freed exactly once on success, projection failure,
observation failure, and mapped rejection.

The TypeScript projection SHALL be strict, deeply immutable, iteratively
validated, and preserve optionality, scalar/array cardinality, relationship
kind, endpoint kind, and bounded extensions. It SHALL NOT expose generated
types, explicit public `any`, caller-controlled diagnostics, private JWK
material, legacy component names, or silently rounded unsafe integers.

One immutable packet and provider-neutral assertion body SHALL execute on every
applicable target. A Rust rejection or projection limitation SHALL be terminal;
no second parser runs. Rollback SHALL remain explicit provider selection before
initialization.

The existing Rust-core-first graph SHALL remain one-way. Unknown binding impact
selects all target jobs. Unselected targets emit inapplicable receipts; selected
target failures keep the terminal aggregator red without rewriting core proof.

The package remains private and unpublished. Resource measurements are not
support budgets. No output may claim external SDK-TS adoption, default routing,
browser/Node/React support, React Native evidence, publication, stable API,
certification, or production readiness.

## Consumer and product impact

The only consumer-visible addition is a private opt-in
`parseDidDocument(json)` value API in the existing TypeScript/WASM canary. It
does not alter Rust callers, the default provider, the published SDK-TS
package, or downstream products. Consumers that explicitly select the private
provider receive immutable values and stable redacted errors; documents with
unsafe JavaScript integers are rejected for typed projection rather than
rounded.

## Scope limitations

The train is limited to parsing and projecting generic W3C DID Documents.
Resolution, dereferencing, registration, method execution, PRISM/Cardano
effects, networking, persistence, secret custody, credentials, DIDComm, agent
runtime, React Native/UniFFI, and mdoc are not included.

## Activation and rollback

Implementation begins only after strict research/constraint readiness, a
signed/DCO planning-only commit, and an exact issue #502 preimplementation
receipt. Local completion requires focused Rust/WASM/package/target tests,
document fuzz and differential evidence, factory validation, full flake checks,
distinct exact-diff review, and archived OpenSpec evidence.

Rollback removes the private document provider method, mapping, vectors, and
target assertions as one unreleased unit while preserving the merged DID value
canary and every existing Rust API. No external consumer rollback is claimed.

## Evidence

Readiness is proved by `research-ready`, `constraints-ready`, strict OpenSpec
validation, the issue #502 exact-base preflight receipt, the canonical
change-ledger/mapping/vector records, and the focused/full commands listed in
`research.md`. Target evidence remains local and partial until external
consumer rehearsal, accepted budgets, support, publication, and native-target
decisions are separately recorded.
