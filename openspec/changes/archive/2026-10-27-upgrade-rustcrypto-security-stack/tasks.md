# Tasks

## 1. Planning

- [x] 1.1 Reproduce both RustSec findings and map their inverse dependency trees.
- [x] 1.2 Evaluate the patched RustCrypto and Tokio lines, licenses, MSRVs and feature cones.
- [x] 1.3 Record ADR 0178, constraints, compatibility posture and exact verification plan.
- [x] 1.4 Commit planning only and write the exact issue #525 preimplementation receipt.

## 2. Implementation

- [x] 2.1 Upgrade the workspace curve family and Tokio resolution without feature expansion.
- [x] 2.2 Adapt crypto implementation details and record the exposed-type source migration.
- [x] 2.3 Regenerate root and fuzz locks and prove the vulnerable versions are absent.
- [x] 2.4 Add or refine migration-sensitive regression evidence if required.

## 3. Verification and delivery

- [x] 3.1 Pass crypto vectors, workspace feature matrices, docs, clippy and portable targets.
- [x] 3.2 Pass root/fuzz audit and deny policy, fuzz dependency policy and full Nix checks.
- [x] 3.3 Run exact-head independent CLI review, using Codex immediately if Claude is unavailable.
- [x] 3.4 Bind PR #530, prepare the guarded OpenSpec archive, and leave metrics and merge to exact-head operational closeout after protected CI is green.

## 4. Stop boundary

- [x] 4.1 Keep broader API redesign, new algorithms, advisory suppression, downstream changes and publication out.
