# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

`nix run .#crypto-candidate` now completes deterministic package assembly,
archive inspection, extraction, consumer-profile builds, and the packaged
crypto test suite. It then generates public API with Rust 1.98.1 and
`cargo-public-api 0.52.0` and rejects the committed baseline.

The exact multiset difference contains no removal. It adds these methods, repeated
only where cargo-public-api renders the same re-exported types:

- `Ed25519PublicKey::has_canonical_encoding`
- `Ed25519PublicKey::is_weak`
- `X25519PublicKey::has_canonical_encoding`
- `X25519PublicKey::is_contributory`

The methods were introduced by already-merged DID-key hardening work
(`9e053008`, `f2c770d7`, and `b76fc224`) and are present at protected source
revision `6ebd4633c987539689ae180fbccbb629572ca45a`. The crypto train is recorded
as `published`, and signed tag `v0.1.0-rc.1` exists, so its baseline must remain
the historical API of that artifact. This change does not alter implementation
or visibility.

## Normative sources

The canonical `release-candidate-trains` specification permits rebaselining
only before first publication. The crypto train index and signed tag prove that
window is closed. The candidate descriptor pins Rust 1.98.1 and
cargo-public-api 0.52.0; the published baseline digest is
`3e2c09eddba2de063436c76c8c4901b4cf5a28e0e70c659c12958a561da98909`.

## Candidate decisions

| Candidate | Evidence | Decision |
| --- | --- | --- |
| Suppress the API check | Hides real candidate drift | `not-adopt` |
| Replace the rc.1 baseline | Falsifies an already-published artifact | `not-adopt` |
| Keep exact equality forever | Rejects compatible additive development | `not-adopt` |
| Preserve origin and require zero removals | Matches a published predecessor contract | `adopt` |

No dependency or external library is involved.

## Compatibility and dependency evidence

The current diff is additive-only and records already-delivered API. There is
no runtime, cryptographic, wire, error, dependency, unsafe-code, MSRV, portable
target, package, baseline, tag, or publication change.

## Security, privacy and maintenance evidence

The report is bounded, deterministic, baseline-hash-bound, and contains public
API text only—never key material or caller data. A multiset comparison prevents
duplicate rendered re-export lines from hiding removals. Initialization against
a published origin fails closed.

## Rejected or deferred candidates

Suppressing the check, changing the published baseline, retaining permanent
exact equality, or widening the change to product code are rejected. No
candidate is deferred.

## Open questions and blockers

No research blocker remains. Implementation must prove the baseline stays
byte-identical, the exact multiset removal set is empty, the current additions
are the known four methods/re-export views, and the complete candidate passes.
Release approval and a fresh protected slow run remain separate.

## Evidence commands

Planning uses `scripts/factory research-ready`, `constraints-ready`, `validate`,
`check`, a signed/DCO planning commit, and issue #587 preflight. Delivery uses
the pinned ordinary candidate path, exact hash/diff review, report and mutation
checks, normal `nix run .#crypto-candidate`, full Nix, independent
Codex or Claude CLI review, protected CI, guarded merge, and metrics.
