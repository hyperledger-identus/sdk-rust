# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-29
Source retrieval date: 2026-10-29
Research blockers: none

## Problem and existing implementation

The current implementation is intentionally canonical-only.
`identus-sd-jwt-vc` implements exact draft-22 over the bounded RFC 9901
engine. Its canonical parser accepts only envelope format `dc+sd-jwt`; profile
validation requires protected `typ=dc+sd-jwt`; and all issuance emits that
identity. This is correct for new credentials and must remain unchanged.

The named released consumer `@hyperledger/identus-sdk` 8.1.4 at immutable
revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e` declares
`CredentialType.SDJWT = "vc+sd-jwt"`, routes the same OEA format, and imports
`@sd-jwt/sd-jwt-vc@^0.7.1`, resolved to 0.7.2. It also tolerates millisecond
`exp` and `nbf` values. Those are migration facts, not the Rust contract.

The SDK-TS checkout was inspected read-only with `git show` at the immutable
revision. Its observed working checkout was already dirty at
`b5d0ec99515b9433c9a082ab707d75a6041379f2` before this slice; no consumer
file, branch, index, submodule, dependency, or generated artifact was changed.

## Normative sources

- Canonical profile: `draft-ietf-oauth-sd-jwt-vc-22`, dated 2026-10-27,
  official text SHA-256
  `da2ade6128e9088e28e482866aa744f8287ac7eb6f11a5713bb2166827299124`.
- Legacy source profile: `draft-ietf-oauth-sd-jwt-vc-03`, dated 2024-03-04,
  `https://www.ietf.org/archive/id/draft-ietf-oauth-sd-jwt-vc-03.txt`,
  official text SHA-256
  `9599225eb8732c0ef3f9a6524589e637f9272ba742da185c43ecfd272b81bf5a`.
- Stable disclosure base: RFC 9901 and the canonical `sd-jwt-rfc9901`
  capability. Historical draft-era base behavior is not reintroduced.
- Consumer: `hyperledger-identus/sdk-ts` revision
  `4bf86ebf69d5e96616a148e4c973f831f95fa38e`, Apache-2.0, selectors
  `packages/shared/domain/src/models/Credential.ts`,
  `packages/lib/sdk/src/plugins/internal/oea/types.ts`,
  `packages/lib/sdk/src/pollux/utils/jwt/SDJWT.ts`,
  `packages/lib/sdk/src/pollux/utils/jwt/CreateSDJWT.ts`, and
  `packages/lib/sdk/tests/pollux/SDJWT.test.ts`.
- Migration oracle: `@sd-jwt/sd-jwt-vc` 0.7.2, Apache-2.0, npm integrity
  `sha512-rryYmnoJHRSNqHcrs0Atta+bfJzU2yT7mYumR2D4lTfxJKWZd0OHHFq57uZSEm/wXPI6uytUJXYbEboCqLUAtw==`,
  npm SHA-1 `565a4fcebfe97915a995fd26b0e3cb1fdd56ff6a`, package Git revision
  `d3cc53b66be9ef40ede9f86a931b7a43f55dbd3a`, and retrieved tarball SHA-512
  `aebc989a7a091d148da8772bb3402db5af9b7c9cd4db24fb998ba64760f89537f124a5997743871c5ab9eee652126ff05cf23abb2b5425761b11ba02a8b500b7`.

The package metadata says “sd-jwt draft 7 implementation”, while its VC source
explicitly links draft-03, emits `vc+sd-jwt`, protects only the draft-03
reserved claim set, and embeds network status-list retrieval. The immutable
source and draft text, not the package description, determine compatibility.

## Draft lineage and common subset

Draft-03 and draft-22 share these credential-level semantics used by the
adapter: signed SD-JWT compact input, required protected type (with different
values), required collision-resistant `vct`, required `iss` in draft-03, the
non-selective common claims `iss`, `nbf`, `exp`, `cnf`, and `vct`, and optional
selective `sub` and `iat`.

They diverge materially:

- protected/media identity changed from `vc+sd-jwt` and
  `application/vc+sd-jwt` to `dc+sd-jwt` and
  `application/dc+sd-jwt`;
- draft-22 adds `aka_vcts`, `vct#integrity`, Type Metadata, different issuer
  discovery choices, and revised status-list references;
- draft-03 points at an early JWT/CWT Status List draft and the 0.7.2 package
  performs network retrieval and status acceptance inside verification;
- SDK-TS accepts millisecond date heuristics, while RFC/JWT NumericDate and
  the canonical Rust profile use seconds;
- a protected JWS header cannot be renamed without invalidating the issuer
  signature.

Therefore the adapter rejects every diverging semantic rather than guessing.
It proves only that a verified legacy credential is compatible with the
canonical common subset and must be reissued to become draft-22.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Explicit input-only adapter in `identus-sd-jwt-vc` | `adopt` | Smallest cohesive owner; reuses staged RFC 9901 verification, existing limits, errors, and canonical claim checks without a dependency or automatic fallback. | Remove after the named consumer and supported stored credentials migrate. |
| Treat `vc+sd-jwt` as a string alias for `dc+sd-jwt` | `reject` | The protected value is signed and the drafts differ semantically; rewriting is cryptographically invalid and silent aliasing mislabels the credential. | Never without a new issuer signature and explicit reissuance. |
| Import `@sd-jwt/sd-jwt-vc` 0.7.2 behavior | `oracle` | It is the exact consumer engine but embeds TypeScript DTO/runtime, draft-era status networking, dynamic errors, and broader unbounded behavior. | No production adoption; only immutable comparison evidence. |
| Add a separate legacy crate | `reject` | One input transition does not justify another release unit or duplicate the profile/base graph. | Two independent consumers require a separately versioned legacy engine. |
| Keep unconditional rejection | `reject` | A released named consumer and stored-format identifier establish a real bounded migration need under ADR 0193. | Remove the adapter once the migration window closes. |

No donor code, candidate type, dynamic error, task runtime, DID resolver, HTTP
client, status engine, or package dependency enters the production graph.

## Public, wire, and error contract

The adapter is selected only through an explicitly named draft-03 adapter
type and capability ID. Its parser accepts a generic credential envelope only
when its format is exactly `vc+sd-jwt`; its validator consumes an already
issuer-verified and disclosure-processed RFC 9901 state whose protected type
is exactly `vc+sd-jwt`.

Accepted claims are limited to the common profile surface. `status`,
`aka_vcts`, and `vct#integrity` are rejected; Type Metadata and draft-22 status
evidence are not accepted by the adapter API. Numeric dates use canonical
seconds and the SDK-TS millisecond heuristic is rejected. Canonical
`parse_draft22_envelope`, `Draft22SdJwtVc::validate`, and `issue_draft22`
remain unchanged and continue to reject or never emit the legacy identity.

The resulting type is explicitly legacy and cannot convert into
`Draft22SdJwtVc`. Its receipt exposes only fixed profile/evidence categories,
bounded counts, and `canonical_reissuance_required=true`. New error categories
are static and value-free. No credential, claim, identifier, URI, key, date,
disclosure, or comparison value appears in Debug, Display, errors, receipts,
metrics, or docs output.

## Compatibility and dependency evidence

The production dependency graph is unchanged. The adapter reuses private
helpers inside `identus-sd-jwt-vc`, which already depends inward on
`identus-sd-jwt`, `identus-credentials`, `identus-crypto`, and `identus-jose`.
The npm package and SDK-TS remain Apache-2.0 reference-only sources. No source
or fixture is copied; all vectors are clean-room semantic cases with source
hashes and transformation records.

License and provenance are therefore source-bound without importing donor
code. The direct and resolved dependency cone is the existing locked Cargo
graph. No new unsafe code or native code enters the crate, and supply-chain
evidence remains the Cargo lock, Cargo-deny policy, immutable npm receipt, and
clean-room fixture record. The public facade boundary remains the explicit
Identus adapter and legacy result types; no third-party public type is
re-exported. Public and wire compatibility are additive because canonical
draft-22 entry points and encodings remain byte-for-byte unchanged.

The adapter inherits the crate's stable/MSRV and compile-only WASM, iOS, and
Android target eligibility. This does not add FFI, JavaScript, browser/mobile
runtime, package publication, or certification claims.

## Security, privacy and maintenance evidence

The adapter adds no new cryptography. It requires the same issuer signature,
disclosure digest, duplicate/collision, key-binding, and bounded compact
evidence as the RFC 9901 engine. Existing base/profile byte, count, depth,
retained-output, and work limits apply before proportional processing.
Legacy-only checks are constant-field membership and protected-type/envelope
comparisons. The adapter performs no network, status lookup, time acceptance,
trust decision, persistence, rendering, or telemetry effect.

Threats are automatic fallback, signature-breaking relabeling, draft
confusion, millisecond-date acceptance, old status semantics, correlating
diagnostics, and an adapter that becomes permanent. Explicit entry points,
closed common-subset validation, redacted outputs, no issuance, and a recorded
removal gate address those threats.

The maintenance, release, and security posture is deliberately transitional:
the adapter is release-reviewed with the owning crate, is never an issuance
path, and is removed only through measured consumer migration. Before
publication, rollback removes the additive adapter and its evidence; after a
release, rollback is a new compatible release that disables or removes the
explicit entry point under the recorded consumer-visible gate.

## Quality and end-to-end plan

The pending declaration ID is
`credentials.sd-jwt-vc.legacy-draft03-adapter.quality.v1`.

- Property evidence sweeps the common claims, non-selective placement, exact
  legacy/canonical identity, date representations, unsupported draft features,
  order, boundaries, and staged ownership.
- Fuzz evidence routes arbitrary bytes and bounded claim graphs through the
  explicit parser and adapter without an automatic canonical fallback.
- Benchmarks compare accepted common-subset validation, early identity reject,
  and unsupported-draft-feature reject against the canonical profile path.
- Differential evidence freezes SDK-TS 8.1.4 plus package 0.7.2 observations
  and resolves every acceptance difference against draft-03, draft-22, RFC
  9901, and the SDK resource contract.
- Offline E2E issues a legacy-typed clean-room credential through the existing
  RFC base, verifies it, validates it only through the explicit adapter,
  proves the canonical parser/profile reject it, and proves canonical issuance
  still emits only `dc+sd-jwt`.

## Rejected or deferred candidates

Automatic fallback, protected-header rewriting, legacy issuance/output,
millisecond-date normalization, draft-03 status retrieval/acceptance, Type
Metadata synthesis, issuer trust, HTTP, persistence, language DTO/error/task
compatibility, FFI, publication, certification, and downstream mutation are
out of scope.

## Open questions and blockers

No planning blocker remains. Draft-03 and draft-22 make a safe full semantic
alias impossible, so the adapter deliberately supports only the proven common
subset. Any request to accept another draft-03 surface requires a new issue,
source-bound evidence, and explicit migration decision.

## Evidence commands

Planning uses `scripts/factory doctor`, `backlog-live`, `research-ready`,
`constraints-ready`, strict OpenSpec validation, `factory check`, a signed
planning commit, and `preflight ... --issue 572 --write`. Implementation adds
focused all-feature/no-default tests, the cross-language vector validator,
property/fuzz/benchmark/differential/E2E routes, workspace format/Clippy/tests/
docs, factory/Nix, MSRV/target/dependency/advisory evidence, and Claude CLI or
immediate Codex CLI fallback review. Unrun implementation gates at planning
time are not inferred from research readiness.
