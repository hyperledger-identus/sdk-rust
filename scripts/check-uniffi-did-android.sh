#!/usr/bin/env bash

set -euo pipefail

repository_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
crate_root="$repository_root/crates/uniffi-did"
tool_manifest="$repository_root/tools/uniffi-bindgen/Cargo.toml"
template_root="$repository_root/tests/uniffi-did-android/package-template"
evidence_root="$repository_root/target/uniffi-did-android"
gradle_home="$repository_root/target/uniffi-did-android-gradle"
ndk_version=27.0.12077973
android_api=21
compile_api=35
system_image="system-images;android-35;google_apis_playstore;arm64-v8a"
emulator_port=5580
emulator_serial="emulator-$emulator_port"

fail() {
    printf 'uniffi-did-android: %s\n' "$1" >&2
    exit 1
}

[[ $(uname -s) == "Darwin" ]] || fail "macOS is required for the arm64 emulator gate"

android_sdk=${ANDROID_SDK_ROOT:-${ANDROID_HOME:-}}
[[ -n $android_sdk ]] || fail "ANDROID_SDK_ROOT or ANDROID_HOME must name the Android SDK"
android_sdk=$(cd "$android_sdk" && pwd)
ndk_root=${ANDROID_NDK_ROOT:-"$android_sdk/ndk/$ndk_version"}
[[ -d $ndk_root ]] || fail "exact Android NDK $ndk_version is unavailable at $ndk_root"

host_tag=darwin-x86_64
toolchain="$ndk_root/toolchains/llvm/prebuilt/$host_tag"
linker="$toolchain/bin/aarch64-linux-android${android_api}-clang"
llvm_nm="$toolchain/bin/llvm-nm"
llvm_readelf="$toolchain/bin/llvm-readelf"
avdmanager="$android_sdk/cmdline-tools/latest/bin/avdmanager"
emulator="$android_sdk/emulator/emulator"
adb="$android_sdk/platform-tools/adb"

for executable in "$linker" "$llvm_nm" "$llvm_readelf" "$avdmanager" "$emulator" "$adb"; do
    [[ -x $executable ]] || fail "required Android executable is unavailable: $executable"
done
for command in awk cargo cmp cp curl diff du find gradle grep java mkdir python3 rm rustc sed seq shasum sleep sort stat tr unzip xargs; do
    command -v "$command" >/dev/null || fail "required command is unavailable: $command"
done

java_home=${IDENTUS_JAVA_HOME:-${JAVA_HOME:-}}
if [[ -z $java_home ]]; then
    java_home=$(/usr/libexec/java_home -v 17)
fi
[[ -x $java_home/bin/java ]] || fail "JDK 17 is unavailable"

[[ $evidence_root == "$repository_root"/target/* ]] || fail "unsafe evidence path"
rm -rf "$evidence_root"
mkdir -p "$evidence_root" "$gradle_home"

jna_aar="$evidence_root/jna-5.18.1.aar"
curl --fail --location --silent --show-error \
    https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.18.1/jna-5.18.1.aar \
    --output "$jna_aar"
expected_jna_sha=7f053e3ec99e14dd71259c82c1c8a02738d64a13c31226b2acc170f3060951e0
actual_jna_sha=$(shasum -a 256 "$jna_aar" | awk '{ print $1 }')
[[ $actual_jna_sha == "$expected_jna_sha" ]] || fail "JNA 5.18.1 AAR checksum drifted"

normalize_archive() {
    local source_archive=$1
    local normalized_archive=$2
    local normalized_tree=$3
    python3 - "$source_archive" "$normalized_archive" "$normalized_tree" <<'PY'
import pathlib
import sys
import tempfile
import zipfile

source = pathlib.Path(sys.argv[1])
destination = pathlib.Path(sys.argv[2])
tree = pathlib.Path(sys.argv[3])
tree.mkdir(parents=True)

with zipfile.ZipFile(source) as archive:
    for info in archive.infolist():
        path = pathlib.PurePosixPath(info.filename)
        if path.is_absolute() or ".." in path.parts:
            raise SystemExit(f"unsafe archive path: {info.filename}")
        archive.extract(info, tree)

def canonical_zip(input_root: pathlib.Path, output: pathlib.Path) -> None:
    with zipfile.ZipFile(output, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for file in sorted(path for path in input_root.rglob("*") if path.is_file()):
            relative = file.relative_to(input_root).as_posix()
            info = zipfile.ZipInfo(relative, (1980, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            archive.writestr(info, file.read_bytes())

for nested in sorted(tree.rglob("*.jar")):
    with tempfile.TemporaryDirectory() as temporary:
        nested_tree = pathlib.Path(temporary) / "tree"
        nested_tree.mkdir()
        with zipfile.ZipFile(nested) as archive:
            archive.extractall(nested_tree)
        canonical_zip(nested_tree, nested)

canonical_zip(tree, destination)
PY
}

build_package() {
    local label=$1
    local build_root="$evidence_root/build-$label"
    local rust_root="$build_root/rust"
    local generated_root="$build_root/generated"
    local project_root="$build_root/project"
    local library="$rust_root/aarch64-linux-android/release/libidentus_uniffi_did.so"
    local aar="$project_root/sdk/build/outputs/aar/sdk-release.aar"

    mkdir -p "$build_root"
    cp -R "$template_root/library" "$project_root"

    CARGO_INCREMENTAL=0 \
    CARGO_TARGET_DIR="$rust_root" \
    CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$linker" \
        cargo build --locked --release --package identus-uniffi-did \
        --target aarch64-linux-android

    CARGO_TARGET_DIR="$repository_root/target/uniffi-bindgen-tool" \
        cargo run --quiet --locked --manifest-path "$tool_manifest" \
        --bin identus-uniffi-bindgen -- \
        generate --library "$library" --config "$crate_root/uniffi-global.toml" \
        --language kotlin --out-dir "$generated_root" --no-format

    mkdir -p "$project_root/sdk/src/main/kotlin/org/hyperledger/identus/did" \
        "$project_root/sdk/src/main/jniLibs/arm64-v8a"
    cp "$generated_root/org/hyperledger/identus/did/identus_uniffi_did.kt" \
        "$project_root/sdk/src/main/kotlin/org/hyperledger/identus/did/"
    cp "$library" "$project_root/sdk/src/main/jniLibs/arm64-v8a/"

    JAVA_HOME="$java_home" GRADLE_USER_HOME="$gradle_home" \
        gradle --no-daemon --console=plain -p "$project_root" \
        --write-locks :sdk:assembleRelease
    [[ -f $aar ]] || fail "Gradle did not produce the SDK AAR"
    cmp "$template_root/library/sdk/gradle.lockfile" \
        "$project_root/sdk/gradle.lockfile" || fail "library dependency lock drifted"

    cp "$project_root/sdk/gradle.lockfile" "$build_root/gradle.lockfile"
    normalize_archive "$aar" "$build_root/identus-did.aar" \
        "$build_root/normalized-tree"
}

build_package a
build_package b

cmp "$evidence_root/build-a/rust/aarch64-linux-android/release/libidentus_uniffi_did.so" \
    "$evidence_root/build-b/rust/aarch64-linux-android/release/libidentus_uniffi_did.so"
diff -ru "$evidence_root/build-a/generated" "$evidence_root/build-b/generated"
cmp "$evidence_root/build-a/gradle.lockfile" "$evidence_root/build-b/gradle.lockfile"
diff -ru "$evidence_root/build-a/normalized-tree" \
    "$evidence_root/build-b/normalized-tree"
cmp "$evidence_root/build-a/identus-did.aar" "$evidence_root/build-b/identus-did.aar"

library="$evidence_root/build-a/normalized-tree/jni/arm64-v8a/libidentus_uniffi_did.so"
[[ -f $library ]] || fail "AAR is missing the arm64-v8a SDK library"
native_paths=$(find "$evidence_root/build-a/normalized-tree/jni" -type f | sort)
[[ $native_paths == "$library" ]] || fail "AAR contains an unexpected native payload"
! find "$evidence_root/build-a/normalized-tree" -type f -print0 | \
    xargs -0 grep -aFl "$repository_root" | grep -q . || fail "AAR contains an absolute worktree path"

elf_header=$($llvm_readelf -h "$library")
grep -q 'Class:[[:space:]]*ELF64' <<<"$elf_header" || fail "SDK library is not ELF64"
grep -q 'Data:[[:space:]]*2.s complement, little endian' <<<"$elf_header" || \
    fail "SDK library has unexpected endianness"
grep -q 'Machine:[[:space:]]*AArch64' <<<"$elf_header" || fail "SDK library is not AArch64"

elf_notes=$($llvm_readelf -n "$library")
grep -q '15 00 00 00 72 32 37' <<<"$elf_notes" || fail "SDK library lacks API-21 NDK-r27 identity"
grep -q '31 32 30 37 37 39 37 33' <<<"$elf_notes" || fail "SDK library has an unexpected NDK build"

elf_dynamic=$($llvm_readelf -d "$library")
grep -q 'Shared library: \[libdl.so\]' <<<"$elf_dynamic" || fail "libdl need is missing"
grep -q 'Shared library: \[libc.so\]' <<<"$elf_dynamic" || fail "libc need is missing"
[[ $(grep -c 'Shared library:' <<<"$elf_dynamic") == 2 ]] || fail "unexpected ELF runtime dependency"
grep -q 'BIND_NOW' <<<"$elf_dynamic" || fail "BIND_NOW is missing"
grep -q 'FLAGS_1.*NOW' <<<"$elf_dynamic" || fail "NOW is missing"

elf_segments=$($llvm_readelf -l "$library")
grep -q 'GNU_RELRO' <<<"$elf_segments" || fail "GNU_RELRO is missing"
grep -q 'GNU_STACK.*RW ' <<<"$elf_segments" || fail "non-executable GNU_STACK is missing"

symbols=$($llvm_nm --dynamic --defined-only --extern-only "$library")
for symbol in \
    uniffi_identus_uniffi_did_fn_func_binding_api_version \
    uniffi_identus_uniffi_did_fn_func_parse_did \
    uniffi_identus_uniffi_did_fn_func_parse_did_url \
    ffi_identus_uniffi_did_uniffi_contract_version; do
    grep -q "$symbol" <<<"$symbols" || fail "required exported symbol is missing: $symbol"
done

consumer_root="$evidence_root/consumer"
cp -R "$template_root/consumer" "$consumer_root"
cp "$evidence_root/build-a/gradle.lockfile" "$evidence_root/library-gradle.lockfile"
JAVA_HOME="$java_home" GRADLE_USER_HOME="$gradle_home" \
    gradle --no-daemon --console=plain -p "$consumer_root" --write-locks \
    -PidentusDidAar="$evidence_root/build-a/identus-did.aar" :app:assembleDebug
consumer_apk="$consumer_root/app/build/outputs/apk/debug/app-debug.apk"
[[ -f $consumer_apk ]] || fail "Gradle did not produce the consumer APK"
cmp "$template_root/consumer/app/gradle.lockfile" \
    "$consumer_root/app/gradle.lockfile" || fail "consumer dependency lock drifted"

avd_home="$evidence_root/avd-home"
prefs_root="$evidence_root/android-prefs"
emulator_home="$evidence_root/emulator-home"
mkdir -p "$avd_home" "$prefs_root" "$emulator_home"
image_dir="$android_sdk/system-images/android-$compile_api/google_apis_playstore/arm64-v8a"
[[ -d $image_dir ]] || fail "exact emulator image is unavailable: $system_image"

export ANDROID_AVD_HOME="$avd_home"
export ANDROID_PREFS_ROOT="$prefs_root"
export ANDROID_EMULATOR_HOME="$emulator_home"
printf 'no\n' | "$avdmanager" create avd --force --name identus_did_api35 \
    --package "$system_image" --device pixel_6 >/dev/null

emulator_pid=
cleanup() {
    if [[ -n ${emulator_pid:-} ]]; then
        "$adb" -s "$emulator_serial" emu kill >/dev/null 2>&1 || true
        kill "$emulator_pid" >/dev/null 2>&1 || true
        wait "$emulator_pid" >/dev/null 2>&1 || true
    fi
}
trap cleanup EXIT

"$emulator" -avd identus_did_api35 -port "$emulator_port" -no-window \
    -no-audio -no-boot-anim -no-snapshot -wipe-data -gpu swiftshader_indirect \
    >"$evidence_root/emulator.log" 2>&1 &
emulator_pid=$!

booted=false
for _ in $(seq 1 120); do
    if [[ $("$adb" -s "$emulator_serial" shell getprop sys.boot_completed 2>/dev/null | tr -d '\r') == 1 ]]; then
        booted=true
        break
    fi
    kill -0 "$emulator_pid" 2>/dev/null || fail "Android emulator exited before boot"
    sleep 2
done
[[ $booted == true ]] || fail "Android emulator did not boot"

"$adb" -s "$emulator_serial" install -r "$consumer_apk" >/dev/null
"$adb" -s "$emulator_serial" logcat -c
"$adb" -s "$emulator_serial" shell am start -W \
    -n org.hyperledger.identus.did.consumer/.MainActivity >/dev/null

runtime_log=
for _ in $(seq 1 30); do
    runtime_log=$("$adb" -s "$emulator_serial" logcat -d -s IdentusDidProof:I '*:S')
    if grep -q 'IDENTUS_DID_ANDROID_OK' <<<"$runtime_log"; then
        break
    fi
    if grep -q 'IDENTUS_DID_ANDROID_FAIL' <<<"$runtime_log"; then
        printf '%s\n' "$runtime_log" >&2
        fail "Android behavior contract failed"
    fi
    sleep 1
done
grep -q 'IDENTUS_DID_ANDROID_OK' <<<"$runtime_log" || fail "Android success marker was not observed"
printf '%s\n' "$runtime_log" >"$evidence_root/runtime.log"

{
    printf 'rust=%s\n' "$(rustc --version)"
    printf 'java=%s\n' "$("$java_home/bin/java" -version 2>&1 | awk 'NR == 1')"
    printf 'gradle=%s\n' "$(gradle --version | awk '/^Gradle / { print $2 }')"
    printf 'ndk=%s\n' "$ndk_version"
    printf 'android_min_api=%s\n' "$android_api"
    printf 'emulator_image=%s\n' "$system_image"
    printf 'emulator_abi=%s\n' "$("$adb" -s "$emulator_serial" shell getprop ro.product.cpu.abi | tr -d '\r')"
    printf 'emulator_api=%s\n' "$("$adb" -s "$emulator_serial" shell getprop ro.build.version.sdk | tr -d '\r')"
    printf 'sdk_library_bytes=%s\n' "$(stat -f %z "$library")"
    printf 'sdk_aar_bytes=%s\n' "$(stat -f %z "$evidence_root/build-a/identus-did.aar")"
    printf 'consumer_apk_bytes=%s\n' "$(stat -f %z "$consumer_apk")"
} >"$evidence_root/receipt.txt"

cleanup
trap - EXIT
printf 'uniffi-did-android: verification passed\n'
