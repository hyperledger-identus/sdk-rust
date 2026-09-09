# IOTA Identity DID syntax oracle research

Research class: routine
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

`identus-did` owns a self-contained, bounded DID and DID URL parser with W3C
DID Core 1.0/RFC 3986 tests and a prior narrow `did_url_parser` comparison.
Issue #164 requires independent framework oracles without shipping framework
dependencies. ADR 0069 selected IOTA Identity for that role; this change limits
the first executable seam to DID/DID URL syntax.

## Normative sources

- [W3C DID Core 1.0](https://www.w3.org/TR/did-core/) identifier grammar and
  DID URL composition.
- [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986) URI characters,
  components and percent encoding.
- Exact candidate [`identity_did 1.5.1`](https://crates.io/crates/identity_did/1.5.1),
  released from signed annotated tag `v1.5.1` object
  `841d457fcc9354861df616263e075f2ce751a230`, commit
  `917b3815d35e139c66e05e94d50f95b03536c998`.
- Crate archive SHA-256
  `63a3f3d72a407f1b1a93835a0cabdf1acd9c5b11fa4c4eee41d666eadfab0f35`;
  license Apache-2.0; no declared MSRV.
- Upstream main was independently assessed at
  `7dd527087b96bf26ea97d486225eff8278c9a89c`; it is maintenance context,
  not the immutable executable source revision.

## Compatibility and dependency evidence

The consumer is the existing Identus DID syntax facade. The fixture will call
only borrowed string parsing and exact string access on both implementations.
No candidate type, error, normalization, method policy or serialization enters
the SDK API.

`identity_did 1.5.1` directly depends on `did_url_parser`, `form_urlencoded`,
`identity_core`, `identity_jose`, Serde, Strum and thiserror. The exact fixture
resolves 188 packages across all targets (184 registry packages plus the
fixture and three Identus path packages); the host normal/build tree contains
159 distinct package/version pairs. This is disproportionate for a syntax
oracle and materially larger than the existing direct `did_url_parser` oracle.

Public and wire compatibility are unchanged because the oracle is an isolated
test executable. Root support remains Rust 1.98.1. Host, WASM, iOS and Android
compile observations will be recorded separately and will not activate target
support. Rollback removes the fixture and report only.

The bounded corpus covers plain DIDs, method-specific colons, parameters,
path/query/fragment, valid percent encoding, malformed delimiters/escapes,
Unicode/control input and Identus's 2,048-byte DID / 4,096-byte DID URL ceilings.
Thirty attributed cases pass. Twenty-three have identical acceptance and
representation outcomes. Seven classified divergences remain: IOTA accepts two
non-conforming grammar cases and two inputs beyond Identus resource ceilings,
and normalizes away empty query and/or fragment delimiters in three accepted
DID URLs. IOTA rejects the three surrounding-whitespace cases that the lower
level `did_url_parser` comparison accepted, demonstrating wrapper policy but
not an independent parser implementation.

## Security, privacy and maintenance evidence

The candidate parses public identifiers; there is no secret or PII custody.
Caller input and candidate diagnostic strings must not enter fixture output.
First-party fixture code forbids unsafe. Candidate-owned source contains unsafe
in `identity_core 1.5.1` and `did_url_parser 0.3.0`. The all-target lock has 27
packages with custom-build targets. `wasm-bindgen-shared` is the only package
declaring a native `links` value; no C/C++ native library is linked by the host
fixture.

The supply-chain evidence pins registry checksum, signed release tag, license,
exact lock and advisory policy. Version 1.5.1 was released in April 2025;
upstream main remained active at the retrieval revision. Maintenance is credible,
but current beta development and broad model coupling reinforce oracle-only use.

The published crate declares no MSRV. Rust 1.98.1 host tests and strict Clippy
pass. Compile-only checks pass for `aarch64-apple-ios` and
`aarch64-linux-android`. `wasm32-unknown-unknown` fails because transitive
`getrandom 0.2.17` lacks its required `js` feature. These observations do not
establish supported targets.

`cargo deny` and `cargo audit --deny warnings` both reject the exact fixture
lock. The graph contains unmaintained `ansi_term 0.12.1`, `atty 0.2.14`,
`dotenv 0.15.0` and `proc-macro-error 1.0.4`; RustSec also reports the
`atty 0.2.14` unaligned-read unsoundness advisory. No safe upgrade is available
inside the pinned candidate graph.

## Candidate decisions

| Candidate | Initial disposition | Evidence needed |
| --- | --- | --- |
| `identity_did 1.5.1` production dependency | `not-adopt` | Already prohibited by ADR 0069's model/cone boundary. |
| Isolated DID syntax executable oracle | `reference-only` | Frozen manual evidence retained; not admitted to CI because of cone, advisories, unsafe reach and WASM failure. |
| DID document/VC/IOTA method behavior | `defer` | Separate capability issue, models and normative corpus. |
| Upstream main or beta source | `reference` | Never an executable dependency for this bounded change. |

## Additional policy sources

ADR 0008 governs the Identus parser and ADR 0069 governs IOTA Identity's oracle
role. The repository's dependency-research-readiness, SSI repository-disposition
and root-boundary specifications apply. No oracle behavior is normative.

## Rejected or deferred candidates

Production adoption, DID documents, method/network types, credentials, JOSE,
resolution, signing, normalization, fuzzing an unbounded random corpus,
downstream integration, publication and release are excluded. A later oracle
must receive a separate issue and capability-specific provenance.

## Open questions and blockers

There is no unresolved blocker. The executable fixture is retained as frozen,
manual reference evidence, but it is not a CI oracle. Refresh or removal
requires a new issue when IOTA publishes a materially narrower syntax package,
eliminates the denied advisories and compiles for the SDK WASM target. Any
unresolved normative ambiguity must remain a named case rather than be silently
classified from majority agreement.

## Evidence commands

```text
cargo +1.98.1 generate-lockfile --manifest-path <fixture>/Cargo.toml
cargo +1.98.1 test --manifest-path <fixture>/Cargo.toml --locked
cargo +1.98.1 clippy --manifest-path <fixture>/Cargo.toml --locked --all-targets -- -D warnings
cargo +1.98.1 tree --manifest-path <fixture>/Cargo.toml --locked --edges normal,build
cargo deny --manifest-path <fixture>/Cargo.toml --config deny.toml check
cargo audit --file <fixture>/Cargo.lock --deny warnings
cargo check --manifest-path <fixture>/Cargo.toml --locked --target <target>
scripts/factory research-ready add-iota-did-syntax-oracle
scripts/factory constraints-ready add-iota-did-syntax-oracle
```

The first seven commands were executed with the outcomes recorded above. Full
Nix and hosted CI remain delivery gates; target-specific commands are compile
observations only.
