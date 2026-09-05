# Review: narrow the DID crypto feature cone

## Pre-implementation review

**Revision reviewed:** `develop@ed6cbed292e27de4096adb2cb81622591ce1c7c5`
plus the active OpenSpec contract.

**Result:** ready to implement; no unresolved blocker.

### Architecture and cohesion

- The source inventory is exhaustive across `crates/did`: production code uses
  `identus-core`, `identus-derive`, `serde` and `serde_json`; tests add only the
  `uriparse` oracle. There is no `identus_crypto` import.
- Removing the edge increases cohesion. DID owns syntax and structural public
  material; algorithm/key validation remains in the operation-owning consumer.
- Replacing the edge with a smaller crypto feature was rejected because even
  the JWK helper API is unused and would preserve false coupling.
- `identus-derive` is genuinely used by newtypes and port declarations and is
  therefore retained.

### API and compatibility

- No Rust item, trait, type, error, serialized representation or behavior
  changes.
- Cargo does not permit a transitive dependency to be imported without a
  direct manifest declaration, so removing this edge cannot remove an
  intentional DID public API.
- A multi-dependency consumer may have accidentally relied on Cargo feature
  unification from DID to enable crypto defaults. That implicit configuration
  is deliberately unsupported: the unpublished SDK requires the consumer that
  uses crypto to request its exact capabilities.
- Adding an empty/default DID feature map would add configuration surface
  without selectable behavior and is rejected.

### Security and correctness

- Existing public-JWK validation still rejects registered private members and
  simultaneous JWK/multibase material. Removing an unused crate cannot bypass
  those local checks.
- The contract explicitly prevents structural DID parsing from being
  represented as curve-point, signature or authorization validation.
- The exact-dependency guard catches silent reintroduction of a permitted but
  unreviewed domain-to-domain edge.

### Performance and delivery

- Baseline `cargo tree -p identus-did --no-default-features --depth 2` shows
  the unused edge activating 14 direct crypto dependencies, including four
  curve packages plus hashing, derivation and COSE support.
- The expected outcome is a smaller compile graph for DID-only consumers. It
  is measurement evidence only; no environment-dependent build-time threshold
  is introduced.
- Existing platform/MSRV gates are sufficient because DID has no feature
  variants. Focused default/no-default Cargo checks and graph receipts cover
  the change without adding duplicate permanent Nix jobs.

### Scope and provenance

- No donor code or fixture is copied and all downstream repositories remain
  read-only.
- Correcting the canonical JOSE dependency text from core/crypto to
  core/crypto/DID records ADR 0037 and the already merged verifier edge; it
  introduces no additional implementation scope.
- The change is reversible, issue-linked and appropriate for one focused PR.

## Post-implementation review

Pending implementation and verification.
