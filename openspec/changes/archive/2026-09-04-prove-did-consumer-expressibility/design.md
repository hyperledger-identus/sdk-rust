## Context

The accepted DID Core contract already owns bounded syntax, documents,
resolution/dereferencing results, object-safe query/registration ports, an
immutable method registry, cache seams and generic dereferencing. Issue #5
keeps one acceptance gap open: executable evidence that the current model is a
strict enough generic boundary for the four initial Rust consumers.

Immutable read-only evidence:

- NeoPRISM `8becb225132efb1d9302b2c5f6ed4d87b84e8685`, Apache-2.0,
  `lib/did-core`.
- midnight-identity `427f8571950c42967a18726cbcbefecc19ef8d79`,
  Apache-2.0, `crates/midnight-did-domain`.
- Lace ID Portal `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`,
  evidence-only because repository license evidence remains unresolved,
  `crates/core/src/did.rs`.
- Oxid `685f9670af4846d52697a4cfeb94779758ae1075`, Apache-2.0,
  `crates/identity/domain`.

The tests are independently authored from public W3C shapes and API
observations. No donor source, test, fixture or production identifier is copied.

## Goals / Non-Goals

**Goals:**

- Name one compatibility case for each initial Rust consumer family.
- Exercise different useful intersections instead of four duplicate smoke
  tests: open document shapes, strict method-specific projections, legacy
  error migration and object-safe multi-method composition.
- Preserve every existing input bound and redaction rule.
- Produce a durable compatibility matrix that future SDK changes must pass.

**Non-goals:**

- Byte-for-byte compatibility with downstream private structs or error types.
- DID method syntax/semantics, VDR, chain, HTTP, storage, cache coordination,
  trust, cryptosuite or wallet policy.
- Copying Lace material or treating its unresolved license as permission.
- Publishing a crate, mutating a consumer, or activating issue #50.

## Decisions

### Decision 1: keep the suite in the public crate's integration tests

The suite lives under `crates/did/tests` and uses only public `identus-did`
interfaces. It therefore detects accidental public-boundary regressions without
creating a new conformance crate API or exposing test-only adapters.

### Decision 2: independently author projections, not donor fixtures

Each case cites an immutable source revision but builds a minimal synthetic
shape from standards-defined fields. This proves representability while
avoiding source-copy ambiguity, production data and the unresolved Lace
license. Semantic JSON equality is required where wire shape is relevant;
method-specific validation remains an explicit adapter step.

### Decision 3: cover four different boundary questions

- NeoPRISM proves embedded/reference relationships and current resolution
  result handling while its historical keyword maps only through the explicit
  migration helper.
- Midnight proves a rich `did:midnight` document with public JWKs, services and
  extension preservation without moving network semantics upstream.
- Lace proves a small service-facing DTO can use the object-safe resolver seam
  and preserve open document fields without importing HTTP status policy.
- Oxid proves stricter wallet-shaped method validation can project into and out
  of the generic document while custody and trust remain outside.

### Decision 4: no production change unless the tests expose a real gap

The expected implementation is test and documentation evidence only. Any
public API or wire change discovered as necessary would require the contract to
be revised and reviewed before implementation.

## Threat Contract

**Assets:** chain-neutral ownership, public-only key material, bounded untrusted
JSON, exact consumer attribution, and honest compatibility claims.

**Threats addressed:** silently dropping extension members, accepting private
JWK material, confusing generic syntax with method validity, lossy relationship
representation, accidental downstream dependency introduction, and claiming
compatibility from copied or unlicensed fixtures.

**Residual boundaries:** the suite does not prove downstream compilation,
runtime interoperation, ledger correctness, cryptographic verification,
transport behavior, storage security, cancellation safety or product policy.

## Test and Verification Strategy

- Run the named compatibility test under all features and no default features.
- Re-run the complete `identus-did` and workspace suites.
- Run formatting, strict Clippy, rustdoc, dependency-boundary/factory checks and
  the full Nix flake matrix.
- Review the exact diff separately for provenance, secret handling, dependency
  drift, overclaiming and consumer-boundary leakage.

## Migration Plan

1. Land the issue-linked specification commit before tests.
2. Add the four public-API compatibility cases and a concise architecture
   matrix without changing downstream repositories.
3. Record exact verification and distinct local review evidence.
4. Sync the canonical capability, archive the change and open a ready PR.
5. Keep consumer adoption/publication as separate authorized work.
