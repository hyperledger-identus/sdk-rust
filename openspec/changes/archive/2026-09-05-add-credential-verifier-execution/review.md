# Pre-implementation architecture, API and security review

- **Date:** 2026-09-05
- **Issue:** #87 under #6 / `IDR-009` and #20
- **Develop base:** `e22fa8eab93026ba763e581873f4e5e233aaf035`
- **Result:** contract is implementable with no unresolved blocker

## Findings

1. The six-stage report is already the accepted validity result; adding a
   second outcome or caller-supplied aggregate would reintroduce contradiction.
2. Oxid and Midnight prove asynchronous execution because issuer/key and status
   evidence can require external resolution. A synchronous port would force
   blocking or a runtime into downstream adapters.
3. The repository already standardizes boxed borrowing futures for object-safe
   async ports. Reusing that convention is smaller than adding an async-trait
   dependency or executor.
4. The port must use the bare noun `CredentialVerifier` and the
   `#[identus::port]` marker. A `*Port` suffix would violate the accepted naming
   contract.
5. Passing `&CredentialEnvelope` would unnecessarily expose private holder
   material. A borrowed request view can enforce least authority at the type
   boundary without copying credential bytes.
6. Malformed structure and invalid proof are evidence outcomes, not
   infrastructure failures. A completed adapter must encode them in report
   stages so callers cannot confuse invalidity with retryable unavailability.
7. Three operational errors are sufficient for the first seam. Dynamic causes,
   retry flags and endpoint details would leak policy or sensitive context.
8. Dispatch must use the declared validated format. Payload-prefix probing in
   a generic registry would enable ambiguity and couple core behavior to the
   first concrete formats.
9. A bounded immutable registry is justified by the parent issue's open-format
   extension requirement and by multiple known independent formats. It avoids
   a central closed enum and mutable runtime registration.
10. Duplicate exact bindings should fail at setup; fallback and priorities are
    deliberately absent. A composition root may register its own multiplexer
    behind one exact format if it has a real need.
11. Sixty-four entries preserve the DID registry's established ceiling, keep
    setup allocation bounded and leave ample room for format/version tokens.
12. Request creation and dispatch can avoid artifact buffer cloning. The boxed
    future allocation is accepted and should be measured, not hidden behind a
    flaky timing assertion.
13. No concrete crypto, DID, status, schema, clock, runtime, transport, storage
    or trust dependency belongs in this slice.
14. Lace supplies protocol-level negative-path evidence but no reusable Rust
    credential verifier contract. NeoPRISM and Apollo provide no code input for
    this seam.

Verdict: READY to implement after ADR 0031 and strict OpenSpec validation pass.

# Post-implementation architecture, API, security and performance review

- **Date:** 2026-09-05
- **Reviewed production head:** `86e8c91f56827b8bbed5b17b3fb899359195f2e8`
- **Exact diff:** `develop@e22fa8e...86e8c91`
- **Result:** no unresolved finding

## Exact-diff findings

1. Production behavior is isolated in one cohesive `verifier` module in the
   existing credential-semantics crate. The only dependency change is the
   repository-standard build-time port marker; no external, runtime, feature,
   chain, protocol, storage or trust dependency was added.
2. `CredentialVerificationRequest` can be constructed only from an accepted
   envelope and contains three borrowed references: format, payload and
   optional detached proof. Private material is structurally absent; request
   creation clones no artifact buffer.
3. Request Debug contains only the format token and artifact lengths. The port
   and all operational error values carry no payload, proof, endpoint, dynamic
   cause or private material.
4. The boxed borrowing-future signature is object-safe and follows the SDK's
   existing DID port convention. Concrete adapters remain free to inject their
   async runtime dependencies without pushing them inward.
5. Successful execution returns only the existing canonical six-stage report.
   Tests prove valid, invalid and indeterminate results; invalid proof remains
   an `Ok(Invalid report)` and cannot be confused with operational failure.
6. Unsupported, unavailable and internal operation errors are fixed, copyable
   and redaction-safe. None maps to `VerificationFailed`; trust remains absent
   from both the request and result.
7. The builder owns each already-validated format directly as a `BTreeMap`
   key, rejects duplicate exact bindings before replacement and rejects entry
   65. It performs no redundant format-string allocation.
8. The built registry is immutable and clone-cheap through `Arc`. It dispatches
   solely by the declared `CredentialFormat`; the production path does not
   inspect or copy payload/proof bytes and has no fallback or retry ambiguity.
9. Format enumeration is lexically deterministic and adapter details are not
   rendered. Empty/unknown registries fail closed without invoking a different
   verifier.
10. The release diagnostic polled a ready adapter through the production
    registry at about 21.2 million dispatches/second on the pinned toolchain.
    The accepted per-call boxed-future allocation is visible in that number and
    no host-dependent threshold was introduced.
11. Focused all-feature/no-default tests, strict Clippy, warning-denied docs,
    workspace variants, factory checks and all 27 compatible local Nix gates
    passed, including Rust 1.85 MSRV and 347 principal tests.
12. Oxid, midnight-identity, Lace ID Portal, NeoPRISM and Apollo postflight
    revisions and pre-existing worktree states exactly match preflight.

Verdict: READY for specification synchronization and pull-request review.
