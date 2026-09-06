# oid4vci-pre-authorized-token-request Specification

## Purpose
TBD - created by archiving change add-oid4vci-pre-authorized-token-request. Update Purpose after archive.
## Requirements
### Requirement: Mandatory Pre-Authorized Token Request fields are constructed deterministically

The SDK SHALL consume one `CredentialOfferWithPreAuthorizedTokenInput` and one
`PreAuthorizedTokenRequestLimits` to create a `PreAuthorizedTokenRequest` only
when all requirements in this capability succeed.

The request SHALL contain exactly one `grant_type` with the Final
Pre-Authorized Code grant identifier, exactly one `pre-authorized_code` copied
from the offer, and exactly one `tx_code` if and only if the predecessor bound
one. Their deterministic order SHALL be `grant_type`, `pre-authorized_code`,
then optional `tx_code`. The spelling of the second wire parameter SHALL be
`pre-authorized_code`.

Success SHALL prove only deterministic mandatory-field construction from the
validated predecessor. It SHALL NOT prove client eligibility, authentication,
endpoint trust/reachability, server acceptance, Transaction Code correctness,
single use, replay safety, or successful issuance.

#### Scenario: no-code request contains two mandatory fields

- **WHEN** the predecessor binds no Transaction Code and construction succeeds
- **THEN** the body contains only grant type and Pre-Authorized Code, each once
  and in deterministic order

#### Scenario: requested Transaction Code is included exactly once

- **WHEN** the predecessor binds one Transaction Code and construction succeeds
- **THEN** `tx_code` follows the two mandatory fields exactly once

### Requirement: Token Request form encoding and output are exact and bounded

Parameter names and values SHALL use UTF-8 `application/x-www-form-urlencoded`
encoding compatible with RFC 6749 Appendix B. ASCII alphanumerics and `*`,
`-`, `.`, and `_` SHALL remain literal, ASCII space SHALL become `+`, and every
other UTF-8 octet SHALL use uppercase `%HH` encoding.

`PreAuthorizedTokenRequestLimits` SHALL accept only a positive maximum encoded
body byte count and SHALL default to 16,384 bytes. Construction SHALL compute
the exact encoded size with checked arithmetic before allocating the body.
Arithmetic overflow or output above the configured maximum SHALL fail with one
static oversized-request error; a zero maximum SHALL fail distinctly.

#### Scenario: Appendix B characters use canonical octets

- **WHEN** secret input contains space, percent, ampersand, plus, pound, and
  euro characters
- **THEN** the encoded value contains
  `+%25%26%2B%C2%A3%E2%82%AC`

#### Scenario: exact limit succeeds and excess fails before allocation

- **WHEN** the exact encoded body length equals the positive configured limit
- **THEN** construction succeeds
- **AND WHEN** the limit is one byte smaller
- **THEN** construction fails with the static oversized-request error

### Requirement: Headless transport access is explicit and redaction-safe

`PreAuthorizedTokenRequest` SHALL own and expose the already validated Token
Endpoint by reference, static `POST` method and
`application/x-www-form-urlencoded` media-type guidance, exact body byte count,
and Transaction Code presence. It SHALL expose the exact body only through an
explicitly sensitive accessor documented against logging, URLs, telemetry,
caching, generic serialization, and long-lived storage.

Construction SHALL consume and drop the predecessor after encoding, erasing
its zeroizing secrets. The encoded body and owned endpoint SHALL zeroize on
drop. The request SHALL have no Clone, Display, or Serde contract. Its Debug
representation and every new fieldless error, stable `oid4vci.*` code/message,
and core-error bridge SHALL echo no body, Pre-Authorized Code, Transaction
Code, endpoint, issuer, description, or retained JSON.

The transition SHALL add no new dependency, manifest, lockfile, feature,
parser, or existing-limit change and SHALL preserve Rust 1.85, browser-WASM,
Android ARM64, and iOS ARM64 portability. HTTP execution, async, authentication,
response parsing, crypto, DID, storage, chain, product, and consumer code SHALL
remain outside its dependency cone.

#### Scenario: explicit transport access preserves secret hygiene

- **WHEN** a caller inspects endpoint, method, media type, length, presence,
  Debug, Display errors, or bridged errors
- **THEN** raw secret material is available only from the sensitive body
  accessor and appears in none of the other surfaces

#### Scenario: portable policy-neutral gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** request construction passes without dependency-cone, feature,
  target, or downstream drift
