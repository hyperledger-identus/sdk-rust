# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/559
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 require chain-neutral inward dependencies,
cohesive ownership, and isolated effects. SDK-SEC-001 and SDK-SEC-003 require
safe Rust and finite untrusted work. SDK-LIM-007 leaves delegated QoS with the
caller. SDK-COMPAT-001 through SDK-COMPAT-005 govern stable, MSRV, target, and
compiler evidence. ADR 0187 authorizes a separate method crate; ADR 0190 fixes
the injected retrieval boundary.

## Introduced or changed constraints

- `identus-did-web` depends inward on `identus-core` and `identus-did`; no HTTP,
  DNS, TLS, URL-runtime, broad SSI, or sibling method crate enters production.
- The public identifier, authority, path segments, port, request URL, response
  media type, and response body all have explicit finite ceilings.
- Identifier validation completes before request construction or effect
  invocation. Response status and media type are checked before JSON parsing.
- Public errors are stable categories and never include identifiers, URLs,
  headers, response bytes, document contents, or transport diagnostics.
- The core invokes exactly one caller-supplied fetch operation per resolution;
  redirect, retry, cache, cancellation, timeout, TLS, DNS, proxy, privacy, and
  SSRF behavior remain outside the crate.
- A successful result requires a bounded valid DID document whose `id` equals
  the exact requested DID.

## Introduced or changed limitations

The first profile excludes IP literals, single-label hosts, Unicode/IDNA input,
raw or noncanonical percent escapes, default port 443, dot/empty path segments,
non-JSON representations, registration operations, and concrete transports.
It does not prove DNS or TLS trust and does not make a remote document trusted.

## Consumer and product impact

Rust adopters gain an opt-in portable method core and implement a small runtime
adapter under their own network policy. Existing crates retain behavior. No
language SDK, service, wallet, or downstream repository is migrated.

## Activation and rollback

Activation requires exact #559 preflight, ADR 0190, provenance, all declared
quality and target evidence, immutable ledger/catalog entries, exact-diff CLI
review, dependency and stop-boundary audits, a distinct deterministic E2E run,
green protected CI, and an issue-linked PR. Before publication, rollback removes
the additive crate and evidence without data migration.

## Evidence

Evidence authorities are issue #559, ADRs 0187 and 0190, pinned primary
sources, attributed oracle fixtures, OpenSpec/factory receipts, quality and
target gates, deterministic E2E, exact-diff review, and protected CI.
