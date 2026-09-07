# Semantic review

- **Reviewer:** Codex fresh contract pass
- **Date:** 2026-09-07
- **Scope:** issue #141, OpenID4VCI Final section 8.3, proposal, design,
  capability delta and complete IDR-023 replacement
- **Result:** no unresolved blocker before implementation

## Findings

1. **Cleared — immediate/deferred validity was initially easy to conflate.**
   The public type is explicitly immediate-only and a `transaction_id` returns
   an unsupported-deferred error. The contract does not claim to parse every
   successful Final response.
2. **Cleared — opaque credentials need usefulness without format ownership.**
   Exact JSON is retained for strings and objects, representation kind is
   explicit, and string values additionally expose the decoded semantic value.
   No base64url or credential-format rule enters this crate.
3. **Cleared — extension tolerance can create resource and smuggling risk.**
   Unknown values are accepted only under full JSON byte/depth/node limits;
   explicit object-member caps bound name tracking, and duplicate names fail.
4. **Cleared — parsing could be mistaken for trust or transport evidence.**
   The type is a body core only, documentation names the missing HTTP,
   provenance, verification, correlation, notification and storage layers, and
   diagnostics retain no input.
5. **Cleared — legacy Portal wire shape conflicts with Final.**
   It is recorded as incompatible evidence and is neither copied nor accepted.
   Final normative text and Oxid's array shape control this slice.

## Gate decision

The change is additive, reversible, issue-linked, chain-neutral, and within the
standing IDR-023 mandate. No protected product, governance, release, licensing,
security-disclosure, repository-administration or downstream-write boundary is
crossed. Implementation may proceed.

# Exact-diff implementation review

## Review boundary

- Base: `2f5663abae7a11fd8ec43c4c4e20ead9bdd4afc3`
  (`origin/develop`).
- Reviewed implementation head:
  `e6583b74ca664e01784295c9b9045da9b4f86824`.
- Reviewed every changed path and the complete `origin/develop...HEAD` diff,
  including public types, bounds, scanner integration, zeroization, errors,
  tests, ADR, blueprint, inventory, backlog replacement and OpenSpec artifacts.
- Confirmed `Cargo.toml`, `Cargo.lock`, and `crates/oid4vci/Cargo.toml` are
  byte-identical to the base.

## Findings

1. **Verified — immediate and deferred branches cannot be conflated.** A
   syntactically valid `transaction_id` selects the explicit unsupported
   result even when `credentials` is also present; `interval` alone is invalid.
2. **Verified — credential payloads are opaque and byte-faithful.** String and
   object values retain their exact validated JSON spelling and order; strings
   additionally expose their decoded semantic value without interpreting a
   credential format.
3. **Verified — remote input is independently bounded.** Positive whole-body,
   depth, node, response-member, entry-count, entry-member, per-credential,
   aggregate-credential and notification limits cover parsing and retention.
4. **Verified — extension and duplicate behavior fails safely.** Unknown
   values are fully syntax- and resource-validated before discard. Escaped or
   literal duplicate member names fail at the response, entry and nested
   object layers.
5. **Verified — sensitive material is explicit and redacted.** Credential and
   notification allocations zeroize on drop, accessors carry sensitive names
   and warnings, and Debug plus fieldless errors reveal only shape and counts.
6. **Verified — parsing makes no trust or transport claim.** The public
   contract excludes HTTP validation, correlation, encryption, credential
   format/signature verification, issuer trust, status, storage and
   notification delivery.
7. **Verified — compatibility and scope are additive.** Dependencies,
   features, manifests, lockfile, target policy, downstream repositories,
   chain-specific code, publication and release state are unchanged.

## Decision

No unresolved correctness, security, privacy, compatibility, provenance,
target, test, documentation or product-scope finding remains. The exact diff
is locally approved for guarded archive and ready PR delivery to `develop`.
