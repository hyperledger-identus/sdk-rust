## ADDED Requirements

### Requirement: Runtime-neutral split clock ports

The foundation capability SHALL provide bounded millisecond Unix,
monotonic and duration values plus independent object-safe `WallClock` and
`MonotonicClock` ports. Clock reads SHALL be injected and fallible; domain code
SHALL NOT read ambient system time. Monotonic values SHALL NOT claim a Unix
epoch or serialized cross-process identity.

#### Scenario: consumers inject only the time semantics they need

- **WHEN** a DID cache needs elapsed-time expiry and a credential verifier
  needs a civil timestamp
- **THEN** each SHALL depend on only its respective clock interface without an
  OS, async runtime or concrete time-library dependency

#### Scenario: time arithmetic fails closed

- **WHEN** millisecond duration addition overflows or an injected clock is
  unavailable
- **THEN** the operation SHALL fail deterministically through a stable,
  redaction-safe clock error without wrapping or substituting zero
