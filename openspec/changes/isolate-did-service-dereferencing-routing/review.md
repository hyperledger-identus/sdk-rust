# Review evidence

## Scope reviewed

Reviewed the exact issue #461 diff from protected
`develop@1a3aca558734cd317860862a7a91284cf5991b18` through
`77dcb43ddc1de55868f1181d48f75400c1aee0af`, including planning, receipt,
characterization, production code, synchronization, public API inventories,
code-health output, and all local gate evidence.

## Architecture and cohesion

- `dereference_services` remains the private entry point and delegates to one
  private context; no public or crate-wide abstraction was added.
- `select_services` owns selector expansion, one conjunctive document-order
  scan, cloning, and the existing service collection. `route` owns only
  endpoint forcing and media result dispatch.
- Existing `expand_service_id`, `filtered_document_result`, and
  `endpoint_result` retain complete URI, document, and endpoint invariants.
- The change introduces no helper per condition, callback graph, generic
  filter framework, dependency inversion, or visibility expansion.

## Behavior and security

- Selector expansion still precedes selection; selection still precedes empty
  rejection; empty rejection still precedes media negotiation.
- Service ID and type constraints remain conjunctive, and source ordering and
  clone count are unchanged.
- `relativeRef` and fragments still force endpoint output before absent or DID
  document media handling; explicit URI-list and unsupported media branches
  retain their previous order.
- Endpoint maps, relative references, fragments, metadata, static errors, and
  redaction remain owned by the same existing helpers and tests.
- No caller-controlled value reaches new diagnostics and no network or resolver
  work was added.

## Rust implementation

- The context uses ordinary borrowing and one moved metadata value; no unsafe,
  interior mutability, trait object, synchronization, hidden clone, or lifetime
  widening was added.
- Early returns and moves are explicit. Clippy, rustdoc, native and portable
  compilation, MSRV, and canonical-toolchain tests are green.
- The normalized public inventory and manifests are unchanged.

## Code-health and residual decisions

- The 66 / 8 / 20 signal disappears with no replacement function or module
  signal and without changing thresholds or exclusions.
- `DidDocument::validate` remains the cohesive aggregate exception justified by
  issue #408 research; cache orchestration and parser exceptions keep their
  existing owners.

## Findings

No blocking correctness, security, API, architecture, performance, allocation,
portability, or Rust-quality finding remains. Canonical baseline rebinding and
OpenSpec archive remain intentionally deferred to the protected-head closeout
PR after implementation merge.
