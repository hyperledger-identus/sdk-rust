# ADR 0010: own the current DID resolution result contract

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** IDR-005 roadmap mandate and sdk-rust issue #39
- **Related work:** issues #5, #10, #37 and #38; OpenSpec
  `add-did-resolution-results`

## Context

The SDK owns validated DID syntax and documents but no common result envelope.
PRISM, Midnight, Lace and Oxid therefore carry incompatible metadata and legacy
error records. The W3C DID Resolution v1 Candidate Recommendation Draft dated
28 August 2026 now specifies URL-valued RFC 9457-style error objects, while its
DID URL dereferencing contract remains explicitly at risk.

## Decision

1. `identus-did` owns immutable validated resolution and serialized DID URL
   dereferencing result values; resolver traits and bindings remain IDR-006.
2. Resolution permits exactly success, ordinary error and deactivation states.
   An explicit validation step checks a returned document against the requested
   DID and enforces same-method canonical/equivalent syntax.
3. Errors use a required absolute type URI with bounded RFC 9457 title, detail,
   instance and extensions. All nine current W3C URLs are classified. Legacy
   keywords are accepted only by an explicit adapter migration helper.
4. Common document metadata is typed. Proof and method-specific values remain
   bounded open JSON and do not receive trust semantics in the generic layer.
5. Dereferenced content remains bounded open JSON with typed projections for
   known DID-domain values. A closed enum and arbitrary native bytes are
   rejected because the standard is at risk and bytes require a binding.
6. Media types, whole-second UTC datetimes and opaque version ids use small
   self-contained validators with no HTTP or datetime dependency.
7. Result JSON has a 512 KiB envelope and reuses the existing collection, map,
   property-name, string, depth and aggregate-node resource policy.
8. Package publication, method/chain behavior, I/O, HTTP, caching and downstream
   adoption remain separately reviewed work.

## Consequences

- PRISM and Midnight result producers plus Lace/Oxid consumers gain one neutral
  contract without async, chain, transport or product dependencies.
- Strict current wire parsing prevents silent confusion between legacy keyword
  failures and current error objects while adapters retain a deliberate bridge.
- The at-risk dereferencing surface can evolve without breaking a closed Rust
  resource enum, at the cost of callers explicitly requesting typed projections.
- Structural validity does not authenticate documents, prove DID equivalence,
  authorize keys or make network dereferencing safe.

## Provenance

| Evidence | Revision | Result |
| --- | --- | --- |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | Apache-2.0; no generic result or dependency needed |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` | Apache-2.0; current error/result needs, transport coupling rejected |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | Apache-2.0; common metadata adapted, legacy errors rejected |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | Evidence-only; repository license unresolved |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | Apache-2.0; consumer metadata adapted, product provenance rejected |

Normative behavior is derived from the pinned W3C DID Resolution v1 Candidate
Recommendation Draft, W3C DID Core 1.0, RFC 9457 and media-type grammar. No
donor source is copied.

## Rollback

Revert the issue #39 pull request. No published package, stored SDK data or
downstream repository is changed by this decision.
