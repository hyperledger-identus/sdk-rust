# Design

## State transition

`UnverifiedRequestObject` continues to parse protected metadata and
`VerifiedRequestObject` continues to prove signature plus correlation. A new
consuming async transition validates security claims and invokes the replay
port exactly once. Success returns `RequestObjectSecurityEvidence`, which owns
the same redacted payload lineage and becomes the only public entry to DCQL and
authorization routing.

## Claims and policy

The existing streaming scanner first establishes one bounded, duplicate-free
JSON object. The security transition then captures `aud` as one non-empty
string or a singleton string array, mandatory integral `iat`, optional integral
`exp` and `nbf`, and optional non-empty `jti` from that verified payload.
Wrong types, empty values, arithmetic overflow, fractional NumericDate, or
resource-limit violations fail closed. The sole audience must equal the
discovery-bound expected value byte-for-byte.

The policy owns an expected audience, finite maximum age, and finite allowed
skew. Validation uses checked integer arithmetic: expired, not-yet-valid,
issued-in-future, and stale inputs produce distinct static categories.

## Atomic replay boundary

The guard receives borrowed, explicitly sensitive evidence after local policy
validation: bounded `jti` when present, the exact verified payload, client ID,
audience, issued-at, and optional signed expiry/not-before values. It atomically
accepts or rejects the event. The SDK neither derives a storage key nor
dictates persistence. Guard unavailability and replay rejection are distinct
redacted failures.

## Evidence

Clean-room Final/RFC vectors cover static and dynamic audiences, stale/future
times, `exp`/`nbf`, audience mismatch, replay rejection, and redaction. E2E tests
exercise invocation through JWS verification to typed routing. Deterministic
generated property cases sweep age/skew/time boundaries, and a stateful test
guard proves check-and-record behavior; fuzzing covers security-claim scanning
and policy. A directional benchmark measures the full local transition.
Portable builds prove the port remains target-neutral.
