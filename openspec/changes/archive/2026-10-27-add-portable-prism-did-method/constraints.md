# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/521
Constraint blockers: none

Prerequisite issue #524 merged as PR #527 before the exact preimplementation receipt.

## Existing entries affected

The generic DID, method registry, crypto, input-resource, reusable-module,
error, naming, unsafe-code and target constraints remain in force. Issue #524
changes only the generic lexical envelope; it does not authorize method
semantics or wider JSON/crypto/HTTP limits. ADR 0176 narrowly changes the
dependency-boundary name rule: the exact source-less in-tree
`identus-prism-did` package is allowed, while registry/Git/prefixed PRISM and
all chain, donor and product families remain prohibited.

## Introduced or changed constraints

- Raw `PrismOperation` is at most 96,000 bytes; a signed envelope is at most
  131,072 bytes. Bounds are checked before protobuf allocation.
- A long-form identifier is at most 128,075 bytes. Native document projection
  uses a separate 32 MiB preflight budget derived from actual projected key and
  service counts plus three complete operation budgets.
- Key and service identifiers contain at most 50 ASCII fragment characters.
- Service type wire text contains at most 100 ASCII grammar characters.
- Service endpoint wire text contains at most 300 Unicode scalar values and
  1,200 UTF-8 bytes, with bounded duplicate-name-safe JSON parsing. Typed JSON
  endpoint input is depth/node/collection preflighted iteratively and rejected
  values are destroyed without recursive stack growth.
- A state has at most 50 active keys and 50 active services. At most 4,096
  retired key IDs and 4,096 retired service IDs are retained; retired full
  values are dropped, tombstones do not count against active limits, and
  identifiers cannot be reused.
- At most 127 unique additional absolute context URIs are accepted. The DID
  Core context added during projection produces at most 128 entries.
- Update actions execute in wire order against a clone and commit atomically.
- Exact inbound operation bytes, not protobuf re-encoding, are signed and
  hashed.
- Public errors and `Debug` output do not contain caller wire bytes, DIDs,
  service endpoint values, key material or signatures.

Dependencies and targets additionally require:

- Production dependencies are `identus-core`, `identus-crypto`, `identus-did`,
  `prost`, `base64`, `serde` and `serde_json` only when directly
  required by the implementation.
- `prost` is pinned to 0.14.4, built without consumer-time code generation.
- The crate remains compatible with native, `wasm32-unknown-unknown`, iOS and
  Android library targets and does not require a runtime or operating-system
  service.

## Introduced or changed limitations

- The pinned PRISM draft leaves context count/size and total operation size
  unbounded. SDK-Rust publishes a finite safe profile and does not claim to
  accept every theoretically encodable protobuf.
- URI syntax is validated without rewriting signed text. Scheme-specific
  normalization remains caller/profile responsibility.
- VDR operation tags and VDR keys are recognized at the exact-wire boundary,
  but VDR payload semantics belong to issue #522.
- Local long-form resolution is complete in this slice. Short-form published
  resolution needs ordered history supplied by a chain-neutral port.
- Generic DID-document and resolution-result JSON ingress remains at 256 KiB
  and 512 KiB; the larger native projection budget is not a permissive parser.

## Consumer and product impact

Rust adopters gain a portable PRISM method crate and local long-form
resolution. Existing crates do not change behavior except for the separately
reviewed #524 prerequisite. SDK-TS and NeoPRISM are unchanged; their future
adapters can depend on the new crate without dictating its DTOs or errors.

## Activation and rollback

Activation requires merged #524, exact issue #521 preflight, pinned fixtures,
complete OpenSpec/ADR/ledger evidence, independent CLI review (Claude first,
with Codex CLI as the mandatory availability fallback), all required
local/Nix/target tests and green protected CI. Before publication, rollback is
a focused removal of the new crate and registry entries. After publication,
breaking wire or public-contract changes require a superseding compatibility
record and release decision.

## Evidence

Issue #521 and its maintained evidence comment, parent #493, prerequisite
#524, the three pinned repositories, the disposable exact-wire prototype and
the final checked-in vector/provenance registries are the evidence authorities.
