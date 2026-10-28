# identus-sd-jwt-vc

Bounded SD-JWT VC semantics pinned to `draft-ietf-oauth-sd-jwt-vc-22`.

The crate layers canonical `dc+sd-jwt` profile validation over
`identus-sd-jwt` RFC 9901 mechanics. It performs no HTTP retrieval,
certificate path building, issuer authorization, status acceptance, trust,
storage, replay policy, rendering, or product decisions.
