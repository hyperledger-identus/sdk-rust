# Constraints and limitations

Impact class: routine
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/388
Constraint blockers: none

## Existing entries affected

- `SDK-RELEASE-001`: canonical unpublished manifests and publication denial
  remain effective.
- The DID candidate stays limited to `identus-did` and
  `identus-did-resolver-http` at `0.1.0-rc.1`.
- Compiler and target support is package-specific; the HTTP binding does not
  inherit WASM, iOS, or Android support.
- Final production-promotion evidence must bind one exact revision and every
  required slow lane must pass.
- ADR 0191 accepts only RUSTSEC-2026-0374 on one exact build-time path through
  2027-01-31 and may not suppress a vulnerability or unrelated warning.

## Introduced or changed constraints

None. This slice makes existing release gates durable and navigable.

## Introduced or changed limitations

- Candidate evidence is not publication, stable compatibility, certification,
  vulnerability-free status, device/browser runtime support, or production
  readiness.
- NeoPRISM parity is source-pinned downstream evidence and does not imply that
  every SDK consumer or every chain state has been tested.
- QoS measurements are diagnostic and host-dependent; semantic parity is the
  interoperability signal.
- Independent approval cannot be self-issued by the implementing agent.

## Consumer and product impact

Engineers gain one durable readiness packet and reproducible evidence map. No
consumer source, product behavior, package identity, dependency, API, wire
format, target support, or release state changes.

## Activation and rollback

The documentation PR may merge only after local release-shaped checks,
factory/OpenSpec/site gates, and exact-head review pass. Final readiness may be
declared only after the merged exact revision receives a complete green slow
receipt and independent approval. Rollback reverts the additive report and
navigation; no external release state is changed.

## Evidence

Issues #381/#388, closed prerequisite issues #382/#384/#386/#387/#564, the
train descriptor and staged lock, archived verification receipts, the retained
historical failed natural slow run, a future exact-SHA complete run, and immutable
NeoPRISM #324/#325/#327 receipts provide the evidence set.
