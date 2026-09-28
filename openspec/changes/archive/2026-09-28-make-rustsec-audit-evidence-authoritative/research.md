# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-28
Source retrieval date: 2026-09-28
Research blockers: none

## Problem and existing implementation

The current implementation generates `rust-audit` through Crane
`cargoAudit` at locked Crane revision
`469fd08d0bcf6926321fa973c6777fbc87785dd7`. That helper invokes
`cargo audit -n -d <pinned-db> --ignore yanked`. In `cargo-audit 0.22.2`,
`--ignore` accepts an advisory ID and `--no-yanked` is the distinct switch for
disabling registry-index yank checks. The Nix sandbox has no populated
authoritative crates.io index, so the current command emits a noisy lookup
error for essentially every locked registry package and nevertheless exits
successfully after the vulnerability scan.

The Nix devshell currently resolves `cargo-audit 0.22.2` from locked nixpkgs
revision `7241bcbb4f099a66aafca120d37c65e8dda32717`. Against advisory-db
revision `1e3b508975f52402873fe274892f9b86fc57c544`, the explicit command
`cargo audit --no-fetch --db <db> --no-yanked --format json` succeeds, reports
1,145 parsed advisories, zero matching vulnerabilities, and no warnings. The
same database contains and successfully parses the CVSS 4.0 record
`RUSTSEC-2026-0073`.

## Normative sources

The primary sources are the signed/published `cargo-audit 0.22.2` release, its
CLI/configuration implementation, RustSec advisory-db schema and immutable
repository input, Crane's immutable `cargoAudit.nix`, and the repository's
existing Nix-tooling capability. The cargo-audit release added the official
`--no-yanked` option through rustsec/rustsec#1574 specifically for network
sandboxes. Crane's locked revision and current main still use
`--ignore yanked`, so a repository override is necessary.

Primary source URLs:

- https://github.com/rustsec/rustsec/releases/tag/cargo-audit%2Fv0.22.2
- https://github.com/rustsec/rustsec/pull/1574
- https://github.com/rustsec/rustsec/blob/main/cargo-audit/src/commands/audit.rs
- https://github.com/rustsec/advisory-db/blob/1e3b508975f52402873fe274892f9b86fc57c544/crates/libcrux-poly1305/RUSTSEC-2026-0073.md
- https://github.com/ipetkov/crane/blob/469fd08d0bcf6926321fa973c6777fbc87785dd7/lib/cargoAudit.nix

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| `cargo-audit` | 0.22.2 / published 2026-06-05 | `adopt` | Current Nix package parses the pinned CVSS 4.0 database and provides `--no-yanked`; it remains the narrow official RustSec lockfile scanner. | A newer pinned release fixes a security/parser defect or materially changes report semantics. |
| Crane `cargoAudit` | `469fd08d` | `conditional-adopt` | Retain the reproducible helper but override its incorrect default argument explicitly. | Crane changes its default to `--no-yanked` and the locked revision is deliberately updated. |
| `cargo-deny advisories` | 0.20.2 | `not-adopt` for this gate | It overlaps the advisory and yank concerns and has distinct configuration/report semantics; replacing the accepted RustSec gate adds migration without solving hermetic index availability. | A separate dependency-policy issue selects one authoritative scanner with parity evidence. |
| Live crates.io index in Nix | mutable service/index | `not-adopt` | Network or host cache access would make the supposedly hermetic derivation non-reproducible. | A content-addressed registry-index snapshot and freshness policy are accepted. |
| Explicit unavailable yank status | repository-owned evidence schema | `adopt` | Truthfully separates a missing data source from advisory success and removes per-package noise. | A separately authoritative, fresh yank data source becomes part of slow/release evidence. |

## Compatibility and dependency evidence

The public and wire compatibility of SDK crates is unchanged. The change adds
no production or development dependency and does not alter Cargo resolution.
Its direct and resolved dependency cone is the existing Nix quality-tool cone.
The facade remains the `rust-audit` check and repository factory evidence;
third-party output details do not become an SDK API. Rust 1.89.0 MSRV and all
supported target/feature promises are unaffected because the audit executable
runs only in the Nix host environment.

## Security, privacy and maintenance evidence

The tool is RustSec-maintained with Apache-2.0 OR MIT license provenance and is
supplied by the immutable nixpkgs input; the advisory data is pinned
independently with repository and revision provenance.
Neither fixture nor result contains consumer data or secrets. No new unsafe or
native production code is reachable. Supply-chain evidence remains separated:
the audit gate scans known advisories, while `cargo-deny` continues license,
source, and ban policy. Yank evidence is unavailable in the hermetic lane and
must not be promoted to a success claim.

The maintenance and release posture is bounded by an exact tool version, Crane
revision, advisory-db revision, CVSS 4.0 probe, and update trigger. Protocol or
draft currency is not applicable; CVSS 4.0 parser currency is the relevant
data-format compatibility requirement.

## Rejected or deferred candidates

Replacing the Nix scanner, allowing network access, importing a mutable
registry cache, and accepting Crane's ambiguous output are rejected. An
authoritative yank gate is deferred because no immutable, freshness-governed
index input currently exists. Rollback restores the previous Crane argument,
but that also restores misleading noise and is appropriate only if the new
probe itself is proven wrong.

## Open questions and blockers

There are no blockers for the advisory correction. Exact authoritative yank
evidence remains explicitly unavailable rather than an implementation blocker
for the advisory scan. It remains a visible release-evidence limitation until
a separate issue supplies a governed data source.

## Evidence commands

- `nix eval` resolved Nix `cargo-audit 0.22.2` and immutable input paths.
- `cargo audit --help` confirmed `--ignore <ADVISORY_ID>` and `--no-yanked` are
  distinct.
- `cargo audit --no-fetch --db <pinned-db> --no-yanked --format json` parsed
  the full pinned database and returned a zero-vulnerability JSON result.
- `nix log <rust-audit-output>` reproduced the per-package yank-index errors.
- GitHub primary-source inspection pinned rustsec/rustsec#1574, the
  `cargo-audit 0.22.2` release, and Crane `cargoAudit.nix` at `469fd08d`.

Unrun at planning time: deterministic fixture tests, mutated vulnerable and
incompatible-tool cases, the updated `rust-audit` derivation, full factory
checks, Nix lint/checks, and protected fast CI. Tasks require them before
delivery.
