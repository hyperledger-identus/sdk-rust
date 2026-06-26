---
name: nix-architecture
description: The standard Nix architecture and conventions for this repo — how the flake, devshells, checks, apps, and Rust toolchain/packaging are structured under nix/. Use whenever working with Nix files in this repo (reading, writing, editing, adding, or reviewing flake.nix or anything under nix/**/*.nix) so the agent follows the established conventions instead of improvising. Also use when setting up the devshell, adding a flake check or app, packaging a Rust crate, running nix flake check or nix develop, or otherwise touching the repo's Nix setup. Trigger on any nix-related task in this repo, even when the user doesn't explicitly name the skill.
---

# Nix Architecture — Identus Standard Conventions

## What this skill is

These are the standard Nix architecture and conventions for this repo: how the flake,
devshells, checks, apps, and Rust toolchain/packaging are structured. The canonical
reference implementation of these conventions is the workspace repo's `nix/` directory.
Consistency across Identus repos is the whole point — follow these rather than
improvising.

These are **conventions, not templates to copy verbatim**. Understand the *why* behind
each pattern so you apply it correctly even when a situation doesn't match a canned
example. The snippets illustrate the *shape* of a convention; they are not files to
reproduce wholesale.

## Architecture overview

The flake is built with **flake-parts**, and the heavy lifting is split into a `nix/`
directory of flake-parts modules. Each top-level concern (devshells, checks, apps) is
its own subdirectory with its own entry point, imported by the root `flake.nix`.

```
flake.nix              # flake-parts; imports nix/{apps,checks,devshells}
nix/
├── apps/              # `nix run`-able helpers
│   └── default.nix    # perSystem aggregator → apps.*
├── checks/            # `nix flake check` derivations
│   └── default.nix    # perSystem aggregator → checks.*
└── devshells/         # `nix develop .#<name>` shells
    └── default.nix    # defines devshells.<name>
```

**Why split this way?** Each concern is independently reviewable, diff-friendly, and
reasoned about in isolation. Adding a new check = one new file + one line in the
aggregator, not a monolithic edit. The aggregator `default.nix` in each directory is the
single import point the root flake pulls in.

## flake.nix structure

`flake.nix` always follows this structure: flake-parts `mkFlake`, importing
`devshell.flakeModule` plus the three module dirs; one system; `pkgs` injected once via
`perSystem._module.args.pkgs` with `allowUnfree` and the overlays this repo needs.

```nix
{
  description = "...";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devshell.url = "github:numtide/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";  # present when the repo uses Rust
  };

  outputs = inputs@{ flake-parts, nixpkgs, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devshell.flakeModule
        ./nix/apps
        ./nix/devshells
        ./nix/checks
      ];
      systems = [ "x86_64-linux" ];

      perSystem = { system, ... }: {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [ (import rust-overlay) ];
        };
      };
    };
}
```

### Conventions that always hold

- **Single system.** `systems = [ "x86_64-linux" ]` unless a repo genuinely needs more.
  Adding a system is a deliberate decision (it doubles CI surface); don't do it
  speculatively.
- **One `pkgs` injection point.** `perSystem._module.args.pkgs` is the only place
  `nixpkgs` is imported. Every module reads `pkgs` from `perSystem`'s argument
  (`{ pkgs, ... }:`), never re-imports `nixpkgs`. This keeps overlays (rust-overlay)
  applied consistently everywhere.
- **`allowUnfree = true`** at the import, because JDKs (temurin) and some toolchains are
  unfree and Identus repos use them.
- **Overlays applied at the top.** Add a new overlay here (e.g. rust-overlay) and it's
  available to every module automatically.
- **No `nixConfig`.** Binary-cache `nixConfig.extra-substituters` /
  `extra-trusted-public-keys` is a *personal* convenience for the workspace owner's
  machine, not a shared standard — it imposes a trust decision on every contributor's
  Nix client. Omit it from repo flakes unless the repo publishes its own cache.

## The perSystem aggregator pattern

Every `nix/<concern>/default.nix` is a flake-parts module that, under `perSystem`,
populates that concern's output attribute (`apps`, `checks`, `devshells`). Each *item*
is a separate file, pulled in with `pkgs.callPackage` (for derivations) or imported
(for devshell modules).

```nix
# nix/checks/default.nix
{ perSystem = { pkgs, ... }: {
    checks.lint-nix = pkgs.callPackage ./lint-nix.nix { };
    # checks.cargo-test = pkgs.callPackage ./cargo-test.nix { };  # one line per item
  };
}
```

**Why:** the aggregator is the single import target for the root flake, keeping
`flake.nix` stable. Items live in their own files so they're independently testable
(`nix build .#lint-nix`) and reviewable. `callPackage` lets `pkgs` supply the inputs and
lets a caller override them.

## The callPackage-able derivation shape

A file meant for `pkgs.callPackage` is a **plain attribute-set function**: its arguments
are its build inputs, supplied by `callPackage` from `pkgs`. No wrapper that bakes in
inputs.

```nix
# nix/checks/lint-nix.nix
{ lib, stdenv, deadnix, statix, nixfmt }:
stdenv.mkDerivation {
  name = "lint-nix";
  src = lib.cleanSourceWith { /* see the lint-nix section */ };
  nativeBuildInputs = [ deadnix statix nixfmt ];
  buildPhase = "true";
  doCheck = true;
  checkPhase = ''
    deadnix -f
    statix check .
    nixfmt --check flake.nix
    find nix -name '*.nix' -type f -print0 | xargs -0 -I {} nixfmt --check {}
  '';
  installPhase = "touch $out";
}
```

**Why:** `callPackage ./lint-nix.nix { }` reads the function args and supplies them from
`pkgs` (you don't hand-wire `deadnix`/`statix`/`nixfmt` at the call site). Tests run in
`checkPhase` with `doCheck = true`; `touch $out` makes it a buildable check
(`nix build .#lint-nix`).

## The devshell shape

Devshells use **numtide `devshell`**. Each devshell is a flake-parts module setting
`devshells.<name>` with `devshell.name`, a `packages` list grouped by **comment-delimited
sections**, and an `env` list of `{ name; value; }` attrs.

```nix
# nix/devshells/default.nix  (single-project form)
{ perSystem = { pkgs, ... }: {
    devshells.default = {
      devshell.name = "my-repo";
      packages = with pkgs; [
        # rust toolchain
        (rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        })
        # C toolchain (build-script deps)
        stdenv.cc
        pkg-config
        # dev tools
        git just jq curl which
        # nix
        nix nixfmt
      ];
      env = [
        { name = "SSL_CERT_FILE"; value = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"; }
        { name = "LANG"; value = "C.utf8"; }
      ];
    };
  };
}
```

**Why comment-grouped sections?** A devshell accumulates packages across concerns
(toolchain, build, docker, database, dev tools, nix). Section comments keep the list
scannable and diffs coherent — adding a package lands it in the right group. **Why the
env attr-list?** It's devshell's native form and keeps each env var explicit and
overridable.

### Single-project vs workspace form

This repo is a **single-project** flake: `nix/devshells/default.nix` defines
`devshells.default` directly as above.

The *workspace* repo additionally uses two patterns that single-project repos should
**not** adopt:
- `base.nix` as a **function** returning a devshell attrset, shared between `default`
  and user shells — only useful when multiple shells share a common base.
- `devshells/users/<name>/default.nix` **personal shells** — only useful when one flake
  serves multiple humans.

Don't reintroduce these in a single-project repo; they add indirection with no payoff.

## Rust devshells

Conventions for a Rust devshell (the example above illustrates them):

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

## The lint-nix check and format-nix app (standard for every flake repo)

Every repo adopting this standard carries **both**:

1. **`nix/checks/lint-nix.nix`** — a `cleanSourceWith`-filtered `mkDerivation` running
   `deadnix -f` (unused bindings), `statix check .` (anti-patterns), and
   `nixfmt --check` over `flake.nix` + `nix/**/*.nix`, wired in via
   `checks.lint-nix = pkgs.callPackage ./lint-nix.nix { };`.

   The `cleanSourceWith` filter scopes the source to **only nix files** (the repo root's
   `flake.nix` plus `nix/`), so the check is fast and only lints what we standardize on.
   The filter is relative-path based (`lib.removePrefix (toString ./../..)`) so it ports
   cleanly between repos without editing:

   ```nix
   src = lib.cleanSourceWith {
     filter = path: _:
       let base = builtins.baseNameOf path;
           rel  = lib.removePrefix (toString ./../..) (toString path);
       in base == "flake.nix" || lib.hasPrefix "/nix" rel;
     src = ./../..;
   };
   ```

2. **`nix/apps/format-nix.nix`** — a `writeShellApplication` running `nixfmt` over
   `flake.nix` and `find nix -name '*.nix'`, wired via `nix/apps/default.nix`:

   ```nix
   { perSystem = { pkgs, ... }: {
       apps.format-nix = {
         type = "app";
         program = "${pkgs.lib.getExe (pkgs.callPackage ./format-nix.nix { })}";
         meta.description = "Format Nix files using nixfmt";
       };
     };
   }
   ```

**Why pair them?** The check *gates* (`nix flake check` fails if nix is unformatted or has
dead bindings); the app *fixes* (`nix run .#format-nix` formats in place). Both operate
on the exact same file set, so the gate and the fixer agree. These are the standard for
every flake repo here — don't skip them.

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
already has the rust-overlay overlay applied — see "One `pkgs` injection point"). Override
with nightly/extensions/targets via `.override { ... }` only when a repo needs them.

**Add the `crane` input only when a repo actually defines Rust checks or packaging.** A
pure-devshell repo keeps the minimal inputs (flake-parts, nixpkgs, devshell, rust-overlay)
and adds no crane machinery.

## Packaging (Rust)

Packaging is **not** part of the default standard — add `packages.*` only when there is
a real shippable artifact (a binary, a wasm lib, a staticlib). When you do, **use crane**
(`craneLib.buildPackage`) under `nix/pkgs/` (or similar) following the same
perSystem-aggregator + callPackage-able-file pattern as `nix/checks/`. Don't reach for
`buildRustPackage` or naersk.

## Anti-patterns (do not do these)

- **Don't** reintroduce `base.nix`-as-function or `devshells/users/<name>/` in a
  single-project repo — they exist only for multi-user/multi-shell workspace flakes.
- **Don't** reach for `buildRustPackage` or `naersk` for Rust checks/packaging — use crane.
- **Don't** define `packages.*` speculatively — wait for a real artifact.
- **Don't** hardcode a `rustc`/`cargo` version — always go through rust-overlay's
  `rust-bin.*` so the toolchain is reproducible and overridable.
- **Don't** re-import `nixpkgs` inside a module — read `pkgs` from `perSystem`'s argument
  so overlays apply consistently.
- **Don't** add a second `system` speculatively — it doubles CI surface; do it deliberately.
- **Don't** add `nixConfig` binary-cache/trust settings to a repo flake — that's personal
  config, not a shared standard.
- **Don't** skip the lint-nix check / format-nix app — they're the standard for every
  flake repo here.

## Formatting & style

- **`nixfmt`** is the formatter (the snippets in this skill are nixfmt-formatted). Run
  `nix run .#format-nix` to format; the lint-nix check enforces it via `nixfmt --check`.
- **`deadnix`** (unused bindings) and **`statix`** (anti-patterns) are enforced by the
  lint-nix check. Don't leave dead `let` bindings or `with` expressions that statix flags.

## Verification

After editing `flake.nix` or anything under `nix/`, **always** run the full check gate
before considering the work done:

```bash
nix flake check
```

`nix flake check` runs every `checks.*` — the lint-nix check (deadnix/statix/nixfmt) plus
any Rust checks (cargo-fmt/clippy/test via crane). It's the single source of truth for
"did my nix edit land cleanly," because it validates the nix *and* the Rust check wiring
together.

If the change only touches nix files and you want faster feedback, the scoped pair is
`nix run .#format-nix` then `nix build .#lint-nix` — but `nix flake check` is the gate you
report against.

**Environment fallback:** if the sandbox can't build the check derivation (no network,
restricted sandbox, missing tools), fall back to invoking the linters directly if
they're in `PATH`:

```bash
deadnix -f
statix check .
nixfmt --check flake.nix
find nix -name '*.nix' -type f -print0 | xargs -0 -I {} nixfmt --check {}
```

If even those aren't available, **surface the environment limitation** to the user rather
than silently skipping verification. Don't claim success for nix edits that haven't been
linted.
