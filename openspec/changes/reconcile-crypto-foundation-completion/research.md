# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

The current implementation is audited at exact revision
`fb2cc64ebe09dc382de9d903a6136f7aa8132c47`.
IDR-004 requests policy-neutral key, algorithm, JWK, JWS, COSE, CBOR,
hashing, randomness, and secret-provider interfaces with standards vectors,
misuse-resistant review, and fuzzing. The executable Apollo manifest reports
27 capabilities: 14 parity, 6 SDK-exceeds, 7 accepted differences, and zero
gaps. It maps 22 evidence suites, records 86.004646% Rust line coverage against
Apollo's 74.81865284974093% baseline, and carries compile receipts for WASM,
iOS ARM64, and Android ARM64. Issues #95, #98, #99, #100, and #104 are closed.

## Normative sources

Issue #286, canonical IDR-004, Apollo parity epic #9/discussion #178, JOSE epic
#8 and its five children, the machine-readable parity manifest, the support
policy, source-distribution decision, unpublished candidate #266, and the
canonical SSI program specification control this audit. RFC/COSE/CBOR and
algorithm standards remain pinned by their delivered child slices.

Primary source URLs include the pinned
[Apollo tree](https://github.com/hyperledger-identus/apollo/tree/ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c),
[RFC 7515](https://www.rfc-editor.org/rfc/rfc7515.html), and
[RFC 8725](https://www.rfc-editor.org/rfc/rfc8725.html). Protocol/draft
currency remains the final or explicitly pinned version recorded by each
delivered child; this audit does not advance a draft.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Mark IDR-004 functionally delivered | `adopt` | Every requested behavior/evidence class has immutable implementation or accepted-difference proof and the parity manifest has zero gaps. | A mapped capability/vector is invalidated or a required IDR-004 surface is missing. |
| Keep IDR-004 in progress until crates.io publication | `reject` | Publication is separately owned by IDR-011 and protected release authority. | The backlog model merges component delivery and release, which would require a new governance decision. |
| Claim production consumer adoption | `reject` | NeoPRISM PR #324 is green but open; no merged production adoption is proven. | A named downstream merges exact-revision adoption evidence. |
| Close #298/#299 as part of this audit | `reject` | They are bounded input-hardening slices and require their own OpenSpec-first implementation. | Their acceptance evidence merges independently. |
| Deprecate Apollo | `reject` | Compatibility-source lifecycle requires release plus downstream governance. | An immutable released candidate is adopted and the Apollo maintainers approve a lifecycle plan. |

## Compatibility and dependency evidence

This audit changes no Rust source, package manifest, dependency cone, feature,
MSRV, target, public API, wire representation, license, or provenance. The
existing unpublished `0.1.0-rc.1` candidate remains publication-prohibited.
Exact version and feature evidence remains in the locked candidate and Apollo
manifest. The direct and resolved dependency cone is unchanged. The public
facade boundary continues to hide backend crate types and errors; public and
wire compatibility is unchanged.

## Security, privacy and maintenance evidence

No cryptographic code or secret-bearing path changes. The report must preserve
negative evidence: #298 and #299 remain open resource-boundary hardening;
coverage is line-only; portable evidence is compile-only; performance is
SDK-baseline-only; bindings are unsupported; and no production or certification
claim follows. Maintaining one completion report reduces ambiguous epic state.
No unsafe or native code is introduced or newly reachable. Supply-chain
evidence, license/provenance, maintenance, release, and security posture remain
those of the exact locked workspace and candidate receipts. Rollback is a
documentation/backlog revert and creates no data or API migration.

## Rejected or deferred candidates

Publication-gated completion, downstream adoption claims, Apollo deprecation,
and catch-all implementation are rejected above. FFI, registry publication,
runtime mobile/browser proof, #298/#299 hardening, and consumer migration stay
deferred to their existing owners.

## Open questions and blockers

No blocker prevents the functional completion decision. Remote reconciliation
of discussion #178 and issue #8 must wait for the merged exact report so links
cannot point at an unmerged candidate.

## Evidence sources

- Apollo `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` and the executable SDK
  parity manifest/report at `develop@fb2cc64ebe09dc382de9d903a6136f7aa8132c47`.
- Closed issues #9, #95, #98, #99, #100, #104, #211-#214, and #266.
- Open issues #163, #298, #299, program #20, and release row IDR-011.
- NeoPRISM PR #324 at `e8c504c1d2f7ac39c33eb32c50ff6c99204bb91a`:
  open, with all observed checks green on 2026-09-17.

## Evidence commands

Commands run: Apollo manifest validation/rendering, repository searches over
crypto/JOSE/target/candidate evidence, GitHub issue-state inspection, and
NeoPRISM PR #324 status inspection. Unrun before implementation: generated
completion-report validation, canonical backlog tests, full repository health
check, discussion mutation, and issue closure.
