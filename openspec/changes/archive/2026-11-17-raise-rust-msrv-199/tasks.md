## 1. Contract and decision

- [x] 1.1 Record the Rust 1.99 sponsor direction, affected consumers,
      candidate-line migration, advisory trigger, and rollback.
- [x] 1.2 Add successor ADR 0200 and pass strict research and constraint
      readiness before implementation.
- [x] 1.3 Commit the planning-only contract and bind the issue #596
      preimplementation receipt.

## 2. Toolchain and compatibility implementation

- [x] 2.1 Pin Rust 1.99.1 for MSRV, primary, and etalon providers and update the
      exact stable-overlay lock.
- [x] 2.2 Move current candidate/support/constraint records to the 0.2 line and
      add explicit migration guidance without rewriting historical evidence.
- [x] 2.3 Update executable policy/checker/fixture expectations and reject
      drift back to older compilers or the 0.1 candidate line.

## 3. Supply-chain repair

- [x] 3.1 Resolve the isolated UniFFI generator lock to patched
      `rustix >= 1.1.6` without changing its public contract.
- [x] 3.2 Pass the exact UniFFI generator audit and focused dependency checks
      without a `rustix` advisory waiver.

## 4. Verification and delivery

- [x] 4.1 Run focused support, constraint, candidate, source, factory, Cargo,
      and supply-chain gates.
- [x] 4.2 Run the complete compatible-host `nix flake check` and record every
      unrun or host-incompatible lane.
- [x] 4.3 Complete a distinct exact-diff/security/compatibility review, resolve
      blockers, archive the change, and verify signed/DCO commits.
- [x] 4.4 Push the base branch, open the issue-linked PR, monitor exact-head CI,
      and stop the DID Document child on any real base failure.
