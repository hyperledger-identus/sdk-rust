# Semantic contract review

## Scope reviewed

- Issue #123 and exact base `b22ba18cde2b6e420499ee8922ab1bd2f3e2ad1e`.
- OpenID4VCI 1.0 Final sections 3.5, 4.1.1, and 6.1 at immutable HTML
  SHA-256 `f123c3178cacd27688b15b762098a045e9eb35eccfe2f5f18a357c3815e06ba7`.
- Existing grant, server-binding, error, limit, and redaction contracts.
- Read-only Oxid and Lace evidence listed in issue #123.

## Findings

1. **Resolved — avoid overstating Transaction Code validity.** The draft
   boundary proves exact input presence and resource safety only. Advertised
   mode/length remain UI guidance; the Authorization Server remains the code
   validator.
2. **Resolved — erase failed input.** The design requires wrapping the owned
   string before empty/size checks so rejection paths zeroize it.
3. **Resolved — avoid a premature public secret accessor.** The prepared state
   exposes presence only; later in-crate request construction may borrow raw
   material privately.
4. **Resolved — keep protocol scope bounded.** Token serialization, HTTP,
   client identity/authentication, responses, replay, and downstream adoption
   are explicit non-goals.

## Decision

The proposal, design, capability requirements, program replacement, and task
map are semantically complete, objectively testable, reversible, and within
the standing mandate. No unresolved blocker remains before implementation.
