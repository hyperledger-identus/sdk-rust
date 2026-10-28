# sd-jwt-vc-draft22 Specification Delta

## MODIFIED Requirements

### Requirement: conformance evidence is source-bound and migration-aware

The implementation SHALL bind draft-22 Appendix A, negative and boundary
vectors to the exact source hash, add property/fuzz/benchmark/differential
evidence, and preserve a deterministic trigger for a later draft or RFC.
SDK-TS legacy evidence SHALL remain migration-only and canonical output SHALL
remain `dc+sd-jwt`.

Differential evidence SHALL pin at least two independently maintained profile
engines by immutable version, source revision, package/archive integrity,
license, toolchain, dependency graph, advisory receipt, and source selector.
Their observations SHALL be reduced to stable semantic categories in a
bounded offline packet, and every disagreement SHALL be resolved against the
exact draft-22 hash rather than implementation consensus. No oracle SHALL
enter the production dependency graph.

#### Scenario: the IETF profile changes

- **WHEN** the current IETF draft number, publication state, normative source hash, protected type, media type or claim rules differ from the pinned manifest
- **THEN** the existing capability remains unchanged and a new issue/decision/profile version is required

#### Scenario: an oracle disagrees with draft-22

- **WHEN** an immutable oracle accepts or emits behavior contrary to the pinned draft or SDK resource contract
- **THEN** the divergence is recorded and does not weaken the SDK requirement

#### Scenario: an oracle does not expose a required surface

- **WHEN** the pinned public and tested API cannot express one neutral packet decision
- **THEN** its observation is `not-exposed`, is not counted as agreement, and remains visible in the qualification report

#### Scenario: the offline differential packet runs

- **WHEN** normal SDK tests execute the versioned packet without donor runtimes or network access
- **THEN** every Rust result matches the draft-22 normative result for protected identity, type claims, registered-claim placement, holder confirmation, Type Metadata and status references

#### Scenario: a legacy Identus credential is compared

- **WHEN** SDK-TS 8.1.4 behavior uses `vc+sd-jwt`
- **THEN** it remains labeled migration-only under issue #572 and cannot satisfy either independent canonical-oracle slot or change canonical Rust output
