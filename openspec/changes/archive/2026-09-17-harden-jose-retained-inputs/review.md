# Local review

- **Review date:** 2026-09-17
- **Review angle:** public construction closure, resource bounds, wire
  compatibility, diagnostic redaction, dependency direction, and downstream
  migration
- **Scope:** implementation diff after planning head `d31a159652b3c9194e30facd3dcd363c67a43e34`
- **Result:** passed after the compatibility correction below

## Resolved finding

The first worker implementation required each standard-base64 `x5c` member to
round-trip through a canonical re-encoding. That would have rejected some
previously accepted but decodable wire values and exceeded issue #299's
construction-closure scope. The additional equality check was removed. The
accepted `x5c` contract remains non-empty, bounded, and decodable with standard
base64, matching the prior parser and builder behavior.

The first Nix closure also found non-canonical Taplo layout in the new
`archive-intent.toml`. The file was formatted and the exact `lint-toml`
derivation then passed.

## API and architecture review

- `JwsKeyId`, `JwsX5c`, and `Oid4vciProofJwtClientId` keep their storage
  private and expose no mutable, unchecked, or owning escape hatch.
- Every public retained-value entry is fallible. Builders revalidate values
  under their own possibly tighter limits, so construction under roomy limits
  cannot bypass a narrower protocol boundary.
- The private Serde staging types may temporarily own parsed values only inside
  the bounded parser path; conversion to a public `ProtectedHeader` reuses the
  same validation before return.
- Serialization and resolver dispatch borrow exact accepted values, preserving
  JSON, compact-token, and provider inputs.
- The enum alternatives and high-level builders remain cohesive with JOSE and
  OID4VCI. No generic bounded-value abstraction or cross-crate dependency was
  introduced for three profile-specific invariants.
- Workspace search found raw variant syntax only in compile-fail examples and
  private implementation matching/wrapping. Public source migration is explicit
  in ADR 0130 and occurs before publication at version `0.0.0`.

## Security, privacy, and compatibility review

Length, control-character, cardinality, and base64-decodability checks occur
before a successful typed return. Static errors and redacted `Debug` output do
not expose key IDs, client IDs, certificates, claims, or compact tokens. The
change adds no unsafe code, secret material, algorithm, trust decision, network
access, native dependency, feature, or lockfile change.

Caller or transport allocation before SDK entry remains honestly disclosed;
this API cannot retroactively bound an already-owned `String` or `Vec`. Only the
now-closed JOSE retained-enum clause is removed from `SDK-LIM-007`; the
Multihash and outer-allocation limitations remain.

No unresolved blocking finding remains.
