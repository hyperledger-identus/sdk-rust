# ADR 0083: retain the strict OID4VCI form codec

- **Status:** Accepted
- **Date:** 2026-09-08
- **Issue:** [#158](https://github.com/hyperledger-identus/sdk-rust/issues/158)
- **Supersedes:** ADR 0061's production-adoption disposition for
  `form_urlencoded`
- **Decision authority:** OID4VCI strict transport specifications and the
  consumer-payoff/parser-equivalence dependency gates

## Context

ADR 0061 selected `form_urlencoded 1.2.2` to replace local form encoding and
decoding. The candidate is a focused, maintained implementation of the WHATWG
browser algorithm, declares Rust 1.51, supports `no_std` plus allocation, and
has a two-package minimal normal dependency cone.

Exact source review exposed a semantic mismatch. The candidate parser:

- delegates percent decoding to `percent-encoding`, which returns a literal
  `%` when two following hexadecimal digits are absent; and
- calls `decode_utf8_lossy`, which replaces invalid UTF-8.

The SDK's credential-offer transport deliberately rejects malformed percent
escapes, raw non-ASCII input, decoded invalid UTF-8, decoded NUL and output over
the configured decoded-byte ceiling. Those negative behaviors are part of the
canonical protocol contract, not incidental implementation details.

The candidate serializer does match the SDK's literal set, space-to-plus rule
and uppercase `%HH` output. Serializer-only adoption still would not remove the
checked encoded-length calculation, preallocation, fixed field order,
secret-bearing `Zeroizing<String>` ownership or strict decoder. A direct
production dependency would therefore replace only a small loop while adding
another cone and reachable unsafe string conversion.

## Decision

Do not adopt `form_urlencoded 1.2.2` for the current OID4VCI boundary. Retain
the local bounded codec and its existing tests. No Cargo, API, error, request
byte or parsing behavior changes in this decision.

The candidate may remain a source-level differential oracle. Reconsider a
production dependency only if either:

1. a maintained release provides strict, non-lossy parsing compatible with all
   SDK malformed-percent, UTF-8, NUL, completeness and resource-limit rules;
   or
2. multiple current SDK protocol consumers need the same exact serializer and
   measurement shows that reuse removes meaningful implementation or
   correctness risk after checked sizing, deterministic order and secret
   ownership remain at the facade.

Any reconsideration requires a new issue, refreshed artifact and supply-chain
evidence, an ADR/spec update, negative differential tests and private
integration behind Identus-owned types.

This issue also generalizes the factory rule: parser replacement requires
accepted **and rejected** input parity. Nominal format support and valid-input
vectors are insufficient.

## Provenance

- crate: `form_urlencoded 1.2.2`, MIT OR Apache-2.0;
- crates.io checksum:
  `cb4cb245038516f5f85277875cdaa4f7d2c9a0fa0468de06ed190163b1581fcf`;
- packaged VCS revision:
  `91377f48bf35011d042aa5abef9e7f2a0a625aaa`, verified GitHub commit;
- package metadata: `dirty: true`, so the crates.io archive is authoritative;
- packaged `src/lib.rs` SHA-256:
  `766b5d679064e01f7e6cce6f127a23885f79806ce3bccc49e4fe41933b83fd8d`,
  matching the file at the recorded revision;
- minimal features: default features disabled, `alloc` enabled;
- minimal cone: `form_urlencoded` plus `percent-encoding`;
- declared compiler floor: Rust 1.51.

This provenance proves what was assessed. It is not a permanent security or
compatibility approval for a later release.

## Consequences

- OID4VCI keeps strict, bounded and redaction-safe malformed-input behavior.
- The SDK continues maintaining a small form codec rather than adding a narrow
  dependency whose useful subset has low current payoff.
- AI agents must compare negative parser behavior before replacement.
- Browser-compatible parsing remains available as an oracle but is not
  confused with the protocol's fail-closed boundary.

## Alternatives rejected

### Prevalidate, then call the candidate parser

This keeps the full security-sensitive scan, then performs another parse and
potential allocation. It adds complexity without deleting risky mechanics.

### Adopt only the serializer

Checked size calculation, field ordering, zeroizing allocation and strict
decoding remain local, so the dependency replaces too little code.

### Weaken the SDK parser to browser behavior

Accepting malformed escapes or replacing invalid UTF-8 contradicts the current
canonical specification and hides invalid issuer-controlled input.

## Verification and rollback

The source tree retains the exact codec implementation. Existing positive,
malformed-percent, invalid-UTF-8, NUL, boundary, ordering and redaction tests
must stay green. Cargo manifests and the lockfile must be byte-identical to the
base. Revert this focused decision PR to restore prior research wording; no
runtime or data migration is involved.
