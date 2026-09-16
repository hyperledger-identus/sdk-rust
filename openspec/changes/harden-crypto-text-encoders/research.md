# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-17
Source retrieval date: 2026-09-17
Research blockers: none

## Problem and existing implementation

At base `508917b0416147668b9d12949eec987f0b82946b`, both codec types own a
canonical `String`. `FromStr` rejects text over `MAX_CRYPTO_TEXT_BYTES = 4096`
before decoding, but blanket `From<B: AsRef<[u8]>>` encodes without a retained
size check. For hex, 2,048 input bytes encode to exactly 4,096 text bytes. For
unpadded Base64url, 3,072 input bytes encode to exactly 4,096 text bytes and
3,073 encode to 4,098.

The workspace has five shipping Base64url encoding calls in `jwk.rs`; each
receives a fixed 32-byte coordinate/thumbprint. Hex has no shipping encoding
caller outside its module tests. `FromStr` in both modules currently decodes a
bounded string and re-enters the unbounded `From` implementation. The migration
will use one private trusted encoder for those already bounded paths and the
fixed 32-byte JWK paths.

Local consumer searches found no direct sdk-rust `HexStr::from` or
`Base64UrlStrNoPad::from` call in oxid, lace-id-portal, midnight-identity, or
neoprism. Neoprism has many calls to its own `identus_apollo` implementation;
those demonstrate migration shapes but are not changed by this SDK issue.
Apollo is Kotlin and has no Rust codec caller.

The current implementation was inspected at pinned source revision
`508917b0416147668b9d12949eec987f0b82946b`. The public facade boundary is the
SDK-owned `HexStr`, `Base64UrlStrNoPad`, and redacted crypto `Error`; dependency
types do not cross it.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Named `try_from_bytes` plus specific `TryFrom` implementations | `adopt` | Makes the byte boundary visibly fallible, supports borrowed/owned/array callers, checks before encoded allocation, and preserves the type/wire representation. | A future major API establishes a single generic bounded-bytes abstraction. |
| Keep blanket `From` and add only a fallible alternative | `reject` | The unbounded public path remains reachable, so the limitation cannot be removed and callers can bypass the new API. | Never while #298 requires closure of the exception. |
| Panic or truncate in `From` | `reject` | Turns hostile size into denial of service or data corruption and violates Rust conversion expectations. | Never. |
| Raise/remove the text budget | `reject` | Expands retained resource exposure and changes an existing security contract. | A separately authorized protocol requirement supplies new limits. |
| New codec dependency | `reject` | Existing `hex` and `base64` dependencies already provide canonical encoding; the missing behavior is SDK boundary policy. | Existing dependencies cannot represent the required profile. |

## Compatibility and dependency evidence

Removing the blanket `From` implementation is an intentional source migration,
not a silent semver-compatible change. The crate remains version `0.0.0`, is
unpublished, and its source-distribution policy prohibits external release.
Repository callers move atomically. Future consumers use
`try_from_bytes(value)?`, `TryFrom`, or parse canonical text with `FromStr`.
Successful canonical strings, equality/hash/display, decoded bytes, serde JWK
output, error variants, and stable redacted core errors remain unchanged.

No direct downstream mutation is required by current evidence. If a later
consumer appears before release, it receives a separate adoption issue rather
than widening this generic crypto crate.

`cargo tree -p identus-crypto --edges normal` confirms the direct codec
dependencies remain `hex 0.4.3` and `base64 0.22.1`; the complete resolved
dependency cone is unchanged because no package or feature changes. The
workspace and crate declare Rust 1.98.1 as the current MSRV/etalon policy for
this development phase.

## Security, privacy and maintenance evidence

The raw-byte ceiling is checked before encoding allocates the retained string.
Checked length arithmetic prevents integer wrap. Errors disclose only codec,
limit, and numeric actual size; input bytes never enter Debug/Display. The
private trusted encoder is callable only inside `identus-crypto` and is limited
to bytes already proven bounded by parsing or fixed-size internal key material.

Supply-chain evidence is unchanged: no dependency, feature, build script,
unsafe code, native library, license, advisory, registry source, or lockfile
change is introduced. Existing pure-Rust codec support continues across Linux,
WASM, iOS, and Android. Maintenance remains in the cohesive codec modules;
release and publication remain prohibited, and no certification or support
claim changes. Protocol or draft currency is not applicable to canonical RFC
4648 Base16/Base64url encoding; the existing supported profiles do not change.

## Normative sources

- Sponsor-directed issue #298 and accepted ADR 0125.
- Public primary source URL: [RFC 4648](https://www.rfc-editor.org/rfc/rfc4648).
- Archived change `2026-09-08-bound-crypto-text-codecs` and canonical crypto
  spec for the existing parser/text budget.
- Existing workspace `hex` and `base64` crates at the locked registry versions;
  no code or fixtures are imported.
- Local source inventory of sdk-rust plus oxid, lace-id-portal,
  midnight-identity, neoprism, and Apollo on 2026-09-17.

License and provenance remain the repository Apache-2.0 implementation plus
the already locked registry packages (`hex` MIT/Apache-2.0 and `base64`
MIT/Apache-2.0). No donor source or fixture is copied.

## Rejected or deferred candidates

The candidate table records the rejected unbounded compatibility path, panic or
truncation, changed budget, and unnecessary dependency. A general bounded-bytes
abstraction is deferred until multiple consumers demonstrate a reusable domain
contract; adding one here would couple two simple codecs to speculative policy.

## Open questions and blockers

No blocker remains. The source break is explicitly authorized by issue #298
before publication, and the named downstream inventory found no sdk-rust call
site requiring coordinated mutation.

## Evidence commands

Commands run before implementation: repository and named-consumer `rg`
inventories; `cargo tree -p identus-crypto --edges normal`; issue, ADR 0125,
archived codec change, canonical spec, source-distribution policy, constraint,
and input-boundary review. Commands intentionally unrun before implementation
are new exact/one-over tests, public API/SBOM checks, minimal/all-feature and
workspace tests/Clippy, supported-target/Nix closure, final factory checks,
fresh review, and protected exact-head CI.

## Evidence plan and rollback

Exact/one-over tests cover borrowed slices, owned vectors, and arrays; ordinary
vectors prove canonical parity; error tests prove redaction and precedence.
API/source-distribution evidence proves the blanket `From` implementation is no
longer public. Workspace tests/strict Clippy, minimal/all features, supported
target/Nix checks, factory gates, and exact-head CI remain required.

Rollback restores the blanket `From` implementations, call sites, public docs,
and the codec clause in `SDK-LIM-007` atomically. No persisted or wire data
migration is needed.
