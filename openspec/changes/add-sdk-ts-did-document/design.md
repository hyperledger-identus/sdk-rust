# Design

## Dependency direction

```text
identus-did::DidDocument
    -> identus-wasm-did binding API v2
        -> private web/node generated artifacts
            -> strict TypeScript document projection
                -> browser / Node.js / React web fixtures
```

Portable crates gain no inward WASM, npm, TypeScript, React, or host dependency.
React continues to own lifecycle only and contains no document parser.

## Rust and generated boundary

`parseDidDocument(rawJson)` calls `DidDocument::from_json_str`, rejects any
JavaScript-unsafe integer in the validated extension tree with a closed adapter
code, serializes the validated model deterministically, and returns an owned
`DidDocumentView { serialized, id }`. API version increases to 2 and reviewed
generated declarations/symbol inventories fail closed on drift.

The binding exposes only `did.invalid_document` and the explicit unsupported
lossy-number code for this operation. It never returns local `DocumentError`
text. The plain TypeScript projection copies everything before `free()` and
does not retain a generated reference.

## Typed projection

An iterative bounded parser/validator consumes only Rust-produced normalized
JSON. It builds frozen plain values for contexts, controllers, aliases,
verification methods, tagged relationships, services/endpoints, and extension
maps. Core and extension properties are separated in convenience DTOs while
`serialized` preserves the exact normalized wire representation.

The validator preserves absent/present and scalar/array distinctions. It
rejects unexpected generated shapes, unsafe numbers, excessive traversal, and
mutability escape as a binding-contract failure. Rust remains the final domain
validator; TypeScript checks generated-wire integrity rather than defining a
second DID Document grammar.

## Vectors and donor dispositions

The packet is source-bound to W3C DID Core, the Rust document tests, and donor
revision `4bf86e…`. Each donor journey records immutable path, test identity,
content hash, disposition, rationale, replacement evidence, and reconsideration
trigger. Generic donor parse/service assertions are adapted. Method creation,
resolver, DIDComm routing, signing-key, secret-custody, and cloud-agent E2E
journeys are excluded or platform-owned, never silently skipped.

One browser-neutral assertion module compares normalized JSON and typed values,
not object identity. Node ESM/CommonJS, Chromium, React SSR, and hydrated client
execution import it unchanged. React lifecycle checks remain separate.

## Fuzz, coverage, and resources

The `did_document` fuzz target accepts arbitrary bytes, attempts bounded parse,
and on success proves deterministic serialize/reparse equality and public-only
verification material. Corpus seeds cover representative, duplicate, private
JWK, exact/over-limit, relationship, service, extension, and hostile-depth
shapes. Per-target max length reaches the 256 KiB document ceiling.

Coverage inventories every public document operation and success/error/free
branch. Measurements extend the current target protocol with ordinary and
near-limit document latency, output size, initialization, and RSS. Values are
diagnostic only.

## Target isolation

Artifact assembly builds and compares independent `web` and `nodejs` outputs,
requires API v2 symbols, and copies a closed file inventory. Binding impact
routing includes the document fixture, mapping, quality declaration, fuzz
target, public API, and target assertions. Every selected target writes an
exact-head receipt; the existing terminal aggregator remains unchanged in
meaning.

## Delivery boundary

This branch is a local-first implementation train under issue #502. It may be
reviewed locally after the complete gate, but it is not pushed, published, or
activated by this change. The next external action requires an explicit user
decision after the local candidate and evidence packet are complete.
