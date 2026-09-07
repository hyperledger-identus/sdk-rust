## 1. Contract and dependency decision

- [x] 1.1 Correct issue #153 after exact candidate semantic/cone research
- [x] 1.2 Complete proposal, crypto delta, design, research, constraints and ADR
- [x] 1.3 Pass strict OpenSpec, research-readiness and constraint-readiness gates

## 2. Standards-correct derivation

- [x] 2.1 Enforce every BIP-32-valid seed length from 16 through 64 bytes
- [x] 2.2 Validate master and child scalars without modular reduction by reusing
      the existing `k256` primitive
- [x] 2.3 Permit zero child tweak, reject zero child result and check depth
      overflow while preserving the hardened-only facade
- [x] 2.4 Keep all secret intermediates zeroizing and failures redacted

## 3. Conformance and boundary evidence

- [x] 3.1 Retain official BIP-32 vectors 1–4 and Apollo parity
- [x] 3.2 Add every seed boundary plus synthetic zero/order master, order child,
      zero-tweak, zero-result and depth-overflow cases
- [x] 3.3 Prove public API, minimal/default/KMP graph and diagnostic stability

## 4. Validation and delivery

- [x] 4.1 Run focused tests, formatting, Clippy, docs, Rust 1.85, Rust 1.98,
      supported portable targets, deny, audit and complete Nix/factory gates
- [x] 4.2 Perform distinct correctness/security review and record findings
- [x] 4.3 Rebase onto `develop` after PR #181 merges, then rerun the focused gate
- [x] 4.4 Sync canonical spec, archive safely and prepare the signed/DCO
      issue-linked PR packet; hosted CI/review and green-only merge remain PR evidence
