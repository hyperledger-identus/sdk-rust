## Why

The SDK blueprint states that generic crates cannot depend on Midnight,
Compact, PRISM/Cardano, NeoPRISM, Lace ID Portal, Oxid or product runtime
policy, but the current conformance guard only checks dependency direction
between workspace crates. A Cargo alias, target-specific dependency,
patch/replacement override, donor Git URL or escaping local path can therefore
violate `IDR-002` without failing CI.

The repository also distinguishes its Rust `1.85.0` MSRV from the pinned
NeoPRISM-etalon nightly, but it has no complete compatibility contract for
hosts, browser WASM, mobile compile checks, features, FFI readiness, binary
size or build time. Closed issue #14 proves only the toolchain alignment, not
the `IDR-003` support matrix. GitHub issue #22 defines this focused R0 slice
before implementation.

## What Changes

- Enforce the chain-neutral boundary across every direct Cargo dependency and
  override section, renamed package, resolved package and dependency source.
- Reject path dependencies that escape the repository and donor Git/path
  references even when their Cargo alias appears neutral.
- Publish a machine-readable support policy and a human-readable compatibility
  contract without overstating compile-only evidence as runtime support.
- Add separate MSRV, NeoPRISM-etalon, host, browser-WASM, mobile and feature
  gates for the surfaces that the repository actually implements.
- Make policy validation reject duplicate identities, detached Nix check trees
  and workspace gates whose effective package selection omits a member.
- Bind every policy gate to its Crane operation and parse actual Cargo target
  and complete feature-option semantics instead of relying on substrings.
- State explicitly that the placeholder bindings crate is not a supported FFI
  and that size/build-time observations are not compatibility budgets yet.
- Repoint `IDR-002` and `IDR-003` to issue #22 and update their status only
  after the merged evidence satisfies their acceptance criteria.

## Capabilities

### New Capabilities

- `sdk-dependency-boundary`: governs chain/product dependency identity,
  sources, paths and resolved-closure enforcement.
- `sdk-support-policy`: governs the Rust/toolchain, host, target, feature, FFI,
  binary-size and build-time compatibility contract.

### Modified Capabilities

- None.

## Impact

This change affects conformance-only guard code, Nix checks, structural
validation, architecture documentation, OpenSpec and the two canonical
foundation rows. It adds no runtime SDK API and no donor dependency. Apollo,
NeoPRISM, midnight-identity, Lace ID Portal and Oxid remain read-only; `main`,
publishing and downstream adoption remain out of scope.
