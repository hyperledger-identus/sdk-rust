# Tasks

## 1. Evidence and contract

- [x] 1.1 Pin the method specification, NeoPRISM donor and SDK-TS consumer revisions.
- [x] 1.2 Record donor decomposition, dependency and exact-wire decisions.
- [x] 1.3 Record finite method-profile and donor-deviation decisions.
- [x] 1.4 Pin historical ledger fixtures and file-level provenance.
- [x] 1.5 Finalize ADR and output budget; activate the compatibility ledger record with implementation.
- [ ] 1.6 Record exact issue #521 preimplementation receipt after #524 merges.

## 2. Wire and syntax

- [ ] 2.1 Add crate scaffold, limits and redacted errors.
- [ ] 2.2 Add checked-in private prost messages and regeneration check.
- [ ] 2.3 Add bounded duplicate-aware exact-byte envelope scanner.
- [ ] 2.4 Add canonical/long-form syntax, hashing and bounded base64url.

## 3. Operations and state

- [ ] 3.1 Add validated keys, services, contexts and create operation.
- [ ] 3.2 Add ordered update actions and deactivate operation.
- [ ] 3.3 Add signature verification over exact bytes.
- [ ] 3.4 Add deterministic tombstone state and atomic reducer.

## 4. Projection and integration

- [ ] 4.1 Project active method state into generic DID document values.
- [ ] 4.2 Add local long-form resolver and method registry binding.
- [ ] 4.3 Preserve opaque protocol-version/VDR extension payloads for #522.

## 5. Evidence and delivery

- [ ] 5.1 Port donor/spec/historical vectors with provenance.
- [ ] 5.2 Add negative, property, fuzz and differential state evidence.
- [ ] 5.3 Pass native/WASM/mobile compile, clippy, docs and dependency gates.
- [ ] 5.4 Capture directional parse/verify/project/state benchmarks.
- [ ] 5.5 Complete independent CLI review before push: use Claude first and Codex CLI when Claude is unavailable or quota-limited.
- [ ] 5.6 Open one issue-linked cohesive PR, pass protected CI and publish metrics.
- [ ] 5.7 Archive the completed OpenSpec change and merge to `develop`.

## 6. Stop boundary

- [x] 6.1 Keep Cardano, VDR payload semantics, persistence, HTTP, CLI, node composition and downstream deletion out of this change.
