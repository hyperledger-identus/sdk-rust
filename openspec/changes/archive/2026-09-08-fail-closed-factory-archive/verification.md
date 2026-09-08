# Verification receipt

## Identity

- Delivery issue: `#194`.
- Exact base: `93970e2a721304458d41e5daba634482fe9cd7da`.
- Reviewed implementation head:
  `c6f193d7060e95221216c7706222e94de1e4d26a`.
- Decision: factory archive success is proven from the requested filesystem
  transition and complete artifacts, not inferred from OpenSpec exit zero.

## Behavioral evidence

- The hermetic factory contract passes the pre-existing lossy-modification
  case and proves that OpenSpec archive is not invoked after that preflight
  fails.
- A pre-existing dated destination fails before any OpenSpec command runs and
  never prints the wrapper success marker.
- A zero-exit/no-op OpenSpec archive fails while the active change remains
  intact and never prints success.
- A move missing `research.md` fails after mutation and never prints success.
- A complete move preserves all six mandatory files plus `specs/`, passes the
  resulting factory validation and prints the exact success marker.
- The fixture now changes into its temporary Git repository before invoking
  the copied facade; this repairs the older test's accidental use of the outer
  worktree as `factory_root`.

## Local evidence

- `bash -n scripts/factory scripts/tests/factory-contract.sh` passed.
- `scripts/tests/factory-contract.sh` passed, including the new state-transition
  cases and the existing structural, policy and preservation suites.
- Focused Nix `factory-contract`, `lint-text` and `lint-nix` checks passed;
  `lint-text` ran Markdown, YAML, EditorConfig and ShellCheck.
- Full `nix flake check --print-build-logs` passed all 29 compatible
  aarch64-Darwin checks under Rust/Cargo 1.98.1. This included the factory,
  lint, TOML, format, build, strict Clippy, docs, deny, audit derivation,
  default/all/minimal feature, WASM, iOS and Android gates.
- Workspace nextest passed 621/621 tests with 22 configured skips; KMP crypto
  passed 113/113 and the remaining crypto/entropy profiles passed 8/8, 4/4,
  3/3 and 1/1 as applicable.
- The audit derivation retained known nonfatal offline yanked-index diagnostics,
  and Nix fixup retained known nonfatal Darwin audit-script segmentation
  warnings. Neither changed the successful check result.
- `git diff --check` passed. Cargo manifests, `Cargo.lock`, Nix inputs, SDK
  crates and public/runtime surfaces are unchanged from the base.
- Exact-diff review found no unresolved correctness, security, compatibility,
  provenance, data-loss or test-isolation blocker. All three implementation
  history commits have valid GPG and DCO signatures.

## Scope receipt

- Hardened only `scripts/factory archive` and its repository contract test.
- Documented the postcondition in the factory guide and canonical OpenSpec
  delta.
- Added no dependency, SDK API, wire behavior, persisted data, target or
  downstream repository change.

## Residuals

- The wrapper detects but cannot automatically reverse arbitrary partial
  OpenSpec mutation.
- A local-midnight boundary can conservatively reject a valid next-day archive;
  retrying observes the deterministic new destination.
- x86_64-Linux is not buildable on this local Darwin host; hosted Linux `fast`
  remains the authoritative merge gate.
