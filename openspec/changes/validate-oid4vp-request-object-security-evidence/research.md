# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The current implementation verifies signed Request Objects and exposes typed
routing directly from `VerifiedRequestObject`. Consumers therefore need to
recreate audience, freshness, and replay validation. Existing Rust consumers
such as Oxid and Lace ID Portal need one portable SDK facade rather than a
product-specific policy implementation. The current implementation already has
the bounded scanner and state lineage needed for a narrow successor.

## Normative sources

- OpenID for Verifiable Presentations 1.0 Final, section 5.8, requires the
  Request Object `aud` to equal the discovered issuer for dynamic discovery or
  `https://self-issued.me/v2` for static discovery:
  https://openid.net/specs/openid-4-verifiable-presentations-1_0.html.
- The same Final text says a Wallet ignores a Request Object `iss`. Therefore
  the expected dynamic audience comes from caller-owned discovery evidence,
  never from the signed object's ignored `iss` claim.
- RFC 9101 defines Request Object JWT claims and incorporates `exp`, `nbf`,
  `iat`, and `jti`: https://www.rfc-editor.org/rfc/rfc9101.html.
- RFC 7519 defines audience, NumericDate, skew, and JWT-ID semantics:
  https://www.rfc-editor.org/rfc/rfc7519.html.

The protocol currency is OpenID4VP 1.0 Final with current errata, RFC 9101, and
RFC 7519; no superseded draft behavior is treated as normative.

## Candidate decisions

| Candidate | Exact version/features or revision | Decision | Reason |
| --- | --- | --- | --- |
| Protocol-local state and scanner | repository revision `d33321e1dd6ae47cad934d34e6ba87b366f82190` | `retain-local` | Preserves bounded payload lineage and the Rust-native facade. |
| `identus-core::WallClock` | workspace package, default features | `adopt` | Existing target-neutral clock injection boundary. |
| `jsonwebtoken` | 11.1.0, default `use_pem` | `not-adopt` | Duplicates JOSE/parsing and lacks the atomic replay transition. |
| `jwt-compact` | 0.8.0, default clock/chrono/ciborium features | `oracle` | Useful API reference but a broader dependency and DTO surface. |

## Compatibility and dependency evidence

The current architecture already has the bounded scanner and state lineage
needed for a narrow successor. The direct and resolved dependency cone does
not grow: `identus-oid4vp` continues to use `identus-core`, `identus-jose`,
`fluent-uri`, `serde_json`, `siros-dcql`, and `zeroize`. Exact Cargo versions
remain locked by the workspace. Public compatibility is a declared pre-release
source break; wire compatibility follows Final section 5.8 and RFC JWT claims.
The public facade exports only SDK-owned types and no dependency DTO.

MSRV remains Rust 1.89 and the primary compiler remains Rust 1.98.1. Target
evidence remains native, `wasm32-unknown-unknown`, iOS, and Android libraries.
No new unsafe code or native code is introduced. Existing license provenance
is Apache-2.0 repository code plus clean-room cases derived from OpenID and IETF
document licenses. The supply-chain lock and advisory policy remain unchanged.

Rollback before publication removes the new state and restores the prior
pre-release API. The migration is explicit and no wire or persistence format is
written by the crate.

## Security, privacy and maintenance evidence

Maintenance, release, and security ownership stays with the OID4VP maintainers.
The implementation adds finite time arithmetic, bounded strings/arrays, atomic
replay acceptance, static errors, and redacted `Debug`. It does not log or own
clock, persistence, trust, key authorization, network, or product policy. The
release remains unpublished and experimental until normal gates pass.

The current bounded scanner already rejects duplicate top-level names and
enforces payload bytes, depth, nodes, members, and decoded-string limits.
`identus-core::WallClock` is the repository clock boundary. OID4VCI demonstrates
the accepted pattern of protocol-local time policy plus an atomic caller-owned
replay port. ADR 0159 keeps protocol policy outside `identus-jose`.

## Rejected or deferred candidates

`jsonwebtoken 11.1.0` and `jwt-compact 0.8.0` were reassessed. Both duplicate
the existing JWS and bounded JSON machinery; their default feature cones add
PEM/time or chrono/CBOR concerns and their validation model does not provide
the SDK's atomic replay transition. They remain research references, not
dependencies. The resolved production dependency cone does not grow.

System time, product storage, discovery networking, verifier trust, a generic
JWT abstraction, and sideways dependency on OID4VCI are deferred. A check-then-
store replay API is rejected because it cannot provide atomic acceptance.

## Open questions and blockers

No planning blocker remains. Integer-second NumericDate is the initial bounded
profile. The reconsideration trigger is an adopted interoperability profile or
fixture requiring fractional NumericDate, or a maintained JWT crate satisfying
all bounded facade and atomic replay constraints.

## Reconsideration triggers

Reassess a JWT crate only if it accepts the SDK's bounded verified payload,
keeps algorithms in `identus-jose`, supports injected time and atomic replay,
does not export foreign DTOs, preserves portable targets/MSRV, and materially
reduces maintained code. Reassess fractional NumericDate support if an adopted
Final interoperability profile or fixture requires it.

## Evidence commands

Commands used include `cargo info jsonwebtoken@11.1.0`, `cargo info
jwt-compact@0.8.0`, `cargo tree -p identus-oid4vp --edges normal --depth 2`,
and repository inspection with `rg`/`sed`. Planning gates are `scripts/factory research-ready
validate-oid4vp-request-object-security-evidence`, `scripts/factory
constraints-ready validate-oid4vp-request-object-security-evidence`, strict
validation, and issue-bound preflight. Implementation evidence must include
focused/all-feature/no-default tests, E2E vectors, fuzz, benchmark, Clippy,
rustdoc, portable checks, factory checks, and exact-head CLI review. Those
implementation commands and hosted CI are unrun at planning time.
