## Why

The support-policy gate introduced by #23 discovers Crane checks and infers
their Cargo semantics by parsing Nix source text with regular expressions.
That implementation is deliberately fail-closed, but comments, quoting,
interpolation and harmless Nix refactors remain an unnecessary policy parser.

Issue #24 replaces that transitional mechanism with one declarative execution
contract consumed by both Nix and the offline validator. This makes gate
semantics reviewable data and reduces friction for agents adding SDK feature or
target surfaces.

## What Changes

- Add a versioned TOML gate manifest covering every Rust compatibility gate's
  name, Crane operation, toolchain, source/artifact class and structured Cargo
  selection.
- Generate the actual Nix/Crane checks from that manifest instead of repeating
  Cargo argument strings across hand-written modules.
- Refactor support-policy validation to compare policy claims directly with
  structured gate data and stop interpreting Cargo semantics from Nix text.
- Preserve and translate the #23 adversarial regression suite, including
  dynamic/interpolated expressions, comments, quoting and dead-code decoys.
- Add a deterministic 20-sample benchmark command and record process-cold and
  in-process-warm p50/p95 evidence on Linux and macOS.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `sdk-support-policy`: makes the compatibility/execution boundary declarative
  and shared by its Nix producer and offline validator.
- `nix-tooling`: generates the Rust quality and compatibility check matrix from
  versioned repository data.

## Impact

- **Issue:** #24, follow-up to #23 and child of #20.
- **Affected surface:** repository factory scripts/tests, Nix check modules,
  support-policy documentation, ADR 0020 and hosted timing diagnostics.
- **Public/wire compatibility:** unchanged; no published Rust API or protocol
  behavior changes.
- **Dependencies:** no Cargo dependency or lockfile change.
- **Consumers:** Apollo, NeoPRISM, midnight-identity, Lace and Oxid remain
  read-only.
- **Rollback:** revert the focused PR; no release, persisted data, repository
  setting, downstream repository or reserved `main` branch changes.
