# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-10-27
Source retrieval date: 2026-10-27
Research blockers: none

## Problem and existing implementation

The current implementation and consumer evidence are the repository manifests,
lockfiles, crypto vectors and HTTP resolver tests at the pinned revision below.
At SDK revision `67c30d599a538b873646a6e857809f2106391bb1`, `k256 0.13.4`
and `p256 0.13.2` resolve through `elliptic-curve 0.13.8` to
`crypto-bigint 0.5.5`. RUSTSEC-2026-0387 requires `crypto-bigint >=0.7.6`;
that major transition is available through `k256 0.14.0`, `p256 0.14.0` and
`elliptic-curve 0.14.3`. Tokio 1.53.1 is affected by RUSTSEC-2026-0386 and is
patched from 1.53.3; 1.54.1 is the current compatible 1.x release.

The SDK uses the curve crates only inside `identus-crypto`, including scalar
derivation, SEC1 encoding, ECDSA sign/verify and the existing JVM compatibility
paths. Tokio is limited to the HTTP resolver's test/runtime adapter. No
third-party curve or Tokio type is part of a stable SDK facade.

## Normative sources

- RustSec RUSTSEC-2026-0387 and the referenced RustCrypto fix.
- RustSec RUSTSEC-2026-0386 and tokio-rs/tokio#8587.
- Published `k256 0.14.0`, `p256 0.14.0`, `elliptic-curve 0.14.3`,
  `crypto-bigint 0.7.10`, and Tokio 1.54.1 crate metadata and source.
- Existing `crypto`, target, dependency, unsafe-code and audit specifications.

Primary source URLs include
https://rustsec.org/advisories/RUSTSEC-2026-0387.html,
https://rustsec.org/advisories/RUSTSEC-2026-0386.html,
https://crates.io/crates/k256/0.14.0,
https://crates.io/crates/p256/0.14.0, and
https://crates.io/crates/tokio/1.54.1.

License and provenance remain the published crates.io packages and their
RustCrypto/tokio-rs repositories. All selected crates use Apache-2.0 OR MIT or
MIT terms already allowed by repository policy. Published MSRVs are Rust 1.85 for the curve crates and
Tokio and Rust 1.87 for `elliptic-curve`, below SDK-Rust's Rust 1.89 MSRV.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| RustCrypto curve family 0.14 | `adopt` | Smallest maintained coordinated line that removes the vulnerable arithmetic closure. | Vector or target incompatibility cannot be corrected behind the facade. |
| Patch `crypto-bigint` alone | `not-adopt` | Cargo cannot place 0.7 under `elliptic-curve 0.13`; forcing it would fork upstream traits. | Upstream publishes a compatible 0.13 backport. |
| Alternative secp libraries | `not-adopt` | Expands unsafe/native and API migration risk while an upstream-family correction exists. | RustCrypto no longer meets portability or maintenance requirements. |
| Tokio 1.54.1 | `adopt` | Patched, same 1.x facade, MSRV-compatible, no feature expansion. | A newer mandatory fix is published before delivery. |
| RustSec ignore | `not-adopt` | Would hide a timing and memory-safety finding and violate issue #525's stop rule. | Only a separately accepted, time-bounded risk decision may reconsider. |

## Compatibility and dependency evidence

The change is behavior-neutral at the SDK boundary. Exact existing vectors
must prove DER/raw signatures, low-S normalization, public/private encodings,
JWK/COSE conversion, HD derivation and JVM verification fallbacks. Root and
fuzz lockfiles must contain no `crypto-bigint 0.5.5` or Tokio 1.53.1. Native,
WASM, iOS and Android checks remain mandatory because the curve implementation
is portable production code. The direct and resolved dependency cone, exact
versions and enabled features will be captured from both final lockfiles.

## Security, privacy and maintenance evidence

No secret representation or error boundary changes. Public and wire
compatibility are invariants, and the SDK facade remains the only supported
boundary. The selected crates remain
pure Rust in the enabled feature cone and no new unsafe code is introduced by
SDK-Rust. Supply-chain evidence includes dependency diffs, features, licenses,
RustSec results and inverse
trees are retained. Rollback is a PR revert only if the old vulnerable closure
is no longer accepted for delivery; otherwise a forward fix is required.

Protocol or draft currency is not applicable: this is a dependency security
remediation with unchanged cryptographic protocol encodings.

## Rejected or deferred candidates

An advisory ignore, a private fork, old/new family coexistence, and replacement
with a native backend are rejected. Unrelated dependency refreshes are deferred.

## Open questions and blockers

There are no planning blockers. Source compatibility of the 0.14 trait changes
is intentionally resolved during implementation; any public-behavior delta or
vector failure stops the upgrade and reopens candidate research.

## Evidence commands

Exact commands and unrun checks are separated. Planning evidence includes
`cargo info`, current RustSec scans, inverse trees,
manifest/source searches, strict OpenSpec validation and factory readiness.
Implementation must run focused crypto tests and vectors, both lockfile audit
and deny gates, all-feature/no-default workspace tests, clippy, docs, portable
targets, fuzz dependency policy, Nix checks and an exact-head independent CLI
review. Claude is preferred when available; Codex CLI is used immediately if
Claude is unavailable or quota-limited.
