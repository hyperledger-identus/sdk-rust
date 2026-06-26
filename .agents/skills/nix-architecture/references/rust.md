# Rust — Toolchain, Checks, and Packaging

This reference covers the Rust-specific layer for Identus repos that use Rust (most do).
The general flake / devshell / checks / apps structure lives in `SKILL.md`; this file adds
the Rust conventions on top of it. Read this when the repo has a `Cargo.toml`, or when you
are adding Rust checks or a `packages.*` artifact.

## Rust devshells

Conventions for a Rust devshell (the devshell example in `SKILL.md` illustrates the
shape):

- **Toolchain via rust-overlay**, `rust-bin.stable.latest.default` by default. Switch to
  nightly or add targets (e.g. `wasm32-unknown-unknown`) via
  `.override { extensions = [...]; targets = [...]; }` only when a repo actually needs
  them — match the conservative default, record the override pattern.
- **Include `rust-src` and `rust-analyzer`** as toolchain extensions, for zero-config
  editor setup.
- **`stdenv.cc` and `pkg-config`** belong in a `# C toolchain (build-script deps)`
  section — cargo build scripts of transitive deps need a C toolchain.
- **Env baseline:** `SSL_CERT_FILE` (`${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt`) and
  `LANG = "C.utf8"` are the universal env baseline; add repo-specific env (`JAVA_HOME`,
  etc.) only when needed.
- **Section ordering:** `# rust toolchain`, `# C toolchain (build-script deps)`,
  `# dev tools`, `# nix`. Adapt the package list per repo; the *sections and order* are
  the convention.

## Rust checks via crane + rust-overlay

When a repo defines Rust checks or packaging, use **crane** (add it as a flake input).
Crane is the build/check *driver*; rust-overlay (oxalica) supplies the *toolchain*. They
are complementary — crane does not provide a toolchain.

```nix
perSystem = { pkgs, ... }: let
  craneLib = crane.mkLib pkgs;
  rustToolchain = pkgs.rust-bin.stable.latest.default;
  commonArgs = {
    src = craneLib.cleanCargoSource ./.;
    nativeBuildInputs = [ rustToolchain pkg-config ];
  };
  cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { doCheck = false; });
in {
  checks.cargo-fmt    = craneLib.cargoFmt     (commonArgs // { inherit rustToolchain; });
  checks.cargo-clippy = craneLib.cargoClippy  (commonArgs // { inherit cargoArtifacts; });
  checks.cargo-test    = craneLib.cargoNextest (commonArgs // { inherit cargoArtifacts; });
};
```

**Why crane over alternatives:**
- **vs `buildRustPackage`** (nixpkgs): crane handles cargo-registry vendoring and deps
  caching correctly; `buildRustPackage` requires you to manually hash the cargo lock and
  doesn't share deps across checks.
- **vs `naersk`**: naersk is older and less actively maintained; crane is the current
  community default.
- **The shared `cargoArtifacts` (`buildDepsOnly`) FOD is the real win** — deps build *once*
  and are reused by both clippy and test checks, instead of rebuilding per check.

**Toolchain:** rust-overlay's `rust-bin.stable.latest.default` (the same `pkgs` that
already has the rust-overlay overlay applied — see "One `pkgs` injection point" in
`SKILL.md`). Override with nightly/extensions/targets via `.override { ... }` only when a
repo needs them.

**Add the `crane` input only when a repo actually defines Rust checks or packaging.** A
pure-devshell repo keeps the minimal inputs (flake-parts, nixpkgs, devshell, rust-overlay)
and adds no crane machinery.

## Packaging (Rust)

Packaging is **not** part of the default standard — add `packages.*` only when there is a
real shippable artifact (a binary, a wasm lib, a staticlib). When you do, **use crane**
(`craneLib.buildPackage`) under `nix/pkgs/` (or similar) following the same
perSystem-aggregator + callPackage-able-file pattern as `nix/checks/`. Prefer crane over
`buildRustPackage` or naersk because it shares its `cargoArtifacts` FOD with the checks, so
packaging and tests build the dependency closure once instead of per derivation.

## Rust anti-patterns

- **Don't** reach for `buildRustPackage` or `naersk` for Rust checks/packaging — use crane.
  Crane handles cargo-registry vendoring and deps caching correctly, and its shared
  `cargoArtifacts` FOD lets clippy, test, and packaging reuse one dependency build; the
  alternatives either make you hand-hash the cargo lock or don't share deps across checks.
- **Don't** hardcode a `rustc`/`cargo` version — go through rust-overlay's `rust-bin.*`
  so the toolchain is reproducible and overridable, and so the overlay applies consistently
  to both the devshell and the checks (see "One `pkgs` injection point" in `SKILL.md`).
