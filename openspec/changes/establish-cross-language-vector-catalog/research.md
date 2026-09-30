# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-30
Source retrieval date: 2026-09-30
Research blockers: none

## Problem and existing implementation

`identus-did` already implements bounded DID and DID URL parsing with exact
limits (`MAX_DID_BYTES = 2048`, `MAX_DID_URL_BYTES = 4096`), stable public
codes (`did.invalid_did`, `did.invalid_did_url`), and redacted caller input.
`crates/did/tests/did_syntax.rs` covers grammar, boundaries, serde, allocation,
and errors. Those tests are Rust-specific and do not carry the provenance and
adapter-facing identity required for reuse by other SDKs.

The repository has mature domain-specific inventories such as Apollo crypto
parity, but no generic catalog joins a portable payload to source authority,
license, redistribution, immutable revision, exact selector, boundary class,
and supersession. ADR 0173 and the A1 blueprint assign that gap to issue #420.

## Normative sources

- W3C DID Core 1.0, immutable Recommendation dated 2022-07-19, sections 3.1
  and 3.2, defines DID and DID URL syntax:
  https://www.w3.org/TR/2022/REC-did-core-20220719/
- SDK-Rust revision `edf03abcc963d37703daa61192940394bdc553ca`
  provides the canonical current parser, limits, error codes, and test harness.
- SDK-TS revision `4bf86ebf69d5e96616a148e4c973f831f95fa38e`
  provides historical consumer cases at
  `packages/lib/sdk/tests/castor/DIDParser.test.ts` and
  `packages/lib/sdk/tests/castor/DIDUrlParser.test.ts`.
- SDK-Swift revision `ebbfdb666a3b0c9905dd443589e03ccf2ae8087b`
  provides corresponding cases in `EdgeAgentSDK/Castor/Tests`.
- SDK-KMP revision `5a8fda770bbbca84192979b2c8e6090ee94790e3`
  provides corresponding cases in `sdk/src/commonTest/.../castor`.

All assessed repositories declare Apache-2.0. The donor cases are
`consumer-regression` evidence, not independent normative authorities. Exact
source identity and redistribution remain mandatory despite compatible
licensing.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| TOML catalog plus JSON packets | `adopt` | Matches repository tooling while keeping payloads portable to non-Rust consumers. | A consumer cannot load the JSON contract without semantic loss. |
| One generated JSON packet with literal and deterministic repeated inputs | `adopt` | Preserves reviewability while testing exact 2–4 KiB byte limits without giant literals. | The generator proves ambiguous in another language. |
| Copy all donor fixtures | `not-adopt` | Duplicates data and falsely promotes consumer behavior. | A later capability approves an explicit, redistributable packet. |
| Network-fetch sources in CI | `not-adopt` | Makes evidence non-reproducible and donor availability a build dependency. | Never for the normal gate; retrieval may remain a research task. |
| Introduce JSON Schema runtime validation | `not-adopt` | Adds a dependency before the local contract proves value. | Schema consumers require generated validators across languages. |
| Python standard-library validator and Rust packet test | `adopt` | Uses existing repository infrastructure and proves both metadata and behavior. | The factory removes Python or packet scale requires a compiled tool. |

## Compatibility and dependency evidence

The shared TS/Swift/KMP examples include `did:aaaaaa:aa:aaa`, percent and
punctuation cases, a long Prism-shaped DID, and invalid prefix/method/segment
cases. Their repetition demonstrates migration relevance but also copied-test
risk. The seed records those cases with pinned source selectors while W3C and
accepted Identus contracts retain higher authority.

The packet supports literal inputs and a closed deterministic repeat form. Its
expected results contain only language-neutral strings, components, stable
error codes, and a redaction boolean. The Rust harness is a dev/test edge from
`identus-conformance` to `identus-did`; it adds no production dependency or
public API. Rust 1.89.0 MSRV, Rust 1.98.1 etalon, target tiers, and release
surface remain unchanged.

The current implementation and exact version/features are pinned above. The
direct and resolved dependency cone is unchanged because the only new Rust edge
is a test/dev dependency. Supply-chain evidence is limited to repository-owned
data and standard-library validators. Public and wire compatibility are
unchanged. The facade boundary remains outside Rust core, rollback is file
removal before merge or explicit supersession afterward, and protocol or draft
currency is fixed to the final DID Core 1.0 Recommendation rather than a draft.

## Security, privacy and maintenance evidence

Only public synthetic or standards-derived values are admitted. The validator
rejects non-public data declarations, absolute/traversing/symlinked payload
paths, invalid SHA-256 digests, unpinned repository sources, missing license or
redistribution decisions, duplicate IDs, unknown fields, and broken
supersession. CI is entirely offline.

Payload hashes make accidental mutation visible. Stable IDs and explicit
replacement fields preserve history. The catalog does not contain secrets,
credentials, personal data, production identifiers, executable code, native
libraries, or unsafe Rust. Mutation tests prove the fail-closed behavior of
the validator rather than only its happy path.

## Rejected or deferred candidates

Remote registries, YAML, a database/service, automatic donor harvesting,
generated bindings, method-specific DID vectors, credential/protocol packets,
and migration of every existing vector inventory are deferred. Existing
Apollo and OID4VC inventories keep their domain authority and can be linked by
future external-record references.

## Open questions and blockers

None. The packet format, authority vocabulary, exact source revisions, seed
capability, validator strategy, and stop boundary are sufficiently bounded for
implementation.

## Evidence commands

Planning commands validate evidence through `scripts/factory research-ready
establish-cross-language-vector-catalog`, `scripts/factory constraints-ready
establish-cross-language-vector-catalog`, and `scripts/factory validate
establish-cross-language-vector-catalog`. Implementation will add focused
catalog mutation tests and a Rust conformance packet test before the full
factory gate. Those implementation checks are intentionally unrun at planning
time.
