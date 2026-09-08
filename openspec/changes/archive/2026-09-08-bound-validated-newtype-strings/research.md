# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-09-09
Source retrieval date: 2026-09-09
Research blockers: none

## Problem and existing implementation

At base revision `8f2bfc3b888ec56656aad71a629b380a1c1d7c25`, generated
validated `String` `parse`/`FromStr` paths clone before calling their validator.
Repository search found exactly two production uses: `identus_did::DidMethod`
and `identus_core::Url`. Both validators accept `&str`; no workspace validator
depends on `&String`. Owned `TryFrom<String>` and validating serde already own
their input and cannot recover a pre-allocation guarantee.

`DidMethod` validates the closed lowercase-ASCII/digit method-name grammar. It
has no length limit and formats rejected caller input into `Error::InvalidMethod`.
The enclosing `Did` parser already validates borrowed bytes before allocation
and limits a bare DID to 2,048 bytes.

## Normative sources

- [W3C DID Core 1.0, 19 July 2022 Recommendation](https://www.w3.org/TR/2022/REC-did-core-20220719/)
  defines the method-name grammar. ADR 0008 and the canonical `did-core`
  specification already pin it as the stable normative source revision.
- ADR 0063, `SDK-SEC-003` and `SDK-LIM-007` require explicit resource limits,
  redaction-safe errors, and honest disclosure of inherited gaps.
- The canonical domain-newtype specification owns generated constructor
  semantics; `MAX_DID_BYTES` owns the enclosing DID resource policy.

No donor repository code is needed. Apollo parity and NeoPRISM extraction are
unaffected: this is generic first-party boundary hardening.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Validate borrowed `&str`, allocate after success | `adopt` | Enforces the intended bound-before-allocation property with one allocation on success. | Rust gains a more general zero-copy owned conversion mechanism. |
| Retain `&String` validator compatibility | `not-adopt` | Necessarily allocates before borrowed validation and preserves the defect. | A published compatibility promise or named consumer demonstrates an unmodifiable `&String` validator. |
| Add a second borrowed validator macro attribute | `not-adopt` | Enlarges configuration and leaves the insecure ordering selectable without a current consumer. | Distinct owned and borrowed validation semantics become a demonstrated requirement. |
| Derive method ceiling from `MAX_DID_BYTES` | `adopt` | Preserves every method name that can fit in an accepted bare DID. | The enclosing DID budget or minimum grammar changes. |
| Arbitrary smaller standalone ceiling | `not-adopt` | Would make `DidMethod::parse(did.method())` fail for a valid maximum-size DID. | A standards or consumer profile explicitly imposes a smaller generic limit. |
| Third-party parser/validation crate | `not-adopt` | The gap is allocation ordering plus six bytes of closed ASCII policy; a dependency cannot replace macro behavior and would widen the cone. | The SDK adopts a broader validated-type framework through a separate evidence-backed ADR. |

## Compatibility and dependency evidence

The macro contract change is source-breaking only for a hypothetical validated
`String` newtype whose validator accepts `&String` but not `&str`. The workspace
has none, `identus-derive` is version `0.0.0`, and publication remains
prohibited. Accepted values, owned conversion, serde wire shape, registry and
Registration APIs are unchanged.

The 2,042-byte ceiling is `2,048 - 4 - 1 - 1`: literal `did:`, separator, and
minimum method-specific id. Length is checked before ASCII traversal; exact and
one-over tests establish the boundary and precedence. Validator-created errors
carry static detail, so rejected caller text cannot reach `Debug`, `Display`,
serde custom errors or the stable `IdentusError` bridge through this path.
Outer transports and serde can still allocate before the SDK sees the value;
`SDK-LIM-007` remains effective.

No dependency, unsafe, native code, license, MSRV, target, chain or product
surface changes. No new privacy data is processed or retained.

The direct and resolved dependency cone is unchanged and no facade boundary is
introduced. The exact version/features question is therefore not applicable to
a new package: the workspace remains on Rust/MSRV 1.98.1 and its existing
features. Public and wire compatibility is unchanged for accepted input;
rollback is the focused revert described below.

## Security, privacy and maintenance evidence

The current implementation performs avoidable allocation and unbounded method
character work on rejected input. The proposed order caps work before copying,
and static validator detail prevents caller-controlled text from crossing
diagnostic or serde-error surfaces. It introduces no secret processing, PII,
unsafe Rust, native code, FFI, build script or network behavior.

There is no third-party license, provenance or supply-chain addition to audit.
The first-party Apache-2.0 repository remains the provenance source. Protocol
or draft currency is settled for this slice: the W3C source is a stable
Recommendation, not a draft, and its grammar does not change; the method
ceiling remains an SDK resource policy. Maintenance consists of one public
constant and tests tied to `MAX_DID_BYTES`. Release remains prohibited during
bootstrap and the change creates no target portability difference.

## Rejected or deferred candidates

Retaining `&String`, adding a second validator attribute, choosing an arbitrary
smaller ceiling and adding a validation crate are `not-adopt` decisions for the
reasons in the candidate table. A custom allocation-counting global allocator
test is deferred: expansion/source tests can prove call ordering without unsafe
test machinery. Serde streaming limits are deferred to the parent resource
audit because they require a transport/deserializer contract, not this value
type.

## Open questions and blockers

None for implementation. Stop rather than merge if a repository `&String`
validator is discovered, accepted enclosing DIDs fail standalone method
parsing, error output retains the rejection canary, or a new dependency is
required.

## Evidence commands

Pre-implementation commands include `scripts/factory doctor`, `rg`, `git
rev-parse HEAD`, `openspec status`, `openspec validate ... --strict` and
`scripts/factory check --change bound-validated-newtype-strings`. Focused,
workspace, Nix and hosted checks are explicitly unrun until implementation.
