# Pre-implementation semantic and misuse-resistance review

- **Date:** 2026-09-03
- **Issue:** #44 (child of #5 / `IDR-006`)
- **Develop base:** `e0e276d4aeae097363db4b705ef571676322f9e5`
- **Reviewed contract:** OpenSpec `add-did-method-registry` and ADR 0012
- **Result:** no unresolved blocker

## Findings

1. **Ownership:** generic method composition belongs beside generic query ports
   in `identus-did`; concrete methods, VDRs, transports and wallet policy do
   not.
2. **Reuse:** implementing the existing ports avoids a second dispatcher API
   and lets current/future consumers inject either a single method adapter or a
   registry without orchestration changes.
3. **Determinism:** immutable `BTreeMap` dispatch removes vector-order priority,
   wildcard ambiguity and mutable-registry races while supporting borrowed
   method lookup.
4. **Bounds:** 64 methods is ample for a wallet and caps configuration memory
   and diagnostic work. Duplicate ownership is rejected, not replaced.
5. **Volatility:** dereferencing remains independently optional; a resolver-only
   binding does not claim support for the at-risk feature.
6. **Failure model:** unsupported dispatch is a W3C result, while duplicate or
   excessive setup is a local/core-bridged construction error. No transport or
   adapter error taxonomy crosses the seam.
7. **Concurrency:** the built map and adapters are shared through `Arc`, so
   cloning and concurrent reads need no lock or async runtime.
8. **Provenance:** all downstream sources remain read-only evidence; no donor
   has a generic registry worth copying, and no fixture/source will be copied.
