# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-09
Evidence head: e00a221826d0d96c22ff2d3cc7199e2a1c281a25
Specification commit: 1833d3e6f6a7850f95fcc2fd62ccd03d11c59159
Implementation commits: cbcba29193b7dc9ec978d96595d1dbcfdf182efe and e00a221826d0d96c22ff2d3cc7199e2a1c281a25
Unresolved blockers: none

## Scope reviewed

The review inspected the complete
`develop@f157820aa312e56aa558361f8332609933c6b8e2...e00a2213` diff,
issue #230, ADR 0100, Android package/specification assets, both generated
builds, the isolated AVD consumer, workflow change and local receipts.

## Findings

1. **Architecture and cohesion — accepted.** Android, Gradle, JNA and UniFFI
   mechanics remain in the existing leaf binding boundary, dedicated shell and
   verification assets. Generic DID, core, crypto and JOSE source is unchanged.
2. **ABI and artifact model — accepted.** The existing ABI version 1 and
   function/type surface are unchanged. The AAR contains generated Kotlin and
   exactly one arm64-v8a SDK shared object; it contains no copied JNA library.
3. **Dependency boundary — accepted.** The consumer declares exact JNA 5.18.1
   `@aar` separately. Both library and consumer locks are tracked and checked,
   and the upstream AAR is independently pinned by SHA-256. Neither Cargo lock
   changes and no Android type crosses a Rust public boundary.
4. **Platform identity and hardening — accepted.** The packaged library is an
   API-21 ELF64 little-endian AArch64 object built by exact NDK 27.0.12077973.
   It has RELRO/NOW, a non-executable stack, only `libdl`/`libc` needs and the
   required contract/checksum/API symbols.
5. **Determinism and hygiene — accepted.** Two independent Rust, generator and
   Gradle trees produce byte-identical libraries, Kotlin and normalized AAR
   trees. Archive metadata and inner-JAR metadata are normalized explicitly;
   unsafe archive paths and absolute checkout paths fail closed. All generated
   output and AVD state remain below ignored `target/`.
6. **Runtime behavior — accepted.** A fresh isolated arm64 API-35 emulator loads
   the local AAR plus explicit JNA and observes version, valid DID/DID URL,
   invalid input, oversized input and redacted errors. No user AVD is mutated.
7. **Security and privacy — accepted.** No authored unsafe Rust or new Rust
   source is introduced. The reachable surface contains public identifiers
   only and adds no key, secret, signing, storage, network, callback, async,
   handle, JNI or Keystore contract.
8. **CI and support accuracy — accepted.** Only the dedicated bindings shell
   gains Android std and curl. The proof is weekly/manual macOS evidence; fast
   Linux PR CI is unchanged. Support remains explicitly not-supported.
9. **Delivery scope — accepted.** No AAR, APK, generated bridge, signature,
   Maven coordinates or release artifact is committed or published. Donor and
   downstream repositories are unchanged.

## Residual limitations

- Only arm64-v8a on one API-35 arm64 emulator is execution evidence.
- Physical devices, other ABIs, runtime matrices, Maven metadata/publication,
  signing, Keystore, app lifecycle and release compatibility remain unproven.
- Hosted Android SDK, system image and emulator capability are observed
  slow-lane inputs rather than supported consumer inputs.

## Review decision

The change is bounded, deterministic, reversible and consistent with the
existing versioned facade and mobile limitations. No unresolved architecture,
security, privacy, dependency, licensing, correctness or delivery finding
remains for hosted review.
