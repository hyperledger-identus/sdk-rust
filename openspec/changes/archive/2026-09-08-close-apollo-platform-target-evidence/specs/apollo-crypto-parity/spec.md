## ADDED Requirements

### Requirement: Portable-target parity closes on one policy-bound receipt

The Apollo parity ledger SHALL record one successful slow-workflow receipt for
the exact SDK closing revision. It SHALL name the Rust toolchain, portable
package and feature surface, support-policy source and exactly the WASM, iOS
and Android compile targets. Each portable target row SHALL match the
support-policy target ID, `compile-checked` tier, named target-specific gate,
packages, features, limitation, common closing revision and Actions run.

#### Scenario: Host-only evidence substitutes for a portable gate

- **WHEN** a portable row names `fast`, a host build or another target's gate
- **THEN** the validator SHALL fail and identify the target/gate mismatch

#### Scenario: Target evidence is stale or unrelated

- **WHEN** a portable row uses another revision, a non-success conclusion or a
  URI other than the declared GitHub Actions closing run
- **THEN** the validator SHALL fail before the parity report can claim closure

#### Scenario: Support policy changes after the closing receipt

- **WHEN** a portable target's toolchain, packages, features, tier, gate or
  limitation differs from the recorded closure
- **THEN** validation SHALL fail until a new exact target receipt is recorded

### Requirement: Human target report exposes evidence and limitations

The deterministic Markdown report SHALL show the closing revision, Rust
toolchain, portable packages, features and slow-run link. Every target row
SHALL show its evidence revision and URI. It SHALL continue to distinguish
compile-only WASM/mobile evidence from runtime, packaging and language-binding
support.

#### Scenario: Reviewer renders the target report

- **WHEN** the reviewer renders the current parity manifest
- **THEN** the output SHALL contain the exact portable closing revision and
  independently openable slow-run link
- **AND** WASM, iOS and Android SHALL remain labeled `compile-checked`
- **AND** language bindings SHALL remain `not-supported`
