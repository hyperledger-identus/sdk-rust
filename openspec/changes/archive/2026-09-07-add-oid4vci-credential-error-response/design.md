# Design: bounded OID4VCI Credential Error Response parsing

## Context

OpenID4VCI 1.0 Final section 8.3.1.2 defines a JSON Credential Error Response
with required `error` and optional `error_description`. It says implementations
SHOULD use seven named codes, so a reusable parser must recognize those values
without rejecting valid future or ecosystem extensions. The description is
remote developer information rather than localized or trusted wallet guidance.

The SDK's Token Error Response already demonstrates a bounded, duplicate-safe,
redaction-safe parser shape. This slice applies that reviewed policy to the
different Credential Endpoint vocabulary without importing the Token
Endpoint's `error_uri` field or RFC 6749 code classification.

Read-only Oxid at `5ba38b9bbc9326c294b353daaf2a074eca18c22f`
contains an independent consumer-side OID4VCI flow but no reusable Final error
boundary. Lace ID Portal at
`804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` contains issuer error/service
surfaces but no matching Final Credential Error code surface. No consumer
source or fixture is copied.

## Goals / Non-Goals

**Goals:**

- bound complete input, JSON depth/nodes, code bytes, and description bytes;
- reject duplicate decoded names, malformed shape, and unsafe string grammar;
- retain the exact extension-compatible code and classify seven Final values;
- expose descriptions only through a deliberately untrusted accessor;
- zeroize retained strings and redact response/error diagnostics;
- preserve Rust 1.85 and native/mobile/browser targets.

**Non-Goals:**

- HTTP status, media type, headers, authentication challenges, or transport;
- RFC 6750 Authorization errors or OAuth Token Error Responses;
- request correlation, endpoint provenance, trust, retry, blame, remediation,
  localization, notification, or UI policy;
- superseded `c_nonce` fields, error URI, deferred issuance, or encryption;
- credential parsing/verification/storage, chain extensions, FFI, consumer
  adoption, publication, release, or `main`.

## Decisions

### Preserve valid extensions and classify exact Final values

`CredentialEndpointErrorCode` owns the exact bounded case-sensitive code.
`CredentialEndpointErrorKind` recognizes the seven Final values and maps every
other syntactically valid code to `Extension`. Since the Final uses SHOULD,
rejecting every unregistered value would make the core needlessly brittle.
Classification carries no recovery, retry, blame, or UI semantics.

Both `error` and `error_description` use the Final's NQSCHAR-compatible ASCII
set. They must be non-empty; quote, backslash, controls, and non-ASCII fail.

### Reuse the bounded JSON scanner without merging protocol types

The existing duplicate-safe scanner gains a Credential Error Response field
projection. Known fields receive independent decoded-byte limits. Unknown
fields are fully syntax/depth/node/duplicate checked and discarded. The new
public types remain distinct from OAuth token error types because their
registries and allowed members differ.

### Retain the minimum and redact by construction

The response owns only byte count, exact code, and optional description.
Strings use `Zeroizing<String>`. Debug reports count, known kind, and
description presence; public errors remain fieldless and static. No Clone,
Display, Serde, raw JSON, parser cause, navigation, or network surface is added.

## Risks / Trade-offs

- **Extension code is not semantically known** -> the exact value is retained
  but classified only as `Extension`; callers must not invent recovery policy.
- **Remote description can contain misleading text** -> access is explicitly
  named untrusted and no display implementation is provided.
- **Body-only parse can be used on the wrong HTTP outcome** -> docs and types
  explicitly disclaim status, authentication, provenance, and correlation;
  HTTP binding remains a later slice.
- **Parallel parser projections add code** -> they preserve distinct standards
  contracts while sharing the already-reviewed bounded scanner machinery.

## Migration Plan

Land as an additive unpublished API. Existing callers remain source compatible.
A later slice may bind it to exact HTTP error semantics, and downstream adoption
remains separately issue-linked. Rollback is one focused revert before release.

## Open Questions

None. HTTP binding, Authorization errors, recovery policy, deferred issuance,
and encryption remain explicit issue-first work.

## Provenance

- OpenID4VCI 1.0 Final section 8.3.1.2, HTML SHA-256
  `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- Oxid read-only evidence:
  `MediaNoxLabs/oxid@5ba38b9bbc9326c294b353daaf2a074eca18c22f`,
  `crates/adapters/openid4vci/src/lib.rs`, Apache-2.0.
- Lace ID Portal read-only evidence:
  `input-output-hk/lace-id-portal@804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`,
  `crates/issuer-http/src/error.rs`, `crates/issuer-services/src/credential.rs`,
  and `crates/issuer-services/src/error.rs`; no source or fixture is used and no
  license grant is relied upon.
- No production source or fixture is copied or transformed.
