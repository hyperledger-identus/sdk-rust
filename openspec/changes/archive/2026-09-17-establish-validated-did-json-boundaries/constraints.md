# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/315
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-003` governs the recursive-input trust boundary and requires exact
  resource and ownership evidence.
- `SDK-LIM-007` remains effective but loses only the native DID rejected-JSON
  recursive-cleanup clause after complete evidence.
- Outer preallocation, caller-budgeted work, and the historical `Multihash`
  retained-compatibility exception remain unchanged.

## Introduced or changed constraints

Every public validated DID domain object SHALL contain recursive JSON only
after the existing DID depth, node, property, collection, name, and string
budgets pass. Every public native DID constructor that takes ownership of raw
recursive JSON and can reject SHALL arm the shared iterative cleanup boundary
before any validation branch. DID-specific collection limits SHALL be enforced
by the validated DID owner rather than generic `OneOrMany<T>` representation.

## Introduced or changed limitations

No runtime limitation is introduced. Caller, transport, generic Serde, FFI,
or JavaScript allocation before SDK entry remains outside the guarantee. The
source migration is pre-release compatibility work, not a published SemVer or
support promise. Iterative cleanup can require worklist memory proportional to
already-owned breadth.

## Consumer and product impact

Direct raw context-object construction becomes a compile-guided migration to a
fallible `ContextObject`. Successful DID document JSON and domain behavior stay
unchanged. No inspected named consumer uses the affected sdk-rust constructor,
and no downstream repository is mutated.

## Activation and rollback

Activation requires hostile-depth and exact-boundary tests, wire round trips,
API closure, workspace/portable-target evidence, fresh security/architecture
review, a signed issue-linked PR, and green exact-head fast CI. Rollback
restores the raw variant and generic policy check together with the former
limitation text.

## Evidence

Issues #315 and #297, ADR 0125, the user-supplied architecture research, the
durable decision comment, this OpenSpec change, the machine input-boundary
inventory, tests, API/SBOM evidence, review, and CI form the required chain.
