## 1. Research and specification

- [x] 1.1 Inventory every current multibase consumer, validator, fixture and planned method seam.
- [x] 1.2 Reassess multibase 0.9.3 under Rust 1.98.1 and compare exact features, incremental cone, licenses, maintenance, unsafe/native reach and targets with bs58 plus existing base64.
- [x] 1.3 Refresh issue #156 and specify prefixes, canonicality, bounds, compatibility, non-scope, rollback and stop conditions before implementation.
- [x] 1.4 Add ADR 0085 and update the living dependency portfolio/negative ledger.

## 2. Dependency and implementation

- [ ] 2.1 Add exact bs58 0.5.1 once at workspace level with defaults disabled and alloc only; reuse the existing base64 workspace dependency.
- [ ] 2.2 Add an Identus-private z/u decoder/encoder that rejects empty, unsupported, malformed and non-canonical values without exposing upstream types or diagnostics.
- [ ] 2.3 Route VerificationMethod publicKeyMultibase validation through the private engine after a 4 KiB precheck justified by the recorded resource probe.
- [ ] 2.4 Replace invalid placeholder fixtures with attributable canonical bytes while preserving valid JSON and public accessor behavior.

## 3. Verification and delivery

- [ ] 3.1 Add official and negative tests for z/u vectors, alphabets, padding, unknown prefixes, empty payloads, canonicality, encoded bounds, serde and native parity.
- [ ] 3.2 Pass format, factory, Rust 1.98, minimal/default, WASM, Android, iOS, dependency/security and complete compatible Nix gates; record exact counts and intentionally unrun checks.
- [ ] 3.3 Perform a distinct exact-diff architecture/security review with no unresolved blocker and produce the immutable receipt.
- [ ] 3.4 Synchronize the canonical spec, archive safely and prepare the signed/DCO issue-linked PR; hosted CI, merge and issue/program receipts remain GitHub evidence.
