{ pkgs, sdk-rustLib }:

let
  inherit (pkgs) lib;
  inherit (sdk-rustLib) rustTools;

  # Custom wasm-bindgen-cli built from source at version 0.2.121 to match
  # the wasm-bindgen crate dependency in identus-crypto-wasm.
  wasmBindgenCli = pkgs.callPackage ./wasm-bindgen-cli.nix { inherit sdk-rustLib; };

  # Minimal source tree: only the paths needed to build the WASM crate and
  # stage the static web demo.  Filtering avoids pulling in unrelated files
  # (nix/, .git/, target/, justfile, etc.) that would cause unnecessary
  # rebuilds.
  #
  # IMPORTANT: for directories, we must return `true` for any directory that
  # is an ancestor of (or equals) a needed path, otherwise `builtins.path`
  # will skip the entire subtree.
  src = builtins.path {
    path = ./../..;
    name = "identus-crypto-wasm-demo-src";
    filter = path: type:
      let
        root      = toString ./../..;
        rel       = builtins.substring (builtins.stringLength root) (-1) (toString path);
        isDir     = type == "directory";
        isFile    = type == "regular";
        keepRoot  = isDir && rel == "";

        # Directories / files whose contents we need (relative to root).
        neededPrefixes = [
          "/lib/identus-crypto-wasm"
          "/lib/identus-crypto"
          "/lib/identus-core"
          "/examples/wasm-app"
        ];

        # A directory is needed if:
        #   - it is an ancestor of (or equals) a needed prefix, OR
        #   - it is a descendant of a needed prefix.
        # This ensures `builtins.path` descends into both ancestor
        # directories AND subdirectories of needed subtrees (e.g.
        # `src/` inside `lib/identus-core/`).
        isNeededDir = isDir && (
          lib.any (prefix: lib.hasPrefix rel prefix) neededPrefixes
          || lib.any (prefix: lib.hasPrefix prefix rel) neededPrefixes
        );

        # A root-level file that is part of the workspace definition.
        isRootFile = isFile && (
          rel == "/Cargo.toml" || rel == "/Cargo.lock"
        );

        # A regular file inside one of the needed subtrees.
        isInsideNeeded = isFile && lib.any
          (prefix: lib.hasPrefix prefix rel)
          neededPrefixes;
      in
      keepRoot || isNeededDir || isRootFile || isInsideNeeded;
  };

  # rustPlatform from the project's nightly toolchain — we only use it for
  # the cargo setup hooks (vendor dir), not for the build hook itself (which
  # hardcodes --target @rustcTargetSpec@).
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustTools.rust;
    rustc = rustTools.rust;
  };
in

rustPlatform.buildRustPackage {
  pname   = "identus-crypto-wasm-demo";
  version = "0.1.0";

  inherit src;

  cargoLock = rustTools.cargoLock;

  # Disable the default cargoBuildHook so we can write our own buildPhase
  # that targets wasm32-unknown-unknown.
  dontCargoBuild = true;

  nativeBuildInputs = [
    wasmBindgenCli
    pkgs.clang
    pkgs.llvmPackages.llvm
  ];

  doCheck = false;   # WASM binaries cannot execute natively

  # Environment variables for WASM cross-compilation with C dependencies.
  # We use the *unwrapped* clang binary (clang.cc) to avoid nixpkgs wrapper
  # flags (e.g. -fzero-call-used-regs) that are incompatible with WASM.
  CC_wasm32_unknown_unknown = "${pkgs.clang.cc}/bin/clang";
  AR_wasm32_unknown_unknown = "${pkgs.llvmPackages.llvm}/bin/llvm-ar";

  # Minimal CFLAGS for blst WASM compilation.  Note that cc-rs already passes
  # --target=wasm32-unknown-unknown based on the cargo target; we only need
  # the BLST_NO_ASM define.
  CFLAGS_wasm32_unknown_unknown = "-D__BLST_NO_ASM__";

  buildPhase = ''
    runHook preBuild

    echo "Building identus-crypto-wasm for wasm32-unknown-unknown..."
    cargo build \
      -j "$NIX_BUILD_CORES" \
      --target wasm32-unknown-unknown \
      --offline \
      --release \
      -p identus-crypto-wasm

    runHook postBuild
  '';

  installPhase = ''
    mkdir -p "$out"
    wasm-bindgen \
      target/wasm32-unknown-unknown/release/identus_crypto_wasm.wasm \
      --target web \
      --out-dir "$out"
    cp -r examples/wasm-app/* "$out/"
  '';
}
