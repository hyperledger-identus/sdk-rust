# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-10-26
Source retrieval date: 2026-10-26
Research blockers: none

## Problem and existing implementation

`identus-did` currently accepts at most 2,048 bytes for a bare DID and 4,096
bytes for a DID URL or generic absolute URI. `DidMethod` derives its 2,042-byte
ceiling from the bare DID limit. The resolution cache retains normalized keys
up to 65,536 bytes.
Generic DID-document and resolution-result wire parsers accept at most 256 KiB
and 512 KiB respectively. The WASM, UniFFI, fuzz, cross-language-vector, and
language-adapter contracts all encode parts of the old lexical policy.

## Normative sources

- PRISM method specification:
  `input-output-hk/PRISM-did-method-spec@fdd9007e23dc08d94d1a2b66b2dc26d1db499422`.
- NeoPRISM donor:
  `MediaNoxLabs/neoprism@a2ddbc5a8a316841e2c8df3831e7904fef01c356`.
- SDK-TS release evidence:
  `hyperledger-identus/sdk-ts@005c6664de1e399aca723c6a33c1b005c70c3fa1`
  (`@hyperledger/identus-sdk@8.1.4`).
- Exact isolated wire model: pinned PRISM protobuf shape encoded with
  `prost 0.14.4`.

The model encoded 50 verification methods and 50 services. Full long-form DID
sizes were 37,686 bytes for compressed keys and bounded ASCII endpoints,
39,953 bytes for uncompressed keys, 51,003 bytes after adding 32 contexts of
256 bytes, 99,953 bytes with 300 four-byte endpoint characters, and 111,003
bytes for that Unicode case plus the contexts. The largest pinned NeoPRISM and
SDK-TS fixtures were only 162 and 698 bytes, so examples are not an adequate
upper-bound oracle.

The proposed future method profile limits raw create-operation bytes to 96,000
and its unpadded base64url state to 128,000. Adding `did:prism:`, the 64-hex
initial-state hash and separator yields at most 128,075 bytes, below a 128 KiB
generic ceiling. This slice admits that envelope but does not implement or
claim the method profile.

The current implementation is the local `identus-did` parser at the exact
`develop` base named by the preimplementation receipt. W3C DID Core remains
the generic grammar authority; the pinned PRISM protocol draft is authority
only for method-specific shape. Each source URL, revision, license, and
provenance record is immutable evidence. SDK-TS and NeoPRISM are consumer and
donor evidence rather than normative sources.

## Candidate decisions

| Surface | Decision | Reason |
| --- | --- | --- |
| `Did` | `adopt` 131,072 bytes | Fits the measured finite PRISM profile with 2,997 bytes of headroom. |
| `DidUrl` | `adopt` 139,264 bytes | Adds an independent 8 KiB path/query/fragment allowance. |
| `Uri` | `adopt` 139,264 bytes | DID document method/service IDs use `Uri`; `From<DidUrl>` must preserve its validation invariant. |
| `DidMethod` | `retain-local` at 2,042 bytes | Embedded state belongs to method-specific ID, not the method name. |
| Crypto text codecs | `not-adopt` for PRISM payload | PRISM encoding is a method-specific responsibility. |
| Cache key | `retain-local` at 65,536 bytes with bypass | Avoids multiplying 128 KiB identities across up to 4,096 retained entries. |
| Generic JSON ingress | `retain-local` | Raising hostile JSON parsing to tens of MiB would weaken unrelated callers. |
| HTTP resolver | `retain-local` | Long-form creation/resolution is local; request-target capacity is deployment-owned. |

## Compatibility and dependency evidence

The public and wire grammar do not change; the accepted public envelope grows.
Existing valid values retain exact serialization, equality, hashing, borrowed
views, and stable errors. Consumer mappings and boundary vectors change in the
same commit. The facade boundary remains `identus-did`; no method crate points
back into the generic layer.

No production dependency is added. The direct and resolved dependency cone is
therefore unchanged, including features and target graph. Rust version/MSRV
remains the repository's pinned 1.98 etalon. The isolated `prost 0.14.4` model
is research tooling only and does not enter the manifest or supply-chain.

## Security, privacy and maintenance evidence

The implementation adds no dependency, unsafe block, native code, build
script, network call, or secret-bearing state. Supply-chain evidence is thus
unchanged. Caller input remains redacted and no full DID enters cache status or
errors. Maintenance and release posture remain owned by `identus-did`; the
change expands a documented public policy rather than adding a second parser.
Native, WASM, and mobile/UniFFI target evidence is required before merge.

## Amplification and adapter evidence

The PRISM specification projects the requested long-form DID into document
`id`, controllers, verification method IDs and controllers, relationship
references, and service IDs. With one master key excluded, 49 projected keys,
and 50 services, the identifier can occur about 198 times. A 128,075-byte DID
therefore contributes roughly 25.4 MB before keys, services, contexts,
metadata, or JSON framing. That is an output-amplification concern for #521,
not justification to raise generic untrusted JSON ingress.

Generic DID documents represent verification-method IDs, relationship
references, and service IDs as `Uri`. Leaving its old 4 KiB ceiling would make
large DID URLs parse successfully but fail projection, while the infallible
`From<DidUrl> for Uri` conversion would construct a value that its own parser
rejects. The URI ceiling must therefore be at least the DID URL ceiling.

The larger total envelope must not give a short DID a 136 KiB suffix. Existing
dereferencing accepts parameters such as `relativeRef` and can project them
across bounded service collections, so suffix size multiplies allocation and
validation work. An independent 8 KiB combined path/query/fragment ceiling
preserves the intended state-bearing allowance while bounding that work.

WASM and UniFFI allocate foreign-language strings before Rust validation, so
their outer runtimes still own preallocation. Each facade nevertheless needs a
100 KB PRISM-shaped acceptance canary and one-over rejection through the shared
constants. The Axum resolver has no SDK 8 KiB DID path limit, but common HTTP
infrastructure may; method routing must not send embedded initial state through
network resolution accidentally.

## Complexity and prototype evidence

An isolated release-mode parser probe remained linear: approximately 1.6 GiB/s
of scanned input from 40 KB through 131 KB, with about 13,000 parses/second at
131 KB on the discovery machine. A disposable clean clone of `origin/develop`
at `5da04223c1c4e029cb368870877e56d699d40388` implemented the candidate policy
without pushing. It passed the factory contract, DID syntax/cache tests,
all-target/all-feature DID clippy, cross-language conformance and checker
mutation tests, WASM/UniFFI host tests, WASM target checks, and the DID fuzz
target check.

The prototype discovered two cross-cutting constraints: language-adapter
bound symbols must remain literal public constants, and the portable vector
generator's prior 10,000-repeat safety cap could not express the new exact
boundaries. The generator can remain bounded at 262,144 repeats.

## Security, compatibility, and rollback

Parsing remains a single ASCII byte pass and rejects a one-byte-over input
before grammar traversal or success allocation. Errors remain redacted.
Accepted oversized cache identities bypass reads, writes, and clock/backend
work; infrastructure failures continue to obey the configured failure mode.
Existing smaller values remain source-compatible, while exported constant
values deliberately change. Cross-language mapping limits and vectors change
atomically.

Rollback before release is a focused revert. After release, lowering the
accepted public ceilings requires a compatibility decision and change-ledger
entry. The independent method-name, crypto, cache-retention, JSON-ingress, and
transport budgets are not implicitly coupled to the new lexical ceiling.

## Rejected or deferred candidates

Unbounded parsing, reusing the 4 KiB crypto codec, globally raising JSON
ingress, enlarging retained cache keys, and requiring HTTP transport are
`not-adopt` decisions. PRISM protobuf/semantic validation, projection/output
budgets, and any explicit high-budget re-ingress parser are deferred to #521.

## Open questions and blockers

There is no blocker for this bounded lexical prerequisite. The protocol draft
may evolve; its exact revision is pinned here. The reconsideration trigger is
a normative PRISM limit or measured valid operation exceeding 128,075 bytes,
or target evidence showing the selected lexical ceiling is impractical. Such a
change requires a new measured ADR rather than an implicit limit increase.

## Evidence commands

Run `scripts/factory research-ready`, `scripts/factory constraints-ready`,
`scripts/factory preflight`, `scripts/factory check`, focused DID and
conformance tests, adapter host tests, WASM target checks, the DID fuzz-target
check, formatting, clippy, Nix file hygiene, and independent Claude review.
Issue evidence is maintained at
https://github.com/hyperledger-identus/sdk-rust/issues/524#issuecomment-6405416846.
Unrun checks in this disposable prototype are the official preflight, exact
Nix target matrix, hosted protected CI, and independent review; those commands
belong to the fresh issue branch.
