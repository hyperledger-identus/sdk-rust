# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/599
Constraint blockers: none

## Existing entries affected

- `SDK-ARCH-001` remains effective: portable Rust owns domain semantics and
  platform adapters depend inward.
- `SDK-SEC-003` remains effective: untrusted serialized input is bounded and
  validated before semantic use.
- `SDK-COMPAT-002`, `SDK-COMPAT-004`, and `SDK-COMPAT-005` remain effective for
  exact mapping, target, and consumer evidence.
- `SDK-LIM-002` remains effective: the private facade is not a supported
  binding commitment.
- `SDK-LIM-003` remains effective: browser WASM and native targets are not
  runtime-certified.
- `SDK-LIM-009` remains effective: isolated target failures do not overwrite a
  completed core result, but the existing terminal `fast` verdict fails.
- The exact Rust 1.99.1 source/compiler contract and TS1 SC-01 through SC-09
  remain controlling.

## Introduced or changed constraints

The train introduces an injected asynchronous host-effect seam around
synchronous generated Rust request/result functions, strict immutable
TypeScript values, shared cross-target evidence, and fail-closed impact routing.
It does not introduce a default transport or method implementation.

## Introduced or changed limitations

Raw host response text SHALL reach the bounded Rust result parser before any
TypeScript JSON parse. Resolution SHALL use `from_json_str_for` with the exact
requested DID. Dereferencing SHALL use its distinct current result parser.

The generated binding SHALL normalize and validate DID/DID URL and options
before host invocation, increment the experimental API version, expose only
closed owned views and stable redacted failures, and free each generated value
or error exactly once on success, host failure, projection failure, observer
failure, and mapped rejection.

The host effect SHALL receive a frozen plain request, return raw UTF-8 JSON
text, and be invoked at most once per public operation. No automatic retry,
fallback, cache, HTTP, fetch, header, credential, URL construction, timeout,
cancellation, decompression, or SSRF policy is supplied by this train. The host
adapter SHALL enforce its outer allocation/response budget before WASM ingress.
`response.text()` alone is not an allocation guarantee; a concrete transport
adapter must cap/stream bytes at its earliest boundary. The public facade SHALL
accept structured options only, not caller-supplied raw option JSON.

The TypeScript projection SHALL be strict, deeply immutable, iteratively
validated, and preserve W3C success/error/deactivation branches, optionality,
metadata, document/content distinctions, problem details, and bounded
extensions. It SHALL NOT expose generated types, explicit public `any`, caller
data in diagnostics, prototype-pollution shapes, or silently rounded integers.

One immutable vector packet and provider-neutral assertion body SHALL execute
on each applicable target. React SHALL reuse the browser/node providers and
contain no domain parser. Unknown binding impact selects all target jobs;
unselected targets emit inapplicable receipts; selected failures keep the
terminal aggregator red without changing the core proof.

The package remains private, opt-in, non-default, and unpublished. Local
measurements are not support budgets. No result may claim external SDK-TS
adoption, default routing, network/method coverage, React Native evidence,
publication, stable API, certification, production readiness, or mdoc.

## Consumer and product impact

The consumer-visible addition is a private capability-oriented resolution and
dereferencing API configured with an injected host effect. Existing DID syntax
and DID Document operations, Rust callers, external SDK-TS, and product routes
remain unchanged. A valid W3C operation error resolves to an immutable result;
invalid wire/domain data and host execution failure reject with distinct
stable SDK-owned errors.

## Scope limitations

The train covers generic options/request normalization, injected invocation,
result validation/correlation, projection, and evidence. It excludes concrete
method resolution, HTTP/fetch implementation, cache/retry/timeout policy,
registration, PRISM/Cardano effects, persistence, trust/authorization,
credentials, DIDComm, agent orchestration, React Native/UniFFI, publication,
release, and mdoc.
Native non-JSON dereferencing bytes are not represented by this train.

## Activation and rollback

Implementation begins only after strict research/constraint readiness, a
signed+DCO planning-only commit, and an exact issue #599 preimplementation
receipt. Local completion requires focused Rust/WASM/package/target/property/
fuzz/differential/resource evidence, full factory/Nix gates, a distinct
exact-diff review, and guarded archive evidence.

Rollback removes or disables the unreleased resolution host route, mapping,
vectors, and assertions as one unit while preserving merged syntax/document
facades and every existing Rust API. Provider selection occurs before
initialization; an operation never silently changes provider after failure.

## Evidence

Readiness is proved by `research-ready`, `constraints-ready`, strict OpenSpec
validation, the issue #599 exact-base preflight receipt, and the commands in
`research.md`. Target evidence remains local and partial until external
consumer rehearsal, accepted budgets, support, publication, and native-target
decisions are separately authorized.
