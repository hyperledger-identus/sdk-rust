# Research readiness

Research class: cryptography-security
Research status: ready
Decision date: 2026-10-28
Source retrieval date: 2026-10-28
Research blockers: none

## Problem and existing implementation

The current implementation is the separately locked generator and the current
consumer evidence is the generated Swift/Kotlin API plus host smoke programs.
Natural scheduled run `40260649503` at SDK revision
`5da04223c1c4e029cb368870877e56d699d40388` passed Linux, Nix, release-candidate,
portable-target and host preparation evidence. Its macOS native-binding step
failed only when `cargo audit --deny warnings` reported RUSTSEC-2026-0374.
Current `develop` still resolves this exact inverse path:

```text
basic-toml 0.1.10
askama_derive 0.16.1
askama_macros 0.16.1
askama 0.16.1
uniffi_bindgen 0.32.0
uniffi 0.32.0
identus-uniffi-bindgen 0.0.0
```

The generator is an unpublished nested workspace with its own lock. Neither
`basic-toml` nor Askama enters the production runtime workspace or generated
host packages.

## Normative sources

- RustSec RUSTSEC-2026-0374, published 2026-10-20, classifies `basic-toml` as
  unmaintained and does not describe a vulnerability.
- UniFFI 0.32.0/0.32.2 published metadata and source define the generator
  dependency on Askama 0.16.
- Askama 0.16.1 published metadata and source define the `basic-toml` path.
- ADR 0141 and the `native-did-bindings` specification own generator isolation
  and host-language evidence.
- The immutable failing run is
  https://github.com/hyperledger-identus/sdk-rust/actions/runs/40260649503.

Primary sources:
https://rustsec.org/advisories/RUSTSEC-2026-0374.html,
https://crates.io/crates/uniffi/0.32.2, and
https://github.com/mozilla/uniffi-rs/blob/main/CHANGELOG.md.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| UniFFI 0.32.2 patch upgrade | `not-adopt` | It still depends on Askama 0.16 and does not remove `basic-toml`. | A compatible release removes the path. |
| Private Askama/UniFFI fork | `not-adopt` | Owning a template-engine fork for an unpublished generator exceeds the present risk. | A vulnerability appears and upstream cannot remediate promptly. |
| Remove audit or host E2E | `not-adopt` | This destroys the evidence that detected the drift. | Never for convenience. |
| Unbounded RustSec ignore | `not-adopt` | It can outlive the dependency facts and mask future graph changes. | Never. |
| Exact expiring acceptance | `adopt` | The finding is maintenance-only, build-only and isolated; a fail-closed guard preserves visibility while upstream catches up. | Dependency path changes, a vulnerability is published, or 2027-01-31 arrives. |

## Compatibility and dependency evidence

There is no SDK or generated ABI change. The guard must compare the full inverse
path, exact versions and root tool identity before applying one advisory ID.
`cargo deny` remains unmodified. Root and fuzz `cargo audit --deny warnings`
remain ignore-free. The host-generation scripts remain the behavior oracle.
The exact version and feature set remain UniFFI 0.32.0 with only its generator
CLI feature in the nested tool. The direct dependency is UniFFI; the resolved
dependency cone reaches Askama and `basic-toml` only in that nested workspace.
Public and wire compatibility are unchanged, and the `identus-uniffi-did`
facade boundary continues to prevent generator types from entering SDK APIs.
Native Apple/Android and JVM target evidence remains mandatory.

## Security, privacy and maintenance evidence

The accepted crate parses Askama build configuration while compiling the local
generator. It handles no wallet secret, DID input, network response, generated
runtime call or consumer data. The principal risk is maintenance/provenance
drift. Exact graph matching, expiry and unchanged deny policy bound that risk.
Any vulnerability, path/version drift or expiry stops the lane.

Licenses remain MIT OR Apache-2.0 in the accepted graph. MSRV is unchanged.
No protocol or draft-currency decision applies.
The reviewed cone introduces no SDK-authored unsafe code and no additional
native-code dependency. Supply-chain evidence is the exact lock, inverse tree,
RustSec record, cargo-deny result and source provenance. Maintenance, release
and security posture remains experimental and unpublished. Rollback is a PR
revert to strict failure; a clean release requires the upstream exit instead.

## Rejected or deferred candidates

A private fork, generator replacement and UniFFI major migration are deferred.
They become research candidates if upstream does not remove the path before the
expiry date.

## Open questions and blockers

There are no planning blockers. The implementation must prove that ignored and
unexpected advisories still fail, and that a simulated post-expiry date fails.

## Evidence commands

Planning evidence includes the immutable run, current inverse tree, published
crate metadata and strict OpenSpec validation. Implementation evidence includes
guard unit fixtures, the live locked generator audit/deny checks, full host
generation and Apple/Android/Kotlin/Swift E2E scripts, Nix checks, exact-head
CLI review and protected CI.

Exact commands include `cargo tree --locked -i basic-toml@0.1.10`, `cargo deny
--locked check`, `cargo audit --deny warnings`, the new guard fixture suite and
the existing `scripts/check-uniffi-did-*.sh` programs. These implementation and
protected-CI checks are explicitly unrun at planning time.
