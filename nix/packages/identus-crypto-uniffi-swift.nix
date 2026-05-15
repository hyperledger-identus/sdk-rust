{
  pkgs,
  sdk-rustLib,
  uniffi-bindgen,
}:

let
  inherit (pkgs) lib;
  inherit (sdk-rustLib) rustTools;

  # rustPlatform from the project's nightly toolchain.
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustTools.rust;
    rustc = rustTools.rust;
  };

  # Minimal source tree: only the paths needed to build identus-crypto-uniffi.
  # Filtering avoids pulling in unrelated files (nix/, .git/, target/,
  # justfile, examples/, etc.) that would cause unnecessary rebuilds.
  #
  # IMPORTANT: for directories, we must return `true` for any directory that
  # is an ancestor of (or equals) a needed path, otherwise `builtins.path`
  # will skip the entire subtree.
  src = builtins.path {
    path = ./../..;
    name = "identus-crypto-uniffi-swift-src";
    filter =
      path: type:
      let
        root = toString ./../..;
        rel = builtins.substring (builtins.stringLength root) (-1) (toString path);
        isDir = type == "directory";
        isFile = type == "regular";
        keepRoot = isDir && rel == "";

        # Directories / files whose contents we need (relative to root).
        neededPrefixes = [
          "/lib/identus-crypto-uniffi"
          "/lib/identus-crypto"
          "/lib/identus-core"
        ];

        # A directory is needed if it is an ancestor of (or equals) a
        # needed prefix, OR it is a descendant of a needed prefix.
        isNeededDir =
          isDir
          && (
            lib.any (prefix: lib.hasPrefix rel prefix) neededPrefixes
            || lib.any (prefix: lib.hasPrefix prefix rel) neededPrefixes
          );

        # Root-level workspace definition files.
        isRootFile = isFile && (rel == "/Cargo.toml" || rel == "/Cargo.lock");

        # A regular file inside one of the needed subtrees.
        isInsideNeeded = isFile && lib.any (prefix: lib.hasPrefix prefix rel) neededPrefixes;
      in
      keepRoot || isNeededDir || isRootFile || isInsideNeeded;
  };
in

rustPlatform.buildRustPackage {
  pname = "identus-crypto-uniffi-swift";
  version = "0.1.0";

  inherit src;
  inherit (rustTools) cargoLock;

  # Disable the default cargoBuildHook so we can write our own buildPhase.
  dontCargoBuild = true;

  nativeBuildInputs = [ uniffi-bindgen ];

  buildPhase = ''
    runHook preBuild
    cargo build -j "$NIX_BUILD_CORES" --offline --release -p identus-crypto-uniffi
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
    mkdir -p "$out/bindings/swift"
    # Find the built shared library (.so on Linux, .dylib on macOS)
    # buildRustPackage may set a custom target directory, so we search
    # broadly rather than assuming target/release/.
    LIB=$(find target -name "libidentus_crypto_uniffi.so" -o -name "libidentus_crypto_uniffi.dylib" 2>/dev/null | head -1)
    if [ -z "$LIB" ]; then
      echo "Error: libidentus_crypto_uniffi library not found under target/"
      find target -name "*.so" -o -name "*.dylib" 2>/dev/null || echo "  (no .so or .dylib files found)"
      ls -la target/ 2>/dev/null | head -20
      exit 1
    fi
    echo "Found library: $LIB"
    cp "$LIB" "$out/"
    uniffi-bindgen generate \
      --library "$out/$(basename "$LIB")" \
      --language swift \
      --out-dir "$out/bindings/swift/"
    runHook postInstall
  '';

  doCheck = false;

  meta = {
    description = "Swift bindings for Identus crypto (UniFFI) — shared library + generated .swift file";
    homepage = "https://github.com/hyperledger-identus";
    license = lib.licenses.asl20;
  };
}
