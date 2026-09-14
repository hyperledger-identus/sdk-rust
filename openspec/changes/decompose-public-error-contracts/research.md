# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-14
Source retrieval date: 2026-09-14
Research blockers: none

## Problem and existing implementation

The inspected repository is `hyperledger-identus/sdk-rust` at protected
`develop@353030a7f263b9a1fba9deac0312ed228e61d761`. Issue #271 records the
maintainability baseline and issue #278 owns the smallest-surface pilot before
any broad migration.

`identus-credentials` exposes two fieldless, non-exhaustive error enums:

| Surface | Variants | Current projection | Current characterization |
| --- | ---: | --- | --- |
| `CredentialError` | 44 | A 176-line `to_identus_error` match plus a separate 56-line `Display` match | Five domain integration tests repeat partial code/kind/capability assertions |
| `CredentialVerificationError` | 3 | A `const` bridge match and a separate `Display` match in `verifier.rs` | One verifier integration test covers all three variants |

The 44 public `error::error_code::*` constants, both enum shapes, every local
display string, every public code/kind/capability/message projection, and the
absence of an error source are valid current behavior. The local display and
public message are deliberately different for several `Invalid*` variants;
normalizing them would be a compatibility change, not cleanup.

Neither enum implements Serde. The crate has no error wire schema or binding
projection. `CredentialVerificationError::to_identus_error` is currently
`pub const fn`; `CredentialError::to_identus_error` is currently non-const.
Those details are included in the compatibility boundary.

## Normative sources

- Issue [#271](https://github.com/hyperledger-identus/sdk-rust/issues/271)
  defines the outcome, acceptance and credentials-first staging.
- Issue [#278](https://github.com/hyperledger-identus/sdk-rust/issues/278)
  defines the independently revertible credentials delivery slice.
- Canonical `core-error-conventions`, `credential-core`,
  `credential-metadata`, `credential-status`, and `credential-verification`
  specifications define the existing two-surface and redaction behavior.
- `crates/core/src/lib.rs` defines the current `ErrorCode`, `ErrorKind`,
  `CapabilityId`, `IdentusError`, and `Display` contract.
- `crates/credentials/src/error.rs`, `verifier.rs`, and their integration
  tests are the exact behavior source for the planning golden fixture.
- ADR 0110 supplies cohesion, orthogonality, dependency-direction and
  public-boundary criteria. No external standard, donor repository, fixture,
  cryptographic source, or draft protocol applies to this internal refactor.

## Candidate decisions

| Candidate | Revision/version | Decision | Evidence and reason | Reconsideration trigger |
| --- | --- | --- | --- | --- |
| Private crate-local `ErrorContract` and domain catalogues | exact SDK base above | `adopt` | Makes code, kind, capability, local display and public message one reviewable record per variant while retaining crate ownership and the existing `identus-core` edge. | Pilot evidence finds worse readability or compile/runtime cost. |
| Keep the existing exhaustive matches | exact SDK base above | `not-adopt` | Behavior is correct, but repeated projection and split partial tests are the issue's measured review problem. | The selected pilot fails its objective and no simpler local representation works. |
| One shared workspace error crate or public trait | not applicable | `not-adopt` | Creates a new dependency/release axis and central ownership for domain contracts; issue #271 explicitly forbids a central god error crate. | Multiple completed pilots prove a minimal cross-crate capability with independent consumer value and a dedicated ADR. |
| Cross-workspace derive/procedural macro | `identus-derive` at exact base | `not-adopt` | Adds macro diagnostics and expansion policy to a 47-variant pilot and couples unrelated crates before the local shape is proven. | At least two crate-local pilots demonstrate identical irreducible boilerplate and compile-fail needs. |
| `build.rs` or external schema code generation | not applicable | `not-adopt` | Adds a second build toolchain, generated-source drift and less transparent compile failures for static Rust data. | A future generated wire schema has a separate accepted requirement. |
| Third-party enum/catalogue helpers such as `strum` or `thiserror` | no dependency selected | `not-adopt` | They do not encode the Identus code/kind/capability/redaction contract and would enlarge the cone without removing the domain decision. | A future dependency assessment demonstrates material benefit across multiple crates. |
| Golden fixture generated from post-refactor source on every test run | not applicable | `not-adopt` | It would be tautological and could bless drift in the same change that introduced it. | Never for characterization; deliberate behavior changes replace the golden through their own decision. |
| Checked-in pre-refactor golden fixture | exact SDK base above | `adopt` | A planning-only artifact independently pins 47 current rows before implementation and can be consumed by the pilot regression test. | An explicitly authorized behavior or public API change supersedes a row. |

## Selected architecture evidence

The private contract record owns only compile-time values already required to
construct an `IdentusError`: `ErrorCode`, `ErrorKind`, `CapabilityId`, local
display text, and public message. Domain catalogue modules group the records by
the invariant that causes them to change: envelope/artifact, metadata/schema,
status, verification report, and verification execution. One exhaustive
router per public enum binds every variant to exactly one record. `Display`
and `to_identus_error` consume that same record.

The enum declarations and public constant declarations remain explicit and
unchanged. This avoids generating public documentation or accidentally
changing enum attributes while still giving the compiler an exhaustive match.
The three operational verifier errors use the same private record shape while
retaining their current distinct kinds and `const` bridge.

The planning fixture
`golden/credentials-error-contract-v1.csv` is derived from the exact base and
contains 47 unique variant rows. It includes constant name and visibility so
the 44 public construction constants stay public and the three verifier
constants do not become a new public surface. It contains only static public
diagnostics and no caller, credential, secret, endpoint, or fixture data.

## Compatibility and dependency evidence

- **Public Rust:** no enum, variant, non-exhaustive marker, derive, error-code
  constant, re-export, method receiver/return type, method constness, trait
  implementation or documentation promise changes.
- **Behavior:** local `Display`, `to_identus_error` code/kind/capability/public
  message/full display and `Error::source() == None` remain exact.
- **Wire/FFI:** no Serde, wire representation, binding annotation or FFI is
  added. There is no existing credential error wire format to migrate.
- **Dependencies/features:** no manifest, feature, runtime, native, unsafe or
  third-party dependency changes. The existing `identus-core` edge remains.
- **Targets/compiler:** Rust 1.98.1 and the existing native/WASM/mobile compile
  policy remain unchanged. The implementation is ordinary safe `const` data
  and matches.
- **SemVer:** intended as non-breaking and behavior-preserving. Any public API
  or golden delta is a blocker and moves to a separate compatibility decision.

## Security, privacy and maintenance evidence

| Risk | Boundary and required evidence |
| --- | --- |
| A catalogue row accidentally includes runtime/caller data | `ErrorContract` accepts only `Copy` values and `&'static str`; canary tests prove rejected/correlating values do not enter either display surface. |
| Local and public displays are silently normalized | The pre-refactor golden stores both columns independently and the test compares both exactly. |
| A new variant is left unmapped | The enum-to-contract router is an exhaustive match with no wildcard; compilation fails. |
| A golden row is omitted, duplicated or ambiguously parsed | The test validates the fixed schema, exact row count, unique `(error_type, variant)` and unique public constant/code identities where required. |
| The fixture becomes a production parser or runtime dependency | It is test-only characterization data; production modules neither include nor parse it. |
| The abstraction becomes a shared god error layer | The type and catalogues are private to `identus-credentials`; future crates receive separate issues and may retain different local shapes. |
| Refactoring advances protocol or resource behavior | Only error projection paths are touched; constructors, parsers, limits and OID4VCI remain unchanged and existing tests must pass. |

No new untrusted production input is introduced. The test-only CSV is bounded
by the checked-in file and parsed under an exact schema; malformed or duplicate
rows fail the test rather than influencing library behavior.

The crate-local shape avoids a new shared maintenance or release boundary. The
existing credentials owner remains responsible for its catalogue, and the
reconsideration triggers below require evidence before another crate adopts a
similar shape.

## Rejected or deferred candidates

The candidate table records the current matches, shared runtime abstraction,
procedural macro, build script, dependency helpers, and self-generated golden
as `not-adopt`. A macro may be reconsidered only after at least two independent
crate-local pilots demonstrate the same irreducible mechanics; it is not part
of this credentials slice.

## Evidence commands

- `./scripts/factory doctor` passed at the exact base.
- Source inventory found exactly 44 `CredentialError` variants/constants and
  three `CredentialVerificationError` variants.
- Source extraction cross-checked every code and both display surfaces before
  the golden was written.
- `cargo metadata --no-deps --format-version 1` confirmed the existing
  credentials dependency set is only `identus-core`, `identus-derive`, and
  `zeroize`; the plan changes none of them.
- Factory readiness, focused tests, public API comparison, feature/target
  checks, Nix and complexity measurements are deliberately unrun
  implementation evidence at this planning-only stage.

## Open questions and blockers

No semantic blocker remains. Exact private filenames may be adjusted during
implementation if the domain boundaries and one exhaustive router remain
visible. Any need for a public helper, shared crate, new dependency, generated
public enum, message change, protocol change, or #7/#168 behavior stops this
slice and requires a new decision.

## Reconsideration triggers

- The credentials pilot does not reduce duplicated mapping logic or improve
  domain discoverability in the recorded before/after evidence.
- A public API, display, source or wire comparison differs.
- A second crate needs the same mechanism and provides evidence for or against
  a shared compile-time helper; no shared runtime layer is implied.
