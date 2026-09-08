# DID URL parser dependency decision

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation in `crates/did/src/did.rs` is a 455-line,
dependency-free ASCII parser plus immutable `Did`/`DidUrl` facades. It rejects
above 2,048/4,096 bytes before parsing or allocation, caches borrowed component
offsets, retains owned input allocation, separates bare DID from DID URL and
maps errors into stable redacted SDK codes. Existing consumer evidence covers
W3C, NeoPRISM, Midnight, Lace and Oxid shapes, and the archived fuzz receipt
records one million units per DID target without invariant failure.

## Normative sources

- W3C DID Core 1.0 DID syntax and DID URL grammar:
  https://www.w3.org/TR/did-core/#did-syntax
- RFC 3986 percent encoding and relative resolution:
  https://www.rfc-editor.org/rfc/rfc3986#section-2.1 and
  https://www.rfc-editor.org/rfc/rfc3986#section-5

The W3C grammar requires a final method-specific-id `idchar`; RFC 3986
`pct-encoded` is exactly `%` followed by two HEXDIG characters. SDK length and
allocation rules are explicit product resource constraints, not W3C claims.

## Candidate decisions

`did_url_parser 0.3.0` completed its `spike` with a `retain-local` decision.
Common valid examples match, but grammar, exact serialization, resource,
allocation, mutability and unsafe stop conditions fail. The 30-case curated
corpus has 7 mismatches; 17,284 generated comparisons have 95 mismatches.

## Compatibility and dependency evidence

Candidate version `0.3.0`, defaults disabled, feature `alloc`; upstream release
revision `cdde0daface0e24fef88c53d46f5bf6129780f02`. Its direct normal dependency
is `form_urlencoded 1.2.1`, with resolved normal dependency
`percent-encoding 2.3.1`: three packages including the candidate, all absent
from the current lock. The crate has one mutable DID/DID-URL type, not the SDK's
separate immutable public types. Its private offsets cannot back an Identus
facade without retaining the foreign allocation; `TryFrom<String>` copies.

The candidate's `query_pairs` interprets generic DID query data as HTML form
encoding. That surface and relative joining are not current parser consumers.
Wrapping around known mismatches would retain local validation/copying and add
the dependency cone, so it does not meet the reuse objective.

Public and wire compatibility require unchanged immutable SDK types, exact
strings, component accessors, serde validation and redacted errors; no upstream
type or diagnostic may enter the public facade.

## Security, privacy and maintenance evidence

The repository is unarchived; the latest release/source commit is 2025-01-06.
Version 0.3.0 declares no MSRV. The tag is a lightweight reference to an
SSH-signed commit; signer identity was not locally verifiable. License is MIT
OR Apache-2.0 and packaged source matches the pinned commit byte-for-byte.
This license and provenance evidence does not override semantic or security
stop conditions.

The candidate allocates and preserves input before validating it, has no byte
limits, stores untrimmed input after parsing a trimmed view, exposes unchecked
setters, and includes reachable `unsafe { from_utf8_unchecked(...) }` in its
public join path. It has no native-code dependency. Its own 19 tests pass on
Rust 1.98.1, but only 1,024 positive generated DID cases exist and path/query/
fragment property tests are TODO. WASM, Android and iOS `no_std + alloc` checks
pass. Strict candidate Clippy fails three Rust 1.98 lifetime-syntax warnings.
The minimal normal cone passes current advisory scanning; the published
development lock fails because old `proptest` reaches vulnerable `rand 0.7.3`.
This supply-chain evidence separates the clean consumer graph from upstream's
warning-failing development lock rather than implying a runtime advisory.

## Rejected or deferred candidates

No second parser is evaluated in this bounded issue. Broad SSI frameworks and
IOTA Identity remain conformance oracles under ADR 0069, not production parser
dependencies. `fluent-uri 0.4.1` is already used for generic RFC 3986 `Uri` but
does not implement the DID method-specific grammar. A corrected future
`did_url_parser` release is deferred to the reconsideration trigger.

Protocol or draft currency is stable for this decision: W3C DID Core 1.0 and
RFC 3986 are Recommendations/standards rather than an evolving protocol draft.

## Open questions and blockers

No research blocker remains. Upstream changes could repair individual issues,
but opening or contributing them is outside this issue's authority. Adoption
is stopped independently by multiple current-release incompatibilities.

## Evidence commands

Commands executed include `cargo test --locked`, `cargo check
--locked --no-default-features --features alloc`, target-specific variants for
WASM/Android/iOS, `cargo clippy ... -- -D warnings`, `cargo tree --edges normal`,
source `shasum -a 256`, repository API metadata, the deterministic comparison
corpus and full SDK `nix flake check`. Intentionally unrun checks are candidate
sanitizers, Miri, Windows and a historical MSRV matrix: production adoption
already fails semantic stop conditions, and Rust 1.98.1 is the active etalon.

Reconsideration trigger: a new pinned candidate release must reject surrounding
controls/whitespace, trailing-colon and non-HEXDIG percent inputs; provide
pre-allocation limits or a parser-only borrowed API; preserve owned reuse and
immutable invariants behind the facade; remove or isolate unsafe join code; and
pass the same corpus, strict lint and supported targets.

Rollback is documentation-only: revert the issue-linked PR. No production
dependency, public behavior, wire value, persisted state or consumer changes.
