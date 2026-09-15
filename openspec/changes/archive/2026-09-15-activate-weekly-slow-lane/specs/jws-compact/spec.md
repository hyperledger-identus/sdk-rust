## MODIFIED Requirements

### Requirement: Reproducible bounded JWS fuzz campaigns

The repository SHALL expose one documented JWS command interface for corpus
replay, fixed-seed smoke and time-boxed soak through the pinned sanitizer
compiler, runner and runtime. PR/push smoke SHALL fix the seed, run count, input
ceiling, per-input timeout, memory ceiling, corpus reload and worker count.
Native hosted weekly/manual soak and exact local reproduction SHALL remain
independently bounded. The JWS PR/push workflow SHALL run for changes to the
root dependency declarations and every local crate in JOSE's dependency cone,
plus the flake entrypoint and complete Nix configuration surface used by the
campaign.

An original reviewable corpus and dictionary SHALL cover the RFC example,
independently reconstructed Oxid/Lace shapes and existing structural,
canonicality, header and limit rejection families without donor fixture bytes.
Exact-text seeds MAY use a documented harness-only `text:` transport that
removes one repository line ending; unprefixed arbitrary bytes SHALL remain
unchanged.
The corpus SHALL seed at least one accepted derived-limit suffix and complete
encoded `kid`, public/private `jwk`, ambiguous-reference, and valid/invalid
`x5c` header families. A documented `limits:` prefix MAY occupy the ten limit
selection bytes and remove one repository line ending from its suffix.
Failures SHALL be retained as untrusted artifacts, minimized and promoted to
committed corpus plus a deterministic regression before merge. Execution time
MAY be recorded but SHALL NOT become a machine-specific threshold.

#### Scenario: ordinary JWS fuzz CI is repeatable

- **WHEN** the same revision runs the pull-request JWS fuzz gate
- **THEN** the target receives the same seed, run count and resource limits in
  the pinned Nix shell and terminates deterministically

#### Scenario: longer search remains bounded and diagnosable

- **WHEN** a hosted weekly/manual or locally reproduced soak finds a sanitizer
  or invariant failure
- **THEN** it stops within the documented envelope and preserves the input for
  minimization without custom logging of its bytes
