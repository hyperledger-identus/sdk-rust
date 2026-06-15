# Conformance Fixtures

This directory is the stable home for specification conformance fixtures.

Fixture schema rules are governed by
`docs/architecture/adr-fixture-schema-policy.md`. New fixtures must include a
schema version, source reference, owner crate, expected validation outcome, and
redaction policy unless a later ADR grants a specific exception.
Machine-readable fixture-family schemas live under `fixtures/schema/`.

The first checked-in layout is intentionally empty of protocol vectors. Each
mode directory is committed now so future protocol work has a durable place for
fixtures and the conformance test crate can enforce the layout.

Fixture policy:

- `static-model/`: model, parser, and negative-shape fixtures.
- `vector/`: deterministic input/output vectors.
- `transcript/`: protocol transcript replay fixtures.
- `interop/`: cross-implementation and reference-suite fixtures.
- `infrastructure/`: optional tests requiring Docker, services, ledgers, or
  device adapters.
