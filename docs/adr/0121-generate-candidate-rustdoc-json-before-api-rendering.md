# ADR 0121: generate candidate rustdoc JSON before API rendering

- **Status:** Accepted under sponsor direction
- **Date:** 2026-09-15
- **Issue:** [#276](https://github.com/hyperledger-identus/sdk-rust/issues/276)
- **Supersedes:** ADR 0113 decision 7's implicit `cargo-public-api` execution
  path; all other ADR 0113 decisions remain effective
- **Review no later than:** any Rust or candidate release-tool update

## Context

ADR 0113 correctly selects Nix-pinned Rust 1.98.1 and a subprocess-scoped
`RUSTC_BOOTSTRAP=1` for unstable rustdoc JSON inspection, but left JSON
generation implicit inside `cargo-public-api 0.52.0`. Manual slow canary
`34972066673` proved that implementation was not self-contained: the parser
observed stable Cargo, selected literal rustup toolchain `nightly`, and failed
on a clean runner where that undeclared toolchain was absent.

Parser source shows that a supplied `--rustdoc-json` file bypasses its JSON
builder. Version 0.52.0 still probes Cargo/rustup metadata during CLI startup
and may compute an unused nightly value; the local JSON path prevents that
value from executing a compiler.

## Decision

1. Generate all-feature `identus_crypto.json` explicitly with the candidate
   application's Nix-pinned Rust/Cargo 1.98.1 toolchain.
2. Scope `RUSTC_BOOTSTRAP=1` to that single `cargo rustdoc` subprocess and use a
   fixed target directory inside disposable candidate scratch.
3. Require the exact expected JSON regular file before continuing.
4. Give the completed file to locked `cargo-public-api 0.52.0` for rendering;
   it does not own candidate compiler execution or JSON generation.
5. Do not add rustup, a nightly candidate compiler, a floating toolchain
   download or another dependency. ADR 0113's package, publication and release
   controls remain unchanged.

## Consequences

Candidate API evidence is reproducible on clean Nix runners with no installed
rustup nightly. The receipt truthfully remains Rust 1.98.1 evidence, and the
separately pinned sanitizer nightly retains its only accepted role.

Rustdoc JSON and `RUSTC_BOOTSTRAP` remain unstable exact-version-bound tooling
interfaces. This decision does not authorize unstable Rust in SDK source and
does not make candidate preparation a publication or compatibility promise.

## Alternatives rejected

- **Install rustup nightly in CI:** adds mutable state and a network/toolchain
  authority outside Nix.
- **Reuse the sanitizer nightly:** changes candidate compiler evidence and
  broadens an unrelated exception.
- **Spoof Cargo's version:** relies on a parser implementation quirk and makes
  the evidence misleading.
- **Remove public-API evidence:** loses an accepted candidate drift control.

## Verification and rollback

The checker and mutation suite require explicit `cargo rustdoc`, scoped
bootstrap, the exact JSON identity and parser JSON input. A clean exact-head
candidate completed with an empty `RUSTUP_HOME`, Rust/Cargo 1.98.1 and the
unchanged committed API baseline. Full factory and compatible Nix checks pass.

Rollback reverts this ADR and the runner/checker changes, returning the
candidate job to a known-red state without external artifact cleanup.
