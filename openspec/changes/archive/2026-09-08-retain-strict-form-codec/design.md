## Context

Two current OID4VCI seams implement form mechanics:

- credential-offer transport decodes one preselected query value with strict
  percent syntax, raw-ASCII, decoded UTF-8/NUL and per-byte capacity checks into
  zeroizing storage;
- pre-authorized token requests calculate exact encoded length before
  allocation, then serialize a fixed ordered field set into a zeroizing body.

`form_urlencoded 1.2.2` implements the WHATWG browser algorithm. Its encoding
surface matches the SDK's literal set and uppercase escapes. Its parser instead
uses `percent_decode`, which emits a literal `%` when the following two bytes
are not hexadecimal, and `decode_utf8_lossy`, which replaces invalid UTF-8.
That behavior is useful for browser compatibility but contradicts the SDK's
existing fail-closed protocol boundary.

## Goals and non-goals

Goals:

- stop an incorrect adoption before Cargo or behavior changes;
- preserve exact artifact/source evidence and a finite reconsideration trigger;
- teach the research gate to compare negative parser behavior explicitly;
- keep existing strict OID4VCI behavior unchanged.

Non-goals:

- refactor the local form helpers;
- weaken malformed-percent, invalid-UTF-8, NUL or capacity rejection;
- add `form_urlencoded`, `percent-encoding`, `url` or an HTTP runtime;
- change field ordering, percent-escape case or secret-bearing APIs.

## Decisions

### Retain the local codec

Classify `form_urlencoded 1.2.2` as `not-adopt` and the existing codec as
`retain-local`. Prevalidating every input before calling the permissive parser
would retain the security-sensitive scan while adding allocations and another
semantic layer. Adopting only `byte_serialize` would retain checked output-size
calculation, field order, zeroizing ownership and most of the current encoder.

### Treat the packaged crate as the assessed artifact

Pin the crates.io SHA-256 checksum and `.cargo_vcs_info.json` revision. The
package records a dirty source tree, so the published archive is authoritative.
The assessed `src/lib.rs` is byte-identical to the file at the recorded Git
revision; future research must still start from the new published artifact.

### Require negative-behavior parity for parser replacement

A candidate cannot replace a fail-closed SDK parser because it recognizes the
same nominal format. Research must compare malformed syntax, invalid text,
duplicates, truncation/trailing data and resource limits relevant to the
capability. A deliberate acceptance change belongs in a separately reviewed
specification.

## Risks and mitigations

- **Risk:** the SDK continues to own encoding/decoding code. **Mitigation:** the
  seam is small, bounded and covered by exact and negative vectors.
- **Risk:** local code drifts from the WHATWG serializer. **Mitigation:** retain
  the crate/source as an oracle and keep Appendix B/Unicode vectors.
- **Risk:** future agents reconsider from popularity alone. **Mitigation:** the
  negative ledger names the strict-parser and consumer-payoff triggers.

## Rollback

Revert the documentation-only change to restore the earlier portfolio wording.
There is no runtime, dependency or data migration. A later adoption requires a
new issue, refreshed research and an explicit ADR/spec update first.
