# Tasks

## 1. Contract and preflight

- [x] 1.1 Finalize RFC 9901 provenance, candidate dispositions, crate boundary, staged API, limits, static errors, and rollback.
- [x] 1.2 Add archive intent for the complete modified engine-selection requirements.
- [x] 1.3 Pass research readiness, constraint readiness, strict OpenSpec validation, and exact issue #568 preimplementation receipt.

## 2. Crate foundation

- [x] 2.1 Add the unpublished `identus-sd-jwt` package, workspace dependency, component metadata, module ownership, and architecture guards.
- [x] 2.2 Implement validated `SdJwtLimits`, checked work accounting, bounded JSON traversal/writing, and redacted `SdJwtError` codes/goldens.
- [x] 2.3 Implement claim paths, issuance/selection plans, disclosure values, static receipt types, and redacted `Debug` behavior.

## 3. RFC 9901 mechanics

- [x] 3.1 Implement bounded compact splitting and exact issuer/key-binding JWS retention.
- [x] 3.2 Implement object/array disclosure issuance, salts, digest indexing, and decoys through injected capabilities.
- [x] 3.3 Implement issuer-signature verification and finite disclosure-graph processing with duplicate/collision/ambiguity/unused/reserved-claim rejection.
- [x] 3.4 Implement deterministic disclosure closure and presentation serialization from validated selections.
- [x] 3.5 Implement key-binding JWT construction and holder verification for exact type, `sd_hash`, audience, nonce, and issued-at expectations.

## 4. SDK integration and end-to-end evidence

- [x] 4.1 Add bounded `CredentialEnvelope` conversion without implying verification or trust.
- [x] 4.2 Map generic presentation selected-claim intents to validated SD-JWT claim paths without changing presentation-core DTOs.
- [x] 4.3 Add a deterministic offline public-API E2E covering issue, envelope storage, selection, presentation, key binding, issuer/holder verification, and redacted receipt.

## 5. Conformance and quality evidence

- [x] 5.1 Add provenance-bound applicable RFC 9901 examples and clean-room negative/exact-boundary vectors.
- [x] 5.2 Add round-trip, disclosure-closure, ordering, selection, and staged-state property routes.
- [x] 5.3 Add bounded fuzz targets for compact ingress, disclosures, claim graphs, selection, and key binding.
- [x] 5.4 Add normal, exact-boundary, and adversarial graph/work benchmarks with retained-output evidence.
- [x] 5.5 Add two immutable differential-oracle packets and an RFC-based disagreement ledger.
- [ ] 5.6 Activate `credentials.sd-jwt.rfc9901.v1`, `IDR-020`, and the RFC 9901 quality declaration only after all required routes pass.
- [x] 5.7 Capture native, MSRV, dependency/advisory, WASM, iOS, and Android evidence without converting compile-only results into runtime claims.

## 6. Review and delivery

- [ ] 6.1 Run focused and workspace format, Clippy, tests, docs, factory, Nix, conformance, and exact-head evidence routes.
- [x] 6.2 Run Claude CLI review or immediate Codex CLI fallback; resolve every actionable blocker/high/medium finding and rerun proportionate gates.
- [ ] 6.3 Archive OpenSpec, open the signed/DCO issue-linked PR, monitor protected CI, merge only when green, and publish factory metrics.

Exact-head review resolved one actionable P1 finding: partial array presentations now
reindex their materialized disclosure paths before a presentation can be selected
again. The regression route, focused and workspace tests, Clippy, docs, factory,
portable target builds, benchmark, and a 300-second sanitizer soak pass at
`1e22043f450f31be4a5387ed66444cb7214dbd8e`. The full Nix route remains open:
its single-job retry reached `rust-msrv-release-wasm32` and then failed while
archiving `zerocopy` with `No space left on device`; this is retained as an
environmental failure, not converted into passing evidence.

## 7. Stop boundary

- [x] 7.1 Keep SD-JWT VC, profile identifiers, JWS JSON Serialization, HTTP, DID/trust/replay/storage policy, bindings, publication, certification, downstream mutation, and issue #569 work out.
