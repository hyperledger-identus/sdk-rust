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
