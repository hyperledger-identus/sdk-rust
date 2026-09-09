## Context

The research fixture proved the generator but deliberately avoided a production
workspace dependency. The foundation now needs a cohesive outer-boundary crate,
a reproducible generator that cannot inflate runtime artifacts accidentally,
and an honest compatibility boundary that stops at host verification.

## Crate and dependency layout

Create `crates/uniffi-did` as package `identus-uniffi-did`, built as `rlib` and
`cdylib`. Its only direct runtime dependencies are `identus-did` and exact
UniFFI 0.32.0 without CLI features. Keep the inherited `identus-bindings`
placeholder unchanged.

Create a nested unpublished workspace under `tools/uniffi-bindgen` with its own
lock and a tiny `uniffi::uniffi_bindgen_main()` binary. It uses the exact same
UniFFI release with the CLI feature. Root Cargo builds therefore do not unify
generator template/parser dependencies into the production runtime graph.

## ABI surface

Export `binding_api_version() -> u32`, `parse_did(String)` and
`parse_did_url(String)`. Successful calls return owned `DidView` and
`DidUrlView` records. Failures use a closed `DidBindingError` with constant
`code` data for invalid DID, invalid DID URL and unexpected internal failure.
No domain or dependency type appears in an exported signature.

The wrapper invokes the existing parsers inside `catch_unwind` and discards any
panic payload. Normal parser errors are mapped from their redacted Identus code;
tests prove that caller text and a canary panic payload are absent from returned
error display/debug/code values. The process-wide panic hook is outside an
individual library's control; #226 therefore promises return-data containment,
not global host log suppression.

## Ownership, memory and threading

All arguments and results are owned strings/records copied by generated UniFFI
scaffolding. The crate exports no borrowed references, raw pointers, objects,
handles, callbacks or futures. Functions are stateless and have no affinity to
a runtime or thread. Allocation and FFI transport remain UniFFI/runtime
responsibilities under the exact dependency pin.

## Generation and host tests

A repository script builds the release `cdylib`, invokes the separate locked
generator twice in compiled-library mode, diffs both complete trees, and compares
normalized Swift/Kotlin API snapshots. On macOS it compiles and executes Swift
and Kotlin/JVM smoke programs against the library. Tool absence is a failure for
the host-language mode, never a silent skip.

Generated implementation sources remain ignored. Reviewed normalized API
snapshots and exact runtime/tool locks are committed. The support policy and
bootstrap inventory classify the component as experimental and retain the
global no-FFI support limitation.

The host script and the nested bindgen lock's deny/audit checks run on the
macOS leg of the existing scheduled/manual slow workflow through a dedicated
Nix `bindings` shell. Gradle and Java therefore do not enter the default shell
or the Linux-only fast pull-request line during the current active-development
phase. Missing Swift, Java or Gradle tooling remains a hard failure in that
slow lane.

## Compatibility and rollback

ABI version `1` covers exported functions, record fields and error cases/codes.
Breaking changes require an ADR, version bump and snapshot/consumer migration.
Before publication the entire component can roll back by removing its crate,
tool, tests and inventory entry; the domain crate and persisted values remain
unchanged.
