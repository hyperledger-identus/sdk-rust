# Research readiness

Research class: foundational
Research status: ready
Decision date: 2026-09-08
Source retrieval date: 2026-09-08
Research blockers: none

## Problem and existing implementation

The current implementation at the pinned revision is fully contained in
`crates/crypto/src/{hex,base64,jwk}.rs` plus its crate tests.
`HexStr` and `Base64UrlStrNoPad` each own a canonical `String`. Their
`FromStr` implementations currently allocate a decoded `Vec<u8>` for the full
input and then allocate a second canonical string through the infallible
`From<bytes>` implementation. No byte check occurs before those allocations.
Their fields are private, so all parsed instances use these funnels.

The infallible `From<B: AsRef<[u8]>>` path is different: the caller already
owns trusted bytes and explicitly requests their text encoding. Making that
path fallible would break the existing primitive API and fixed-width JWK,
thumbprint and key constructors without improving an untrusted parsing
boundary. It remains unrestricted and is documented as caller-budgeted.

`PublicKeyJwk::from_parts` parses `x` and `y` through
`Base64UrlStrNoPad::from_str`, then checks for exactly 32 decoded bytes. The
generic JSON deserializer has already allocated its `String` fields before this
function runs. The codec ceiling can bound subsequent decode/re-encode work,
but cannot claim to bound transport bodies, JSON nesting, extension values or
the earlier string allocation.

The largest codec-shaped string literal found in sdk-rust is 192 characters;
the largest in the current Apollo and NeoPRISM Apollo ports is 144 characters.
Supported JWK coordinates are 43 characters, secp256k1 public keys and
signatures are below 150 characters, and encoded derivation fixtures are below
200. These inventories are compatibility evidence, not a claim that literals
enumerate every future consumer.

## Normative sources

- [RFC 4648 sections 3.3 and 3.5](https://www.rfc-editor.org/rfc/rfc4648.html#section-3.3)
  require rejection of non-alphabet characters by default and define canonical
  pad-bit behavior; sections 5 and 8 define base64url and base16. The RFC does
  not impose one universal encoded-message size limit.
- [RFC 7517](https://www.rfc-editor.org/rfc/rfc7517.html) defines JWK values and
  delegates `BASE64URL(OCTETS)` to JWS; it does not define an implementation
  resource ceiling for generic codec strings.
- [RFC 8037 section 2](https://www.rfc-editor.org/rfc/rfc8037.html#section-2)
  requires OKP `x` to contain a base64url-encoded public key and excludes `d`
  from public keys. Its Ed25519 and X25519 examples use 32-byte public values,
  matching the SDK's supported profiles.
- `openspec/specs/crypto/spec.md` already limits the public COSE Key parser to
  4,096 bytes and supported JWK coordinates to exactly 32 decoded bytes.
- `SDK-SEC-003` requires explicit resource limits at materially changed
  untrusted-input boundaries; `SDK-LIM-007` retains the incomplete inherited
  audit until #168 proves repository-wide coverage.

The pinned starting revision is
`92069a155f80bae7c4e54f59d7e93d35e37fd867`. RFC 4648, RFC 7517 and RFC 8037
are stable Standards Track inputs rather than mutable SSI drafts. No external
crate version or registry state is changed by this slice.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| 4,096 encoded UTF-8 bytes | `adopt` | Matches the existing crypto COSE boundary, leaves over 20× headroom above all inventoried donor/current fixtures, and bounds peak decode/canonicalization work. | A named standards-backed consumer proves a larger representation and an end-to-end outer budget. |
| 8,192 bytes | `not-adopt` | Matches the generic URL ceiling and fuzz input maximum but doubles accepted codec work without a crypto consumer need. | A reusable crypto format between 4 KiB and 8 KiB is accepted into this API. |
| 1,024 bytes | `not-adopt` | Covers current key and signature fixtures but gives little headroom for generic digest/vector uses and creates avoidable compatibility risk. | The wrappers are narrowed to fixed-width key material only. |
| Caller-configurable parser limit | `not-adopt` | Makes validity policy-dependent and complicates `FromStr`, serde use and cross-language conformance. | A separate streaming decoder API is designed for large caller-profiled payloads. |
| Bound infallible byte encoding too | `not-adopt` | Would turn trusted primitive conversion into a fallible API and break fixed-width constructors; callers already own that allocation. | The type is redesigned as a uniformly bounded protocol value rather than a generic encoding wrapper. |
| Preflight JWK at exactly 43 characters | `defer` | The generic ceiling already bounds decode work; early exact-width rejection would change the current distinction between malformed encoding and valid wrong decoded width. | A dedicated encoded-width JWK error contract is approved, or allocation-free validation preserves existing error semantics. |
| Validate text without decoding | `defer` | Could remove the temporary decoded allocation, but reproducing canonical trailing-bit rules adds custom codec logic outside this bounded change. | Profiling shows the bounded allocation is material or the dependency exposes a reviewed validation-only API. |

## Compatibility and dependency evidence

`MAX_CRYPTO_TEXT_BYTES` is additive. Rejection of longer `FromStr` input is a
public behavior change; it is directed by issue #197 during unpublished active
development. The existing closed `Error` enum does not gain a variant: an
internal length error is boxed in `Error::KeyParsing`, preserving callers'
existing category and `source()` behavior. Its stable bridge remains
`crypto.key_parsing`, `ErrorKind::InvalidInput`, capability `crypto`, and the
static public message `key parsing failed`.

Inputs at or below 4,096 bytes retain canonical output, equality, hashing,
display and decoding. Hex remains case-insensitive on input and lowercase on
output. Base64url remains URL-safe, unpadded and canonical. A value created
from more than the parser budget through infallible byte encoding remains
valid and decodable; parser round-trip for such caller-created values is not
promised.

No Cargo manifest, lockfile, feature, target, MSRV, FFI, license or native-code
surface changes. The existing `hex` 0.4.3 and `base64` 0.22.1 dependencies
remain behind SDK-owned types. Apollo and NeoPRISM are read-only conformance
sources and retain the same unbounded donor behavior; no downstream repository
is modified or claimed migrated.

The direct dependency edges used by this slice are the optional workspace
`hex = "0.4"` and `base64 = "0.22"` declarations, activated by the existing
`hex` and `base64` features; JWK additionally uses the existing `serde` and
`serde_json` edges. The locked resolved dependency cone is unchanged: the
exact selected codec packages are `hex` 0.4.3 and `base64` 0.22.1, while the
default-feature crate graph remains the recorded `cargo tree --locked`
closure. No package is added or removed.

License and provenance remain Apache-2.0 in the Hyperledger Identus sdk-rust
repository at the pinned revision. No donor source is copied. Supply-chain
evidence remains the unchanged lockfile, existing `cargo-deny` and audit gates,
and SDK-owned facades that prevent third-party codec types from crossing the
public boundary. Public and wire compatibility are unchanged for in-budget
values; only the explicitly documented oversized parse class is rejected.

## Security, privacy and maintenance evidence

The limit check must be the first input-dependent operation in each `FromStr`
implementation. `str::len()` measures UTF-8 bytes in constant time. Valid and
malformed oversized text therefore receives the same resource-first rejection
before decoder allocation or alphabet/canonicality inspection. The private
length error may report the static codec name plus maximum and actual byte
counts for local diagnostics, but must not retain or print input contents.

At the accepted maximum, hex can decode to 2,048 bytes and base64url can decode
to 3,072 bytes. The subsequent canonical string is at most 4,096 bytes. This
is a small deterministic envelope compared with the 4,096-byte COSE parser and
8,192-byte public JWK fuzz input ceiling. The caller-supplied `&str`, an owned
source `String`, JSON body and serde field may already be allocated, so outer
transport, decompression, body, nesting and field limits remain mandatory.

No secret is logged. `HexStr` and `Base64UrlStrNoPad` may carry key material,
so tests use repeated public bytes and sentinels, never production secrets.
Authored unsafe remains forbidden and no new dependency, build script, network
operation or native library is introduced. Maintenance cost is one shared
public constant, one private error type and symmetric boundary tests.
The maintenance, release and security posture therefore changes only by
making the accepted parser envelope explicit; no publication or target support
claim is activated.

## Rejected or deferred candidates

Replacing `hex` or `base64` is not justified: both dependencies already satisfy
the required canonical codecs, and the defect is missing SDK boundary policy,
not absent algorithm support. A streaming codec would be a separate API for
large payloads and is not needed by keys, signatures, digests or current
derivation vectors. Fixed-width wrapper types may later improve semantic
cohesion but are outside this resource-only change.

Rollback is atomic: remove the shared limit/checks/tests/spec clause and restore
the codec wording in `SDK-LIM-007`. Raising the budget is a new material
decision; lowering it requires named consumer compatibility evidence.

## Open questions and blockers

No blocker remains. #168 stays open for the rest of the inherited boundary
inventory, and #9 stays open for its broader Apollo/downstream acceptance.

## Evidence commands

- `git rev-parse HEAD` pinned the starting revision to `92069a1`.
- `rg` searches over sdk-rust, Apollo and NeoPRISM located every wrapper/JWK
  consumer and measured codec-shaped literal maxima of 192, 144 and 144
  characters respectively.
- `cargo tree`/manifest inspection established that no dependency change is
  required and that the wrappers remain SDK-owned facades.
- RFC Editor sources were retrieved on 2026-09-08.
- Unrun implementation evidence is exact/one-over focused tests, valid and
  malformed oversize precedence, canonicalization, JWK inheritance, error
  redaction, dependency/feature equality, unsafe/native scans, full Nix gates,
  exact-diff review and hosted Linux CI.
