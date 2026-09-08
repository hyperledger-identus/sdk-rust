## 1. Contract and research

- [x] 1.1 Pin issue #179, ADR 0078, upstream revision, license and residual risks
- [x] 1.2 Compare zeroization and feature alternatives and select the narrow patch
- [x] 1.3 Pass strict OpenSpec, research-readiness and constraint-readiness gates

## 2. Upstream implementation

- [x] 2.1 Fork the upstream repository and create a focused branch from `6539dc9`
- [x] 2.2 Redact `XPrv` `Debug` and `Display` with regression tests
- [x] 2.3 Replace manual unsafe zeroing with default-disabled zeroize 1.8.2
- [x] 2.4 Disable cryptoxide defaults and enable only `ed25519`, `sha2`, and `hmac`

## 3. Verification and contribution

- [x] 3.1 Run formatting, tests, Rust 1.81, `no_std`, target and feature-tree gates
- [x] 3.2 Run dependency audit, unsafe scan and exact-diff security review
- [x] 3.3 Push the fork branch and open a focused upstream pull request

## 4. SDK evidence delivery

- [x] 4.1 Record immutable upstream links and exact verification outcomes
- [x] 4.2 Update issue #179 and add ADR 0089 without mutating accepted ADR 0078
- [x] 4.3 Sync the dependency-readiness spec and archive this OpenSpec change
- [x] 4.4 Prepare the signed/DCO SDK evidence PR; hosted CI and green-only merge
      remain pull-request evidence
