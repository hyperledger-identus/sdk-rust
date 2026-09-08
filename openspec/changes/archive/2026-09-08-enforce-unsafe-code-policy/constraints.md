# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/169
Constraint blockers: none

## Existing entries affected

`SDK-SEC-001` remains prohibited policy but gains uniform compiler,
conformance and negative-test enforcement for authored first-party source.
`SDK-LIM-008` is narrowed, not removed: exact Rust 1.98.1 intentionally skips
some procedural-macro expansion spans, tracked by #189.
`SDK-COMPAT-004`/`SDK-COMPAT-005` remain exact Rust 1.98.1; Cargo/rustc from that
etalon implement the mechanism. `SDK-DELIVERY-001` is satisfied by issue #169
and this specification-first change.

## Introduced or changed constraints

No new prohibition is introduced: unsafe first-party Rust was already
prohibited. The material change makes the assurance machine-enforced across
every opted-in workspace package and authored source target. `forbid` cannot
be lowered in source. Any future exception remains a separate material change with
a dedicated issue, safety ADR, indexed record, exact scope/invariants/owner,
specialist review, security cost, activation and rollback.

## Introduced or changed limitations

The first-party compiler gate does not inspect dependencies, prove semantic
safety, add a new doctest execution promise, or replace feature/target coverage
accounting. Rustc skips unsafe procedural-macro expansion spans that allow
internal unsafe, so `SDK-LIM-008` retains that narrow gap under #189. The
current policy-exception set is empty. Local aarch64-Darwin cannot prove hosted
x86_64-linux execution; hosted `fast` remains merge authority.

## Consumer and product impact

Consumers gain stronger evidence for an existing memory-safety policy and do
not change code, dependencies, compiler, features, targets, API, wire data or
runtime behavior. External dependency unsafe remains disclosed through each
dependency decision rather than being misrepresented as forbidden.

## Activation and rollback

Enforcement and limitation narrowing activate together only when issue #169's
PR passes full local and hosted gates and merges to `develop`. Reverting that PR
restores the prior root lint configuration and broader `SDK-LIM-008`; no data
migration or downstream mutation is involved.

## Evidence

Evidence includes official Cargo/rustc semantics, the Rust 1.98.1 disposable
target experiment, static root/member configuration guards, deterministic
compile-fail target/macro probes, first-party source inventory, complete Nix
checks and hosted Linux policy/DCO/hygiene/fast results.
