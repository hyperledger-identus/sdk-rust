# ADR 0016: reject duplicate DID document JSON names before deserialization

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** IDR-005 roadmap mandate and sdk-rust issue #38
- **Related work:** issues #5, #35, #37, #41 and OpenSpec
  `harden-did-document-wire-boundary`

## Context

The bounded DID document API currently delegates raw JSON directly to
`serde_json`. RFC 8259 notes that duplicate object names produce unpredictable
cross-implementation behavior; a materialized map cannot prove which value was
received. This is especially unsafe for identifier, verification material,
service, and extension members that may later participate in proof or policy
decisions.

The SDK also owns a dependency-free RFC 3986 URI recognizer while NeoPRISM uses
`uriparse`. Before downstream adapters consolidate on the SDK boundary, its
acceptance profile needs deterministic independent evidence.

## Decision

1. Raw DID document ingestion first checks the 256 KiB byte cap and then runs
   a crate-private streaming JSON visitor that rejects duplicate decoded names
   recursively before typed deserialization.
2. The visitor discards values and retains only the decoded names of currently
   open objects. It rejects more than 64 nested containers, 16,384 values, 128
   members in one object, or 128 KiB of simultaneously live names.
3. Duplicate-name errors disclose no name or value and map through the existing
   `did.invalid_document` public contract. Malformed, trailing, and resource
   failures also remain redacted.
4. Duplicate detection is a lexical guarantee of `from_json_slice` and
   `from_json_str`. Native maps and materialized JSON values cannot represent
   duplicates; their representable semantics continue through the shared
   document validator.
5. The production `Uri` parser remains dependency-free and preserves exact
   spelling. Dev-only `uriparse` 0.6.4, matching NeoPRISM's lock, is an
   independent acceptance oracle rather than runtime authority.
6. Fixed deterministic property cases cover URI/document structure and raw
   mutations in PR CI. Sanitizer cargo-fuzz remains #35; resolution envelope
   reuse remains #41.

## Consequences

- Ambiguous raw DID documents fail before one duplicate value can be silently
  selected, including escaped-equivalent and deeply nested names.
- Valid raw input incurs a second streaming traversal. It avoids a full generic
  tree and stays under explicit availability bounds; release throughput is
  recorded without a hardware-specific threshold.
- Unique-name public and semantic JSON behavior is unchanged. Callers that
  deliberately relied on duplicate collapse receive a pre-release hardening
  break and must send interoperable JSON.
- The scanner becomes a reusable internal primitive for #41 without exposing a
  generic JSON API or adding chain/product/runtime dependencies.

## Provenance

| Evidence | Revision | Paths and use |
| --- | --- | --- |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` | Apache-2.0; `lib/did-core/src/uri.rs`, `did_doc.rs`, manifest/lock; conformance only |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | Apache-2.0; DID document validation/normalization source and tests; evidence only |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | open document round-trip evidence only; repository license unresolved |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | Apache-2.0; bounded holder document and private-JWK evidence only |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | Apache-2.0; no DID parser surface; crypto context only |

Normative sources are W3C DID Core 1.0 (19 July 2022 Recommendation), RFC 3986,
and RFC 8259. No donor source or fixture is copied.

## Rollback

Revert the issue #38 PR. No package release, stored-data migration, consumer
repository, chain state, or reserved `main` branch is changed.
