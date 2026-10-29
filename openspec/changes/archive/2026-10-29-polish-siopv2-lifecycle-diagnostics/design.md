# Design

## Lifecycle ownership

`SiopFlow` remains a finite enum with one event receipt only in states that need
it. Each transition keeps its existing three-way rule: the same receipt in the
same resulting state is idempotent, a different receipt for that state
conflicts, and a transition incompatible with the current state is invalid.
Advancing from `ResponseReady` to `Delivered` or `OutcomeUnknown` deliberately
replaces the approval receipt. The core gains no retained history.

## Diagnostic mapping

Request verification returns `WalletNonceMismatch` only when retrieved outer
wallet-nonce evidence differs from the signed claim. Issuer/client and retrieved
client correlation remain `ClientBindingMismatch`.

Response construction uses private exhaustive mapping boundaries at
`ProtectedHeader::new`, `JwsSigningInput::new`, and `sign_with`. The first
boundary distinguishes the caller's already validated DID relationship from a
JOSE header/string bound: the latter is a response-size failure, not an invalid
DID key identifier. The signing-input boundary preserves explicit protected
header, payload, compact, signature, and arithmetic size failures. The signing
boundary preserves signer rejection, signer unavailability, and algorithm
mismatch. All other encoding or invariant failures map to
`ResponseEncodingFailed`; dependency diagnostics are never surfaced.

## Private cleanup

A private bounded-string predicate removes repeated conditions but retains the
required-array check before optional state and wallet nonce checks. HTTPS and
form helpers remain protocol-private. The unverified request documents the
intentional reparse: the JOSE parse owns non-zeroizing compact, payload, and
signature buffers, while the public state retains one zeroized compact value
and only cloned bounded public header evidence until verification.

## Evidence

Focused tests force failures at each of the three JOSE construction boundaries
and freeze every stable category and current-state lifecycle rule.
E2E tests exercise retrieved nonce mismatch and complete request-to-response
behavior. Property tests retain state preservation and bound coverage.
Conformance vectors prove successful wire bytes remain unchanged. Fuzz and
benchmark evidence demonstrate finite work and no material regression; portable
builds prove the cleanup remains runtime-neutral.
