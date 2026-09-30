# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-30
Source retrieval date: 2026-09-30
Research blockers: none

## Problem and existing implementation

ADR 0170 makes idiomatic Rust DTOs and stable Rust error codes canonical while
allowing outer adapters to preserve legacy language shapes temporarily. The
repository does not yet encode those adapters' field semantics, direction,
loss, version window, migration, observability, rollback, or removal gate.
Without that record, a donor DTO can accidentally leak into Rust or a lossy
conversion can be described as parity.

The current implementation is the `identus-did` contract described below; no
language-adapter registry or consumer adapter is currently implemented here.

Current `identus-did` values preserve the serialized identifier, method,
method-specific ID, and DID URL path/query/fragment—including absent versus
explicitly empty query/fragment. Public failures use `did.invalid_did` and
`did.invalid_did_url` with redacted caller input.

## Normative sources

The first seed uses SDK-TS 8.1.4 at immutable revision
`4bf86ebf69d5e96616a148e4c973f831f95fa38e`:

- `packages/shared/domain/src/models/DID.ts` exposes `uuid`, `schema`,
  `method`, and `methodId`; `uuid` is the serialized DID and `schema` is `did`.
- `packages/shared/domain/src/models/DIDUrl.ts` exposes a DID, path array,
  query-parameter `Map`, and fragment string. This cannot preserve raw query
  ordering/duplicates or absent-versus-empty query/fragment.
- `packages/shared/domain/src/models/errors/Castor.ts` exposes
  `InvalidDIDString` with a human message.
- `packages/lib/sdk/src/castor/parser/DIDUrlParser.ts` also uses
  `InvalidDIDString`, conflating canonical invalid-DID and invalid-DID-URL
  categories.
- `packages/lib/sdk/tests/castor/DIDParser.test.ts` names the positive donor
  selector `should test valid DIDs`; the DID URL parser tests exercise path,
  query-map, and fragment projections. These selectors are evidence locators,
  not Rust API names or normative DTO requirements.

The source is Apache-2.0. Its DTOs and error class are consumer-regression
evidence, not normative or canonical Rust design.

ADR 0170 is the normative architecture decision; the primary source URL for
the pinned consumer is
https://github.com/hyperledger-identus/sdk-ts/tree/4bf86ebf69d5e96616a148e4c973f831f95fa38e.
The exact version/features assessed are SDK-TS 8.1.4 generic DID values, DID URL
values, parsing failures, and their public facade. License and provenance are
Apache-2.0 plus the exact revision and paths above.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Make donor DTOs canonical in Rust | `not-adopt` | Violates ADR 0170 and carries language-specific compromises into the core. | Never without a superseding architecture ADR. |
| Versioned outer-adapter mapping registry | `adopt` | Separates canonical semantics from migration compatibility and supports gradual deprecation. | Two delivery cycles show the contract cannot express real adapters. |
| Claim DID URL conversion is lossless | `not-adopt` | SDK-TS `Map` and strings erase raw query and absence/empty distinctions. | A new SDK-TS DTO preserves canonical information. |
| Preserve one legacy error class while exposing stable code | `adopt` | Supports migration without making a human string or conflated class canonical. | The supported SDK-TS range removes the legacy class. |
| Generate deterministic human documentation | `adopt` | Makes machine decisions reviewable and detects documentation drift. | A repository-native documentation generator replaces it. |
| Implement TypeScript adapter now | `not-adopt` | A1 defines infrastructure; #492 owns the downstream canary. | A1 closes and #492 begins. |

## Compatibility and dependency evidence

The canonical direction is Rust to language facade, with reverse mappings only
where information can be reconstructed or unsupported cases fail explicitly.
The DID value mapping is lossless: `uuid` derives from serialized Rust, schema
is fixed `did`, and method/methodId map directly. The legacy DID URL mapping is
lossy and must declare unsupported raw-query duplicates/order and explicit
empty query/fragment cases instead of silently normalizing them.

Both Rust errors may temporarily map to SDK-TS `InvalidDIDString`, but the
adapter must retain the canonical stable code for observability and future
migration. Human error messages are not compatibility identifiers. Resource
bounds and input redaction remain Rust-owned policy.

The registry and validator add no Cargo dependency, production edge, public
API, unsafe code, or target requirement. Stable #420 vector IDs can be recorded
before the catalog is physically present because combined cross-contract
resolution belongs to parent #504; this validator still checks ID syntax and
nonempty evidence.

The Rust 1.89.0 MSRV and 1.98.1 etalon are unchanged. The direct and resolved
dependency cone remains unchanged; this is repository metadata validated by
the Python standard library. Supply-chain evidence introduces no fetched crate,
native library, or executable donor content. Public and wire compatibility do
not change. The language facade is explicitly outside the canonical Rust core.
Protocol or draft currency is not applicable to generic DID value mapping, but
the DID syntax basis remains the final DID Core 1.0 Recommendation rather than
a draft.

## Security, privacy and maintenance evidence

Mappings name sensitive fields and redaction requirements explicitly. A
language adapter cannot weaken Rust input bounds, expose rejected input, or
invent fallback parsing. Lossy mappings require named unsupported behavior and
an adapter-owned failure path. Async/cancellation ownership is explicit even
when not applicable, preventing a later binding from silently changing it.

Unknown fields, unpinned sources, incomplete field/error coverage, undeclared
loss, missing version/deprecation/removal windows, vague selectors, or absent
rollback/observability fail validation. Deterministic rendering keeps the
review document synchronized with the machine registry.

Validator mutation evidence is isolated from rendering evidence: invalid
registry mutations run through validation with `--render`, so a stale checked-in
Markdown file cannot mask a missing rejection rule. A separate negative test
proves that normal validation rejects rendered-document drift.

Independent discovery review additionally challenged the validator with an
out-of-window language version, a weakened byte ceiling, malformed table
entries, an additive fifth language record, and changes to previously omitted
human-review fields. The resulting contract resolves byte ceilings to public
Rust constants, parses the closed version-window grammar, diagnoses malformed
values without tracebacks, treats the four DID records as required seeds rather
than an exhaustive allowlist, and renders every governed mapping field. The
redundant lossy-error check was removed because the per-error invariant already
requires canonical-code preservation for every valid error record.

Maintenance, release, and security posture are unchanged: no published crate,
release train, unsafe surface, or supported target changes. Rollback removes
the pre-merge files; after stable IDs merge it uses explicit replacement and
deprecation rather than history rewriting.

## Rejected or deferred candidates

SDK-Swift/KMP records, generated bindings, UniFFI/WASM design, actual SDK-TS
code, consumer E2E, and mappings for peer DID, credentials, DIDComm, or the
agent runtime are deferred. The schema remains language-neutral, but the seed
stays narrow enough to review.

## Open questions and blockers

None. The canonical side, pinned consumer, four seed mappings, explicit loss,
validator/renderer behavior, and downstream ownership are decided.

## Evidence commands

Planning commands validate evidence through `scripts/factory research-ready
establish-language-adapter-mappings`, `scripts/factory constraints-ready
establish-language-adapter-mappings`, and `scripts/factory validate
establish-language-adapter-mappings`. Implementation will add validator and
renderer mutation tests before the full factory gate. Those implementation
checks are intentionally unrun at planning time.
