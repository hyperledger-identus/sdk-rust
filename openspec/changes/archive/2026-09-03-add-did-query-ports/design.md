## Context

The current W3C DID Resolution draft defines two abstract functions with
required but possibly empty option maps: `resolve(did, resolutionOptions)` and
`dereference(didUrl, dereferencingOptions)`. Resolution defines `accept`,
`expandRelativeUrls`, `versionId` and `versionTime`; dereferencing defines
`accept` and `verificationRelationship`. Both vocabularies are extensible.

NeoPRISM exposes an `async_trait` resolver returning a legacy public result.
midnight-identity and Oxid use explicit boxed futures, associated or
method-specific DID types, and separate errors. Lace uses an object-safe
`async_trait` seam whose HTTP adapter collapses result and transport concerns.
The reusable intersection is an object-safe query port over the SDK's already
validated identifiers, options and W3C result envelopes—not any donor trait
verbatim.

Immutable evidence inspected before implementation:

- NeoPRISM `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b`, Apache-2.0,
  `lib/did-core/src/resolution.rs` and `lib/did-resolver-http`.
- midnight-identity `427f8571950c42967a18726cbcbefecc19ef8d79`,
  Apache-2.0, `midnight-did-domain` resolver/registrar ports.
- Lace ID Portal `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1`,
  evidence-only because repository-level license evidence remains unresolved.
- Oxid `bfe3b481568dc738f0732c2b27548fab8721fd95`, Apache-2.0,
  `DidResolutionPort` injection and product-specific provenance rejection.

No donor source or fixture is copied.

## Goals / Non-Goals

**Goals:**

- Match the two W3C abstract query signatures with validated SDK values.
- Preserve absent versus explicit false options and lossless extension JSON.
- Make native and serialized option construction enforce the same bounds.
- Support `Arc<dyn DidResolver>` and `Arc<dyn DidUrlDereferencer>` without an
  executor or async-macro dependency.
- Preserve the standard's single result/error channel and clean hexagonal
  dependency direction.

**Non-Goals:**

- HTTP, content negotiation, status mapping, retries, timeout or SSRF policy.
- Method registry/dispatch, cache behavior, clock injection or telemetry.
- The standard dereferencing algorithm, query normalization or recursive I/O.
- Registrar create/update/deactivate, jobs, patches, secret modes or VDR types.
- Trust, authorization, proof interpretation, storage or downstream adoption.

## Decisions

### Decision 1: type common options and keep extensions open

`ResolutionOptions` owns optional `MediaType`, `bool`, `VersionId` and
`DidResolutionDateTime` members plus a bounded extension map.
`DereferencingOptions` owns optional `MediaType` and
`VerificationRelationshipName` members plus the same extension form. Builders
preserve ergonomic immutable construction; validating constructors are the
common endpoint for builders and custom serde.

`VerificationRelationshipName` is a non-empty, trimmed printable ASCII value
bounded to 256 bytes. It deliberately remains open rather than enumerating the
five DID Core relationship names because extension specifications can define
additional relationships. Existing media type, datetime and version values are
reused so the same wire value cannot acquire a second validator.

Reserved common keys cannot appear in the extension map. Each map uses the DID
capability's existing 64-member, 256-byte-name, 64-KiB-string, 32-level and
4,096-node JSON budget. Raw option JSON entry points reject more than 64 KiB
before parsing. Unknown valid extension members round-trip semantically.

### Decision 2: expose object-safe boxed-future ports

`DidResolver` and `DidUrlDereferencer` are `Send + Sync` traits marked with the
repository's `#[identus::port]` attribute. Their methods return named aliases
over `Pin<Box<dyn Future<Output = ...> + Send + 'a>>`. This is object-safe on
Rust 1.85, works with `Arc<dyn ...>`, avoids `async-trait` code generation and
does not choose Tokio, async-std, WASM bindings or another runtime.

The allocation is explicit and accepted at this I/O boundary. Callers that
need allocation-free monomorphized futures may add implementation-specific
methods without changing the standard port. A future local-only WASM adapter
whose JavaScript future cannot be `Send` requires a separately profiled binding
rather than silently weakening the cross-platform shared port.

### Decision 3: return standard result envelopes directly

The resolver future outputs `DidResolutionResult`; the dereferencer future
outputs `DidUrlDereferencingResult`. W3C already represents unsupported
methods/options, not-found and internal failure in result metadata. A second
`Result<_, E>` channel would force every generic caller to reconcile duplicate
failure taxonomies and would prevent the HTTP adapter from remaining a pure
binding.

Implementations must convert internal failures into bounded W3C error results
before returning. Transport adapters may have their own construction/config
errors outside an invocation, but an invoked port always returns one standard
envelope. The core neither logs nor exposes the underlying error detail.

### Decision 4: keep resolution and dereferencing independent

DID URL dereferencing is explicitly at risk in the pinned Candidate
Recommendation Draft. It therefore has a separate option type, future alias
and trait. Implementers may implement either or both ports, and future removal
or signature changes to dereferencing do not alter `DidResolver`.

No default dereferencing algorithm is included in this slice. Its forwarding,
normalization, fragment authorization, recursion/cycle, media and network
policy require their own threat contract.

### Decision 5: defer stateful and policy-bearing ports

Method registry/dispatch and cache/clock abstractions require ownership,
duplicate registration, freshness and concurrency decisions. DID Registration
is still a DIF draft with job states and secret modes that the narrow Midnight
donor trait does not model. Freezing those APIs here would couple a simple read
boundary to unresolved stateful semantics. They remain focused `IDR-006`
children with their own standards pins and ADRs.

## Threat Contract

**Assets:** option integrity, result-channel consistency, runtime neutrality,
extension fidelity, resolver availability and clean chain/product boundaries.

**Threats addressed:** oversized/deep extension input, reserved-key shadowing,
CRLF/control injection in textual options, malformed typed options, native/wire
validation drift, accidental executor coupling and unbounded donor errors.

**Residual boundaries:** the port does not authenticate a resolver, authorize a
verification method, normalize DID queries, prevent network cycles/SSRF, apply
timeouts, prove method results, enforce freshness or make a remote binding
trusted.

## Test and Verification Strategy

- Pin current W3C common option names, empty maps and extension round trips.
- Cover malformed scalar values, reserved collisions and every resource bound.
- Assert port traits through `Arc<dyn ...>` and two independent PRISM- and
  Midnight-shaped mock implementations.
- Verify exact DID/DID URL and option forwarding plus standard success/failure
  envelopes without a runtime dependency.
- Record a release-mode option parse and dynamic-dispatch diagnostic without a
  CI threshold.
- Run focused, workspace, MSRV, wasm/mobile, docs, lint, formatting, OpenSpec,
  supply-chain and full Nix gates, then review the exact implementation head.

## Migration Plan

1. Land the issue-linked OpenSpec delta and ADR as a signed documentation
   commit before implementation.
2. Add option values, ports and conformance tests without downstream edits.
3. Record performance and complete a distinct semantic/misuse review.
4. Sync the canonical did-core spec, archive the change and merge only after
   exact-head hosted CI is green.
5. Implement the remaining IDR-006 children and consumer adoption separately.
