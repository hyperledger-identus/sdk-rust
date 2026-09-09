# Exact-diff architecture and security review

Review status: passed
Review date: 2026-09-09
Evidence head: 3baee32de4d36579f88fee0fd9afcf21bb6039c7
Specification commits: 52d219e, 4b77667
Unresolved blockers: none

The distinct final review inspected the complete base-to-head diff, exported
ABI and generated snapshots, authored and dependency-owned unsafe reach,
runtime/tool dependency separation, panic/error redaction, host-language
execution, support/inventory accuracy and downstream isolation.

## Findings and resolutions

- The boundary exports only immutable owned values. It exposes no secret,
  callback, async, handle or mutable shared-state surface, and the public API is
  explicitly versioned as `1`.
- Stable error codes and redacted messages cross the boundary. The wrapper
  catches Rust unwinds and converts them to the constant internal error code;
  the documented limitation that the process panic hook may still run remains
  explicit.
- SDK-authored production and test source contains no `unsafe`. Unsafe code in
  the admitted UniFFI dependency cone remains dependency-owned, pinned and
  governed by the existing dependency policy.
- Runtime dependencies are isolated in `identus-uniffi-did`. Generator-only
  dependencies are isolated in the separately locked `tools/uniffi-bindgen`
  package. UniFFI's MPL-2.0 exceptions are exact-package scoped rather than
  globally allowed.
- Swift and Kotlin API snapshots are deterministic and reviewed. Both host
  programs compile and execute against the release library, proving successful
  values, stable errors, bounds and version negotiation.
- The first implementation accidentally included Gradle cache/build outputs.
  Those files were removed before review, their exact directories were ignored,
  and the final tracked host fixture contains only source, configuration and the
  dependency lock.
- An intermediate revision added Java to the default shell. The final design
  moved all host-binding tools into a dedicated `bindings` shell so the fast
  path and normal contributor environment remain unchanged.
- `crates/did` and the existing placeholder `crates/bindings` are unchanged.
  Support records retain `SDK-LIM-002`; no XCFramework, AAR, device, mobile
  runtime or publication claim is made.

## Conclusion

The host-only foundation is cohesive, reversible and downstream-neutral. The
diff makes no unsupported product claim and has no unresolved architecture or
security finding. Mobile packaging and device execution remain separate future
work under the parent issue.
