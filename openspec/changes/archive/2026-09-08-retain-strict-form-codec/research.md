# Research readiness

Research class: protocol
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation in `crates/oid4vci/src/transport.rs` performs a
single-pass strict form-value decode and rejects malformed percent escapes, raw
non-ASCII bytes, decoded NUL, invalid UTF-8 and output over the configured
decoded-byte limit. `pre_authorized_token_request.rs` computes the exact
encoded length with checked arithmetic before allocating, then writes a fixed,
ordered, secret-bearing form body into `Zeroizing<String>`.

The preliminary portfolio selected `form_urlencoded 1.2.2` to replace both
mechanics. Exact source review shows only serializer parity; the parser's
acceptance boundary is intentionally more permissive.

## Normative sources

- [WHATWG URL form-urlencoded parser](https://url.spec.whatwg.org/#urlencoded-parsing)
  defines the browser algorithm implemented by the candidate.
- [OAuth 2.0 RFC 6749 Appendix B](https://www.rfc-editor.org/rfc/rfc6749.html#appendix-B)
  references the HTML form encoding rules for OAuth request entities.
- [`form_urlencoded 1.2.2` documentation](https://docs.rs/form_urlencoded/1.2.2/form_urlencoded/)
  and published source define the candidate API.
- Existing canonical OID4VCI credential-offer and pre-authorized-token-request
  specifications define the stricter SDK protocol behavior.

Protocol or draft currency is not the mismatch: the SDK targets final
OID4VCI/OAuth behavior, while the candidate targets browser-compatible form
parsing. The issue is explicit strictness and security-policy ownership.

## Candidate decisions

| Candidate | Version/revision | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| `form_urlencoded` | 1.2.2 / packaged source; VCS `91377f48` | `not-adopt` | Parser preserves malformed `%` and replaces invalid UTF-8; serializer-only reuse has low payoff. | A strict non-lossy mode exists, or multiple consumers make measured serializer reuse materially reduce code/risk. |
| Existing bounded codec | `develop@61210a86` | `retain-local` | Small, strict, zeroizing, pre-sized and covered by final-profile vectors. | Complexity or consumer count grows enough for a candidate to delete meaningful risky mechanics without weakening behavior. |
| Candidate as differential source | 1.2.2 | `oracle` | Useful positive serializer comparison without a production dependency. | No trigger required; refresh source evidence when behavior is compared. |

## Compatibility and dependency evidence

The exact version and features assessed are `form_urlencoded 1.2.2` with
default features disabled and `alloc` enabled. It declares MSRV Rust 1.51. The
direct and resolved dependency cone has two normal packages:
`form_urlencoded` and `percent-encoding`. The crate is `no_std` plus allocation,
so target evidence indicates no inherent host-runtime coupling; no production
target build is claimed because the dependency is not added.

Public and wire compatibility remain exact because this decision changes no
Rust behavior. A future facade boundary would keep third-party types private,
but the current OID4VCI types already own bounds, errors, field ordering and
secret storage. Rollback is a documentation-only revert.

## Security, privacy and maintenance evidence

License and provenance are MIT OR Apache-2.0. The published crate checksum is
`cb4cb245038516f5f85277875cdaa4f7d2c9a0fa0468de06ed190163b1581fcf`;
its metadata records source revision
`91377f48bf35011d042aa5abef9e7f2a0a625aaa` and `dirty: true`. The packaged
`src/lib.rs` SHA-256 is
`766b5d679064e01f7e6cce6f127a23885f79806ce3bccc49e4fe41933b83fd8d`
and matches that revision's file.

Unsafe and native-code evidence: no native code exists in the minimal cone;
the candidate uses justified `from_utf8_unchecked` conversions after ASCII or
UTF-8 checks. The current local codec uses no unsafe. Supply-chain evidence is
the exact packaged checksum/source inspection and the parent portfolio's dated
advisory query, not a fresh audit pass. The maintenance, release and security
posture is active and narrow, but semantic mismatch and low payoff control the
decision.

## Rejected or deferred candidates

Production adoption is rejected for the current seam. Wrapping the permissive
parser with strict prevalidation is also rejected because it retains the local
security scan and adds a second allocation/semantic layer. Serializer-only
adoption is deferred until multiple consumers or code/risk measurements justify
the additional production cone.

## Open questions and blockers

No blocker remains for the decision correction. No implementation is
authorized. A future issue must resolve whether upstream strict parsing or
broader serializer demand satisfies the reconsideration trigger.

## Evidence commands

Exact commands and unrun checks are separated:

- repository searches located the two current form seams and their canonical
  specifications/tests;
- packaged `Cargo.toml`, `.cargo_vcs_info.json`, `src/lib.rs`,
  `percent-encoding` decode source and checksums were inspected locally;
- GitHub API resolved and verified source revision `91377f48`; the packaged and
  revision `src/lib.rs` SHA-256 values were compared and matched;
- `cargo test`, factory and Nix checks remain future tasks and are not claimed
  complete here;
- target, audit and dependency integration checks are unrun because no Cargo
  change is proposed; the existing parent evidence is retained as dated input,
  not represented as a new pass.
