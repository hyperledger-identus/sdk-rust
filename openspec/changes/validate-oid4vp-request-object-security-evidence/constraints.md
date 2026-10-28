# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/544
Constraint blockers: none

## Existing entries affected

SDK-ARCH-001 through SDK-ARCH-003 keep dependencies inward and effects at the
edge. SDK-SEC-003 and SDK-LIM-007 require finite hostile-input work. ADRs 0158
and 0159 define the current JAR and JOSE ownership boundaries. The OpenID4VP
matrix rows owned by #544 are affected.

## Introduced or changed constraints

- Security validation consumes verified signature/correlation evidence before
  typed DCQL or authorization-routing successors are available.
- Expected audience is an explicit caller-owned discovery value. Static mode
  is exactly `https://self-issued.me/v2`; dynamic mode is the exact discovered
  issuer. The ignored Request Object `iss` claim is never authority.
- `aud` and integral `iat` are mandatory. Integral `exp` and `nbf` are honored
  when present. Maximum age and non-negative skew are finite public policy.
- Time comes only from `identus_core::WallClock`; there is no ambient system
  clock, executor, network, filesystem, randomness, or global state.
- Replay acceptance is an atomic caller-owned operation performed after all
  local validation and before evidence is returned. An unavailable or rejecting
  guard fails closed. No check-then-store split is exposed.
- The full verified payload remains under existing byte/depth/node/member/string
  bounds. Security claim strings and audience arrays share those limits;
  duplicate top-level claims fail closed.
- Public diagnostics and `Debug` never render payload, audience, client ID,
  nonce, JWT ID, key material, or dependency diagnostics.
- No new dependency, unsafe code, native library, protocol runtime, persistence,
  transport, trust store, or consumer repository mutation enters the slice.

## Introduced or changed limitations

NumericDate values are accepted only as finite integral seconds. Replay
durability, distributed consistency, expiry strategy, and storage remain caller
responsibilities. Security evidence proves policy acceptance for one verified
object; it does not prove verifier-key authorization, trust, consent, credential
validity, presentation generation, or transport safety.

## Consumer and product impact

Rust consumers must provide trusted discovery evidence, a clock, and an atomic
replay guard before typed routing. Product trust stores, persistence, policy,
telemetry, and adapter DTOs remain consumer-owned. No downstream repository is
mutated by this slice.

## Activation and rollback

Before publication, rollback removes the additive policy/port/state and restores
typed successors to signature evidence. After adoption, consumers migrate by
calling the security transition with their discovered audience, clock, and
atomic replay guard; unsafe legacy behavior is not retained as a compatibility
mode. Activation requires issue-bound preflight, focused and portable evidence,
exact-head independent CLI review, signed/DCO commits, and protected green CI.

## Evidence

Issue #544 and parent #541 authorize the slice. Research sources and commands
are recorded in `research.md`; implementation receipts are recorded only after
the corresponding command succeeds.
