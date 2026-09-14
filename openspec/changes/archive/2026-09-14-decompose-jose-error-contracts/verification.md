# Verification receipt

## Identity and provenance

- Umbrella issue: `#271`; delivery issue: `#280`.
- Exact develop base: `c32c1c8cd0194466a8c7fbfaa8c050f4bf2971a1`.
- Planning-contract head: `6a87eea9595b9e778d82d0802f52d1b0035c5e9e`.
- Reviewed Rust implementation head:
  `d206b108669b5f3eea5a5e9e4d98bbad6c9140a6`.
- Final factory-hardening head:
  `b71ec52435a2a4d06037cc31faccf70b4c45777e`.
- Compiler: repository-pinned Rust `1.98.1`.
- Planning/stable golden: 51 rows, 13,353 bytes, SHA-256
  `528b29913876710a2ee806e30fef044657f3c6c38e7e3efff860cf71060b9592`.
- All slice commits through the final hardening head have valid local GPG
  signatures and DCO sign-offs.

## Compatibility and architecture evidence

- `cargo-public-api 0.52.0` generated simplified `identus-jose` inventories at
  the exact base and implementation head with `nightly-2026-09-02`; both had
  614 lines and `diff -u` was empty. Nightly was used only for unstable rustdoc
  JSON; product builds remain pinned stable Rust 1.98.1.
- `Cargo.toml`, `crates/jose/Cargo.toml`, `Cargo.lock`, features, direct
  dependencies, and workflows have an empty base/head diff.
- The public enum/order/discriminants, constants/paths, derives,
  non-exhaustive marker, root re-exports, `From`, `Display`, `Error`, and
  `pub const fn to_identus_error` remain exact. No public type, serialization,
  wire, protocol/algorithm, FFI/binding, retryability, #7, or #168 behavior
  changed.
- One wildcard-free private macro list routes all 51 variants and generates a
  unit-test inventory. The four catalogues own compact/header (15),
  algorithm/key/registry/signing (11), proof/key/evidence (16), and
  proof/policy/time/replay (9) records. The private three-field record contains
  only code, kind, and static message; capability remains centralized.

## Behavioral and immutable-golden evidence

- All-feature and no-default-feature JOSE runs each passed 58 tests with four
  existing manual performance diagnostics ignored. The crate declares no
  features, so these cover its effective default, minimal, and all-feature
  configurations.
- The new suite independently enumerates all 51 variants/constants and pins
  enum discriminant order, exact code, five kinds, capability, local/public/full
  display, `From`, const conversion, `Debug`, redaction, and both source-free
  results. It adds explicit named `SizeOverflow` characterization.
- Stable and planning fixtures are byte-identical. The checker validates three
  binding-specific hashes/provenance records and reads the planning blob from
  the receipt's immutable `contractHeadSha`.
- Independent adversarial review found that blob lookup alone accepted a
  receipt retargeted to a later coordinated-drift commit. The final checker now
  also validates the exact receipt schema/repository/issue/change/branch,
  fixed source base, timestamp/gates, base-to-contract and contract-to-current
  ancestry, and the canonical planning-only diff boundary before trusting the
  blob.
- The expanded 76-case mutation suite includes a self-contained Git history
  that commits both-copy drift and retargets the receipt to the later commit;
  it is rejected. Existing missing/incomplete/ambiguous active/archive,
  hash/provenance/schema, root/path, leaf/intermediate/root symlink, coordinated
  drift, and single-copy drift cases remain green.
- Nix retains exactly the three stable fixture suffixes and excludes all three
  planning-golden names across active/archive paths. Clean snapshots enforce
  hash, byte identity, and exclusion; Git-backed local/hosted execution adds
  full receipt/history validation.

## Quality, target, and workspace evidence

- Strict package Clippy, warning-denied rustdoc, workspace formatting, Python
  compilation, shell syntax, `git diff --check`, OpenSpec validation, factory
  structure, and the full synthetic factory contract passed.
- Direct package checks and the authoritative Nix target derivations passed for
  `wasm32-unknown-unknown`, `aarch64-linux-android`, and
  `aarch64-apple-ios`. These are compilation checks only, not runtime/device,
  packaging, FFI, or binding support claims.
- The aarch64-darwin Nix workspace gate passed 716/716 tests with 22 skipped
  diagnostics. Strict all-target/all-feature Clippy, rustdoc, factory, and
  source-contract derivations passed. Hosted Linux CI remains required before
  merge.
- The first clean-source Nix attempt happened before the new fixture was
  committed and correctly could not see an untracked flake input. The same
  derivations passed after signed commits; no gate was waived.

## Maintainability measurements

Physical/nonblank production measures exclude tests. They are review evidence,
not a line-count budget.

| Measure | Base | Final | Result |
| --- | ---: | ---: | --- |
| Behavioral decisions | 51 | 51 | unchanged; no compression claim |
| Mapping sites | 2 | 1 | one complete record per variant |
| Wildcard kind defaults | 1 | 0 | every kind explicit |
| Base bridge / new router invocation | 214 | 53 | smaller central review unit |
| Largest catalogue | n/a | 16 records; 100/83 lines | bounded ownership |
| `error.rs` physical/nonblank | 471/461 | 336/322 | central file reduced |
| Relevant production physical/nonblank | 471/461 | 726/645 | +255/+184 disclosed |
| Whole JOSE production physical/nonblank | 3,264/2,983 | 3,520/3,168 | +256/+185 disclosed |

The total grows because 51 named records now live in responsibility-owned
files. The useful reductions are one mapping site, zero wildcard defaults, and
bounded review locality; this does not justify a shared runtime error framework.

## Review and residual boundaries

Independent architecture/API/security review found the missing enum-order
assertion; signed commit `d206b10` added the discriminant check and the reviewer
then cleared the Rust design. Independent factory review found the receipt
retarget weakness; signed commits `21bbbdf`, `811b5c0`, and `b71ec52` added
full receipt/history validation, a Nix-compatible regression, and strict typed
schema identity. The independent reviewer cleared the final signed tree at
`b71ec52435a2a4d06037cc31faccf70b4c45777e` after re-running the live checker,
76-case suite, signature/DCO, snapshot-mode, and diff checks.

This slice adds no algorithm, encryption, MAC, key agreement, JOSE/OID4VCI
feature, endpoint, error, code/message, dependency, wire schema, localization,
binding, consumer adoption, or runtime/device support. OID4VCI remains the
separate final catalogue issue #277.
