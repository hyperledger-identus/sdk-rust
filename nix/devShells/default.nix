_: {
  perSystem =
    { pkgs, sdk-rustLib, ... }:
    let
      inherit (sdk-rustLib.rustTools) rust;

      android-ndk = pkgs.androidenv.androidPkgs.ndk-bundle;

      # Composed Android SDK (platform-tools, emulator, system images for APK build + run)
      androidSdk = pkgs.androidenv.composeAndroidPackages {
        platformVersions = [
          "24"
          "34"
        ];
        buildToolsVersions = [ "34.0.0" ];
        includeEmulator = true;
        includeSystemImages = true;
        systemImageTypes = [ "default" ];
        abiVersions = [ "x86_64" ];
      };
      # NDK prebuilt toolchain directory tag: <os>-<arch> (e.g. "linux-x86_64", "darwin-arm64")
      # Note: NDK uses "arm64" for macOS ARM (not "aarch64")
      ndkHostTriple =
        let
          os = pkgs.stdenv.hostPlatform.parsed.kernel.name;
          arch = pkgs.stdenv.hostPlatform.parsed.cpu.name;
          ndkArch = if os == "darwin" && arch == "aarch64" then "arm64" else arch;
        in
        "${os}-${ndkArch}";
      ndkBin = "${android-ndk}/libexec/android-sdk/ndk/${android-ndk.version}/toolchains/llvm/prebuilt/${ndkHostTriple}/bin";

      # API level for Android minimum supported version
      androidApi = "24";

      # Android target triples as used by Rust
      androidTargets = {
        aarch64 = {
          target = "aarch64-linux-android";
          clang = "aarch64-linux-android${androidApi}-clang";
        };
        armv7 = {
          target = "armv7-linux-androideabi";
          clang = "armv7a-linux-androideabi${androidApi}-clang";
        };
        x86_64 = {
          target = "x86_64-linux-android";
          clang = "x86_64-linux-android${androidApi}-clang";
        };
        i686 = {
          target = "i686-linux-android";
          clang = "i686-linux-android${androidApi}-clang";
        };
      };
    in
    {
      devshells.default = {
        name = "sdk-rust";

        packages = with pkgs; [
          # base
          just
          nix
          nixfmt
          which
          # text linters
          deadnix
          editorconfig-checker
          markdownlint-cli2
          shellcheck
          statix
          yamllint
          # rust
          cargo-edit
          cargo-llvm-cov
          cargo-udeps
          clang
          rust
          # fmt
          ktlint
          swift-format
          taplo
          # wasm
          wasm-pack
          # android cross-compilation
          android-ndk
          cargo-ndk
          # android SDK (platform-tools, emulator, system images)
          androidSdk.androidsdk
          # JDK 17 for Gradle
          jdk17
          # Gradle build tool
          gradle
          # uniffi-bindgen for foreign-language bindings (TASK-9)
          (pkgs.callPackage ../packages/uniffi-bindgen.nix { inherit sdk-rustLib; })
        ];

        env = [
          {
            name = "LANG";
            value = "C.utf8";
          }
          {
            name = "CC_wasm32_unknown_unknown";
            value = "${pkgs.clang.cc}/bin/clang";
          }
          {
            name = "ANDROID_HOME";
            value = "${androidSdk.androidsdk}/libexec/android-sdk";
          }
          {
            name = "ANDROID_NDK_HOME";
            value = "${android-ndk}/libexec/android-sdk/ndk/${android-ndk.version}";
          }
          # Android cross-compilation linker and CC/AR environment variables
          {
            name = "CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER";
            value = "${ndkBin}/${androidTargets.aarch64.clang}";
          }
          {
            name = "CC_aarch64_linux_android";
            value = "${ndkBin}/${androidTargets.aarch64.clang}";
          }
          {
            name = "AR_aarch64_linux_android";
            value = "${ndkBin}/llvm-ar";
          }
          {
            name = "CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER";
            value = "${ndkBin}/${androidTargets.armv7.clang}";
          }
          {
            name = "CC_armv7_linux_androideabi";
            value = "${ndkBin}/${androidTargets.armv7.clang}";
          }
          {
            name = "AR_armv7_linux_androideabi";
            value = "${ndkBin}/llvm-ar";
          }
          {
            name = "CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER";
            value = "${ndkBin}/${androidTargets.x86_64.clang}";
          }
          {
            name = "CC_x86_64_linux_android";
            value = "${ndkBin}/${androidTargets.x86_64.clang}";
          }
          {
            name = "AR_x86_64_linux_android";
            value = "${ndkBin}/llvm-ar";
          }
          {
            name = "CARGO_TARGET_I686_LINUX_ANDROID_LINKER";
            value = "${ndkBin}/${androidTargets.i686.clang}";
          }
          {
            name = "CC_i686_linux_android";
            value = "${ndkBin}/${androidTargets.i686.clang}";
          }
          {
            name = "AR_i686_linux_android";
            value = "${ndkBin}/llvm-ar";
          }
        ];
      };
    };
}
