# Constraints and limitations

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/326
Constraint blockers: none

## Existing entries affected

`SDK-REL-001`, `SDK-REL-002`, ADR 0113, the unpublished candidate capability,
the three selected package manifests, and the public release handbook currently
deny publication. ADR 0133 and `SDK-COMPAT-002` through `SDK-COMPAT-005` already
define the compiler/target evidence. `SDK-ARCH-001`, `SDK-LIM-002`,
`SDK-LIM-003`, `SDK-LIM-005`, and `SDK-LIM-006` continue to keep chain,
product, wallet, FFI, runtime, and consumer behavior outside this train.

## Introduced or changed constraints

Only `identus-derive`, `identus-core`, and `identus-crypto` may be versioned and
published. Their internal requirements are exact and their upload order is
fixed. Release identity is a signed immutable tag plus full SHA contained in
protected `develop`; `main` remains reserved and empty/minimal.

Candidate construction cannot publish. Publication requires a separate manual
workflow, the protected `crates-io` environment, independent maintainer
approval, and a same-run receipt. The first release may use only environment
secret `CARGO_PUBLISH`; later trains may use only crates.io OIDC. Missing
credentials, signature, approval, source identity, receipt, or dependency
availability fail closed.

## Introduced or changed limitations

The first public version remains experimental `0.1.0-rc.1`; it is not final
`0.1.0` or a stability/certification promise. The bootstrap token is a temporary
exception until all three trusted publishers are configured and evidenced.
docs.rs availability is externally asynchronous and does not justify an
unbounded retry or a duplicate publication.

## Consumer and product impact

Rust consumers gain registry-addressable generic derive, core, and crypto
packages at the already selected Rust 1.89.0 floor. Consumers retain ownership
of adapters, chain semantics, custody, storage, identity policy, runtimes, and
deployment. No downstream repository is changed by this slice.

## Activation and rollback

Repository activation requires issue-bound preflight, planning-only commit,
package/candidate/release-policy tests, actionlint, factory and Rust evidence,
signed/DCO PR, green protected CI, and independent maintainer review. External
activation additionally requires the signed exact tag, protected environment
approval, and successful dependency-order run.

Before publication, rollback reverts the activation. After publication, the
version cannot be replaced: preserve evidence, stop dependants, decide whether
to yank, and release a corrected version through the full train.

## Evidence

Issues #3 and #326 plus the sponsor direction in this session provide the
material authority. The natural slow receipt is run 35555024293 at protected
`develop` revision `19d0362038c3f2af6898624ea04347e3cd4648f7`.

