# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-11-19
Source retrieval date: 2026-11-19
Research blockers: none

## Problem and existing implementation

Issue #608 and the review of PR #604 identify one source-compatibility defect:
the peer extension was added to the base provider instead of a capability
extension. The existing package already has target-specific loaders, one React
context, retryable initialization, strict declarations, and complete peer
vector evidence. No donor implementation, new dependency, standards
interpretation, cryptographic primitive, wire format, or protocol engine is
required.

## Normative sources

- Issue #608 records the compatibility defect and the requirement to preserve
  existing three-method providers.
- ADR 0163 and the target manifests keep React Native outside the current
  TypeScript binding support boundary.
- The canonical `did.peer.profiles` capability, adapter mappings, vectors, and
  quality declaration remain the semantic authority.
- TypeScript structural typing is the language contract used by the existing
  public compile fixtures. No external implementation is copied.

## Compatibility and dependency evidence

Restoring the original three-method base interface is source-compatible:
peer factories retain the stronger extension type, while callers that only
parse DIDs remain assignable. The change adds no dependency, wire shape,
generated WASM export, Rust API, peer identifier behavior, or target promise.

Browser, Node ESM/CommonJS, and React Web already exist as isolated targets.
Their loaders and lifecycle state are reused. React Native remains explicitly
unsupported, and binding failures remain isolated from Rust core.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Keep required peer methods on `DidProvider` | `not-adopt` | Preserves the source break for core-only providers. | Never while core parsing is an injection seam. |
| Make peer methods optional on `DidProvider` | `not-adopt` | Weakens call-site guarantees. | TypeScript gains a stronger native capability mechanism. |
| Split `PeerDidProvider extends DidProvider` | `adopt` | Preserves assignability and strong peer typing. | The provider model is intentionally versioned incompatibly. |
| Fall back after initialization failure | `not-adopt` | Silently changes a trust boundary. | A reviewed product policy requires explicit failover. |
| Select injected or peer-WASM before initialization | `adopt` | Provides real rollback and avoids artifact entry on injection. | Loaders use another explicit composition boundary. |

## Security, privacy and maintenance evidence

Provider selection is explicit and terminal. Tests observe initializer calls
and stable categories only; no provider object, DID, input, stack, generated
error, or WASM memory is logged. Reusing one hierarchy and React lifecycle
avoids duplicate state machines.

No secret, credential, key, network request, persistence, unsafe Rust, new
package, or third-party type enters this change. Existing limits, redacted
errors, mappings, vectors, and quality declarations remain authoritative.

## Rejected or deferred candidates

A second React context, optional peer members, automatic fallback, eager WASM
initialization, React Native, publication, external SDK-TS adoption, and mdoc
are rejected or deferred.

## Open questions and blockers

None for local implementation. Hosted CI and protected review are integration
gates, not research blockers.

## Evidence commands

Planning evidence uses factory doctor, strict OpenSpec validation, research and
constraint readiness, issue-bound preflight, and canonical ledger checkers.
Implementation evidence will use package/type/runtime tests, React, CommonJS,
declaration snapshots, isolated target receipts, factory check, and
`nix flake check --print-build-logs`.

Implementation tests, Nix validation, local review, hosted CI, and protected
merge evidence are intentionally unrun at planning time.

## Selected approach

Use TypeScript structural subtyping: keep the original three methods on
`DidProvider`, extend it with `PeerDidProvider`, and return the extension from
peer-capable factories. Add a discriminated selection value whose injected
branch returns without invoking the peer-WASM initializer. Keep selection
terminal so initialization or operation rejection never selects another route.

Alternatives rejected: optional peer methods weaken type safety; a second React
context duplicates lifecycle state; automatic fallback changes failure
semantics; retaining required peer methods preserves the source break.

## Provenance

All affected sources are repository-owned Apache-2.0 files at base
`f7dc731108cfb7d160732dea9bed84c9d0b9cff7`. Existing peer profile, mapping,
vector, quality, and target evidence remains authoritative. No external source
or fixture is copied.
