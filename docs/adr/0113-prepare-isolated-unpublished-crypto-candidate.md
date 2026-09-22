# ADR 0113: prepare an isolated unpublished crypto candidate

- **Status:** Accepted under sponsor direction
- **Date:** 2026-09-14
- **Issue:** [#266](https://github.com/hyperledger-identus/sdk-rust/issues/266)
- **Discussion:** [#252](https://github.com/hyperledger-identus/sdk-rust/discussions/252)
- **Depends on:** ADR 0081 and ADR 0112
- **Review no later than:** before any registry upload or release tag

> **Superseded operational status (2026-09-15):** ADR 0120 activates native
> weekly/manual execution from protected default `develop`; reserved `main`
> remains explicit, minimal and protected.

> **Release activation (2026-09-22):** ADR 0134 supersedes the temporary
> canonical publication denial for exactly the reviewed three-crate train.
> Candidate preparation remains non-publishing; only the separately protected
> workflow may upload the signed, approved artifacts.

## Context

`identus-crypto` has Apollo behavioral parity but is not a reviewable package
artifact. Versioning the whole monorepo or enabling canonical publication would
prematurely release placeholders and unrelated components. Conversely, a
hand-built tarball would not exercise Cargo's normalized package contract.

## Decision

1. The durable Rust brand is `identus-crypto`. `identus-apollo` remains a
   downstream compatibility facade.
2. Prepare `identus-derive`, `identus-core`, and `identus-crypto` together at
   `0.1.0-rc.1`; internal registry requirements are exact
   `=0.1.0-rc.1`.
3. Generate a temporary three-member workspace from a strict descriptor.
   Canonical manifests stay at `0.0.0` with `publish = false`.
4. Assemble every Cargo archive twice and require byte-identical SHA-256
   digests, bounded/allow-listed contents, complete metadata, README, license,
   and normalized manifests without Git or path-only dependencies.
5. Because the internal candidates are not on crates.io, use Cargo
   `--no-verify` only for assembly. Extract the exact archives and verify them
   together through local `[patch.crates-io]` entries. Call this
   archive-closure verification, not registry publication verification.
6. Use locked-nixpkgs `cargo-public-api 0.52.0`, `cargo-semver-checks 0.50.0`,
   and `cargo-cyclonedx 0.5.9` in the explicit candidate path. They do not enter
   the required fast lane.
7. Commit the first all-feature public-API rendering and compare the candidate
   with protected base `8110277c24714206436ae4a3fe678bdc58a84736`.
   `cargo-public-api` still requires rustdoc JSON, so scope
   `RUSTC_BOOTSTRAP=1` to that inspection subprocess; package compilation and
   every other check remain on unmodified Rust 1.98.1.
8. Treat Rust 1.98.1 as preparation evidence only. Publication still requires
   the consumer-driven compiler matrix and the release authority in
   `RELEASING.md`.
9. Emit deterministic checksums, one SBOM per candidate package, API, tool
   versions, feature profiles, limitations, and source identity. Do not add
   publish, tag, upload, release, credential, or `main` mutation commands.

## Consequences

Maintainers can review real Cargo-normalized artifacts and measure package
readiness without creating an immutable public release. The isolated closure
makes its three-package coupling visible and prevents unrelated crates from
silently joining the release.

Candidate preparation is slower than normal development because API and SBOM
tools inspect the full feature graph. That cost belongs to the explicit
release slice and weekly/manual validation, not every feature PR.

## Alternatives rejected

- **Version/publish the entire workspace:** releases unrelated placeholders.
- **Enable three canonical manifests:** weakens the effective publication
  denial before human release activation.
- **Hand-written package/API/SBOM formats:** duplicates maintained Cargo tools.
- **Call archive patches registry proof:** would overstate what was tested.
- **Put candidate tooling in `fast`:** makes ordinary development pay a
  release-only cost.

## Verification and rollback

The descriptor checker and mutation tests enforce scope, versions, metadata,
tool pins, source baseline, canonical publication denial, and absence of remote
mutation commands. The candidate runner performs double assembly, normalized
archive inspection, extracted-closure feature checks, API comparison, SBOM,
and receipt generation.

Rollback reverts this ADR, descriptor, docs, scripts, and Nix application. No
external artifact needs yanking because this decision cannot publish.
