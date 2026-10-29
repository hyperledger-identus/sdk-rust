# ADR 0194: isolate draft-03 `vc+sd-jwt` migration from canonical draft-22

- **Status:** Accepted for issue #572 implementation
- **Date:** 2026-10-29
- **Decision authority:** issue #572, ADR 0193, and milestone A3 under ADR 0004
- **Normative sources:** RFC 9901, `draft-ietf-oauth-sd-jwt-vc-03`, and
  `draft-ietf-oauth-sd-jwt-vc-22`
- **Named consumer:** `@hyperledger/identus-sdk` 8.1.4 at
  `4bf86ebf69d5e96616a148e4c973f831f95fa38e`

## Context

SDK-Rust implements exact draft-22 and canonical `dc+sd-jwt`. The released
TypeScript SDK uses `vc+sd-jwt` through `@sd-jwt/sd-jwt-vc` 0.7.2. Immutable
package source proves that dependency implements draft-03, not draft-22.

The protected type is covered by the issuer's JWS signature. Replacing the
string invalidates the signature. The two drafts also differ in status-list
lineage, Type Metadata, additional type, integrity, and issuer-discovery
semantics. An alias or automatic fallback would therefore conflate different
credential profiles and could accept semantics the canonical verifier never
reviewed.

## Decision

1. Add one explicitly selected, input-only draft-03 adapter inside
   `identus-sd-jwt-vc`; do not create another crate or dependency.
2. Bind the adapter to exact `vc+sd-jwt` envelope and protected identity,
   draft-03 source hash, SDK-TS revision, npm version, package revision, and
   integrity receipt.
3. Reuse the verified RFC 9901 state and canonical validation helpers only for
   the closed semantic subset proven common to drafts 03 and 22.
4. Reject status, `aka_vcts`, `vct#integrity`, Type Metadata, millisecond date
   heuristics, and any future unqualified legacy feature.
5. Return a separate legacy migration type and static redacted receipt. It
   cannot convert into `Draft22SdJwtVc`; canonical identity requires issuer
   reissuance.
6. Preserve canonical parsing and issuance unchanged. No API automatically
   dispatches to the adapter or emits `vc+sd-jwt`.
7. Remove the adapter through a separate consumer-visible change after the
   named consumer and supported stored credentials have measured canonical
   migration and the compatibility window ends.

## Consequences

SDK-Rust can help a real released consumer migrate without making historical
TypeScript behavior normative or maintaining a full obsolete profile. Some
credentials accepted by SDK-TS will be rejected, especially millisecond-date
and draft-era status cases. Those incompatibilities are intentional and
recorded rather than hidden behind best-effort conversion.

The public adapter is temporary compatibility debt. Its explicit name,
capability ID, distinct state, no-output contract, and removal gate prevent it
from becoming the default profile accidentally.

## Reconsideration

Broaden the common subset only when a named stored-credential population,
immutable source evidence, exact normative comparison, security/resource
review, and migration need justify a focused follow-up. Do not broaden it
merely because the historical package accepted a value.

Remove the adapter when the issue #572 removal trigger is satisfied. A second
independent consumer needing durable draft-03 support would require a new
profile-lifecycle decision rather than extending this transition implicitly.

## Rollback

Before publication, remove the adapter, legacy vectors, and consumer-change
record. Canonical draft-22 and RFC 9901 behavior remain unchanged. After a
consumer adopts the adapter, rollback restores unconditional legacy rejection
and follows that consumer's separate migration/rollback plan.
