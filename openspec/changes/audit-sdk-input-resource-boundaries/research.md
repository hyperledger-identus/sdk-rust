# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-16
Source retrieval date: 2026-09-16
Research blockers: none

## Problem and existing implementation

The workspace inventory contains thirteen implemented runtime packages, five
quarantined placeholders, and two verification-only packages. Existing focused
work already bounds core URL retention, crypto text decoding and derivation,
DID syntax/document/resolution/registration JSON, credential and presentation
semantic collections, JWS Compact, OID4VCI protocol bodies, wallet storage
tokens/pages, and the HTTP resolver's Accept/query parsing.

Those facts are distributed across code, tests, component specifications, and
one core-only architecture note. The broad limitation therefore cannot be
narrowed mechanically, and a new crate could be marked implemented without an
input-boundary disposition.

Manual source review found one actionable inherited allocation/work gap in
`MnemonicHelper`: `mnemonics.join(" ")` runs before the dependency rejects word
count or membership; standard passphrase normalization and both standard/KMP
PBKDF2 paths accept arbitrary passphrase bytes. Invalid entropy is already
rejected by the dependency, but the SDK can cheaply reject non-standard lengths
before crossing that dependency boundary.

A second semantic pass on 2026-09-16 found another actionable gap before any
JWK implementation change began. `PublicKeyJwk::from_parts` validates reserved
members and fixed coordinates but retains an arbitrary number and shape of
`BTreeMap<String, serde_json::Value>` extensions. Serde has already allocated
the wire model before that constructor, but the crypto facade can and should
bound what it retains and subsequently clones/serializes. The existing COSE
boundary provides a local precedent for count and nesting budgets.

The current implementation revision assessed is
`66ec2b9b3a7ec35cf21ecc52cdca5bebed0b4d0d`. Current consumer evidence is the
repository's explicit Oxid, Midnight, midnight-identity, NeoPRISM, Lace and
Apollo boundary: none transfers transport or adapter resource ownership into
this focused SDK change.

## Normative sources

- Directed scope and acceptance:
  https://github.com/hyperledger-identus/sdk-rust/issues/168
- Effective forward resource guardrail and inherited limitation:
  `docs/governance/sdk-constraints.toml` entries `SDK-SEC-003` and
  `SDK-LIM-007`.
- Implemented-package population:
  `docs/architecture/sdk-bootstrap-inventory.toml`.
- Existing crate-scoped precedent:
  `docs/architecture/identus-core-input-boundaries.md`.
- Accepted BIP-39 behavior:
  `openspec/specs/crypto/spec.md`, requirement `BIP39 mnemonic helper`.
- BIP-39 defines entropy sizes 128–256 bits in 32-bit steps, mnemonic sizes
  12–24 words, English words, NFKD normalization and PBKDF2-HMAC-SHA512 with
  2,048 iterations: https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Machine inventory plus focused BIP-39 fix and narrow residual limitation | `adopt` | Completes the audit honestly, fixes concrete avoidable work, and preserves intentional primitive/adapter ownership. | A covered boundary family or package cannot be expressed without misleading aggregation. |
| Bound standalone JWK extensions inside the crypto facade | `adopt` | The SDK owns and retains this open JSON; count/depth/node/text budgets preserve useful metadata without unbounded retained work. | A standards profile requires a larger exact budget. |
| Add arbitrary global byte limits to every `&[u8]` primitive and port | `not-adopt` | Hash/sign/verify functions do not retain input and generic ports intentionally delegate payload/work budgets to protocol or adapter owners. | A higher-level supported protocol makes an exact budget normative. |
| Claim typed constructors bound prior allocation | `not-adopt` | Owned `String`/`Vec`, serde, Axum, UniFFI and JavaScript inputs can allocate before SDK validation. | A streaming/preallocation-safe adapter becomes SDK-owned. |
| Leave `SDK-LIM-007` broad after completing the inventory | `not-adopt` | It would hide usable evidence and keep consumers unable to locate exact residual obligations. | The inventory is later proven incomplete. |
| Remove `SDK-LIM-007` entirely | `not-adopt` | Outer allocation and caller-budgeted work remain real by design. | All relevant transports/adapters and operation budgets become SDK-owned, which is not planned. |

## Compatibility and dependency evidence

The inventory/checker is repository-only and standard-library-only. It adds no
Cargo dependency or feature. BIP-39 valid English mnemonics, all five standard
entropy lengths, standard passphrases at or below the explicit byte ceiling,
and KMP-compatible derivation at or below that ceiling retain byte-exact output.
Oversized inputs previously produced expensive invalid work or unbounded work;
rejecting them with the existing redacted `MnemonicInvalid` error is a security
hardening change to an experimental, unpublished API.

The exact dependency version and features remain those pinned by `Cargo.lock`
and `crates/crypto/Cargo.toml`: `bip39` is optional under `derivation`, while
the legacy PBKDF2 path remains under `kmp-compat`. License and provenance remain
the existing repository/dependency records; no source or fixture is copied.

The temporary MSRV/compiler floor remains Rust 1.98.1. All existing target and
feature gates remain applicable. No crate dependency edge, public dependency
type, feature default, wire encoding, or persisted representation changes.
The direct and resolved dependency cone is unchanged because the work adds no
Cargo package. The public facade boundary remains `identus-crypto`; dependency
types and errors do not cross it. Rollback reverts the inventory/checker and
BIP-39 prechecks and restores the broad limitation text.

## Security, privacy and maintenance evidence

The selected design checks word count and per-word UTF-8 bytes before joining,
checks passphrase bytes before NFKD normalization/PBKDF2, and checks entropy
length before the dependency. Errors retain no input or secret value. The
bounded input and derived seed temporaries preserve zeroization behavior.

JWK extension validation walks borrowed JSON without cloning, counts top-level
members, total nodes, maximum depth and aggregate key/string UTF-8 bytes, and
rejects before storing the moved map. The error identifies only the violated
budget class and never includes an extension name or value. Serde allocation
before typed validation remains disclosed under `SDK-LIM-007`.

The inventory distinguishes four security-relevant dispositions instead of
pretending every surface has the same owner. An `sdk-enforced` row requires an
explicit limit and executable evidence. A `caller-budgeted-work` row states
which higher layer owns CPU/bytes. An `outer-preallocation` row records that the
typed SDK check happens after another runtime may allocate. Fixed/no-input rows
make non-parser packages explicit rather than silently omitting them.

Mutation tests will remove implemented package coverage, evidence, limits, and
valid dispositions and will insert placeholder/unknown package claims. The
checker remains offline, bounded in input size/count, symlink-safe, and does not
execute evidence paths.

## Rejected or deferred candidates

Global primitive limits, removal of the residual limitation, and false
preallocation claims are rejected above. Streaming HTTP clients, decompression
budgets, transport timeouts, concrete storage quotas, FFI allocator control,
and cryptographic operation quotas are deferred to the components that own
those adapters or protocols.

No authored unsafe Rust or new native code is introduced. Existing unsafe and
native-code evidence remains governed by the workspace unsafe policy and
dependency audit. Supply-chain evidence is unchanged because no dependency,
action, binary, or network fetch is added. The maintenance, release and
security posture remains experimental, unpublished, and protected by existing
review/CI gates. Protocol or draft currency is limited to the already accepted
BIP-39 behavior; this change does not select a new draft or protocol version.

## Open questions and blockers

No implementation blocker remains. The exact BIP-39 passphrase ceiling is an
SDK security budget rather than a BIP-39 interoperability limit; 4,096 UTF-8
bytes is selected to match the existing generic crypto text ceiling while
remaining far above ordinary wallet passphrases. Inputs above it require an
explicit downstream policy/adapter rather than this convenience helper.

## Evidence commands

- `scripts/check-input-resource-boundaries.py .`
- `scripts/tests/input-resource-boundaries.py`
- crypto unit tests with and without `kmp-compat`
- `scripts/check-factory.sh .` and OpenSpec/factory readiness commands
- full compatible `nix flake check --fallback`
- protected exact-head CI

Exact commands are listed above. Unrun checks at planning time are the new
checker/tests, crypto tests, full Nix and protected CI; they become mandatory
implementation and delivery evidence.

## Reconsideration triggers

- A package changes from placeholder/verification to implemented.
- A public parser, decoder, collection, recursion, redirect, decompressor,
  transport, async port, or FFI boundary is added or materially changed.
- A consumer requires inputs beyond an SDK-enforced ceiling.
- A concrete adapter moves allocation/time/work ownership into this workspace.
- Audit evidence finds an unlisted or incorrectly classified boundary.
