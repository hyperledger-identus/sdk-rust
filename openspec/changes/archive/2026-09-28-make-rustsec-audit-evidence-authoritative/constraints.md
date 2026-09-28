# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/401
Constraint blockers: none

## Existing entries affected

`SDK-DELIVERY-001` requires truthful issue-linked and CI-gated evidence.
`SDK-LIM-009` places exhaustive advisory evidence in the weekly/release slow
line. The existing Nix-tooling capability requires a pinned RustSec audit but
does not distinguish parser compatibility from unavailable yank data.

## Introduced or changed constraints

The authoritative hermetic advisory scan must use the pinned advisory database
with a pinned CVSS 4-compatible `cargo-audit`, must pass a deterministic CVSS 4
parser probe, and must disable yank lookup explicitly with `--no-yanked`.
Advisory success, known vulnerability, incompatible audit tool, and unavailable
yank data are distinct states. No green advisory result may imply yank success.

## Introduced or changed limitations

The hermetic Nix lane does not provide authoritative yank evidence because it
has neither live registry access nor a separately pinned, freshness-governed
crates.io index. It reports that state as unavailable. This narrows an
ambiguous prior claim; it does not waive a discovered yank or create an
allowlist.

## Consumer and product impact

SDK crate APIs, wire formats, dependency resolution, MSRV, targets, and product
behavior are unchanged. Maintainers and release managers gain accurate
security evidence and can see that yank status still needs an independent data
source. No downstream repository changes.

## Activation and rollback

Activation requires the planning-only receipt, ADR 0160, deterministic CVSS 4
and mutation fixtures, exact tool assertion, a quiet full pinned-db audit,
factory/Nix checks, distinct review, signed/DCO commits, and green protected
CI. Rollback reverts the gate as one focused change and restores the prior
known limitation; it cannot relabel the noisy prior output as authoritative.

## Evidence

Issue #401 and Discussion #399 are the durable decision/audit records. The
immutable RustSec, Crane, nixpkgs and advisory-db revisions in `research.md`,
ADR 0160, mutation tests, Nix output, and final PR receipt provide enforcement
evidence.
