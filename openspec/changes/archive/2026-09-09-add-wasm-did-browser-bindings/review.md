# Exact-diff architecture and security review

Review status: completed
Review date: 2026-09-09
Implementation head: 8d605e228cc41bdee55be720d530b03bf9ce6ab1
Specification commit: 8c6cef3a36d51bbb1755d365043285a5ce500ea3
Implementation commits: 8c6cef3a36d51bbb1755d365043285a5ce500ea3 and 8d605e228cc41bdee55be720d530b03bf9ce6ab1
Unresolved blockers: none

## Scope reviewed

The review inspected the complete
`develop@1b926b881caf996322200f6df3b059e01e0bcdaf...8d605e22` diff,
issue #224, ADR 0101, browser adapter and consumer fixture, generated package
trees, TypeScript snapshot, workflow change and local verification receipts.

## Findings

1. **Architecture and cohesion — accepted.** The adapter is an unpublished
   leaf over unchanged `identus-did`; JavaScript ABI and lifecycle policy do
   not enter the generic DID domain crate.
2. **Fast-line dependency cone — resolved.** The initial design placed
   `wasm-bindgen-test` in the root workspace dev cone, causing ordinary root
   checks to compile browser-only test machinery. It now lives in a separately
   locked consumer fixture invoked only by the slow browser proof. The runtime
   crate retains only the exact wasm-bindgen dependency.
3. **API and error boundary — accepted.** API version 1 returns SDK-owned
   classes with explicit getters and `free()`. Invalid input produces only two
   stable redacted codes; caller text and parser details do not cross the ABI.
4. **Behavioral parity — accepted.** Host tests cover the pure Rust facade and
   browser tests consume success and thrown-error values through JavaScript,
   rather than calling only Rust internals. Existing DID/DID URL bounds and
   grammar remain the single source of truth.
5. **Supply chain — accepted.** Rust, wasm-bindgen runtime/CLI, test harness and
   wasm-pack versions are exact. Root and fixture locks are tracked; both
   dependency graphs pass applicable deny/audit gates; licenses and immutable
   source revisions are recorded.
6. **Determinism and drift — accepted.** Two complete package trees are
   byte-identical, the public declaration matches a committed review snapshot,
   and sizes and hashes are recorded without inventing a budget.
7. **Security and authority — accepted.** Authored Rust forbids unsafe code.
   The adapter handles only bounded public identifier strings and adds no key,
   secret, storage, network, DOM, callback, async, thread, worker or ambient
   browser authority.
8. **Browser lifecycle and deployment — accepted.** Initialization, paired
   asset delivery, CSP/CORS/cache ownership, synchronous execution and
   explicit value disposal are stated as consumer obligations. No production
   support or publication claim is made.
9. **CI scope — accepted.** Browser execution is isolated to the weekly/manual
   Ubuntu slow job. The Linux fast PR line remains unchanged.
10. **Delivery scope — accepted.** No generated package or release artifact is
    committed or published, and no donor or downstream repository is mutated.

## Residual limitations

- The evidence covers pinned hosted Chromium and Firefox plus one local Chrome,
  not a production browser-version matrix.
- Node, React Native and a named React bundler remain unproven.
- Package publication, release signing, performance budgets and semver support
  require separate activation decisions.

## Review decision

The browser DID slice is bounded, cohesive, deterministic and reversible. No
unresolved architecture, security, privacy, correctness, dependency,
licensing or delivery finding remains for hosted review.
