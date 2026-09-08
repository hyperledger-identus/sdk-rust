## Context

`identus-derive` currently turns a borrowed `&str` into a `String` before it
invokes a validated string newtype's caller-supplied validator. This is
observable only on rejection, but it defeats validators that use a byte ceiling
to avoid allocating hostile input. The workspace's two validated `String`
newtypes (`DidMethod` and `identus_core::Url`) already use `fn(&str)` validators.

`DidMethod` implements the W3C DID Core method-name grammar but has no
standalone resource ceiling and includes rejected caller text in its local
error. A full `Did` is already limited to 2,048 bytes. The largest method name
that can occur in such a DID is 2,042 bytes: the limit minus `did:` (4 bytes),
the method separator (1 byte), and a minimum one-byte method-specific id.

## Goals / Non-Goals

**Goals:**

- Make generated borrowed validated-string parsing reject before allocation.
- Require validated `String` newtype validators to accept `&str`.
- Bound standalone `DidMethod` values without rejecting a method representable
  in an accepted maximum-size bare DID.
- Make validator-produced method errors static and redaction-safe.
- Preserve owned construction, serde, registry and Registration behavior.

**Non-Goals:**

- Prevent allocation performed by a transport or serde before validation.
- Change owned `TryFrom<String>` or serde ownership behavior.
- Add a general configurable-length facility to the derive.
- Change the 2,048-byte bare-DID limit or W3C method grammar.
- Add a dependency or import method-specific/chain/product policy.

## Decisions

1. Generated `parse(&str)` and `FromStr::from_str` call the validator with the
   borrowed `&str`; they allocate exactly once and only after validation.
   `TryFrom<String>` and serde keep validating their already-owned `String`.
   This changes the validated string validator contract from `&String` or a
   deref target to `&str` for borrowed parsing. The repository has no
   `&String` validator, the crate is unpublished, and the narrow pre-release
   break makes the security property enforceable. Keeping `&String` would
   require allocating before validation; adding a second attribute would
   preserve an unsafe default and enlarge the macro API.

2. `DidMethod` exposes `MAX_DID_METHOD_BYTES = MAX_DID_BYTES - 6`, or 2,042.
   This derives the standalone ceiling from the enclosing DID contract rather
   than selecting an arbitrary protocol limit. A smaller operational limit
   would reject values the generic DID parser accepts; 2,048 would admit method
   names that can never fit in an accepted DID.

3. `validate_did_method` checks empty, then byte length, then ASCII grammar.
   The byte ceiling therefore bounds subsequent work and over-limit input has
   deterministic precedence. It uses byte predicates because the accepted
   grammar is ASCII. All errors created by this validator contain static detail
   rather than rejected input. Changing the public error enum is unnecessary
   and would be a broader compatibility break.

4. No third-party crate is adopted. This slice is one code-generation ordering
   correction, one derived constant and closed ASCII grammar already owned by
   the SDK. A dependency would add a cone and public-policy question without
   replacing either decision.

## Risks / Trade-offs

- [A downstream validated `String` newtype uses `fn(&String)`] → This
  pre-release macro contract will stop compiling; migrate its validator to
  `fn(&str)`. Repository consumers already meet the new contract.
- [Serde still allocates an oversized string before validation] → Retain
  `SDK-LIM-007` and require outer transport/deserializer byte limits.
- [The derived method ceiling changes if `MAX_DID_BYTES` changes] → Define
  the public constant from that source and test maximum enclosing-DID
  compatibility.
- [Static local errors reduce debugging detail] → Tests can inspect the
  rejected value separately; diagnostics remain safe for headless consumers.

## Migration Plan

Land the specification and ADR first, update macro expansion and DID method
validation, then run focused and workspace gates. Rollback reverts the focused
PR. A hypothetical downstream `&String` validator migrates mechanically to
`&str`; no wire or stored-data migration exists.

## Open Questions

None. The remaining outer-allocation and repository-wide resource audit stays
tracked by issue #168 and `SDK-LIM-007`.
