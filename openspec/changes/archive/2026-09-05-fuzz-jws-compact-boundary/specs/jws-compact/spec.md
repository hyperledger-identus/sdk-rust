## MODIFIED Requirements

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

## ADDED Requirements

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
manual triggers while the reserved empty `main` remains the default branch.

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
