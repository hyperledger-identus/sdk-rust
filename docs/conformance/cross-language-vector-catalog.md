# Cross-language vector catalog

The catalog at `cross-language-vector-catalog.toml` is the A1 source of truth
for portable behavior evidence. It does not replace crate-local tests or
domain catalogs. It gives downstream adapters a stable case ID, immutable
payload, provenance, authority, and exact Rust selector to consume without
copying or reinterpreting the evidence.

## Authority

Authority descends from `normative` to `identus-contract`,
`consumer-regression`, `implementation-regression`, and `exploratory`.
Agreement among language SDKs never promotes a consumer regression. A standard
or accepted Identus contract wins unless a reviewed versioned change says
otherwise.

## Packet format

Packets are closed JSON objects. Inputs are either a literal string or a
deterministic ASCII `prefix`/`repeat`/`count`/`suffix` construction. The latter
keeps exact resource boundaries reviewable without committing multi-kilobyte
string literals. Expected results contain portable components or a stable error
code and redaction requirement.

The first packet, `did.syntax.v1`, covers generic DID/DID URL lexical behavior
only. It intentionally excludes DID methods, documents, resolution, networks,
credentials, and protocols.

## Validation and consumption

Run:

```console
python3 scripts/check-cross-language-vectors.py
python3 scripts/tests/cross-language-vectors.py
cargo test -p identus-conformance --test cross_language_did_vectors
```

Normal validation is offline. The catalog pins donor revisions and source
locations but never fetches them. A consumer should select cases by stable ID,
verify the checked-in packet digest, apply its language mapping contract, and
report exact selectors through the consumer change ledger.

Stable IDs are never repurposed. A semantic correction creates a new packet or
replacement record with explicit lifecycle links.
