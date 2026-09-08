# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

`identus-core::Url` owns a `String` and uses the `identus-derive` validated
newtype surface. `parse`/`FromStr`, `try_new`, `TryFrom<String>` and serde
deserialization all reach the same `validate_url(&str)` function. That function
checks scheme grammar, the `://` separator and a non-empty authority, but has
no length check. Its searches and character traversal are linear in the input,
and a successful value retains the entire input allocation.

The type currently has no in-workspace runtime consumer beyond its own tests.
That makes the active-development compatibility cost observable and small, but
does not erase the public API event: adding a variant to the public closed
`UrlError` enum can break exhaustive external matches, and oversized values
that previously parsed will be rejected.

## Normative sources

- [RFC 3986](https://www.rfc-editor.org/rfc/rfc3986.html) defines the generic
  URI component grammar but does not establish one universal implementation
  size limit.
- [RFC 9110 section 4.1](https://www.rfc-editor.org/rfc/rfc9110.html#section-4.1)
  recommends that HTTP senders and recipients support URI lengths of at least
  8,000 octets in protocol elements.
- `crates/core/src/url.rs` is the complete current implementation and shared
  validation funnel.
- Existing SDK bounded lexical values measure UTF-8 bytes: `Did` uses 2,048,
  `DidUrl` and `Uri` use 4,096, and JWS compact input uses 65,536 by default.
- `SDK-SEC-003` requires explicit bounds on changed untrusted-input boundaries;
  `SDK-LIM-007` names the current `Url` gap without claiming other inherited
  boundaries are already audited.

The pinned repository source revision is
`beb0f24cec178121dca25ae7a5ff9585efda3ae5`. The protocol and draft currency
is RFC 3986 as the current URI generic-syntax Internet Standard plus RFC 9110
as the current HTTP semantics standard; this change does not adopt an expired
SSI draft or infer a generic URI ceiling from an HTTP recommendation.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| 8,192 UTF-8 bytes | `adopt` | Meets RFC 9110's 8,000-octet interoperability recommendation, uses a simple SDK budget, and bounds accepted representation/work. | A named non-HTTP consumer proves a larger requirement with its own end-to-end budget. |
| Exactly 8,000 bytes | `not-adopt` | Normatively sufficient for HTTP, but offers no material risk advantage over the conventional 8 KiB budget. | Repository policy standardizes exact decimal protocol limits. |
| 4,096 bytes | `not-adopt` | Aligns with DID URL/URI values but falls below RFC 9110's recommended HTTP support floor for this generic hierarchical URL type. | The type is narrowed to a non-HTTP profile with a 4 KiB contract. |
| 65,536 bytes | `not-adopt` | Bounds work but materially weakens denial-of-service protection without a consumer requirement. | A named protocol requires it and outer budgets remain safe. |
| Caller-configurable limit | `not-adopt` | Makes validity policy-dependent and complicates serde, equality and cross-language conformance for a foundational value. | A separate parser/value split is designed and justified. |
| Outer transport limit only | `not-adopt` | Leaves direct constructors and retained values unbounded and keeps the named inherited gap. | Never as the only defense; outer limits remain complementary. |

## Compatibility and dependency evidence

`MAX_URL_BYTES` is additive. `UrlError::TooLong` and the newly rejected input
class are public behavior changes. The workspace is unpublished active
development at version 0.1.0 and contains no external consumer conformance
claim for `Url`, so issue #193 authorizes the event before implementation.
Existing valid values at or below 8,192 bytes retain identical bytes,
formatting, equality, hashing and serialization.

The existing stable core error code `core.invalid_url`, capability `core` and
`ErrorKind::InvalidInput` remain unchanged. The local error gains a static
public message only; neither the input nor its length is copied into errors.

No dependency, feature, MSRV, target, license, FFI or native-code surface
changes. `identus-core` retains only its existing serde runtime edge and
build-time `identus-derive` edge.

The exact workspace version is unpublished `0.0.0`, Rust/MSRV is 1.98.1, and
the repository provenance and license remain Hyperledger Identus `sdk-rust`
under Apache-2.0. There is no feature selection on `identus-core`. Its direct
dependency cone is `identus-derive` plus `serde`; the resolved normal cone adds
only `proc-macro2`, `quote`, `syn`, `unicode-ident`, and `serde_core` through
those existing edges. No manifest or lockfile change is proposed.

The public compatibility event is the new closed-enum variant and rejected
oversized input class. Wire compatibility is unchanged for in-budget values;
oversized serde strings now fail validation. The facade boundary remains the
SDK-owned `Url`/`UrlError` API—no parser dependency or external URL type crosses
it.

## Security, privacy and maintenance evidence

The length check must run before `split_once`, `chars`, `find`, or any other
input-proportional syntax work. This bounds validation work for accepted and
rejected oversized values after a borrowed `&str` reaches the validator. For
owned construction the caller has already allocated the `String`; for serde,
the generic deserializer materializes the string before the newtype validation
hook runs. Therefore this change bounds accepted/retained `Url` size and the
validator's syntax work, but does not bound transport bodies, JSON parser
allocation, nesting, decompression or network behavior.

The limit is `str::len()` in UTF-8 bytes, not Unicode scalar count. ASCII tests
prove exact 8,192/8,193 boundaries. A multibyte test must create inputs whose
byte lengths cross the boundary while their character counts do not, proving
that every construction path applies byte semantics consistently.

Authored unsafe and native code evidence is unchanged: workspace
`unsafe_code = "forbid"` applies to `identus-core`, and the implementation uses
only safe `str` inspection with no FFI, build script, native library or network
access. Supply-chain evidence is therefore the unchanged locked dependency
cone and existing repository audit; no new package, license or advisory surface
is introduced.

Maintenance and release security posture improve because one inherited
unbounded accepted value becomes a named constant and tested invariant. The
limit is not a publication/MSRV promise beyond the current repository policy,
and release notes must preserve the compatibility event if publication begins
before this active-development history is consolidated.

## Rejected or deferred candidates

Adopting the `url` or `fluent-uri` crate would improve syntax coverage but is
orthogonal to resource budgeting, changes the dependency-free foundation and
was separately deferred for lack of a named `Url` consumer. Scheme allowlists,
normalization, DNS/network lookup and SSRF policy belong to adapters or product
policy, not this value-size change.

Rollback is atomic: remove the constant/variant/tests/spec requirement and
restore the named `Url` text in `SDK-LIM-007`. A later larger limit is a new
resource and compatibility decision, not an undocumented rollback.

## Open questions and blockers

No research blocker remains. The broader inherited-bound inventory stays open
under #168 and `SDK-LIM-007` even after this named gap is removed.

## Evidence commands

- `git rev-parse HEAD` pinned the source revision to `beb0f24`.
- `rg` repository searches established the one validation funnel, lack of
  runtime in-workspace consumers, existing limit constants and byte convention.
- `cargo tree -p identus-core --edges normal --prefix none` recorded the direct
  and resolved dependency cone before implementation.
- RFC Editor sources were retrieved on 2026-09-08.
- Unrun checks at readiness are the implementation-dependent focused core
  tests, serde and multibyte negatives, `cargo tree` equality, authored unsafe
  and native-code scans, workspace feature checks, complete Nix gate,
  exact-diff review and hosted Linux CI. These are tasks, not inferred results.
