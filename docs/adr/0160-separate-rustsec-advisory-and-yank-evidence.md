# ADR 0160: separate RustSec advisory and yank evidence

- **Status:** Accepted
- **Date:** 2026-09-28
- **Decision authority:** issue #401 under ADR 0004 standing authority
- **Related:** Discussion #399, `SDK-DELIVERY-001`, `SDK-LIM-009`

## Context

The slow Nix lane uses Crane `cargoAudit` with a pinned RustSec advisory-db.
At locked Crane revision `469fd08d`, that helper defaults to
`--ignore yanked`. Current `cargo-audit` interprets `--ignore` as an advisory
ID option; the official switch added in 0.22.2 for a network sandbox is
`--no-yanked`. Consequently the hermetic check prints an unavailable crates.io
index error for essentially every package, even though its advisory scan exits
successfully.

There is a second failure class: obsolete local `cargo-audit` releases cannot
parse current CVSS 4.0 advisories. A command that cannot load the complete
pinned database cannot supply vulnerability evidence. Advisory matching and
registry yank state also come from different data sources and must not share a
single ambiguous success label.

## Decision

1. Keep `cargo-audit` as the authoritative RustSec lockfile scanner and retain
   Crane's reproducible Nix integration.
2. Pin or assert `cargo-audit 0.22.2` for this Nix input. It is the reviewed
   release that supplies `--no-yanked` and parses the pinned CVSS 4.0 database.
3. Override Crane's default with the exact official no-yank option. The
   hermetic advisory derivation performs no registry-index lookup.
4. Run a deterministic CVSS 4.0 compatibility fixture before the workspace
   scan. Failure is `incompatible-tool`, not a vulnerability result.
5. Retain structured evidence with distinct advisory and yank states. A clean
   RustSec scan is `success`; a matching advisory is `vulnerability`; absent
   authoritative registry data is `yanked-unavailable`.
6. Do not make the Nix sandbox networked and do not treat a host Cargo cache as
   evidence. An authoritative yank gate requires a separate immutable or live
   data-source/freshness decision.

## Consequences

- The pinned RustSec gate becomes quiet, reproducible, and explicit about what
  it proved.
- An obsolete parser fails deterministically before a green audit can be
  claimed.
- Release evidence continues to disclose unavailable yank status instead of
  silently accepting or suppressing a discovered yank.
- The repository carries a narrow override while Crane retains its ambiguous
  default. That override is removed only after a reviewed Crane update provides
  equivalent semantics.
- No SDK crate API, wire behavior, dependency resolution, compiler floor, or
  target promise changes.

## Alternatives rejected

- **Allow network in the Nix derivation:** breaks the hermetic evidence model.
- **Keep `--ignore yanked`:** it is the wrong CLI option and preserves noise.
- **Replace cargo-audit with cargo-deny:** expands this repair into a scanner
  and policy migration without providing an authoritative hermetic index.
- **Treat index failure as a clean yank result:** converts missing evidence
  into false assurance.

## Rollback and review trigger

Rollback reverts the focused gate and restores the disclosed prior limitation;
it cannot label the old output authoritative. Review this ADR when Crane fixes
its default, `cargo-audit` changes report semantics, the pinned advisory schema
changes, or the project accepts a freshness-governed registry-index source.
