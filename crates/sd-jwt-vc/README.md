# identus-sd-jwt-vc

Bounded SD-JWT VC semantics pinned to `draft-ietf-oauth-sd-jwt-vc-22`.

The crate layers canonical `dc+sd-jwt` profile validation over
`identus-sd-jwt` RFC 9901 mechanics. It performs no HTTP retrieval,
certificate path building, issuer authorization, status acceptance, trust,
storage, replay policy, rendering, or product decisions.

For migration only, `LegacyVcSdJwtDraft03Adapter` accepts the source-bound
draft-03 common subset behind an explicit `vc+sd-jwt` entry point. It never
emits legacy credentials, never aliases them to draft-22, and returns a
distinct state whose receipt requires canonical issuer reissuance. Canonical
parsing, validation, and issuance remain exact `dc+sd-jwt`.

See the [profile architecture](../../docs/architecture/sd-jwt-vc-draft22-profile.md)
and [migration evidence](../../docs/research/platform-core/sd-jwt-vc-draft03-migration.md).
