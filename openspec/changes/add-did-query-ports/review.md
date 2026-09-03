# Pre-implementation semantic and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #43 (child of #5 / `IDR-006`)
- **Develop base:** `f42b62bb222986002ba19194e8c2ffdb03debcdb`
- **Reviewed contract:** OpenSpec `add-did-query-ports` and ADR 0011
- **Result:** no unresolved blocker

## Findings

1. **Ownership:** generic input metadata and abstract query invocation are SSI
   infrastructure in `identus-did`. Method/VDR implementations, bindings,
   cache policy, lifecycle, persistence and trust stay outside.
2. **Standards:** the pinned 28 August W3C draft gives exact abstract signatures
   and common option names. It returns failures in metadata-bearing envelopes,
   so another generic invocation error channel would contradict the portable
   contract.
3. **Async boundary:** explicit boxed `Send` futures are object-safe on the
   Rust 1.85 MSRV and match consumer dependency-injection needs without adding
   `async-trait` or a runtime. The one allocation is visible and occurs at an
   I/O-shaped boundary.
4. **Volatility:** dereferencing is at risk and therefore has an independent
   trait and option type. No resolver implementation is forced to implement it.
5. **Input safety:** common scalars reuse existing validators; open options have
   raw-byte, map, name, string, depth and aggregate-node limits. Printable ASCII
   relationship values reject control/header injection without closing future
   vocabularies.
6. **Stateful deferral:** the DIF DID Registration draft models jobs and secret
   modes absent from the Midnight donor. Registrar, registry, cache and clock
   APIs need focused state/policy decisions and are not safely bundled here.
7. **Provenance:** donors provide Apache-2.0 design evidence except Lace, which
   remains evidence-only due to unresolved repository license metadata. No
   source or fixture is copied and no consumer tree is modified.
8. **Compatibility:** this is additive in an unpublished `0.0.0` package, adds
   no dependency, and leaves #10's HTTP binding as an outer adapter consumer.

## Consumer preflight

| Repository | HEAD | Observed status before SDK changes |
| --- | --- | --- |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` | `main` behind remote by five commits; no local path reported |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | existing `third_party/midnight-did` submodule/worktree dirt |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | existing `.pi-subagents/`, `.pi/` and `tmp/` untracked paths |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | integration behind remote; existing `.claude/` and `.pi/taskflows/` untracked paths |

These trees are read-only evidence and their existing status is not altered or
cleaned by this change.

# Post-implementation semantic and security review

- **Reviewed head:** `cc6edc40a8da75b93e035948713c42f09db168be`
- **Review completed:** 2026-09-03T04:02:00Z
- **Result:** passed with no unresolved finding
- **Effort:** approximately 16 minutes from issue creation to reviewed,
  fully-gated implementation head

The exact diff from the recorded `develop` base was re-read after the full
workspace and Nix gates. Constructors, builders and serde converge on the same
extension validator; raw entry points enforce the byte ceiling before parsing;
reserved keys cannot be shadowed; errors do not reflect option contents; and
resolution and dereferencing remain independently object-safe. Two unrelated
method-shaped implementations compile and execute through `Arc<dyn
DidResolver>`, while the dereferencing port is independently injectable.

The review also reconfirmed that one boxed allocation per asynchronous call is
an intentional object-safety cost, `Send` is the portable native/mobile/WASM
contract tested by this repository, and W3C errors belong in the existing
result envelope. There is no HTTP, cache, clock, dispatch, registration,
storage, chain, wallet, trust or executor dependency in the diff.

Residual work is isolated in #10 (HTTP), #44 (method registry/dispatch), #45
(cache/clock), #46 (dereferencing algorithm), and #47 (DID Registration).
Duplicate raw JSON member detection and deeper result hardening remain #41.
