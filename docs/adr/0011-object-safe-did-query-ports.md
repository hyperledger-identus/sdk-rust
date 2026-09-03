# ADR 0011: expose object-safe DID query ports

- **Status:** Accepted
- **Date:** 2026-09-03
- **Decision authority:** IDR-006 roadmap mandate and sdk-rust issue #43
- **Related work:** issues #5, #10 and #42; OpenSpec `add-did-query-ports`

## Context

The SDK owns current W3C DID identifiers, documents and result envelopes but no
common invocation seam. NeoPRISM, midnight-identity, Lace and Oxid consequently
carry incompatible async resolver traits. The W3C DID Resolution Candidate
Recommendation Draft dated 28 August 2026 defines exact resolution and
dereferencing abstract functions and extensible input option maps.

## Decision

1. `identus-did` owns immutable bounded resolution and dereferencing options
   plus the abstract `DidResolver` and `DidUrlDereferencer` query ports.
2. Common options reuse the existing media-type, version-id and UTC datetime
   values. Verification relationship remains a bounded open ASCII value.
3. Both ports are object-safe `Send + Sync` traits returning named boxed `Send`
   futures. They add no async macro, executor, transport or chain dependency.
4. Invocations return the existing W3C result envelopes directly. Standard and
   infrastructure failures are converted to bounded error metadata rather than
   exposed through a second generic error channel.
5. Resolution and dereferencing remain independent because dereferencing is
   explicitly at risk in the pinned draft.
6. Raw options are limited to 64 KiB and reuse the existing map/name/string,
   depth and aggregate-node JSON policy. Reserved common keys cannot be
   supplied as extensions.
7. HTTP, method dispatch, cache/clock policy, algorithms and DID Registration
   state/secret semantics remain separately reviewed `IDR-006` work.

## Consequences

- Method implementations and remote adapters gain one injectable SDK boundary
  that is usable by NeoPRISM, midnight-identity, Lace and Oxid.
- The boxed allocation is explicit, buys object safety and is confined to an
  asynchronous I/O-shaped boundary.
- Consumers receive one standards-shaped failure channel and keep transport or
  product policy outside the domain crate.
- A non-`Send` JavaScript future, method dispatcher or stateful registrar needs
  a separately profiled adapter/contract rather than weakening this API.

## Provenance

| Evidence | Revision | Result |
| --- | --- | --- |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` | Apache-2.0; resolver signature adapted, async macro/result/HTTP coupling rejected |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | Apache-2.0; boxed-future seam adapted, Midnight DID/registrar semantics rejected |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | evidence-only; object-safe consumer need retained, HTTP/error policy rejected |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | Apache-2.0; injection need retained, Midnight/product provenance rejected |

Normative behavior derives from the pinned W3C DID Resolution draft and W3C
DID Core 1.0. No donor source or fixture is copied.

## Rollback

Revert the issue #43 pull request. No package is published, no persisted SDK
state changes and no downstream repository is modified.
