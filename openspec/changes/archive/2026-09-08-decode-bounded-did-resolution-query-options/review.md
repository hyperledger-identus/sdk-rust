# Exact-diff local review

Review status: completed
Review date: 2026-09-08
Implementation head: 0efc668
Specification parent: 52db3110263bafc56cf5aefb49f57b4d96879fca
Unresolved blockers: none

## Scope reviewed

The review inspected the complete base-to-implementation diff, four public
limit constants, the private decoder and every changed/new test. It reconciled
the behavior with issue #203, ADR 0091, the W3C GET binding, RFC 3986, existing
`ResolutionOptions`, the generic DID URL decoder oracle and ADR 0083. Cargo
manifests and lockfile were compared byte-for-byte through Git.

## Findings

1. **Architecture and cohesion — accepted.** All HTTP query behavior remains
   private to the outer adapter. DID Core and resolver ports do not depend on
   Axum, and no prematurely generic codec API is introduced.
2. **Public and wire compatibility — accepted.** The router signature and
   existing no-query responses remain unchanged. Only four positive ceiling
   constants are additive. Valid queries now reach the resolver; rejected
   queries retain the static W3C 400 envelope.
3. **Parsing and injection boundary — accepted.** The raw 8 KiB ceiling and
   32-member cap precede proportional work. Raw structure is split before one
   strict percent decode, so encoded delimiters cannot inject fields. Literal
   plus is preserved and malformed UTF-8/escapes/controls fail closed.
4. **Ambiguity and typed projection — accepted.** A bounded decoded-name set
   rejects literal or encoded duplicates. Known fields use exact boolean and
   version validators; simultaneous version selectors and query `accept` are
   rejected. Unknown values remain strings and pass the existing extension
   facade.
5. **Representation composition — accepted.** Full-result selection leaves
   option `accept` absent while preserving all query fields. Document selection
   combines the negotiated media type with the same fields. Both are proven by
   end-to-end resolver recordings.
6. **Resource and privacy safety — accepted.** Name/value ceilings are checked
   during bounded allocation; aggregate decoded content remains below the raw
   ceiling. No caller text enters errors or production diagnostics. The
   resolver is never called on invalid input.
7. **Dependency and unsafe posture — accepted.** No manifest or lockfile byte
   changed. Production source adds no unsafe, panic, unwrap, expect, network or
   runtime path. ADR 0083's negative form-codec decision remains appropriate.
8. **Test quality — accepted.** Twenty focused tests cover exact and one-over
   boundaries, common/extension projection, representation merge, literal
   plus, encoded delimiters, duplicates, malformed encoding/types/conflicts,
   no-call and redaction while retaining the original adapter suite.

## Residual limitations

- GET extension values are strings; no undocumented JSON scalar inference is
  performed.
- Query `accept` is rejected and must be expressed through the header.
- Method-specific validation and the operational cost of `noCache=true` remain
  resolver/host responsibilities.
- DID URL dereferencing, POST, OpenAPI, deployment controls, publication and
  downstream adoption remain under parent #10.
- The local shell lacks Nix/nextest/cargo-deny, and its old cargo-audit cannot
  parse CVSS 4.0; hosted fast and scheduled slow gates remain authoritative.

## Review decision

The implementation is cohesive, bounded and independently reversible. No
unresolved correctness, architecture, protocol, security, privacy,
compatibility or changed-dependency blocker remains for hosted review.
