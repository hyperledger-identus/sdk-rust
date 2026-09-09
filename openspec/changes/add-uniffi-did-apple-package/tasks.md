## 1. Contract and research

- [x] 1.1 Create issue #228 and record the immutable develop base.
- [x] 1.2 Research Apple/UniFFI package mechanics, Xcode 26 module risk, static-library composition, tool versions and target availability.
- [x] 1.3 Specify deterministic package, execution, security, compatibility, limitation and rollback contracts.
- [x] 1.4 Pass research and material constraint readiness and commit the specification before implementation.

## 2. Apple package proof

- [ ] 2.1 Add the static library artifact and exact Swift generator entry point without dependency or domain changes.
- [ ] 2.2 Add an ephemeral deterministic XCFramework/SwiftPM assembly script and tracked package/test template.
- [ ] 2.3 Prove arm64 device/Simulator variants, minimum platform, symbols, complete-tree equality and absence of absolute paths.
- [ ] 2.4 Compile and execute the package behavior suite in an iOS Simulator through Xcode.
- [ ] 2.5 Add the Apple proof to the weekly/manual slow macOS lane without changing fast PR CI.

## 3. Review and delivery

- [ ] 3.1 Record the Apple packaging ADR and update architecture/support evidence while retaining `SDK-LIM-002`.
- [ ] 3.2 Run focused, factory, Cargo, dependency and compatible Nix gates plus distinct exact-diff review.
- [ ] 3.3 Complete the receipt inputs and prepare canonical-spec archival plus a signed/DCO issue-linked PR for green-only integration.
