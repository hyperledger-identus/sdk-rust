# Constraint readiness

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/160
Constraint blockers: research evidence pending

## Existing entries affected

`SDK-ARCH-001` and `SDK-ARCH-002` keep transport/runtime/dependency types out
of generic protocol crates and public APIs. `SDK-COMPAT-001` through
`SDK-COMPAT-005` retain exact Rust 1.98.1 and require explicit target evidence.
`SDK-SEC-001` through `SDK-SEC-003` require no authored unsafe, redacted
secrets/errors and bounded untrusted inputs. `SDK-DELIVERY-001` requires the
issue, this contract, review and hosted gates. `SDK-LIM-001`, `SDK-LIM-003`,
`SDK-LIM-005`, `SDK-LIM-006`, `SDK-LIM-007` and `SDK-LIM-009` preserve
unpublished, consumer-owned, resource-bound and temporary fast/slow limits.

## Introduced or changed constraints

- The candidate is exact `oauth2 5.0.0` with default features disabled in a
  separately locked research fixture only.
- No candidate, `url`, `http` or `chrono` type crosses an Identus public API.
- No network client, clock, RNG, browser, listener, runtime or persistence is
  selected by the spike.
- Root locks, root features, supported packages and fast CI remain unchanged.
- A production recommendation is not activation; it requires a separate issue,
  OpenSpec/ADR and consumer-shaped target evidence.

## Introduced or changed limitations

- Candidate behavior is non-normative research evidence.
- The spike does not prove OAuth server behavior, end-to-end browser redirects,
  deployed interoperability, support, certification or production security.
- Target compilation does not prove linking, runtime behavior or package size.
- Known endpoint, scope, duplicate, time and error-model differences remain
  explicit inputs to the final disposition.

## Consumer and product impact

No current consumer or product changes. A future Oxid or issuer protocol engine
may reuse privately admitted mechanics after a separate production decision;
Midnight and donor repositories remain unchanged.

## Activation and rollback

This research capability activates only when issue #160's evidence PR passes
local and hosted gates and merges into `develop`. Rollback removes research
assets only. There is no public/wire, data, release or downstream migration.

## Evidence

Issue #160 and parent #151 provide authority. Final research must record exact
normative and source revisions, license, MSRV, feature and direct/resolved cone,
unsafe/native and supply-chain evidence, target results, compatibility,
limitations, maintenance, reconsideration triggers and exact commands.
