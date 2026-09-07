## ADDED Requirements

### Requirement: Deferred response parsing is complete and bounded

The SDK SHALL expose positive `DeferredCredentialResponseLimits` with
independent maxima for complete JSON bytes, JSON depth, aggregate JSON nodes,
top-level response members, decoded transaction identifier bytes, and exact
interval number lexeme bytes. Every maximum SHALL be non-zero and configurable
depth SHALL NOT exceed the repository-wide supported JSON depth.

`DeferredCredentialResponseCore::parse` SHALL accept exactly one complete
duplicate-safe top-level JSON object under those limits. It SHALL reject empty,
non-object, malformed, trailing, excessive, or duplicate-decoded-name input.
Unknown unique members SHALL be traversed under the complete structural limits
and discarded without semantic access.

#### Scenario: bounded deferred object succeeds

- **WHEN** one complete object has its required deferred members and bounded
  unique extensions
- **THEN** the parser returns only the deferred core values and response size

#### Scenario: malformed or excessive structure fails closed

- **WHEN** bytes, depth, nodes, top-level members, duplicate names, syntax, or
  trailing input violate the configured contract
- **THEN** parsing returns the corresponding static error without partial state

### Requirement: Deferred members are exact and unambiguous

The core SHALL require a non-empty bounded string `transaction_id` and a
positive JSON-number `interval`. It SHALL reject `credentials` and
`notification_id` in this branch. Missing either required member, any forbidden
member, a wrong type, empty identifier, or excessive retained value SHALL fail
closed.

`DeferredTransactionId` SHALL own the decoded identifier in zeroizing storage
and expose it only through an explicitly sensitive accessor.
`DeferredCredentialInterval` SHALL own the exact JSON-number lexeme in
zeroizing storage. A number SHALL be positive only when it has no leading minus
and its mantissa contains at least one non-zero digit. Integer, fractional, and
exponent forms meeting that rule SHALL be accepted exactly; positive zero,
negative zero, negative numbers, and non-number values SHALL be rejected.

No floating-point or integer conversion SHALL occur. The core SHALL assign no
duration, rounding, cap, scheduling, retry, backoff, freshness, invalidation,
or polling semantics.

#### Scenario: positive numeric forms survive exactly

- **WHEN** interval is a positive valid integer, fractional, or exponent JSON
  number within its lexeme bound
- **THEN** its exact source lexeme and decoded transaction ID are retained

#### Scenario: zero negative and non-number forms fail

- **WHEN** interval is mathematically zero, has a leading minus, is not a JSON
  number, is malformed, or exceeds its independent byte bound
- **THEN** parsing returns a static interval error without numeric conversion

#### Scenario: immediate and deferred branches cannot mix

- **WHEN** a deferred object also contains `credentials` or `notification_id`
- **THEN** parsing fails with a static branch-ambiguity error

### Requirement: Deferred state remains least-authority and portable

Success SHALL prove only caller-supplied JSON body syntax. It SHALL NOT prove
HTTP status/media, transport confidentiality or authenticity, endpoint or
Issuer provenance, request correlation, transaction validity or freshness,
authorization, polling safety, response encryption, credential validity or
trust, notification authority, storage, or product policy.

Every new error SHALL be fieldless and map to a stable static `oid4vci.*`
code/message. No transaction ID, interval lexeme, extension, parser cause, or
canary SHALL appear in direct, Debug, or bridged diagnostics. No dependency,
manifest, lockfile, feature, unsafe-code, consumer, chain, or product change
SHALL occur, and Rust 1.85, browser-WASM, Android ARM64, and iOS ARM64
portability SHALL remain green.

#### Scenario: sensitive values stay out of diagnostics

- **WHEN** input and retained values contain unique canaries
- **THEN** core Debug and all direct or bridged errors reveal none of them

#### Scenario: portable dependency gates remain green

- **WHEN** focused, workspace, factory, target/MSRV, supply-chain, and full Nix
  gates run
- **THEN** the deferred core passes without dependency or target drift
