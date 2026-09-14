# Pre-implementation review

- **Issues:** #271 umbrella; #278 credentials delivery
- **Exact base:** `353030a7f263b9a1fba9deac0312ed228e61d761`
- **Review scope:** planning artifacts only
- **Result:** PASS
- **Blocking findings:** none

## Semantic and architecture review

The proposal, ADR, delta specification, design, research, constraints, tasks,
and golden describe one credentials-only internal refactor. The selected shape
is a crate-private compile-time record and five domain-cohesive catalogues. It
adds no shared crate, public trait, derive, build script, dependency, feature,
runtime parsing, unsafe/native code, protocol behavior, or consumer work.

The public enums and constants stay explicit. Only private exhaustive routing
and the duplicated projection/display implementations may change. This keeps
domain ownership in `identus-credentials` and makes a future workspace-wide
abstraction an independently researched decision rather than an implicit
precedent.

## Compatibility review

The plan preserves all 44 `CredentialError` variants and public code constants
plus all three `CredentialVerificationError` variants. It explicitly preserves
variant order, attributes, derives, documentation, re-exports, constant
visibility and paths, method signatures, trait implementations, error-source
behavior, and the absence of Serde/binding surfaces.

The two bridge constness contracts remain intentionally different:
`CredentialError::to_identus_error` remains non-const and
`CredentialVerificationError::to_identus_error` remains `const`. Local display
and public message are stored and compared independently because current values
are intentionally not always equal.

## Golden review

`golden/credentials-error-contract-v1.csv` contains exactly 47 data rows: 44
for `CredentialError` and three for `CredentialVerificationError`. Every row
has the fixed 11-column schema, and enum/variant identities and stable error
codes are unique. Its SHA-256 is
`6148a00b22bdb8551d4df9369c7a9fcf819e1cf1c2654edc8654feef82227c7c`.

The fixture was captured before implementation from the exact base and is an
independent oracle. Implementation must copy the exact bytes to the stable
credentials test-fixture path and may not regenerate or alter either copy.

## Security and scope review

The contract permits only SDK value types and `&'static str`, so it cannot
capture runtime or caller-controlled material. Planned canary and full-variant
tests preserve redaction and `Error::source() == None`. The golden contains
only static diagnostic metadata and no credentials, identifiers, endpoints,
causes, or secrets.

OID4VCI issue #7, resource issue #168, other crates, downstream repositories,
publication, custody, wire models, and new error behavior are expressly out of
scope. Any need for one of them stops this slice and requires a new decision.

## Implementation entry criteria

Implementation may start only after the planning-only signed+DCO commit exists
and its durable preimplementation receipt validates. Public API or golden drift,
a new dependency, or pressure to centralize the catalogue is a blocker rather
than an implementation detail.

## Post-implementation exact-diff review

- **Reviewed head:** `16687bc41d437a4d7d2911ae4e72710d7185933e`
- **Architecture result:** CLEAR
- **Adversarial result:** CLEAR
- **Unresolved findings:** none

The implementation preserves the planned private five-catalogue architecture,
all 47 exact behaviors, both bridge constness contracts, public API, dependency
inventory, redaction, and source semantics. The enum routers are wildcard-free
and the same macro input also produces the exhaustive test inventories.

Review initially found that a copied fixture could drift with its planning
source, that a manually duplicated test inventory was not compile-exhaustive,
and that Nix cleaned sources omitted the stable CSV. Remediation bound both
copies to an immutable hash and exact provenance, generated router and test
inventory from one list per enum, made the Rust test hermetic, and retained only
the stable CSV in the Nix Rust source filter.

Adversarial follow-up then found active/archive ambiguity and symlink aliases at
leaf and intermediate path components. The checker now validates the active
change tree before archive fallback, enumerates every matching archive,
requires exactly one complete regular planning copy, rejects every symlinked
component, and exercises 17 mutation states. Both independent reviewers
approved the final exact head with no remaining finding.
