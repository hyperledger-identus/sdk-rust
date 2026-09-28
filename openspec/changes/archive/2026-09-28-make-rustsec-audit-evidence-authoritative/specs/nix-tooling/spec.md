## MODIFIED Requirements

### Requirement: Cargo audit check

The checks module SHALL include a `rust-audit` check that runs `cargo audit` against a pinned RustSec advisory database via crane's `cargoAudit`.

The check SHALL pin or assert a `cargo-audit` release that parses CVSS 4.0,
SHALL pass a deterministic CVSS 4.0 compatibility probe before accepting real
dependency evidence, and SHALL invoke the pinned database scan with explicit
`--no-fetch` and `--no-yanked` behavior. It SHALL retain a structured result
that distinguishes advisory success, a known vulnerability, an incompatible
audit tool, and unavailable yank evidence. A successful advisory scan SHALL
NOT imply that locked crates were checked against an authoritative registry
yank index.

#### Scenario: Audit check passes with no known vulnerabilities

- **WHEN** `nix flake check` is run and no dependency has a known RustSec advisory
- **THEN** the `rust-audit` check SHALL pass
- **AND** its structured advisory result SHALL report success
- **AND** its separate yank result SHALL report unavailable when no
  authoritative registry index was provided

#### Scenario: Audit check fails on a known vulnerability

- **WHEN** a dependency has a known, unignored RustSec advisory
- **THEN** the `rust-audit` check SHALL fail
- **AND** the result SHALL classify the failure as a known vulnerability

#### Scenario: Audit check rejects an incompatible parser

- **WHEN** the selected audit executable cannot parse the deterministic CVSS
  4.0 compatibility fixture
- **THEN** the `rust-audit` check SHALL fail before scanning the workspace
- **AND** the result SHALL classify the failure as an incompatible audit tool

#### Scenario: Hermetic audit does not emit yank-index noise

- **WHEN** the Nix sandbox runs without an authoritative crates.io index
- **THEN** the audit command SHALL use the official no-yank mode
- **AND** it SHALL NOT emit one unavailable-index diagnostic per locked package
- **AND** the structured yank result SHALL remain explicitly unavailable
