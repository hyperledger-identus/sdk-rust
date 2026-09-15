# Exact-diff architecture and security review

- **Review status:** completed
- **Review date:** 2026-09-15
- **Planning head:** `6cc64e9f30964b66c23db67b2da2b8f2f5bcf9b1`
- **Implementation head:** `7da9d4b0799f105745569c30a217a8f9c61c600d`
- **Unresolved blockers:** none

## Scope reviewed

The review inspected failed exact-head canary run `34983050826`, pinned GitHub
runner inventory revision `f95c0c791f690fa64eaf9788bea06643a4176db5`, ADR
0123, verifier changes, receipt output, structural policy, mutation tests and
the complete diff from `develop@6423a9e0d946c1f438f554a3b1fe6222fb28717a`.

## Findings

1. **Root cause — confirmed.** The workflow installed NDK 27.0.12077973 while
   the verifier preferred ambient `ANDROID_NDK_ROOT`; the current hosted image
   exports 27.3.13750724. The resulting ELF retained r27 identity but failed
   the exact build-number assertion.
2. **Source-of-truth cohesion — accepted.** SDK root plus one version constant
   now determines install location, linker path, metadata, child aliases and
   receipt. No host-specific absolute path is encoded.
3. **Independent provenance — accepted.** Exact `source.properties` verifies
   selected-tool metadata; existing ELF notes independently verify the binary
   producer. A mutable alias cannot satisfy either contract by itself.
4. **Future native dependencies — accepted.** Conventional NDK aliases are
   rebound process-locally to the exact path and the latest alias is removed,
   preventing later child build scripts from choosing a different toolchain.
5. **Architecture and compatibility — accepted.** The change remains in the
   leaf binding verifier and offline governance. Generic crates, Cargo locks,
   public API/ABI/wire behavior, MSRV, target tiers and AAR structure do not
   change.
6. **Security and privacy — accepted.** Ambient supply-chain ambiguity is
   reduced. No production unsafe/native code, secret, PII, credential,
   signing, storage, network or publication authority is added.
7. **Failure behavior — accepted.** Missing exact directory/metadata, revision
   drift, alias drift or ELF mismatch fails closed before emulator execution.
   The checksum receipt records metadata without exposing an absolute path.
8. **Limitations — accurate.** The SDK package identity is exact but the remote
   NDK archive is not vendored/checksum-pinned. Android and FFI remain
   experimental; hosted macOS remains authoritative.

## Resolved hosted-review finding

The first checker revision used substring membership for Android verifier
markers. Codex review correctly identified that a commented copy could preserve
the expected assignment while an earlier executable assignment selected an
ambient NDK. The checker now requires every verifier marker at an active shell
line prefix, and a dedicated mutation proves an inline comment decoy fails.

## Decision

The repair restores truth between installed, selected and evidenced NDK input
without changing the accepted toolchain or product surface. No unresolved
architecture, correctness, security, privacy, compatibility, licensing or
delivery finding remains. Hosted end-to-end execution is still required.
