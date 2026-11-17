# Exact-diff architecture and security review

- **Review date:** 2026-11-18
- **Issue:** #599
- **Develop base:** `6c0fb15a1a91dbcb7e08c0b471d7c5c1aa0624dd`
- **Reviewed implementation head:** `d1ae72d78ed96e25a062f5755528307dd6cabc47`
- **Result:** passed with no unresolved blocker

## Scope reviewed

The fresh-context review inspected the full Rust/WASM/TypeScript/provider diff,
the W3C resolution and dereferencing projections, generated declaration
snapshots, target isolation, immutable SDK-TS donor evidence, resource bounds,
error redaction, generated-value ownership, quality declarations, and all ten
branch commit envelopes. It also executed adversarial probes outside the
checked-in test cases.

## Findings and remediation

1. **Commit provenance — accepted after remediation.** One rebinding commit
   initially carried an invalid signature. It was replaced by
   `4e3daa7b4c35bccbb9976632487e1d9c6ac736c1`; every commit in the final range
   now has a valid GPG signature and DCO trailer.
2. **Host-result resource boundary — accepted after remediation.** Resolution
   and dereferencing host text now pass an allocation-free UTF-8 byte preflight
   before either generated WASM parser. Exactly 524,288 bytes are accepted and
   one byte over is rejected with the operation-specific stable code.
3. **Structured option validation — accepted after remediation.** Recursive
   properties reject forbidden prototype keys, non-plain objects, unsafe
   integers, cycles, shared containers, excessive depth/nodes/members, and
   invalid array shapes before binding or host effects.
4. **Exotic arrays — accepted after a second blocking review.** The original
   recursive clone dynamically called an input-owned `map`, allowing a
   shadowed method or array subclass to suppress traversal. Commit
   `5aff2744b7637471c7b9d8a86aa2498185a8306c` requires exact
   `Array.prototype`, dense enumerable own data indices, no extra or symbol
   keys, and traverses descriptors without dynamic method dispatch. Independent
   probes rejected shadowed-map, subclassed, sparse, extra-property,
   symbol-bearing, accessor-backed, non-enumerable, and cyclic arrays while
   ordinary dense arrays remained accepted and frozen.
5. **Public error classification — accepted after remediation.** Invalid public
   resolution and dereferencing options map to their operation-specific stable
   codes; generated API drift remains a binding-contract failure. Diagnostics
   do not reflect caller, host, DID, or result content.
6. **Donor provenance — accepted after remediation.** All five immutable SDK-TS
   paths include the required `resolver/` segment and every recorded SHA-256
   digest matches the pinned donor revision.
7. **Evidence precision — accepted after remediation.** Property, fuzz,
   measurement, and differential declarations state only what their harnesses
   execute. The representative vector packet is not described as exhaustive
   problem-detail coverage, and directional measurements do not create support
   thresholds.
8. **Compiler policy — accepted.** Cargo policy and Nix MSRV, primary, and
   etalon toolchains consistently use Rust 1.99.1. The newer pinned fuzz nightly
   is tooling-only and does not widen the public MSRV claim.
9. **Runtime and ownership boundary — accepted.** One injected platform host
   effect sits between synchronous Rust preparation and correlated Rust result
   parsing. Generated views are released exactly once before host execution;
   projections are owned and deeply frozen.
10. **Target isolation — accepted.** Browser, Node ESM/CommonJS, React SSR, and
    browser hydration reuse the same facade. React Native is explicitly
    inapplicable rather than silently passing. TypeScript binding failures are
    routed outside the required Rust core lane.

## Residual limitations and non-claims

- Network transport, retry, caching backend, timeout, and in-flight
  cancellation remain platform-owned concerns.
- The local measurement output is directional and deliberately reports no
  accepted support or regression budget.
- `npm ci` reports four high-severity advisories in the unchanged private
  harness dependency graph; this change adds no npm dependency or runtime
  package and does not claim that the existing advisory backlog is closed.
- This change does not publish packages, promise React Native support, or make
  a stable-API/release claim.

## Decision

The final implementation is bounded, correlated, fail-closed, redaction-safe,
and consistent with the one-way Rust/WASM/TypeScript architecture. Both rounds
of fresh-context blocking findings are resolved. No unresolved correctness,
security, architecture, compatibility, provenance, or evidence-integrity
finding remains.
