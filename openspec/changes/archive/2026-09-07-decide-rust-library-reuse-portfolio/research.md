# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-07
Source retrieval date: 2026-09-07
Research blockers: none

## Problem and existing implementation

The current implementation contains local BIP-39, BIP-32, DID/URI, multihash,
media-header and form parsing mechanics. The code audit found missing BIP-39
checksum/NFKD validation and non-conforming BIP-32 invalid-scalar reduction.
The full current implementation and consumer analysis is in the
[research report](../../../../docs/research/rust-library-reuse/report-source.md).

## Normative sources

The research pins [BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki),
[BIP-32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki),
[RFC 3986](https://www.rfc-editor.org/rfc/rfc3986.html),
[RFC 9901](https://www.rfc-editor.org/rfc/rfc9901.html), DID Core and OpenID
final specifications and records upstream crate revisions, versions, paths,
retrieval date and license evidence in its claim-to-source ledger.

## Candidate decisions

| Candidate group | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| `bip39`, `bip32`, `multihash`, `fluent-uri`, `form_urlencoded` | Exact versions in ADR 0061 | `adopt` | Focused standards mechanics reduce demonstrated local correctness risk. | Re-evaluate on conformance, target, security or maintenance regression. |
| `multibase` | 0.9.3 | `conditional-adopt` | Correct surface; current resolution actually requires Rust 1.88. | Rolling MSRV implementation or compatible upstream resolution. |
| `did_url_parser`, `isomdl`, `oauth2`, Askar, UniFFI | Exact revisions in report | `spike` | Promising but parity, cone, state or platform boundaries remain unproved. | Named spike acceptance passes. |
| `identity.rs`, Spruce SSI/OpenID, impierce OpenID4VC, `jose-jwk` | Exact revisions in report | `oracle` | Valuable independent behavior without acceptable production coupling. | A narrow subcrate independently passes all adoption gates. |
| Draft/stale/semantic mismatch candidates | Exact revisions in negative ledger | `not-adopt` | Draft currency, maintenance, coupling or semantic mismatch. | Objective trigger in the negative ledger is met. |
| Local SLIP-0010 and strict HTTP field parsers | Current SDK base | `retain-local` | Candidates do not currently reduce total risk. | Broader demand or a focused replacement proves lower total cost. |

## Compatibility and dependency evidence

The report records declared MSRV, standalone dependency cone, minimal features,
unsafe triage and host/target probes. Rust 1.85 passed approved candidates
individually except `multibase`; the combined set passed Rust 1.90 host and Rust
1.95 WASM, iOS and Android compile checks. Public dependency types remain
private and each follow-up owns API/wire compatibility and rollback evidence.
Exact versions and features, source revision, license and provenance, direct
and resolved dependency cones, and the Identus facade boundary are recorded in
the report or focused follow-up issue.

## Security, privacy and maintenance evidence

Exact-version OSV queries returned no advisories on the retrieval date, but are
not treated as an audit. The local older `cargo-audit` could not parse a current
CVSS 4.0 advisory, so no audit-pass claim is made. Each integration must run the
pinned repository supply-chain gates, inspect reachable unsafe/native code and
preserve redacted errors, bounded inputs and zeroizing secret ownership.
Maintenance, release and security posture plus protocol/draft currency are
recorded per candidate in the linked report and follow-up issues.

## Rejected or deferred candidates

The machine-readable decision is rendered in the
[not-adopted ledger](../../../../docs/research/rust-library-reuse/not-adopted.md).
Every entry includes the candidate/version, reason, current alternative and
reconsideration trigger. No rejected candidate receives a production
integration issue.

## Open questions and blockers

No research blocker remains for this architecture/factory change. Individual
dependency behavior, MSRV rollout and platform integration questions are
deliberately isolated into focused follow-up issues and cannot be inferred as
approved implementations by this record.

## Evidence commands

- `scripts/factory doctor` passed before edits at the exact base SHA.
- Exact minimal-feature dependency probes ran with Rust 1.85, 1.90 and 1.95.
- Rust 1.95 compile checks passed for `wasm32-unknown-unknown`,
  `aarch64-apple-ios` and `aarch64-linux-android`.
- `cargo tree` measured standalone normal dependency cones.
- OSV exact-version queries ran on 2026-09-07.
- Repository factory, OpenSpec, text and script tests are tasks in this change
  and are not claimed complete until their outputs are recorded.
- No unrun check is represented as passing; compile-only and unavailable audit
  evidence is named explicitly above.
