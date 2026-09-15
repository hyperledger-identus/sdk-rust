# Exact-diff architecture and security review

- **Review status:** completed
- **Review date:** 2026-09-15
- **Planning head:** `e127bb6db8c7b62cc9eb70d076961f72630284f4`
- **Implementation head:** `cd816d6e62e2b92f6a4a08720efe2d0cbca05915`
- **Unresolved blockers:** none

## Scope reviewed

The review inspected ADR 0122, the complete diff from merged candidate repair
`47136ed8bc73faba34c78d88768136cd6841364d` through the implementation head,
the failed canary log, official Android package catalogs, workflow install,
native verifier, offline policy checker, mutation suite and factory inventory.

## Resolved finding

The first committed implementation made `check-support-policy.py` read the
native Android verifier, but the factory's isolated self-test fixture did not
copy that newly required input. `nix flake check` failed closed. The verifier
is now a required executable in the factory inventory and is copied into the
self-test fixture; the exact committed-source Nix rerun passed.

## Findings

1. **Least-capability dependency — accepted.** ADR 0100 requires API 35 and
   arm64-v8a, not Google services. The selected default AOSP image satisfies
   the declared runtime need with a smaller software and license cone.
2. **Legal authority boundary — accepted.** Repository automation neither runs
   `sdkmanager --licenses` nor injects a license hash. Missing provisioning
   fails closed for explicit organization-level resolution.
3. **Install/execution integrity — accepted.** The workflow package, verifier
   constant, image directory and `avdmanager` package argument are bound to one
   exact AOSP identity. Mutations cover image and directory drift.
4. **Architecture and cohesion — accepted.** Android concerns remain in the
   slow workflow and leaf binding verifier. Generic Rust crates, public types,
   Cargo manifests and the AAR/JNA architecture are unchanged.
5. **Security and privacy — accepted.** No production native code, unsafe code,
   secret, credential, storage, network or runtime permission is added. The AVD
   remains ephemeral below ignored `target/`.
6. **Compatibility and support — accepted.** API, ABI, wire format, MSRV,
   target tiers, minimum application API and behavior cases do not change.
   Experimental Android and FFI limitations remain effective.
7. **Supply-chain accuracy — accepted with recorded limitation.** The exact SDK
   path is enforced, while the catalog's underlying image revision/archive is
   not hermetically pinned. The retrieved catalog URL/hash and current revision
   are research evidence, not a consumer or release promise.

## Decision

The change is narrow, reversible and more cohesive than the previous Google
Play selection. No unresolved architecture, correctness, security, privacy,
licensing, compatibility or delivery finding remains. Hosted macOS execution
is still mandatory before the canary can be accepted.
