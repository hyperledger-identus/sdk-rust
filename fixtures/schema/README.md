# Conformance Fixture Schemas

This directory contains machine-readable JSON Schema contracts for fixture
families under `fixtures/conformance/`.

The schemas are intentionally narrow at this stage. They enforce the common
policy from `docs/architecture/adr-fixture-schema-policy.md`: schema version,
source evidence, owner crate, expected outcome, and redaction policy. Protocol
crates can add behavior-specific checks as implementations land.

`identus-conformance` validates checked-in JSON fixtures against these schemas
in the default `cargo test --workspace` path.
