# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/514
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 keep protocol policy above primitives and
effects behind ports. SDK-SEC-003 and SDK-LIM-007 require bounded hostile-input
work. ADR 0159 keeps JOSE engine-neutral. ADRs 0170 and 0173 require canonical
Rust contracts and versioned evidence before language adoption. Existing MSRV,
portable-target, dependency-cone, error-redaction and consumer-ledger rules
remain effective.

## Introduced or changed constraints

- The profile is exactly SIOPv2 draft 13 dated 2023-11-28. It is experimental
  and cannot be described as Final or silently upgraded.
- `identus-openid4vc` owns only shared bounded OpenID request retrieval values;
  it cannot become a broad OAuth/OIDC policy or runtime crate.
- `identus-siopv2` owns protocol types, validation, response construction and
  deterministic transitions, never UI, product consent, transport, storage,
  trust, custody or DID-method behavior.
- The first profile admits signed requests, DID subjects,
  `subject_signed_id_token`, exact `openid` scope and `direct_post` only.
- Request signatures require caller-authorized keys. Signature validity does
  not establish RP trust.
- Self-issued ID Tokens use SDK-owned JOSE values and injected signers. No new
  cryptographic primitive or third-party SIOPv2 DTO/error crosses the API.
- Time and event identity are explicit inputs. Duplicate event IDs are
  deterministic; refusal, cancellation and timeout cannot invoke signing.
- Unknown delivery outcome is represented explicitly and never auto-retried.
- All sensitive material is redacted and zeroized where owned.

## Introduced or changed limitations

- No JWK-thumbprint subject, self-attested claims, attester token, dynamic OP
  discovery, federation, authorization code, front-channel fragment/form-post,
  encrypted ID Token, OpenID4VP presentation or certification is supported.
- Only HTTPS RP/redirect values enter this generic profile. Oxid's loopback
  fixture remains a downstream development adapter concern.
- The SDK emits a replay fingerprint but provides no durable replay store.
- Portable compilation does not prove browser deep-link, entropy, OS dispatch,
  physical-device, DNS, TLS or HTTP behavior.
- Draft-13 compatibility can change before a stable release through a new ADR,
  ledger record and migration path.

## Consumer and product impact

The change adds unpublished crates and types. Oxid can later replace its wire
and state duplication through an adapter while retaining its own consent,
activity, persistence and UI DTOs. No consumer is mutated in this slice.

## Evidence

Issue #514 is the standing product authority. ADR 0182 records the draft pin,
dependency decision and crate boundaries. The OpenSpec scenarios bind the
request, token, lifecycle and adapter contracts. The canonical change ledger,
vector catalog and quality declaration will be activated before implementation
is marked ready, and all evidence remains issue-owned.

## Activation and rollback

Activation requires issue #514 preflight, ADR 0182, strict OpenSpec, change and
quality records, vectors, E2E/property/fuzz/benchmark/differential evidence,
workspace and portable gates, an exact-head Claude or Codex CLI review and
protected CI. Rollback removes the additive crate and shared delegation before
publication; existing OID4VP behavior is preserved.
