# ADR 0093: validate borrowed newtype strings before allocation

- **Status:** Accepted for implementation
- **Date:** 2026-09-09
- **Issue:** [#207](https://github.com/hyperledger-identus/sdk-rust/issues/207)
- **Parent:** [#168](https://github.com/hyperledger-identus/sdk-rust/issues/168)
- **Decision authority:** SDK resource-bound and redaction guardrails

## Context

The validated `String` path emitted by `identus-derive` clones a borrowed
`&str` before invoking its validator. This makes a validator's byte ceiling too
late to prevent an attacker-sized rejection allocation. `DidMethod`, one of
two production users, also has no standalone ceiling and embeds rejected text
in its local error. A bare `Did` is already bounded to 2,048 bytes.

All repository validated `String` newtypes use `fn(&str)` validators. Supporting
a validator that accepts only `&String` would require preserving the clone.
The macro is unpublished at version `0.0.0`, so the source compatibility cost
can be made explicit before a release contract exists.

## Decision

Validated `String` `parse(&str)` and `FromStr` SHALL call their `&str`
validator before allocating and construct the owned value only after success.
Owned `TryFrom<String>` and serde keep their existing already-owned behavior.
Consequently, validated `String` validators SHALL accept `&str`; a downstream
`fn(&String)` validator must migrate.

Expose `MAX_DID_METHOD_BYTES` as 2,042, derived from `MAX_DID_BYTES` minus the
four-byte prefix, separator and minimum one-byte method-specific id. Check
empty, then byte length, then the ASCII grammar. Validator-created method errors
use static detail and never include rejected text.

Do not add a dependency. The change is a code-generation ordering correction,
one derived constant and an existing closed ASCII grammar. A crate would not
replace these policy decisions and would widen the dependency cone.

## Consequences

- Invalid borrowed strings can be rejected without cloning them.
- Every method name representable in an accepted maximum-size bare DID remains
  accepted by `DidMethod`.
- Current workspace source, wire, registry and Registration behavior remains
  compatible for accepted input.
- A hypothetical external `fn(&String)` macro validator stops compiling and
  must accept `&str`; this is an intentional pre-release compatibility break.
- Serde and transports may allocate before validation, so `SDK-LIM-007` and
  outer input-budget responsibility remain.
- No dependency, target, chain, product or certification claim changes.

## Alternatives rejected

### Retain `&String` compatibility

This necessarily preserves allocation before borrowed validation and defeats
the security objective.

### Add a second borrowed-validator attribute

No current consumer needs different borrowed and owned validation semantics.
The option would enlarge the macro contract and preserve an unsafe default.

### Select a smaller operational method ceiling

It would make a method extracted from an otherwise accepted generic DID fail
standalone validation. Method-specific profiles may impose smaller downstream
limits without changing the chain-neutral primitive.

### Adopt a validation crate

No crate can correct this macro's allocation order or select the SDK's
enclosing-DID compatibility budget. The existing grammar is smaller and more
auditable than a new dependency boundary.

## Verification and rollback

Macro tests prove the validator observes borrowed input before ownership;
method tests cover exact and one-over limits, precedence, all constructors,
redaction, maximum-size DID and Registration compatibility. Full factory,
workspace, Nix and hosted gates remain required. Revert the focused PR to
restore the old behavior; no wire or stored-data migration exists.
