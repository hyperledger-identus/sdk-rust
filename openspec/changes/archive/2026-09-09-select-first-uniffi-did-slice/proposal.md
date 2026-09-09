## Why

Issue #215 asks the SDK to choose a first foreign-language vertical slice
before production bindings begin under #163. The slice must exercise useful
domain values and stable failures without crossing secret, custody, storage,
network, object-lifetime or asynchronous boundaries prematurely.

The existing bounded `identus-did` DID and DID URL parser is the strongest
candidate: it is deterministic, chain-neutral, already maps failures into
redaction-safe SDK errors, and is compile-checked on the target families that
eventually host Swift, Kotlin and browser consumers.

## What changes

- Pin and evaluate UniFFI 0.32.0, its proc-macro and UDL strategies, and its
  native generator/runtime dependency cones.
- Build an isolated, unpublished research fixture that wraps `identus-did`
  with SDK-owned records and errors; domain crates remain UniFFI-free.
- Generate Swift and Kotlin bindings in library mode and run consumer-shaped
  parse, round-trip, component and redacted-error checks.
- Prove deterministic generation with normalized API snapshots and a drift
  check, without committing generated native sources.
- Decide the native implementation direction in ADR 0097 and report React
  Native and browser React separately with exact compatibility limitations.
- Update the public parity/report records and split accepted implementation
  work into platform-specific issues.

## Capabilities

### New capabilities

- `language-bindings-research`: governs how a production FFI surface is
  selected and proven before the current no-FFI limitation can be changed.

## Non-goals

- No production binding crate, workspace dependency, generated artifact,
  package publication or support claim.
- No seed, mnemonic, private key, signing, storage, resolver networking,
  callback, async cancellation or opaque-object API.
- No React Native, browser WASM or downstream repository implementation.
- No mutation of Apollo, NeoPRISM, Midnight, Oxid, Lace or other consumers.

## Delivery

Issue #215 owns this research under #163 and milestone M2. The research fixture
is deliberately outside the root workspace. The decision requires local
runtime evidence, an exact-diff architecture/security review, factory gates,
hosted Linux `fast`, DCO and signature verification before merge to `develop`.
