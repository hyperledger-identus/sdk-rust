## Context

The first binding slice needs enough shape to reveal generator ergonomics but
must not begin with the hardest FFI risks. DID and DID URL parsing exercises
owned strings, records, enums, fallible calls and resource limits without
secrets, state, threads or asynchronous work.

## Interface decision

Use UniFFI proc macros in an isolated wrapper crate and generate Swift/Kotlin
from compiled-library metadata. Do not annotate or derive UniFFI traits in
`identus-did`. Wrapper functions accept strings, invoke SDK parsers, and lower
success into `DidView` / `DidUrlView` records. They lower failures into a
small SDK-owned `DidBindingError` enum containing stable cases only.

The research wrapper is a `cdylib` plus `rlib` in an independent nested Cargo
workspace. Its exact UniFFI dependency is local to the fixture. Generated
language files go to ignored build directories and are normalized into compact
committed API snapshots; a repeat-generation script fails on drift.

## Error and safety boundary

Invalid caller text is never returned in records, exceptions or diagnostics.
The wrapper maps `Error::to_identus_error().code()` to a closed enum and drops
local structured detail. `catch_unwind` is not added to this deterministic
fixture because UniFFI's generated call scaffolding owns its FFI call status;
production #163 must separately establish the panic profile for every exported
operation and compile mode.

All values are copied across the research ABI. No borrowed pointer, object
handle, callback, future, secret or thread-affine resource exists.

## Platform disposition

- Swift and Kotlin/JVM: prove library-mode generation and host runtime calls;
  accept for a production-native implementation issue if both pass.
- React Native: do not mix `uniffi-bindgen-react-native` 0.31 with UniFFI 0.32;
  research again after an exact compatible release with iOS and Android
  TurboModule runtime evidence.
- Browser React: define a separate `wasm-bindgen` adapter and package/runtime
  proof. Native FFI metadata is not a browser compatibility contract.

## Validation

Run isolated Rust unit tests; generate twice; compare normalized snapshots;
compile and execute Swift; compile and execute Kotlin through Gradle/JNA; audit
Cargo metadata/tree/licenses/unsafe/native reach; prove the root workspace
manifest and lock do not change; then run factory and repository gates.

## Rollback

Delete the isolated research fixture, snapshots, ADR and report. No production
manifest, supported API or downstream artifact changes.
