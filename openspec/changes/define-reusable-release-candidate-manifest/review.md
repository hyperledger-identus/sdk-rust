# Local review

## Scope and identity

- Issue: `#382`
- Exact base: `96cf5f577b5f3585461d34589aad465297693af0`
- Reviewed implementation head: `a0ff1196b7d5959c65e70350298241eb61cea105`
- Scope: additive release-train registry, DID candidate descriptor and READMEs,
  offline policy checker, credential-free local assembler, mutation tests and
  factory integration.

## Findings

1. **Resolved — remote mutation needed an executable boundary.** The initial
   checker rejected publishing vocabulary, but that alone could not stop a
   later subprocess change. The final assembler routes every process through
   a fail-closed allowlist: read-only Git identity, Rust/Cargo version, the
   exact policy checker, Cargo lock/package, and Cargo check/test operations.
2. **No unresolved release-boundary finding.** Canonical DID manifests still
   inherit version `0.0.0` and `publish = false`; no workflow, registry,
   credential, tag, GitHub release or protected-setting surface changed. The
   existing crypto descriptor, publisher and immutable tag remain unchanged.
3. **No unresolved archive-security finding.** Source copying rejects links
   and non-allowlisted suffixes. Inspection bounds compressed bytes, expanded
   bytes and member count; rejects traversal, duplicate paths, links and
   special files; and validates normalized metadata plus exact internal
   versions before extraction.
4. **No unresolved reproducibility finding.** Two VCS-independent stages use
   a fresh credential-free Cargo home. Corresponding archive bytes must match
   before an atomic output rename. Failure cleans the hidden staging output.
5. **No unresolved architecture finding.** The generic closed train registry
   owns identity and lifecycle, while the DID assembler owns only the current
   train's packaging shape. Published crypto tooling is not refactored or
   coupled to unproven abstractions.

## Granularity

The checker (291 lines), assembler (496 lines after command hardening), and
mutation suite (224 lines) have separate responsibilities and no SDK runtime
dependency. Splitting the assembler before a second candidate train would
create an abstraction without two stable consumers. A future train must reuse
the registry contract and may extract proven archive helpers after comparing
both implementations.

## Residual limitations

- This is local candidate evidence, not crates.io availability, SemVer
  stability, platform coverage, API compatibility, certification or release
  authorization.
- Published `identus-core` and `identus-derive` plus public registry access are
  required to verify the extracted closure.
- Platform/slow evidence and publication activation remain separate M5 gates.

## Verdict

Approved locally with no unresolved blocking finding. Exact-head hosted CI is
required before protected merge.
