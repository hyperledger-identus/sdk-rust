# `identus-sd-jwt-vc`

`identus-sd-jwt-vc` is the canonical, bounded Verifiable Digital Credential
profile over the `identus-sd-jwt` RFC 9901 engine. It is pinned to
`draft-ietf-oauth-sd-jwt-vc-22` and emits exact `dc+sd-jwt` credentials.

## What it owns

- protected `typ=dc+sd-jwt` and generic envelope `format=dc+sd-jwt`;
- `vct`, `aka_vcts`, registered numeric dates, `cnf`, and non-selective claim
  placement;
- finite caller-supplied issuer discovery, Type Metadata, and status-list
  evidence;
- redaction-safe receipts and type/status queries;
- an offline OID4VCI/OID4VP/selective-presentation/holder-binding journey.

## What it does not own

No HTTP, certificate parsing or path building, DID resolution, JWK-set
retrieval, issuer authorization, trust anchors, status-list retrieval or
decoding, freshness/acceptance policy, rendering, persistence, replay, UI,
legacy `vc+sd-jwt` output, or language-specific DTO compatibility is included.

## State progression

![SD-JWT VC stages](../diagrams/sd-jwt-vc-draft22.svg)

The profile consumes a disclosure-verified RFC value. Optional evidence can
prove structural correlation only; it never produces a product trust or
credential-acceptance decision.
The generic report does not promote a caller-selected verification key into
issuer resolution: that stage remains `NotChecked` without correlated issuer
discovery evidence.
Draft-22 array and wildcard Type Metadata paths are first-class, and generic
reports keep issuer, schema and status evidence failures in their own stages.

## Evidence

```console
cargo test -p identus-sd-jwt-vc --all-features
cargo bench -p identus-sd-jwt-vc --bench directional
```

The draft identity is bound in code and tests by name, date, canonical URL and
SHA-256. Any later draft or RFC requires a new reviewed compatibility slice.
