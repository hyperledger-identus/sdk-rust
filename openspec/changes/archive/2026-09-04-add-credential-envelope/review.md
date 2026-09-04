# Pre-implementation semantic, API, and security review

- **Date:** 2026-09-05
- **Issue:** #71, child of #6 / `IDR-007` and #20
- **Develop base:** `02011c9ca61dc03502f16b89aca0347d79019ce5`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. Activating the existing `identus-credentials` member is lower coupling than
   adding a speculative `identus-vc-core` package. Because all packages remain
   unpublished, the choice is reversible and does not settle #3.
2. Oxid's closed Midnight format enum cannot be reused. A validated open token
   gives future SD-JWT VC, mdoc, VCDM, Compact, and test adapters an integration
   seam without a core edit.
3. The envelope must preserve opaque bytes rather than parse or verify them.
   Parsed/constructed state is explicitly not verified or trusted state.
4. Separate body, proof, and private-material wrappers improve type safety and
   permit the SDK to apply erasure only to genuinely private material. One
   internal size helper is sufficient; a public generic artifact abstraction
   would weaken semantics.
5. `CredentialPrivateMaterial` needs explicit and drop-time zeroization plus
   manual redacted debug. Payload and detached proof also require redacted
   debug because credential artifacts may contain PII even when not secret key
   material.
6. No serde shape should be invented. Format adapters own wire bytes; a wrapper
   wire model would be premature and add dependencies without enabling the
   first consumer.
7. The error bridge must use static codes/messages and never retain caller
   input. Each local variant can remain specific enough for tests and callers
   without leaking values.
8. Metadata, verification reports, status, disclosure, executable registries,
   storage, and trust policy remain coherent child slices. Omitting them is the
   intended 70–80% delivery boundary, not missing acceptance for #71.
9. Consumer repositories are provenance/conformance inputs only. No downstream
   build, branch switch, formatting, dependency repoint, or deletion is needed.

Verdict: READY to implement after strict structural validation.

# Post-implementation semantic, API, and security review

- **Reviewed implementation:** `78c323053826127acef4aacd2ddadffd67184187`
- **Result:** passed after two local findings were resolved

The exact `develop...78c3230` production diff activates only the existing
credential-semantics crate. Its public dependency cone remains
`identus-core` plus the already governed `zeroize` package. There is no serde,
format codec, chain, network, storage, trust, resolver, product, or consumer
dependency.

The API keeps format dispatch open and resource bounded. Artifact constructors
consume existing vectors without copying; format validation scans once before
its single successful allocation. Envelope construction moves its inputs and
does not parse, normalize, serialize, verify, or clone their bytes. The 1 MiB
payload/proof and 256 KiB private-material caps bound SDK ownership. These are
policy limits, not claims about standards maxima.

Review found and resolved two lifecycle issues before delivery. First, private
material inherited convenient `Clone` and ordinary equality; both were removed
to avoid accidental secret duplication and timing-sensitive comparison.
Second, an invalid transferred private vector would have been released without
SDK erasure; the error path now zeroizes that allocation first. The type still
cannot control caller/compiler copies, allocator state, swap, dumps, hostile
hardware, or persistence, and the contract states those exclusions.

All artifact `Debug` implementations disclose lengths only. Envelope debug
also includes the validated format identifier, which the contract treats as
public dispatch metadata. Construction errors contain no rejected value and
bridge to static `credential.*` codes under capability `credential`.

Focused tests cover exact limits, limit-plus-one rejection, open format
spelling, invalid/control/non-ASCII tokens, explicit erasure, redaction, every
error mapping, a Midnight-shaped opaque fixture, and an unrelated format. The
full Cargo matrices and the pinned Nix flake passed, including MSRV, native,
WASM, Android, iOS, feature, docs, lint, supply-chain, and factory derivations.

Read-only final receipts equal preflight: Oxid
`bfe3b481568dc738f0732c2b27548fab8721fd95` on `integration` with its existing
untracked agent directories; midnight-identity
`427f8571950c42967a18726cbcbefecc19ef8d79` on `develop` with its existing dirty
`third_party/midnight-did`; Lace ID Portal
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` on `main` with its existing
untracked agent/tmp directories. No downstream state changed.

Verdict: READY to synchronize, archive, receipt, and publish as a signed/DCO,
issue-linked pull request to `develop`.
