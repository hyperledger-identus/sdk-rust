# Tasks

## 1. Evidence and contract

- [x] 1.1 Pin the method specification, NeoPRISM donor and SDK-TS consumer revisions.
- [x] 1.2 Record donor decomposition, dependency and exact-wire decisions.
- [x] 1.3 Record finite method-profile and donor-deviation decisions.
- [x] 1.4 Pin historical ledger fixtures and file-level provenance.
- [x] 1.5 Finalize ADR, output budget and compatibility ledger record.
- [x] 1.6 Record exact issue #521 preimplementation receipt after #524 merges.

## 2. Wire and syntax

- [x] 2.1 Add crate scaffold, limits and redacted errors.
- [x] 2.2 Add pinned protobuf sources, checked-in private prost messages and a source/projection drift check.
- [x] 2.3 Add bounded duplicate-aware exact-byte envelope scanner.
- [x] 2.4 Add canonical/long-form syntax, hashing and bounded base64url.

## 3. Operations and state

- [x] 3.1 Add validated keys, services, contexts and bounded create construction.
- [x] 3.2 Add bounded ordered update/deactivate construction and parsing.
- [x] 3.3 Add signature verification over exact bytes.
- [x] 3.4 Add deterministic tombstone state and atomic reducer.

## 4. Projection and integration

- [x] 4.1 Project active method state into generic DID document values.
- [x] 4.2 Add local long-form resolver and method registry binding.
- [x] 4.3 Preserve opaque protocol-version/VDR extension payloads for #522.

## 5. Evidence and delivery

- [x] 5.1 Port donor/spec/historical vectors with provenance.
- [x] 5.2 Add negative/property state evidence and executable differential wire vectors.
- [x] 5.3 Pass native/WASM/mobile compile, clippy, docs and dependency gates.
- [x] 5.4 Capture directional parse/verify/project/state benchmarks.
- [ ] 5.5 Complete independent CLI review before push: use Claude first and Codex CLI when Claude is unavailable or quota-limited.
- [ ] 5.6 Open one issue-linked cohesive PR, pass protected CI and publish metrics.
- [ ] 5.7 Archive the completed OpenSpec change and merge to `develop`.

## 6. Stop boundary

- [x] 6.1 Keep Cardano, VDR payload semantics, persistence, HTTP, CLI, node composition and downstream deletion out of this change.
