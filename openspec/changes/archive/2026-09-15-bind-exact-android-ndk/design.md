## Context

Canary run `34983050826` at
`6423a9e0d946c1f438f554a3b1fe6222fb28717a` passed revision binding, candidate,
coverage, performance, browser DID, Ubuntu full Nix, macOS complete Clippy/full
Nix, and exact AOSP installation. The final verifier produced a valid arm64
API-21 ELF with NDK-r27 identity but rejected its build number because it was
not 12077973.

The verifier currently declares `ndk_version=27.0.12077973` but computes
`ndk_root=${ANDROID_NDK_ROOT:-"$android_sdk/ndk/$ndk_version"}`. GitHub's
current `macos-latest` runner documentation records `ANDROID_NDK_ROOT` as the
preinstalled default 27.3.13750724. The exact installed package is therefore a
dead input whenever the ambient variable exists.

## Decisions

### Select from the reviewed SDK root

Set `ndk_root="$android_sdk/ndk/$ndk_version"` unconditionally. This derives
both package installation and compiler selection from the same reviewed SDK
root/version pair. Validate that the directory and `source.properties` exist
and that `Pkg.Revision` is exactly 27.0.12077973.

### Bind child processes too

After validation, export `ANDROID_NDK`, `ANDROID_NDK_HOME`, and
`ANDROID_NDK_ROOT` to the exact side-by-side path. Unset
`ANDROID_NDK_LATEST_HOME`. The direct linker already uses this path; binding
the conventional aliases prevents a future native dependency/build script
from silently choosing the runner default.

### Preserve independent binary evidence

Keep the existing ELF note assertions for NDK r27 and build 12077973. Add the
selected `source.properties` SHA-256 to the receipt. Metadata proves selected
tool provenance; the ELF note proves the built artifact used it. Neither alone
is sufficient.

## Risks and mitigations

- `sdkmanager` could install malformed metadata: exact revision parsing and the
  independent ELF note fail closed.
- A child tool could inspect another alias: all standard NDK aliases are bound
  or removed in the verifier process.
- The selected old NDK could become unavailable: the exact workflow install
  fails visibly; an upgrade requires a separate ADR and evidence refresh.
- Runner paths can move: selection is relative to declared Android SDK root,
  not a hard-coded hosted path.

## Alternatives

- Accept runner default 27.3.13750724: rejected because it changes a reviewed
  input and would require an explicit upgrade decision.
- Set workflow-level environment aliases: rejected as broader mutable state;
  the leaf verifier owns the native tool boundary and validates it locally.
- Remove the ELF build assertion: rejected because metadata alone cannot prove
  which linker produced the packaged library.

## Rollback

Reverting ADR 0123 and verifier/policy changes restores the known-red mismatch.
No Rust artifact or consumer migration is involved.
