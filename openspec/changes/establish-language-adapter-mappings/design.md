# Design

## Registry shape

`docs/architecture/language-adapter-mappings.toml` is the canonical machine
registry. Its closed top level owns schema/version/status/issue metadata and a
list of closed mapping records. The initial IDs are:

- `did.value.typescript.legacy-v1`;
- `did-url.value.typescript.legacy-v1`;
- `did.error.invalid-did.typescript.legacy-v1`; and
- `did.error.invalid-did-url.typescript.legacy-v1`.

Every record separates a canonical Rust endpoint from a pinned language
endpoint, then records conversion, compatibility, proof, policy, and lifecycle
metadata. Nested field, error, and unsupported-case records are also closed.

## Canonical and legacy semantics

The DID value mapping is bidirectional and lossless for accepted inputs.
Serialized DID derives SDK-TS `uuid`; `schema` is the fixed compatibility value
`did`; method and method-specific ID map directly.

The DID URL mapping is explicitly lossy. SDK-TS path arrays and parameter maps
do not preserve the canonical raw representation, duplicate/order semantics,
or absent-versus-empty query/fragment. The record requires a future adapter to
reject unsupported round trips or use a new additive shape; it may not mutate
`identus-did` to match the legacy DTO.

Both stable Rust failures map temporarily to SDK-TS `InvalidDIDString`. The
record preserves canonical `did.invalid_did` or `did.invalid_did_url` as the
machine-observable code while treating the donor class/message as a bounded
facade. Caller input remains redacted.

## Validation and rendering

`scripts/check-language-adapter-mappings.py` parses with Python `tomllib` and
rejects unknown fields, malformed IDs or revisions, noncanonical Rust sides,
missing direction/compatibility/fidelity/lifecycle metadata, incomplete
field/error coverage, lossy mappings without explicit unsupported behavior,
weakened bounds/redaction, invalid selectors, or incoherent version windows.

The same tool renders deterministic Markdown to
`docs/architecture/language-adapter-mappings.md`; normal checking fails if the
checked-in rendering drifts. Mutation tests alter every critical invariant and
exercise render determinism. Registry mutations use the render-only execution
path so their expected failure must come from schema validation rather than an
unrelated stale-Markdown check. Vector IDs are syntax-checked locally; parent
#504 later validates their cross-catalog resolution.

## Evolution

Schema v1 is closed and language-neutral. Additional languages add records,
not branches in Rust domain types. A mapping can progress through transitional
and deprecated phases only with its migration action, observation signal,
fallback, rollback, and removal gate intact. Semantic changes use a new record
version or explicit replacement.

## Stop boundary

This change ends at mapping metadata and review documentation. It does not add
an adapter implementation, generated binding, consumer test, or release claim.
Issue #492 later proves the first SDK-TS canary using these contracts.
