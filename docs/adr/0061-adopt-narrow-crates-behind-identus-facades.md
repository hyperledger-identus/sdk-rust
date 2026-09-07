# ADR 0061: adopt narrow crates behind Identus facades

> **BIP-32 disposition superseded:** ADR 0080 records the issue-level finding
> that `bip32 0.5.3` is not suitable for `HDKey`; the SDK instead reuses its
> existing `k256` scalar primitive. Other decisions in this ADR remain active.

- **Status:** Accepted
- **Date:** 2026-09-07
- **Issue:** [#151](https://github.com/hyperledger-identus/sdk-rust/issues/151)
- **Decision authority:** IDR-004 through IDR-010, IDR-020 through IDR-025,
  IDR-041 and IDR-044
- **Research:** [Rust library reuse research](../research/rust-library-reuse/report-source.md)

## Context

The SDK has grown correct product-neutral boundaries for redacted errors,
bounded parsing, owned secrets, parsed/verified/trusted states and injected
ports. It also contains hand-written implementations of standards mechanics
and is about to add more protocol and credential formats. Reimplementing closed
algorithms increases defect and maintenance risk; importing a full SSI
framework would instead couple the SDK's public model to another project's
policy, release cadence, target assumptions and drafts.

The current code audit found concrete correctness gaps in BIP-39 validation and
BIP-32 invalid-scalar handling, an unvalidated multihash value, three URI
parsing approaches and duplicated form encoding. It also found that current
Rust SSI suites frequently combine domain types, transport, crypto, storage or
draft-specific behavior in dependency cones much larger than the corresponding
SDK component.

## Decision

The SDK follows a **narrow engine, owned facade** rule:

1. Reuse a maintained crate for a closed standards algorithm or grammar when
   it reduces correctness risk and passes the repository's version, license,
   security, target, feature and dependency-cone gates.
2. Keep the crate private. Public SDK signatures expose Identus types, stable
   redacted errors and explicit lifecycle/security states, not dependency
   types.
3. Keep policy at the SDK boundary: input limits, duplicate rejection,
   algorithm allow-lists, clocks, randomness, network transport, storage,
   trust, consent, custody and chain behavior are not delegated implicitly.
4. Prefer one cohesive crate to an umbrella framework. A suite is a
   conformance oracle unless one of its subcrates independently meets the
   adoption rule.
5. Every dependency change records exact versions/features, resolved cone,
   reachable unsafe/native code, MSRV and target evidence, normative-version
   parity, rollback and a removal/reconsideration trigger.

The approved production candidates are:

- `bip39 2.2.2` behind `MnemonicHelper` ([#152](https://github.com/hyperledger-identus/sdk-rust/issues/152));
- `bip32 0.5.3` behind the hardened-only `HDKey` facade ([#153](https://github.com/hyperledger-identus/sdk-rust/issues/153));
- `multihash 0.19.5` behind the DID multihash value ([#155](https://github.com/hyperledger-identus/sdk-rust/issues/155));
- `fluent-uri 0.4.1` behind exact-preserving Identus URI/URL types ([#157](https://github.com/hyperledger-identus/sdk-rust/issues/157)); and
- `form_urlencoded 1.2.2` behind OID4VCI request/response types ([#158](https://github.com/hyperledger-identus/sdk-rust/issues/158)).

`multibase 0.9.3` is approved only after the effective MSRV is at least Rust
1.88 or upstream publishes a genuinely compatible transitive resolution.
`multibase`, `did_url_parser`, `oauth2`, `isomdl`, Askar and UniFFI require the
bounded work in [#156](https://github.com/hyperledger-identus/sdk-rust/issues/156),
[#159](https://github.com/hyperledger-identus/sdk-rust/issues/159),
[#160](https://github.com/hyperledger-identus/sdk-rust/issues/160),
[#161](https://github.com/hyperledger-identus/sdk-rust/issues/161),
[#162](https://github.com/hyperledger-identus/sdk-rust/issues/162) and
[#163](https://github.com/hyperledger-identus/sdk-rust/issues/163) before
production adoption.

`identity.rs`, Spruce SSI, Spruce OpenID4VP, impierce OpenID4VC and RustCrypto
JOSE remain differential/conformance inputs. They are not SDK public-model or
runtime dependencies under this decision. The negative and deferred decisions
are maintained in the [not-adopted ledger](../research/rust-library-reuse/not-adopted.md),
and [#164](https://github.com/hyperledger-identus/sdk-rust/issues/164) owns the
first dev-only oracle harness.

The [repository portfolio](../research/rust-library-reuse/repository-portfolio.md)
and ADRs 0066 through 0077 refine this rule for twelve SSI repositories. ADR
0066 supersedes the Spruce-specific oracle-only sentence above for a named
narrow crate while leaving the `ssi` umbrella prohibited. A
repository-level `conditional-adopt` permits evaluation of one narrow crate or
optional adapter; it does not approve an umbrella framework or supersede the
evidence requirements in this ADR.

ADR 0008 remains in force. A `did_url_parser` spike may propose superseding its
self-contained parser decision only after full syntax/error/target parity is
demonstrated. This ADR does not silently change already published behavior.

## Consequences

- The SDK avoids maintaining high-risk standard mechanics already implemented
  by focused Rust crates.
- Dependency replacement remains reversible because external types do not
  escape the owning crate.
- Consumers retain stable domain and error contracts across dependency updates.
- A crate compiling on one host is insufficient; target, feature, conformance
  and security evidence becomes part of every integration issue.
- Framework-level features may take longer to integrate because their useful
  core must first be isolated from transport, policy and drafts.
- Conformance oracles may be dev tooling or read-only source evidence without
  entering release artifacts.

## Alternatives rejected

### Implement every standard locally

This preserves dependency minimalism but already produced BIP-39 and BIP-32
edge-case defects. It is justified only for a very small, well-tested boundary
where available crates increase rather than reduce total risk.

### Standardize on one SSI framework

No assessed framework simultaneously matches the SDK's exact standards
versions, security-state model, MSRV/targets, dependency budgets and
chain-neutral boundary. Framework types would become an accidental public API.

### Re-export selected dependency types

This reduces adapter code but turns dependency upgrades into SDK breaking
changes and lets third-party defaults bypass Identus validation and redaction.

### Adopt based on popularity or current compilation

Neither proves protocol conformance, safe feature selection, maintenance or
older-toolchain compatibility. The `multibase` probe demonstrated that even an
apparently compatible resolution can fail because a transitive crate uses a
newer API without accurate MSRV metadata.

## Verification and rollback

This ADR adds no runtime dependency. Each linked integration issue is an
independent, reversible PR and must preserve golden vectors and public API
behavior or document a reviewed migration. Revert that integration PR to roll
back a dependency; update this ADR and the negative ledger if the architectural
disposition itself changes.
