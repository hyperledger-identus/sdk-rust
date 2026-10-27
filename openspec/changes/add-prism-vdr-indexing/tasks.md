# Tasks

## 1. Evidence and specification

- [x] 1.1 Pin SDK and NeoPRISM revisions, donor paths, history and license.
- [x] 1.2 Complete VDR and indexer reusable-module assessments.
- [x] 1.3 Decide effect isolation, atomic commit, replay and cancellation ownership.
- [x] 1.4 Finalize the concrete public API/limit ADR and provenance digests.
- [x] 1.5 Record exact issue #522 preimplementation receipt.

## 2. VDR capability

- [ ] 2.1 Add `identus-prism-vdr` scaffold, finite limits and redacted errors.
- [ ] 2.2 Add typed create/update/deactivate payload decoding.
- [ ] 2.3 Add bounded batch-local/store root traversal with cycle detection.
- [ ] 2.4 Add donor/spec/negative/property/fuzz evidence.

## 3. Indexer capability

- [ ] 3.1 Add `identus-prism-indexer` scaffold and ordered batch values.
- [ ] 3.2 Add pure full-batch planning and deterministic mutation output.
- [ ] 3.3 Add atomic cursor compare-and-set commit/replay contract.
- [ ] 3.4 Add in-memory and NeoPRISM-shaped conformance adapters.

## 4. Architecture and quality

- [ ] 4.1 Replace the complete-family deny with the exact dependency graph guard.
- [ ] 4.2 Add stable quality declarations and compatibility/change-ledger records.
- [ ] 4.3 Capture dependency cone and native/WASM/iOS/Android evidence.
- [ ] 4.4 Capture directional benchmarks and 300-second Linux fuzz evidence.

## 5. Review and delivery

- [ ] 5.1 Run exact-head independent CLI review: Claude when available, otherwise Codex CLI immediately; record reviewer, command and commit.
- [ ] 5.2 Verify every blocker/high/medium finding locally and rerun proportional gates.
- [ ] 5.3 Archive OpenSpec, open the issue-linked cohesive PR, publish factory metrics and merge only with protected CI green.

## 6. Stop boundary

- [ ] 6.1 Keep Cardano, SQL, HTTP, CLI, runtime, deployment, downstream mutation and crates.io publication out of this change.
