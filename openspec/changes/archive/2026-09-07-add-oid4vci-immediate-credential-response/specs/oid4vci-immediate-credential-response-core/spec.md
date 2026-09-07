# oid4vci-immediate-credential-response-core Specification

## ADDED Requirements

### Requirement: Immediate Final Credential Response parsing is explicit and bounded

The SDK SHALL expose positive `ImmediateCredentialResponseLimits` with
independent maximums for complete JSON bytes, JSON container depth, aggregate
JSON nodes, top-level member count, credential count, one credential-entry
member count, one retained exact credential JSON value, aggregate retained
credential JSON bytes, and decoded notification-ID bytes. Every maximum SHALL
be non-zero, depth SHALL NOT exceed the crate-wide configurable maximum, and
invalid limits SHALL fail with one fieldless static invalid-limits error.
Defaults SHALL be finite.

`ImmediateCredentialResponseCore::parse` SHALL accept only one complete JSON
object containing a present, non-empty, bounded `credentials` array. Every
array item SHALL be an object with a present `credential` member whose value is
a JSON string or object. Credential entries SHALL remain in wire order.

#### Scenario: immediate string and object credentials are accepted

- **WHEN** one bounded response contains ordered credential entries whose
  required values are strings or objects
- **THEN** parsing returns one immediate response with the same credential
  count and order

#### Scenario: required response shape is absent or invalid

- **WHEN** the root is not an object, `credentials` is absent or empty, an item
  is not an object, or one required credential value is absent or has another
  JSON kind
- **THEN** parsing fails with the corresponding fieldless invalid-response or
  invalid-credential error and returns no partial response

#### Scenario: independent parser and retention limits are exceeded

- **WHEN** limits are zero or unsupported, or response bytes, depth, nodes,
  top-level members, credential count, entry members, one credential value,
  aggregate credential bytes, or notification ID exceeds its limit
- **THEN** parsing fails with the corresponding static limit error before
  returning a response

### Requirement: Credential values are opaque, lossless and redaction-safe

Every `IssuedCredential` SHALL own the exact validated JSON slice of its
`credential` value in zeroizing storage and expose it only through an
explicitly sensitive accessor. It SHALL expose
`CredentialValueKind::{String,Object}`. A string credential SHALL additionally
own and expose its exact decoded string value through a sensitive accessor; an
object SHALL return no decoded string. Neither value SHALL implement `Clone`,
`Display`, Serde or expose mutable bytes.

`Debug` for the response and credential SHALL report only non-sensitive byte
lengths, count, kind and optional-presence metadata. Direct and bridged errors
SHALL be fieldless and SHALL NOT retain input, offsets, parser causes or
credential/notification data. The caller SHALL remain responsible for erasing
its input allocation.

#### Scenario: exact credential spelling and decoded string are retained

- **WHEN** a string credential uses JSON escapes and an object credential uses
  significant whitespace or numeric spelling
- **THEN** each exact JSON accessor returns the original value slice byte for
  byte, the string accessor returns the decoded semantic string, and kinds are
  reported without format interpretation

#### Scenario: diagnostics remain redacted

- **WHEN** response/credential `Debug` and every direct/bridged error are
  formatted with unique credential, notification and extension canaries
- **THEN** no canary, exact JSON, decoded value, parser cause or input fragment
  is present in diagnostics

### Requirement: Immediate parsing follows Final branch and extension rules

The parser SHALL reject a top-level `transaction_id` with a dedicated
unsupported-deferred error regardless of member order. It SHALL reject any
top-level `interval` and SHALL accept `notification_id` only as a non-empty
bounded opaque string on the immediate branch. Unknown unique top-level and
credential-entry members SHALL be fully validated within all structural limits
and discarded. Duplicate names in any object SHALL fail with the crate-wide
duplicate-property error.

Success SHALL prove only bounded unencrypted immediate JSON body syntax. It
SHALL NOT prove HTTP status/media/cache behavior, endpoint or issuer provenance,
transport authenticity/confidentiality, request correlation, error/deferred or
encryption semantics, credential format validity, cryptographic verification,
schema/status/trust policy, notification execution, consent, storage,
disclosure, retry or replay safety. No dependency, feature, target, unsafe code,
consumer, chain or product mutation SHALL occur, and Rust 1.85,
browser-WASM, Android ARM64 and iOS ARM64 portability SHALL remain green.

#### Scenario: deferred branch is distinguished from malformed input

- **WHEN** a syntactically valid response contains `transaction_id`
- **THEN** immediate parsing fails with the unsupported-deferred error rather
  than accepting it or reporting credential validity

#### Scenario: bounded extensions remain interoperable

- **WHEN** unique unknown members contain arbitrary valid JSON within all
  structural and member limits
- **THEN** parsing accepts and discards them without changing retained
  credential order or values

#### Scenario: duplicate or misplaced Final members fail

- **WHEN** any object repeats a member or an immediate response contains
  `interval`
- **THEN** parsing fails without exposing the repeated or misplaced value

#### Scenario: portable dependency gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain and full Nix
  gates run
- **THEN** the response core passes without dependency, feature, target or
  downstream drift
