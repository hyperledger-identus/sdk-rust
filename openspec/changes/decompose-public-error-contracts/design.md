## Context

At `develop@707a5a22c3fad18724d5c5cac953e7387f7e49d8`,
`CredentialError` has 44 fieldless variants and 44 public stable code
constants. Its `to_identus_error` and `Display` implementations independently
match all variants, while five domain tests repeat partial expected-code lists.
`CredentialVerificationError` adds three fieldless variants, a `const` bridge,
another display match, and a complete test table.

The public and redaction behavior is valid. The design changes only how the
credentials crate owns those static decisions. Issue #271, ADR 0116, the
canonical error/credential specifications, ADR 0110, and the planning golden
are controlling. Other crates and consumer repositories are not part of this
pilot.

## Goals / Non-Goals

**Goals:**

- make each credentials error projection one auditable compile-time record;
- group records by cohesive credential invariant;
- make an unmapped enum variant a compiler error;
- preserve every current public API and both display surfaces exactly;
- retain redaction and source behavior with independent golden evidence;
- measure the local duplication/complexity result before proposing another
  crate.

**Non-Goals:**

- changing an enum, code, message, kind, capability, source, wire model,
  retryability or structured metadata;
- creating a public/shared error framework, trait, crate, derive or generator;
- adding dependencies, features, unsafe/native code, Serde or FFI;
- advancing OID4VCI issue #7, resource audit issue #168, publication, release,
  or downstream adoption;
- migrating JOSE, presentations, OID4VCI or any other crate in this PR.

## Decisions

### D1. Use one private compile-time record shape

Add a crate-private `ErrorContract` under a private `error_contract` module:

```text
ErrorContract
├── code: ErrorCode
├── kind: ErrorKind
├── capability: CapabilityId
├── local_display: &'static str
└── public_message: &'static str
```

Its `const fn` constructor and accessors perform no allocation. Its conversion
helper calls the existing `IdentusError::public`. It is not re-exported,
serialized, bound, or returned by a public method.

`CredentialError::to_identus_error` remains non-const even though the internal
record can be const. `CredentialVerificationError::to_identus_error` remains
const. Preserving constness in both directions avoids an accidental public API
addition as well as a regression.

**Rejected:** a public trait or shared crate. It changes dependency and release
ownership to solve a private representation problem.

### D2. Keep public declarations explicit and split only private catalogues

Keep `crates/credentials/src/error.rs`, its public enum, documentation, and
`error_code` constants explicit. Keep the operational enum in `verifier.rs`.
Add private catalogue files aligned with current invariants:

```text
src/error_contract.rs
src/error_contract/envelope.rs
src/error_contract/metadata.rs
src/error_contract/status.rs
src/error_contract/verification.rs
src/error_contract/verifier.rs
```

The first four groups cover `CredentialError`; `verification` also owns the two
registry-construction variants because their change axis is verification
composition. `verifier` covers the three operational errors. File names are
private implementation details, but these five ownership groups are required.

Each enum keeps one short exhaustive `match` that routes a variant to a named
domain record. It has no wildcard and no fallible/default branch. `Display`
and `to_identus_error` call that router instead of carrying their own mapping
catalogues.

**Rejected:** generating the public enum/constants with a macro. Keeping those
items explicit minimizes rustdoc, attribute, diagnostic and public-inventory
risk in the first pilot.

### D3. Treat the planning golden as an independent oracle

The planning artifact
`golden/credentials-error-contract-v1.csv` contains 47 rows and has SHA-256
`a96ed52d3aed592a0979e85c480ccde8ff7a4526c32fe8602dc616db674a36f4`.
It was captured before implementation from the exact base.

After durable preflight, copy those exact bytes to a stable test-fixture path
inside `crates/credentials/tests/fixtures/`. Verify the copy hash before using
it. The later archive retains the planning original; the test retains the
stable byte-identical copy. No generator updates either copy.

Add a dedicated integration test with an explicit independent enumeration of
all 47 public variants. The test uses only the standard library to parse the
bounded comma-free static columns, rejects an unexpected schema/row count,
duplicates and unknown enum names, and compares:

- variant and code-constant identity/visibility inventory;
- local `Display`;
- public code, kind, capability and public message;
- complete `IdentusError` display; and
- `std::error::Error::source() == None`.

The existing domain canary tests stay in place. Public constant paths and the
absence of public verifier constants/Serde/bindings are checked by the public
API/source inventory because visibility and negative trait implementation are
not runtime CSV properties.

**Rejected:** regenerating the fixture from refactored code. That would make
the implementation its own oracle.

### D4. Use compiler exhaustiveness and golden comparison as complementary gates

The wildcard-free enum router proves that every variant has some contract.
The independent golden proves that the selected contract is the old one. A
temporary local mutation that removes an arm or introduces an unmapped variant
must fail `cargo check -p identus-credentials`; the mutation is not committed.

This is stronger and simpler than a proc macro for the pilot. It also leaves
ordinary Rust diagnostics at the enum's owning module.

### D5. Preserve the exact public and dependency inventory

Capture the base and head public inventories with the repository-pinned public
API tooling and require an empty semantic diff for `identus-credentials`.
Review source additionally for:

- all 44 public `error::error_code` names and values;
- both method signatures and their current constness;
- enum derives, `#[non_exhaustive]`, variant order and re-exports;
- `Display` and `std::error::Error` implementations;
- no Serde/binding annotations or public catalogue symbols.

Run `cargo metadata --no-deps` and the architecture guards to prove no manifest
or dependency edge changed.

### D6. Measure improvement without a blind file-size threshold

Record before and after production SLOC for the two projection/display areas,
the number of independently maintained mapping decisions, and the largest
credentials error function. Success means both public bridges and displays
consume one record per variant, domain records are discoverable independently,
and no complexity merely moves into generated/build code. The result is a
review prompt, not a repository-wide hard line-count budget.

## Risks / Trade-offs

- **Golden/source duplication is intentional** → Keep the golden immutable and
  test-only; its independence is what detects accidental drift.
- **A CSV parser could become incidental complexity** → Use a fixed header,
  comma-free static fields, the standard library, exact row bounds and clear
  test-only failure diagnostics.
- **Private files can fragment a small crate** → Split only the five existing
  invariant groups; keep the exhaustive routers adjacent to their public enums.
- **Local/public message differences can be accidentally collapsed** → Store
  and compare both columns independently.
- **Constness can drift while runtime tests pass** → Include public API
  comparison and a compile-time const-use assertion for the verifier bridge.
- **A future variant can update production and golden together** → A behavior
  change requires a separate issue/decision; this PR accepts no golden delta
  after the planning commit.
- **The approach may not fit data-bearing/source-bearing errors elsewhere** →
  Scope ADR 0116 to credentials and require separate research for later crates.

## Migration Plan

1. Preserve this planning-only OpenSpec, ADR and golden in a signed+DCO commit;
   write and validate the exact preimplementation receipt.
2. Copy the golden byte-for-byte to the stable credentials test-fixture path
   and verify its SHA-256.
3. Add the private `ErrorContract` and domain catalogue records without
   altering public declarations.
4. Replace only the two bridge/display mapping implementations with exhaustive
   routing through those records.
5. Add the dedicated golden/API/source/redaction/exhaustiveness regressions and
   run focused credentials gates immediately.
6. Record public API, dependency, default/minimal/all-feature, target,
   complexity and full factory/Nix evidence.
7. Complete a distinct architecture/API/security review, sync and archive the
   change, then open one issue-linked PR to `develop`.

Rollback is a focused source revert to the explicit matches and removal of the
private modules/test fixture. No data, wire, release or consumer migration is
required.

## Open Questions

None block implementation. Exact private file placement may change only if all
five ownership groups and the exhaustive routers remain obvious. A need for a
public/shared abstraction, new dependency, behavior change, generated public
item, or any #7/#168 work stops this slice.
