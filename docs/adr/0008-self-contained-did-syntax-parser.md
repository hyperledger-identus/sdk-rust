# ADR 0008: own a self-contained DID syntax boundary

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** IDR-005 roadmap mandate and sdk-rust issue #34
- **Related work:** issues #5, #3, #159; OpenSpec `add-did-syntax` and
  `decide-did-url-parser-parity`

## Context

The SDK needs chain-neutral DID and DID URL values before DID documents,
resolvers and PRISM/Midnight method ports can share a stable foundation.
Existing consumers either repeat partial validation or import a broad external
DID stack. W3C DID Core and RFC 3986 provide a small, closed lexical grammar
that can be implemented and tested directly.

NeoPRISM's DID values wrap IOTA Identity, while midnight-identity currently
checks only prefixes and separators. Those are valuable compatibility inputs
but unsuitable implementation foundations for this core boundary. Apollo,
Lace and Oxid add consumer evidence. The SDK needs explicit resource limits,
redacted errors and portable behavior across native, wasm and mobile targets.

## Decision

1. The `did-core` capability owns immutable `Did` and `DidUrl` semantic values.
2. A dependency-free byte parser implements the generic DID Core and composed
   RFC 3986 grammar in one bounded linear pass. It does not decode or normalize.
3. `Did` input is limited to 2,048 bytes and `DidUrl` to 4,096 bytes. These
   are SDK resource policies, not W3C conformance claims.
4. Values own one string and cache component offsets. Accessors borrow slices;
   `TryFrom<String>` and `From<Did> for DidUrl` reuse the owned allocation.
5. Native and serde construction share validation. Errors describe only safe
   invariant categories and map to stable `did.invalid_did` and
   `did.invalid_did_url` codes.
6. Generic syntax accepts unknown methods. Registration, resolution, method
   semantics, authorization and display policy belong to higher layers.
7. Code remains in the current unpublished `identus-did` package. The target
   `identus-did-core` publication name and migration remain governed by issue
   #3 rather than being coupled to this lexical slice.

## Consequences

- PRISM, Midnight and later method crates can depend on one small lexical
  boundary without inheriting IOTA Identity or URL-parser behavior.
- Parsing work and memory are bounded; repeated component reads do not
  allocate; the crate remains wasm/mobile friendly and adds no dependency.
- Exact source spelling is stable, including percent-escape case. Semantic URI
  equivalence and normalization are intentionally not supplied.
- A syntactically valid value remains untrusted until method, resolution and
  protocol policy succeeds.
- Complete DID documents, dereferencing, method ports and fuzzing require
  separately reviewed issues.

## Provenance

| Evidence | Revision | Result |
| --- | --- | --- |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | Apache-2.0; compatibility context, no DID parser extracted |
| NeoPRISM | `8becb225132efb1d9302b2c5f6ed4d87b84e8685` | Apache-2.0; useful behavior, parser delegated to IOTA Identity |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | Apache-2.0; consumer shapes, validation too shallow to copy |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | Evidence-only; repository license unresolved |
| Oxid | `685f9670af4846d52697a4cfeb94779758ae1075` | Apache-2.0; Midnight consumer shapes only |

Normative behavior is derived from W3C DID Core 1.0 and RFC 3986. No donor
source is copied.

## 2026-09-08 dependency reassessment

[ADR 0086](0086-retain-self-contained-did-parser.md) tested exact
`did_url_parser 0.3.0` against an attributable corpus and the complete facade,
resource and target boundary. The candidate failed grammar, exact-storage,
pre-allocation limit, owned-allocation, immutable-invariant and unsafe-reach
conditions. ADR 0008 therefore remains in force; the new evidence does not
change accepted behavior.

## Rollback

Revert the issue #34 pull request. No published package, persisted SDK format
or downstream repository is changed by this decision.
