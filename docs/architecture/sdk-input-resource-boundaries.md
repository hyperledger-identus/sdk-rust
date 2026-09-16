# SDK input-resource boundary inventory

This is the repository-wide audit required by issue
[#168](https://github.com/hyperledger-identus/sdk-rust/issues/168). The
normative inventory is
[`sdk-input-resource-boundaries.toml`](sdk-input-resource-boundaries.toml); this
document explains how to read it.

The audit covers all thirteen runtime packages classified `implemented` in the
bootstrap inventory. Placeholder packages contain no accepted runtime API, and
verification-only packages consume synthetic repository inputs, so neither is
presented as an SDK input surface.

## What “bounded” means

| Disposition | Meaning | Owner |
| --- | --- | --- |
| `sdk-enforced` | Typed SDK code checks explicit byte, element, nesting, recursion or work limits. | The named crate. |
| `fixed-or-no-input` | The surface is fixed-size/static/numeric, compile-time only, or parses no untrusted runtime input. | The named crate maintains that shape. |
| `caller-budgeted-work` | A primitive or generic port does not retain the input and has no honest cross-protocol budget. | Protocol caller or concrete adapter. |
| `outer-preallocation` | A typed SDK limit exists downstream, but another runtime can allocate or scan first. | Transport, deserializer, FFI or language-runtime owner. |
| `known-unbounded-compatibility` | A retained public compatibility surface has no enforceable bound without an explicit API migration. | SDK maintainers disclose it; consumers bound it before entry. |

An owned `String`, `Vec`, JSON `Value`, Axum extraction, UniFFI argument or
JavaScript string has already consumed resources by the time a Rust constructor
sees it. Consequently “the value rejects 8 KiB” is not evidence that the HTTP
server or foreign runtime prevented a larger allocation. Consumers must apply
limits at their earliest owned boundary.

Likewise, placing a universal input cap on `sha256(&[u8])`, `sign(&[u8])`, or a
generic storage trait would be false modularity. Those functions do not retain
messages, and the correct byte/time quota depends on the protocol or adapter.
The inventory makes that caller ownership reviewable instead of silently
calling it unbounded SDK behavior.

## Audit result

- Every implemented runtime package has at least one boundary-family row.
- All public resource constants with `MAX_`/`MIN_` name segments—including
  `DEFAULT_MAX_`—declared by implemented package source are named by an
  inventory limit; the checker enforces this coverage.
- Core, credential, presentation, OID4VCI and wallet retained values have
  explicit fixed or configurable limits. The inventory separately discloses
  three public compatibility exceptions: infallible crypto text encoders,
  directly constructible JOSE retained-input enums, and DID `Multihash`.
- DID document, resolution-result, registration, and option-map policies are
  separate rows. Credential artifacts, descriptors, format/metadata, schema,
  status, verification evidence, registry, and verifier execution are likewise
  independently reviewable.
- The DID method registry caps retained bindings at 64. DID resolution cache
  keys, declared capacity and TTL policy are bounded separately from the
  caller-owned cache/clock adapter QoS and storage allocation.
- The HTTP resolver applies semantic limits after Axum extraction; deployment
  middleware still owns connection, header/request-line and execution limits.
- Bounded DID wire-slice parsers remain the hostile-byte-input path because
  they constrain allocation before typed construction. Once a native DID
  constructor owns recursive JSON, every audited rejection path dismantles the
  tree iteratively; callers no longer need a depth bound solely for safe
  rejection cleanup.
- UniFFI and WASM DID facades delegate to bounded DID parsing after their
  language bridges allocate input.
- Hash/sign/verify primitives and injected resolver, registrar, verifier,
  trust, replay and storage adapters remain explicitly caller-budgeted.
- BIP-39 now rejects impossible entropy, excess word count/word bytes and
  passphrases above 4,096 UTF-8 bytes before dependency or expensive work.
- Standalone public JWK extensions are capped at 32 members, depth 16, 1,024
  JSON nodes and 65,536 aggregate key/string UTF-8 bytes before retention;
  rejected native trees are dismantled iteratively instead of recursively.
- `HexStr::from` and `Base64UrlStrNoPad::from` retain encodings of arbitrary
  caller byte slices; #298 owns their validated-construction migration.
- Direct `JwsKeyReference::KeyId`/`X5c` and
  `Oid4vciProofJwtClient::Identified` construction can retain arbitrary inputs
  before later builder validation; #299 owns their opaque validated migration.

The broad “audit incomplete” wording is therefore retired. `SDK-LIM-007`
remains, narrowed to concrete outer-preallocation obligations, caller-budgeted
work, and known unbounded compatibility obligations above. Finding another
missing input family or a native DID rejection path that bypasses iterative
cleanup is a security regression: restore broader disclosure immediately and
open a focused remediation issue.

## Maintenance rule

Update the machine inventory atomically when:

- a package becomes implemented;
- a parser, decoder, collection, recursion, redirect, decompression, transport,
  async port or binding boundary is added or materially changed;
- a limit or ownership disposition changes; or
- a concrete adapter moves resource ownership into the SDK.

The offline checker proves structure, exact implemented-package and public
resource-limit-constant coverage, evidence existence and limit/disposition coherence.
It cannot prove Rust semantics. Every qualifying change still requires
architecture/security review under `SDK-SEC-003`.
