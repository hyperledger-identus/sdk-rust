## ADDED Requirements

### Requirement: First-party unsafe Rust is forbidden by inherited compiler policy

The root workspace SHALL set Rust lint `unsafe_code` to exact `forbid`, and
every supported workspace package SHALL explicitly inherit the workspace lint
table. The setting SHALL apply to every first-party library, binary,
integration-test, example, benchmark, build-script and proc-macro target built
by the repository's Cargo gates. Existing crate-root forbids MAY remain as
defense in depth but SHALL NOT be the uniform enforcement mechanism.

#### Scenario: First-party target contains unsafe Rust

- **WHEN** any supported opted-in first-party target compiles an unsafe block,
  unsafe declaration or other construct governed by rustc `unsafe_code`
- **THEN** compilation fails under `-F unsafe-code` without relying on reviewer
  source inspection

#### Scenario: Workspace policy is locally weakened

- **WHEN** source attempts to lower the inherited `forbid` with an `allow`
- **THEN** rustc rejects the override or the unsafe construct remains an error

### Requirement: Configuration drift and compiler behavior have independent evidence

The conformance crate SHALL derive the current package set from workspace crate
manifests and SHALL fail if the root policy is absent/not exact `forbid` or any
package omits workspace lint inheritance. A dependency-free, offline,
process-isolated compile-fail fixture SHALL independently prove Rust 1.98.1
propagation across library, binary, integration-test, example, benchmark,
build-script and proc-macro implementation targets and unsafe proc-macro output
compiled in a consumer. The fixture SHALL require both non-zero compilation and
the unsafe-code lint identifier without snapshotting full diagnostics.

#### Scenario: New package omits lint inheritance

- **WHEN** a workspace crate manifest is added without exact
  `[lints] workspace = true`
- **THEN** the conformance test fails and identifies that manifest/package

#### Scenario: Cargo stops propagating a target lint

- **WHEN** a synthetic target containing unsafe Rust unexpectedly compiles or
  fails for an unrelated reason without the unsafe-code lint identifier
- **THEN** the behavioral conformance fixture fails and identifies the target
  class

#### Scenario: Safe workspace remains supported

- **WHEN** the actual workspace and all supported feature/target lanes compile
- **THEN** the inherited policy introduces no API, wire, dependency, MSRV,
  target or runtime change

### Requirement: Unsafe exceptions are separate material decisions

The default exception set SHALL be empty. An unsafe exception SHALL require a
new issue, dedicated safety ADR and indexed exception record naming the exact
crate/target/source scope, inability to use safe Rust or an audited dependency,
safety invariants/evidence, owner and specialist reviewer, security/maintenance
cost, review/exit trigger, activation and rollback. The implementation SHALL
reproduce unrelated workspace lints for only the named package, retain package
boundary `deny(unsafe_code)`, permit only the recorded inner scope and update
the conformance guard atomically. It SHALL NOT weaken unrelated packages or the
visible base `SDK-SEC-001` prohibition.

#### Scenario: Unsafe implementation is proposed without an exception record

- **WHEN** a package stops inheriting the workspace forbid or source attempts
  to allow unsafe without the complete directed exception change
- **THEN** conformance or compiler gates fail before integration

#### Scenario: Exception is later revoked

- **WHEN** the recorded exit trigger is met or an invariant cannot be proven
- **THEN** the scoped unsafe code and manifest deviation are removed and the
  package returns to exact workspace lint inheritance

### Requirement: The assurance claim remains first-party and evidence-bounded

The unsafe-code prohibition SHALL apply to first-party SDK source. It SHALL NOT
be represented as linting external dependency internals, proving logical or
side-channel safety, or exercising unsupported feature/target combinations.
Dependency unsafe/native code SHALL continue to be evaluated in dependency
research and integration ADRs.

#### Scenario: Adopted dependency contains reviewed unsafe code

- **WHEN** Cargo compiles an external dependency under dependency cap-lints
- **THEN** the first-party gate does not claim to reject that source, and its
  unsafe/native posture remains explicit dependency-decision evidence
