# ADR 0125: govern SDK input-resource boundaries by ownership

- **Status:** Accepted for implementation
- **Date:** 2026-09-16
- **Decision authority:** sponsor-directed issue
  [#168](https://github.com/hyperledger-identus/sdk-rust/issues/168)
- **Related:** ADR 0063, ADR 0115, issue
  [#297](https://github.com/hyperledger-identus/sdk-rust/issues/297),
  `SDK-SEC-003`, `SDK-LIM-007`
- **Assessed revision:** sdk-rust
  `66ec2b9b3a7ec35cf21ecc52cdca5bebed0b4d0d`

## Context

Focused deliveries already bound most retained and parsed SDK values, but no
single artifact mapped those boundaries across every implemented package.
`SDK-LIM-007` therefore continued to say the repository audit was incomplete.
That wording was safe but too broad to guide agents or consumers.

The audit also exposed two different classes that cannot truthfully be called
SDK-enforced. A typed constructor cannot prevent an owned value, serde model,
HTTP framework extraction, FFI bridge or JavaScript engine from allocating
first. A borrowed hash/sign/verify function or generic async/storage port has
no correct cross-protocol byte/time quota when it does not retain the input and
does not own the adapter.

Two avoidable gaps remained. BIP-39 input joined arbitrary word strings before
rejecting invalid structure and normalized/PBKDF2-processed an arbitrary
passphrase. Standalone public JWK construction retained an arbitrary open JSON
extension map after serde or native construction.

## Decision

The repository adopts the machine-readable inventory and ownership vocabulary
defined in
[`sdk-input-resource-boundaries.md`](../architecture/sdk-input-resource-boundaries.md).
Every implemented runtime package has one or more boundary-family rows with one
of five dispositions: `sdk-enforced`, `fixed-or-no-input`,
`caller-budgeted-work`, `outer-preallocation`, or
`known-unbounded-compatibility`.

An offline standard-library checker derives the required package set from the
bootstrap inventory and scans each implemented package for public `MAX_`/`MIN_`
resource constants. It rejects missing/extra package coverage, omitted public
resource constants, unknown schema or dispositions, duplicate IDs, absent
evidence, invalid evidence paths, and SDK-enforced rows without explicit
limits. A mutation suite proves these failure modes. Structural success does
not replace semantic architecture/security review.

BIP-39 conversion rejects non-standard entropy lengths before the dependency.
Mnemonic validation rejects more than 24 words and words above the adopted
English word-list maximum before joining. Standard and KMP-compatible seed
derivation reject passphrases above 4,096 UTF-8 bytes before normalization or
PBKDF2. All failures retain the existing redacted `MnemonicInvalid` surface.

Standalone JWK construction rejects more than 32 top-level extension members,
depth above 16, more than 1,024 JSON nodes, or more than 65,536 aggregate UTF-8
bytes across keys and string values before retention. A borrowed iterative walk
keeps validation itself bounded. The existing `ReservedExtension` variant is
broadened to the complete invalid-extension class, preserving the public enum
shape and a redacted stable `IdentusError` surface.

DID method registries retain at most 64 method bindings. Cache-key bytes,
declared cache capacity and positive/`notFound` TTLs retain their existing
portable limits, while injected cache and monotonic-clock storage/time work is
explicitly caller-budgeted. These are distinct families from resolver and
registrar transport work.

The historical public `Multihash` placeholder remains an infallible opaque
`Vec<u8>` compatibility surface under ADR 0082. It is inventoried as
`known-unbounded-compatibility`: consumers must bound bytes or hex text before
entry, and the exception remains in `SDK-LIM-007` until a named consumer owns a
structural and capacity migration. It is not safe for direct hostile input.

DID slice parsers bound wire bytes, depth, nodes, and collections while parsing.
Direct native constructors accepting an already-owned `serde_json::Value` or
map validate the accepted representation but do not yet dismantle every rejected
hostile-depth tree iteratively. `SDK-LIM-007` therefore assigns pre-entry depth
and rejection-cleanup safety to those native callers; they must use the bounded
slice parser for hostile input or establish an equivalent outer bound.

`SDK-LIM-007` remains effective but is narrowed: it names outer preallocation,
native owned-JSON rejection cleanup, caller-budgeted primitive/adapter work,
and the known unbounded `Multihash` compatibility placeholder instead of an
incomplete audit.

## Consequences

- Agents can locate the exact owner, limit and evidence for every implemented
  package before adding or changing a boundary.
- Adding a public `MAX_`/`MIN_` resource constant without inventory coverage
  fails the offline factory contract.
- Consumers still must cap hostile data at their earliest transport,
  deserializer or language-runtime boundary and budget generic operation work.
- Valid BIP-39 and KMP compatibility vectors remain exact; extreme passphrases
  above the SDK budget are rejected by an experimental unpublished helper.
- Ordinary JWK metadata remains compatible; extreme extension documents are
  rejected by the experimental facade. Public budget constants are additive,
  and no wire or persisted representation changes.
- DID registry/cache behavior is unchanged; the audit now records its existing
  limits and the concrete adapter obligations separately.
- `Multihash` behavior is unchanged; its unbounded retention is visible rather
  than misclassified as a bounded DID value.
- Native DID JSON behavior is unchanged; bounded slice parsing remains the
  required hostile-input path until iterative rejection cleanup is comprehensive.
- Package activation and new input families require an atomic inventory update.

## Alternatives rejected

- A universal byte cap on all borrowed primitives and ports would couple
  unrelated protocols and adapters to an arbitrary budget.
- Declaring every typed value fully bounded would conceal work that already
  occurred outside the typed boundary.
- Removing `SDK-LIM-007` would hide real residual consumer obligations.
- Leaving the limitation broad would discard the value of the completed audit.
- A regex-derived public-API proof would overstate source analysis; issue #275
  owns the separate `syn`-based code-health classifier.

## Verification and rollback

Exact/one-over BIP-39 tests cover entropy, words and passphrase bytes, redaction,
standard vectors and KMP parity. Inventory mutations cover schema, package
coverage, dispositions, limits, evidence, duplication and bounded input size.
Factory/OpenSpec, formatting, strict Clippy, all tests/builds and compatible Nix
provide integration evidence.

Revert the issue-linked change to restore prior helper behavior, remove the
inventory gate and reinstate the broad incomplete-audit limitation. A newly
discovered omitted boundary instead requires immediate disclosure and focused
remediation, not silent inventory deletion.
