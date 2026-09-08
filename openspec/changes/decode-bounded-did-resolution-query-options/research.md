# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation in `identus-did-resolver-http` already validates
path input, negotiates three
representations and injects `Arc<dyn DidResolver>`. At base revision
`52db3110263bafc56cf5aefb49f57b4d96879fca`, it returns `invalidOptions` for
every non-empty `RawQuery`. DID Core already owns immutable typed fields for
`accept`, `expandRelativeUrls`, `noCache`, `versionId`, `versionTime` and a
bounded extension map, so the missing behavior is transport decoding only.

The repository's generic DID URL dereferencer contains a private strict query
decoder worth using as a behavioral oracle. It splits structure before one
percent-decoding pass, preserves literal `+`, rejects malformed escapes and
invalid UTF-8, rejects duplicate decoded names, accepts only exact boolean
spellings and feeds typed SDK constructors. It is not extracted into a public
utility because HTTP resolution options and DID URL parameters have different
error types, limits and ownership.

NeoPRISM revision `d4608fe3d662ebaf425c4a6380f6d241cffe74c3`
remains an Apache-2.0 adapter-shape oracle, but its HTTP resolver ignores query
options. It supplies no query implementation to adopt.

## Normative sources

- [W3C DID Resolution v1](https://www.w3.org/TR/did-resolution/) is the
  current Candidate Recommendation Draft published 2026-08-28. The pinned
  source revision is
  [`w3c/did-resolution@71a50058`](https://github.com/w3c/did-resolution/tree/71a50058090417f9947b9f13985fc8b561a4ad59).
- [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986.html) defines URI query and
  percent-encoding semantics.
- ADR 0083 records the source-level assessment of `form_urlencoded 1.2.2` and
  its `percent-encoding` dependency.

The W3C GET binding requires every resolution option except `accept` to be
encoded as a request-URL query parameter. Resolution options are extensible;
common fields are `expandRelativeUrls`, `versionId` and `versionTime`, while
`noCache` is the optional caching control. The standard defines parameter
names as case-sensitive, warns about normalization/cache/injection divergence,
and identifies duplicate scalar names as ambiguous. `versionId` and
`versionTime` are mutually exclusive.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Existing DID URL decoder | base SHA above | `oracle` | Strict single-pass behavior and SDK types match, but ownership, errors and limits differ. | A second identical transport consumer justifies a shared private crate. |
| Private HTTP decoder | local | `retain-local` | About one bounded scan, exact URI semantics and no new dependency/cone. | The code grows beyond a narrow auditable boundary or multiple consumers need the exact contract. |
| `form_urlencoded` | 1.2.2 | `not-adopt` | WHATWG form semantics convert `+` to space, retain malformed `%`, and decode UTF-8 lossily; ADR 0083 has exact provenance. | A strict non-lossy release matches all negative vectors and removes meaningful code. |
| `serde_urlencoded` | 0.7.1 | `not-adopt` | Builds on form semantics and serde coercion, obscuring duplicate handling and the string-only extension contract. | A standards change selects HTML form encoding or a broader typed consumer appears. |
| `url` | 2.x | `not-adopt` | Broad URL/IDNA dependency cone for an already-extracted raw query; query pairs use form-style `+` behavior. | The adapter gains a named need for full URL parsing. |

## Compatibility and dependency evidence

The public API and router signature remain unchanged, and the wire
compatibility delta is explicit: empty/no query requests keep current behavior;
valid non-empty queries change from blanket 400 to one resolver invocation with
typed options; malformed queries keep the same redacted
`invalidOptions`/400 envelope. The facade boundary remains Identus-owned
`ResolutionOptions`; no candidate type crosses it. Rollback restores blanket
rejection without migration.

No Cargo entry changes. The direct and resolved dependency cone is byte-for-
byte unchanged, so candidate versions and features are not applicable to the
integrated graph. The Rust 1.98 MSRV/etalon, host-only target evidence and
existing fast/weekly target matrix remain authoritative.

## Security, privacy and maintenance evidence

The raw query is bounded at 8 KiB before parsing and at 32 parameters while
splitting. Each name is capped at 256 decoded bytes and each value at 4 KiB.
Structural splitting happens on raw `&` and the first raw `=` before exactly
one percent-decoding pass, so encoded delimiters cannot inject fields. Missing
`=`, empty names, controls, malformed escapes, invalid UTF-8, duplicate decoded
names and a query `accept` all fail closed. Literal `+` stays `+`; this is an
RFC 3986 URI query, not an HTML form body.

Known booleans accept only `true` and `false`; version values use existing SDK
validators. Unknown options become `serde_json::Value::String` entries and are
revalidated by `ResolutionOptions`, preventing the transport from inventing
extension-specific scalar types. Empty extension values remain valid scalar
strings; method resolvers own any stricter semantics. Simultaneous version
selectors fail at this HTTP boundary.

Every rejection occurs before resolver invocation and uses a static standard
error result. Query bytes never enter errors or logs. `noCache=true` can cause
expensive downstream work, but support/rejection belongs to the injected
resolver, which can return `featureNotSupported`; host abuse controls remain a
documented deployment responsibility.

No dependency, unsafe, native-code, license or advisory surface changes. The
existing lockfile supply-chain evidence therefore remains applicable. The
current maintenance, release and security posture does not change. The W3C
protocol source is still a draft and requires currency review before
publication/certification.

## Rejected or deferred candidates

`form_urlencoded 1.2.2`, `serde_urlencoded 0.7.1` and `url 2.x` are
`not-adopt` for the reasons in the candidate table. Public shared-codec
extraction is deferred until a second consumer needs the exact same contract.

## Open questions and blockers

No research blocker prevents implementation. Stop rather than merge on a
lossy decoder, form-style plus conversion, unbounded allocation, duplicate
precedence, public parser/candidate types, leaked query data, dependency graph
change, altered no-query bytes or resolver invocation on rejected input.

No open question changes the bounded implementation. Generic core enforcement
of version-selector mutual exclusion is explicitly deferred because it would
change local callers beyond this HTTP transport issue.

## Evidence commands

Exact commands already run are `scripts/factory doctor`, repository `rg` and
`sed` inspection, Git revision/status inspection, and retrieval of the official
W3C source. Implementation checks remain intentionally unrun until code exists:
focused tests, full workspace tests, Rust 1.98/Nix, `cargo deny`, `cargo audit`
and hosted CI. Verification will record exact counts and distinguish every
unrun or host-incompatible check.
