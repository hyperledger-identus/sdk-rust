## Context

The first credential capability must be useful to multiple format adapters
without prematurely defining the complete VC domain. Oxid's credential domain
at `bfe3b481568dc738f0732c2b27548fab8721fd95` proves the need for bounded
credential bytes, detached proofs, private format material, and redacted
formatting. Its wallet IDs, metadata, verification report, disclosure model,
and Midnight-only format enum are not generic inputs to this slice.

The implementation base is
`develop@02011c9ca61dc03502f16b89aca0347d79019ce5`. Oxid,
midnight-identity, Lace ID Portal, and other consumers are read-only. The donor
file and root license SHA-256 digests are recorded in issue #71. The crate is
unreleased with `publish = false`.

## Goals / Non-Goals

**Goals:**

- create one coherent envelope that any credential-format adapter can use;
- preserve caller bytes and format spelling exactly after validation;
- bound all owned input and keep artifact bytes out of formatting and errors;
- erase SDK-owned format-private material on explicit zeroization and drop;
- keep dependency and compile-time cost minimal.

**Non-goals:**

- credential IDs, profile IDs, issuer/subject metadata, claims, disclosure,
  verification stages, status, schemas, trust, storage, or lifecycle policy;
- serde, CBOR, JSON, JWT, SD-JWT VC, mdoc, VCDM, Compact, or proof codecs;
- an executable format-handler registry or a closed list of supported formats;
- FFI, publication, release, consumer adoption, repository settings, or
  changes to `main`.

## Decisions

### D1 — Activate the existing credential-semantics crate

Implement the bounded slice in the existing `identus-credentials` workspace
member. This preserves the enforced crate ring and avoids adding an empty or
speculative package. The name is experimental until namespace/release work
decides whether the published package is `identus-credentials` or
`identus-vc-core`; no compatibility promise exists before release.

### D2 — Open format identifier, no central enum

`CredentialFormat` owns a case-sensitive string of 1–128 ASCII bytes. The
first byte is alphanumeric; subsequent bytes are alphanumeric or one of
`.`, `_`, `+`, `-`, `:`. Parsing validates the borrowed input before allocating
and preserves accepted spelling exactly. This accepts current ecosystem tokens
such as `vc+sd-jwt`, `mso_mdoc`, and `midnight_cbor_phase1`, plus future
profiles, without modifying the SDK.

A handler registry is unnecessary for an artifact envelope and would couple
the first slice to codec, trait-object, concurrency, and feature-policy choices.
It remains a follow-up if two format adapters need matching runtime dispatch.

### D3 — Distinct opaque artifact types

Use `CredentialPayload`, `CredentialDetachedProof`, and
`CredentialPrivateMaterial` rather than a generic byte wrapper. The distinct
types prevent proof/body/private-material mix-ups at call sites and permit
different bounds and lifecycle contracts. A small internal validator keeps the
construction logic DRY without exposing a generic catch-all abstraction.

Payload and detached-proof bounds are 1 MiB each. Private material is bounded
to 256 KiB. Empty artifacts are rejected. Constructors accept already-owned
vectors and therefore reject oversized allocations rather than claiming to
prevent callers from allocating them.

### D4 — Redact all artifacts; zeroize private material only

Every artifact and the envelope use manual `Debug` implementations that show
only public format and lengths. Errors contain enum reasons only and bridge to
static `IdentusError` values.

`CredentialPrivateMaterial` implements `Zeroize` and `ZeroizeOnDrop`, clears a
transferred allocation before returning a construction error, and exposes
neither `Clone` nor ordinary equality. Payload and detached proof are not
automatically zeroized: they are long-lived encoded records and may contain
PII, but they are not secret key/opening material under this primitive
contract. Applying secret-buffer semantics to every credential copy would add
runtime cost without defining the storage/privacy lifecycle. Encrypted
persistence and PII deletion remain downstream responsibilities.

### D5 — No wire contract in the envelope slice

The types intentionally do not derive serde. A format codec owns its normative
wire representation; the envelope preserves its resulting bytes. This avoids
creating a second wrapper wire format before OID4VC and storage requirements
exist. Accessors return borrowed bytes. Any caller-created copy, including a
copy of private material, is caller-owned.

### D6 — Stable redaction-safe error bridge

The local non-exhaustive `CredentialError` enumerates invalid format, empty
artifact, and oversized artifact categories. `to_identus_error` maps them to
stable `credential.*` codes and the `credential` capability without retaining
or rendering caller input.

## Risks / Trade-offs

- An open identifier permits syntactically valid but unsupported formats. That
  is intentional: adapters decide support, while the core preserves identity.
- The first slice does not validate credential semantics or establish trust.
  Naming and documentation must not imply that an envelope is verified.
- Private material intentionally does not implement `Clone`, reducing accidental
  secret duplication. Callers can still create copies from the borrowed bytes;
  those copies are outside the SDK-owned lifecycle contract.
- The 1 MiB/256 KiB limits are SDK resource policy, not standard maxima; future
  evidence can revise them through a compatibility decision before release.

## Migration Plan

1. Land the contract and ADR before implementation.
2. Replace the marker with focused modules and tests; update inventory and
   crate-ring contracts in the same reversible slice.
3. Run focused Cargo checks, workspace/factory gates, full Nix validation, and
   a distinct API/security review.
4. Sync canonical specs, archive the change, and deliver a signed/DCO PR to
   `develop`.

Rollback is a source revert while the workspace remains unpublished. Consumer
adoption and any donor deletion are separate issues after an immutable SDK
candidate exists.
