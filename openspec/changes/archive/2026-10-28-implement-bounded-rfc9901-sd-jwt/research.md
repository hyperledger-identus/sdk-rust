# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The current implementation in SDK-Rust owns bounded generic credential/presentation models, compact JWS,
signature suites, SHA-256, entropy/time ports, and redacted error conventions,
but no RFC 9901 disclosure graph or key-binding engine. Issue #489 and ADR 0193
completed the ecosystem qualification and selected a focused local engine.
Issue #568 turns that accepted decision into executable behavior without
reopening candidate selection.

## Normative sources

- Primary source URL: https://www.rfc-editor.org/rfc/rfc9901.txt. RFC 9901,
  November 2025, is the pinned source revision; retrieved text SHA-256
  `fd4cb79184162fa7ef95f4f63abab8ee6d03c9a04c19b8f83758946464b284eb`.
- ADR 0193 and the archived `qualify-sd-jwt-engine` evidence, including exact
  isolated candidate manifests and lockfiles.
- SDK-TS 8.1.4 at
  `4bf86ebf69d5e96616a148e4c973f831f95fa38e` as consumer-regression evidence
  only.

The evolving SD-JWT VC draft-22 source belongs to #569 and is not normative for
this base. RFC 9901 compact serialization is the only serialization activated
here; JWS JSON Serialization is deferred.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Focused `identus-sd-jwt` engine | `retain-local` | Exact integration with existing bounded JOSE/crypto/credential/presentation seams and no candidate clears all SDK gates. | An immutable narrow engine proves exact RFC/rejection/resource/target parity and lower total maintenance risk. |
| Affinidi `affinidi-sd-jwt` 0.1.8 / `95fde42bd338` | `oracle` | Closest complete base, but Rust 1.95 and incomplete bounds/redaction violate the 0.1.x contract. | A Rust-1.89 release with complete finite-work and redaction evidence. |
| IOTA `sd-jwt-payload` 0.5.1 / `c47e52c31545` | `oracle` | Useful independent payload mechanics but not an envelope, JOSE, key-binding, or policy engine. | A narrow complete engine deletes more maintenance than its dependency/adaptation cost. |
| ReallyMe `reallyme-sd-jwt` 0.4.1 / `60f42c536d6e` | `oracle` | Strong bounded/staged reference, but Rust 1.99 and its own JOSE/crypto/proto stack. | A primary/MSRV-compatible release with an SDK-compatible cone. |
| Spruce `ssi-sd-jwt` 0.6.0 / `16cd58715aa2` | `not-adopt` | Broad 218-package SSI graph, authored unsafe, and exact-lock RustSec failures. | A narrow safe advisory-clean release with explicit RFC 9901 behavior. |
| SDK-TS donor behavior | `conditional-adopt` | Useful migration fixtures only where it agrees with RFC 9901. | Any conflict is resolved in favor of RFC 9901, not TypeScript behavior. |

No candidate becomes a production dependency or public type. Exact candidate
integrity, dependency, advisory, MSRV, source-scan, and upstream-test evidence
remains in the archived parent change and is not duplicated here.

Exact version and features remain pinned by the parent probe manifests and
locks. License and provenance are repeated in `provenance.md` so adapted
fixtures cannot lose attribution.

## Compatibility and dependency evidence

The direct and resolved dependency cone is limited to current workspace crates and existing
workspace versions of `serde`, `serde_json`, and `base64`; it introduces no new
registry dependency or cryptographic primitive. Public behavior is additive
and unreleased. The crate uses Rust-owned staged values and static errors, with
MSRV 1.89.0 and primary Rust 1.98.1 inherited from the workspace.

Public and wire compatibility is additive and unreleased. The facade boundary
contains only Identus-owned staged values and static errors. The supported
first slice is compact RFC 9901 with SHA-256. Signature
algorithms and keys are selected by the existing `identus-jose` allowlist.
Credential/profile formats, trust, DID resolution, storage, HTTP, bindings,
and downstream SDK behavior remain unchanged. Exact target claims are limited
to evidence that actually runs.

## Security, privacy and maintenance evidence

Reachable unsafe and native-code evidence must remain within the already
governed dependency graph; this crate authors no unsafe or native code.
Supply-chain evidence reuses the exact parent locks, checksums, RustSec scan,
and future exact workspace lock/advisory receipt. Untrusted compact, base64url, JSON, disclosure, graph, selection, and
key-binding paths receive explicit byte/count/depth/work/output limits before
proportional work. Staged values prevent structural parsing from masquerading
as cryptographic verification. Diagnostics never include compact input,
claims, disclosure material, salts, digests, keys, signatures, audience, nonce,
or identifiers. The crate owns no private key, network, storage, executor,
global state, trust, replay, or authorization policy.

Maintenance belongs to the SDK-Rust RFC 9901 capability owner. The base/profile
split prevents a changing draft from silently modifying stable behavior.
Rollback is additive and removes an unpublished crate; vectors and decision
history remain.

Protocol or draft currency is explicit: RFC 9901 is stable, while the SD-JWT
VC draft remains a separately pinned capability owned by #569.

## Vector, quality, and end-to-end plan

- Bind applicable RFC examples to exact source section, text hash, retrieval
  date, license notice, transformation, and expected result.
- Add clean-room malformed, duplicate, collision, ambiguity, unused,
  reserved-claim, key-binding, and every exact-boundary negative.
- Prove issuance/parse, disclosure closure, order invariants, selection, and
  staged-state properties.
- Fuzz compact ingress, disclosures, graphs, selection, and key binding under
  bounded sanitizer runs.
- Benchmark normal, exact-boundary, and adversarial graphs with work/retained-
  output evidence.
- Compare frozen outputs from at least two exact independent engines and
  resolve disagreements against RFC 9901.
- Execute an offline issue, envelope-store, presentation-select, key-bind,
  issuer/holder-verify, redacted-receipt journey through public APIs.

## Rejected or deferred candidates

Direct oracle dependencies, donor DTOs/errors, a second JOSE stack, draft-era
base behavior, SD-JWT VC, `vc+sd-jwt`, JWS JSON Serialization, HTTP, DID/trust
policy, persistence, bindings, publication, certification, and downstream
mutation are rejected or deferred for this slice.

## Open questions and blockers

No planning blocker remains. Implementation must stop if RFC examples cannot
pass without weakening the pinned standard, staged evidence, redaction, or
finite-work contract. Default resource limits may be reduced before the
planning commit only when local boundary evidence shows the planned values are
unsafe; no unsupported target claim may be substituted.

## Evidence commands

Planning evidence uses `scripts/factory research-ready
implement-bounded-rfc9901-sd-jwt`, `constraints-ready`, `validate`, and exact
issue #568 `preflight --write`. Implementation evidence will add focused and
workspace tests, Clippy, docs, factory/Nix, MSRV/target/dependency/advisory,
property/fuzz/benchmark/differential/E2E routes, and independent Claude or
Codex CLI review. Unrun evidence is reported exactly rather than inferred.

Claude is preferred when available. If it is unavailable or limited, Codex CLI
runs immediately against the same exact commit.
