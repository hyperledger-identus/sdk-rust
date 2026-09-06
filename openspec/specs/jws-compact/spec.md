# jws-compact Specification

## Purpose

Define bounded, canonical JWS Compact parsing and construction with a closed
validated protected-header surface, byte-exact signing input, explicit
unverified states and redaction-safe diagnostics for reusable SSI profiles.
## Requirements
### Requirement: JWS Compact parsing is allocation-bounded and canonical

The SDK SHALL parse exactly three JWS Compact segments separated by exactly two
period characters. The protected-header and signature segments SHALL be
non-empty; the payload segment MAY encode an empty octet sequence. Parsing
SHALL reject a complete compact value above the configured bound before
decoding and SHALL reject each segment whose decoded-length estimate or actual
decoded length exceeds its configured header, payload or signature bound.

Every segment SHALL use RFC 7515 base64url encoding without padding,
whitespace, line breaks or non-alphabet characters. The parser SHALL require
decode-and-re-encode equality so alternate encodings do not cross the public
boundary. All size arithmetic SHALL fail closed on overflow.

#### Scenario: arbitrary bounded payload bytes survive parsing

- **WHEN** a three-segment compact value contains a valid protected header, an
  arbitrary payload within its byte bound and a non-empty bounded signature
- **THEN** parsing succeeds and returns the exact payload and signature octets

#### Scenario: empty payload remains RFC-compatible

- **WHEN** the payload octet sequence is empty and the compact value therefore
  contains an empty middle segment
- **THEN** parsing succeeds while an empty protected-header or signature
  segment is rejected

#### Scenario: encoded input cannot amplify allocation

- **WHEN** the total compact input, estimated decoded segment or actual decoded
  segment crosses its configured maximum
- **THEN** parsing fails through a static size error without returning partial
  data or echoing the rejected input

#### Scenario: base64url aliases fail closed

- **WHEN** any segment contains padding, whitespace, a non-URL-safe character,
  an invalid remainder or another representation that does not re-encode
  byte-for-byte
- **THEN** the whole compact value is rejected as non-canonical base64url

### Requirement: Protected headers have a small closed validated surface

The protected header SHALL be one complete UTF-8 JSON object containing exactly
one string `alg` member, at most one string `typ`, and at most one key reference
selected from string `kid`, public object `jwk`, or array `x5c`. Unknown or
duplicate members, trailing JSON, wrong JSON types, a missing `alg`, or more
than one key-reference member SHALL be rejected. `alg` SHALL contain 1 through
64 visible ASCII bytes, SHALL be case-sensitive and SHALL NOT equal `none`.
`typ` and `kid`, when present, SHALL be non-empty UTF-8 strings without Unicode
control code points and SHALL fit the configured protected-header string bound.

An inline `jwk` SHALL use the accepted validated public-only JWK type and SHALL
reject private key material. An `x5c` chain SHALL contain 1 through 8 non-empty
standard-base64 certificate strings, each fitting the configured header-string
bound. Encoding SHALL stop once the configured decoded protected-header byte
limit is exceeded rather than allocate the complete oversized serialization.

The codec SHALL NOT infer that an algorithm is registered, asymmetric,
supported or compatible with a key. It SHALL NOT validate certificate paths or
trust and SHALL NOT accept `crit`, `b64`, key-attestation or trust-chain
semantics until a focused profile owns their validation.

#### Scenario: consumer-shaped common headers are reusable

- **WHEN** a header supplies Ed25519, legacy EdDSA or ES256, an optional
  explicit proof type, and at most one DID URL `kid`, public JWK or X.509 chain
- **THEN** the codec preserves the validated header without applying DID,
  proof-type, algorithm-selection, certificate or trust policy

#### Scenario: ambiguous header input is rejected

- **WHEN** a header duplicates a member, supplies multiple key references, uses
  an unknown member or wrong JSON type, omits `alg`, adds trailing JSON, carries
  private JWK material, uses an invalid X.509 chain, or exceeds a bound
- **THEN** parsing or encoding fails through a static header or size error

#### Scenario: unsecured algorithm is not representable

- **WHEN** a caller constructs or parses a protected header with `alg` equal to
  the case-sensitive value `none`
- **THEN** the SDK rejects it before an unverified compact value is created

### Requirement: Exact signing input is preserved in explicit states

Parsing SHALL return an owned type named `UnverifiedCompactJws`. It SHALL retain
the exact accepted compact text and expose the signing input as the exact ASCII
bytes of the received protected-header segment, one period, and the received
payload segment. It SHALL NOT rebuild that input from decoded or reserialized
JSON.

Encoding SHALL first create a `JwsSigningInput` from a validated header and
arbitrary bounded payload. That state SHALL expose exact signing-input bytes.
Attaching a non-empty bounded signature SHALL produce an
`UnverifiedCompactJws` whose compact text uses canonical unpadded base64url and
whose accessors return the original payload and signature bytes. Neither state
SHALL claim cryptographic verification.

#### Scenario: RFC signing input remains byte-exact

- **WHEN** the RFC 7515 compact example is parsed
- **THEN** the exposed signing input and original compact text remain exactly
  byte-for-byte equal to the received values despite header formatting/order

#### Scenario: caller signs the prepared bytes externally

- **WHEN** a caller prepares a header and payload, signs the returned input
  through an external capability and attaches the resulting signature
- **THEN** the final compact value parses to the same header, payload,
  signature and signing input without this crate receiving a private key

### Requirement: Limits and failures are misuse-resistant and redaction-safe

`JwsLimits` SHALL require positive bounds for complete compact bytes, decoded
header bytes, decoded payload bytes, decoded signature bytes and protected
header string bytes. Its SDK defaults SHALL be 65,536; 4,096; 49,152; 1,024;
and 2,048 bytes respectively. Callers MAY select different positive bounds;
those values SHALL be observable without exposing compact content.

All `JoseError` variants SHALL contain no caller-controlled data and SHALL map
to static `jose.*` SDK errors under the `jose` capability. Debug output for
limits, signing-input state and unverified compact values SHALL omit compact
text, payload bytes, signature bytes and `kid`; it MAY expose counts plus
non-secret `alg` and `typ` metadata.

#### Scenario: invalid configuration cannot disable a bound

- **WHEN** any configured maximum is zero
- **THEN** limit construction fails and no parser/encoder can use that
  configuration

#### Scenario: canaries do not enter diagnostics

- **WHEN** a compact value, payload, signature and `kid` contain distinct
  canaries and success/error states are formatted
- **THEN** none of those canaries appears in Display, Debug or the core error
  bridge

### Requirement: Codec conformance and cost remain observable

The test suite SHALL cover the RFC 7515 compact example, independently
reconstructed Oxid and Lace ID Portal shapes, every exact lower/upper size
boundary, deterministic varied-length round-trips and every documented
rejection class. Tests SHALL prove that parsed values remain explicitly
unverified and that no production dependency on a donor or cryptographic
backend exists. The same public parser and builder invariants SHALL remain under
the separately bounded sanitizer campaign.

An ignored release diagnostic SHALL repeatedly parse a representative
proof-shaped compact value, validate the result and print elapsed time plus
operations per second. It SHALL use no machine-specific performance threshold
and SHALL not replace correctness checks.

#### Scenario: bounded matrix round-trips

- **WHEN** deterministic payload and signature values span zero or one byte
  through their configured representative bounds
- **THEN** canonical encoding and parsing preserve every byte and exact signing
  input

#### Scenario: maintainer measures parser cost

- **WHEN** the ignored diagnostic is run in release mode
- **THEN** it validates each parsed value and reports total iterations,
  elapsed time and parse throughput without a timing assertion

### Requirement: Sanitizer-backed JWS Compact boundary fuzzing

The JWS capability SHALL provide a sanitizer-backed target which accepts
arbitrary bytes, exercises complete UTF-8 input under SDK defaults and derives
bounded positive `JwsLimits` tuples for an independent suffix parse. Rejection
SHALL be valid. Every accepted value SHALL preserve exact compact and signing-
input bytes, contain only independently confirmed canonical unpadded base64url
segments, reparse equally and rebuild through the public staged encoder with
equal header, payload and signature semantics. Same-limit reparsing SHALL remain
exact; semantic rebuild MAY minimally widen representation-size limits to fit
the measured canonical protected-header output.

Every rejected input SHALL return a static codec-boundary `JoseError` with its
fixed `jose.*` bridge. Fuzz-only dependencies SHALL remain in the independent
workspace outside every published crate. The target SHALL NOT perform signature
verification or add proof, algorithm, trust, chain, custody or product policy.

#### Scenario: hostile token bytes cannot violate public invariants

- **WHEN** arbitrary bounded bytes contain valid, invalid, non-UTF-8, malformed,
  aliased, ambiguous, deeply structured, empty or over-limit JWS text
- **THEN** parsing rejects without panic through a static error or returns a
  value whose canonicality, exact-input and round-trip invariants hold

#### Scenario: hostile limit tuples remain bounded

- **WHEN** arbitrary prefix bytes select a caller limit combination
- **THEN** every derived maximum is positive and capped by the harness contract,
  and the suffix receives the same accepted/rejected invariant checks

#### Scenario: fuzz tooling stays outside the SDK graph

- **WHEN** production, minimal-feature, MSRV, mobile, WASM or downstream graphs
  are built
- **THEN** cargo-fuzz, libFuzzer, sanitizer and corpus support are absent from
  every published crate interface and dependency cone

### Requirement: Reproducible bounded JWS fuzz campaigns

The repository SHALL expose one documented JWS command interface for corpus
replay, fixed-seed smoke and time-boxed soak through the pinned sanitizer
compiler, runner and runtime. PR/push smoke SHALL fix the seed, run count, input
ceiling, per-input timeout, memory ceiling, corpus reload and worker count.
Maintainer-invoked local or externally scheduled soak SHALL remain
independently bounded. The `develop` workflow SHALL NOT claim GitHub schedule or
manual triggers while the reserved empty `main` remains the default branch. The
JWS PR/push workflow SHALL run for changes to JOSE and its transitive crypto
source.

An original reviewable corpus and dictionary SHALL cover the RFC example,
independently reconstructed Oxid/Lace shapes and existing structural,
canonicality, header and limit rejection families without donor fixture bytes.
Exact-text seeds MAY use a documented harness-only `text:` transport that
removes one repository line ending; unprefixed arbitrary bytes SHALL remain
unchanged.
The corpus SHALL seed at least one accepted derived-limit suffix and complete
encoded `kid`, public/private `jwk`, ambiguous-reference, and valid/invalid
`x5c` header families. A documented `limits:` prefix MAY occupy the ten limit
selection bytes and remove one repository line ending from its suffix.
Failures SHALL be retained as untrusted artifacts, minimized and promoted to
committed corpus plus a deterministic regression before merge. Execution time
MAY be recorded but SHALL NOT become a machine-specific threshold.

#### Scenario: ordinary JWS fuzz CI is repeatable

- **WHEN** the same revision runs the pull-request JWS fuzz gate
- **THEN** the target receives the same seed, run count and resource limits in
  the pinned Nix shell and terminates deterministically

#### Scenario: longer search remains bounded and diagnosable

- **WHEN** a local or externally scheduled soak finds a sanitizer or
  invariant failure
- **THEN** it stops within the documented envelope and preserves the input for
  minimization without custom logging of its bytes
