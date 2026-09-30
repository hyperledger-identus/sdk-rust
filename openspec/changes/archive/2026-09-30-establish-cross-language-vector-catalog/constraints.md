# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/420
Constraint blockers: none

## Existing entries affected

`SDK-ARCH-001` keeps product and chain behavior downstream; `SDK-ARCH-002`
keeps dependency direction acyclic; `SDK-SEC-003` requires bounded untrusted
input; `SDK-DELIVERY-001` requires issue, OpenSpec, review, and evidence before
implementation; `SDK-LIM-005` and `SDK-LIM-006` prevent SDK work from silently
becoming product or consumer-adoption work. ADRs 0164, 0169, 0170, and 0173
govern evidence authority, donor precedence, canonical Rust contracts, and A1
sequencing. None is weakened.

## Introduced or changed constraints

Every active vector has a stable ID, owning packet, capability, authority,
exact provenance, public-data declaration, license and redistribution
decision, immutable payload hash, expected result, applicability, limitation,
and exact Rust selector. Repository sources use a full 40-character revision;
standards use an immutable version/date and section. Unknown fields fail
closed.

Authority precedence is `normative`, `identus-contract`,
`consumer-regression`, `implementation-regression`, then `exploratory`.
Repeated donor examples cannot promote their authority. Cross-record reuse is
by stable ID and payload hash, never by untracked copying. Supersession must be
explicit, acyclic, and preserve the old ID.

Fixture paths are repository-relative, remain under the approved fixture root,
and cannot traverse or resolve through symlinks. Payloads are public,
synthetic, standards-owned, or explicitly redistributable; secrets, personal
data, credentials, and production identifiers are prohibited.

## Introduced or changed limitations

The first packet covers generic DID/DID URL string syntax, exact byte limits,
stable errors, and redaction only. It does not cover method resolution,
documents, Prism/peer DID semantics, cryptography, networks, credentials,
DIDComm, storage, or target-runtime support. A catalog entry is evidence, not
implementation, certification, conformance, release, or adoption.

Normal validation is offline and does not prove that an upstream URL still
exists. Pinned donor revisions preserve assessed evidence but create no future
compatibility promise. Deterministic repeated-input materialization is limited
to one ASCII value and an explicit count so byte boundaries remain portable.

## Consumer and product impact

No consumer or product behavior changes. A future language adapter may consume
the packet, but its DTO/error compatibility belongs to #505 and its actual
SDK-TS adoption belongs to #492. Product, consent, custody, persistence, chain,
and deployment policy remain downstream.

## Activation and rollback

Activation requires a signed/DCO planning commit, issue-bound preflight,
implemented catalog and packet, validator mutation tests, Rust packet proof,
factory integration, protected review, and green fast CI. Before merge,
rollback removes the new records. After stable IDs merge, semantic changes use
a version increment and explicit supersession; silent rewriting is prohibited.

## Evidence

Issue #420, parent #504, ADR 0173, the A1 blueprint/source audit, W3C DID Core
1.0, current `identus-did` behavior, and the three pinned language-SDK test
locations support these constraints.
