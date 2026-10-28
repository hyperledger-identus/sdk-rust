#!/usr/bin/env python3
"""Run the rejected JWE candidate as an isolated interoperability oracle.

This tool deliberately creates a disposable Cargo package outside the repository.
Passing the oracle does not authorize production adoption: the current exact
candidate is expected to fail the RustSec production gate with RUSTSEC-2023-0071.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path


CANDIDATE = "jose-rs"
VERSION = "0.10.0"
BLOCKING_ADVISORY = "RUSTSEC-2023-0071"
BLOCKING_PACKAGE = "rsa 0.10.0-rc.19"
LOCK_SHA256 = "2e893f1a3a046b55c26c9399e761d3ab812d65c9a2bf736862526b9cb47d4638"
TARGETS = {
    "wasm32-unknown-unknown": "wasm",
    "aarch64-apple-ios": "bindings",
    "aarch64-linux-android": "bindings",
}

MANIFEST = """[package]
name = "sdk-rust-jwe-qualification"
version = "0.0.0"
edition = "2024"
rust-version = "1.89"

[dependencies]
base64 = "=0.22.1"
identus-jose = { path = "__IDENTUS_JOSE_PATH__" }
jose-rs = { version = "=0.10.0", default-features = false }
serde_json = "=1.0.145"

[target.'cfg(target_arch = "wasm32")'.dependencies]
getrandom = { version = "=0.4.2", features = ["wasm_js"] }
"""

SOURCE = r'''use std::time::Instant;

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use identus_jose::{
    CompactJwe, JweContentEncryptionAlgorithm, JweEncrypter, JweEncryptionError,
    JweEncryptionRequest, JweKeyManagementAlgorithm, JweLimits, JweRecipientPublicKey,
};
use jose_rs::{JoseHeader, JweAlgorithm, JweEncryption, jwk::Jwk};
use serde_json::Value;

const RECIPIENT: &str = r#"{
  "kty":"EC",
  "use":"enc",
  "alg":"ECDH-ES",
  "kid":"recipient-key",
  "crv":"P-256",
  "x":"Ze2loSV3wrroKUN_4zhwGhCqo3Xhu1td4QjeQ5wIVR0",
  "y":"HlLtdXARY_f55A3fnzQbPcm6hgr34Mp8p-nuzQCE0Zw",
  "d":"r_kHyZ-a06rmxM3yESK84r1otSg-aQcVStkRhA-iCM8"
}"#;
const PLAINTEXT: &[u8] = b"independent decryptability";

struct JoseRsAdapter;

impl JweEncrypter for JoseRsAdapter {
    fn encrypt(&self, request: JweEncryptionRequest) -> Result<CompactJwe, JweEncryptionError> {
        if request.key_management_algorithm() != JweKeyManagementAlgorithm::EcdhEs
            || request.content_encryption_algorithm() != JweContentEncryptionAlgorithm::A128Gcm
        {
            return Err(JweEncryptionError::EncryptionRejected);
        }
        let point = request.recipient().expose_public_sec1();
        let public: Jwk = serde_json::from_value(serde_json::json!({
            "kty": "EC",
            "use": "enc",
            "alg": "ECDH-ES",
            "kid": request.recipient().expose_key_id(),
            "crv": "P-256",
            "x": URL_SAFE_NO_PAD.encode(&point[1..33]),
            "y": URL_SAFE_NO_PAD.encode(&point[33..65]),
        }))
        .map_err(|_| JweEncryptionError::EncryptionRejected)?;
        let mut header = JoseHeader::for_jwe(JweAlgorithm::EcdhEs, JweEncryption::A128GCM);
        header.kid = public.kid.clone();
        let compact = jose_rs::jwe::compact::encrypt_with_jwk_header(
            &public,
            header,
            request.expose_sensitive_plaintext(),
            JweEncryption::A128GCM,
        )
        .map_err(|_| JweEncryptionError::EncryptionRejected)?;
        CompactJwe::new(compact, request.max_compact_bytes())
    }
}

fn round_trip(private: &Jwk, public: &Jwk) {
    let x: [u8; 32] = URL_SAFE_NO_PAD.decode(public.x.as_deref().expect("x"))
        .expect("x base64url").try_into().expect("x size");
    let y: [u8; 32] = URL_SAFE_NO_PAD.decode(public.y.as_deref().expect("y"))
        .expect("y base64url").try_into().expect("y size");
    let limits = JweLimits::default();
    let recipient = JweRecipientPublicKey::p256_coordinates(
        &x,
        &y,
        Some("recipient-key"),
        limits,
    )
    .expect("SDK recipient");
    let request = JweEncryptionRequest::new(
        recipient,
        JweKeyManagementAlgorithm::EcdhEs,
        JweContentEncryptionAlgorithm::A128Gcm,
        PLAINTEXT,
        limits,
    )
    .expect("SDK request");
    let compact = JoseRsAdapter.encrypt(request).expect("SDK adapter encrypt");
    let compact = compact.expose_sensitive_compact();
    let segments: Vec<_> = compact.split('.').collect();
    assert_eq!(segments.len(), 5);
    assert!(segments[1].is_empty(), "direct ECDH-ES has an empty encrypted-key segment");
    let protected: Value = serde_json::from_slice(
        &URL_SAFE_NO_PAD.decode(segments[0]).expect("protected header base64url"),
    )
    .expect("protected header JSON");
    assert_eq!(protected["alg"], "ECDH-ES");
    assert_eq!(protected["enc"], "A128GCM");
    assert_eq!(protected["kid"], "recipient-key");
    assert_eq!(protected["epk"]["kty"], "EC");
    assert_eq!(protected["epk"]["crv"], "P-256");
    assert!(protected["epk"].get("d").is_none());
    let recovered = jose_rs::jwe::compact::decrypt_with_jwk(private, &compact)
        .expect("decrypt with private recipient");
    assert_eq!(recovered, PLAINTEXT);
}

fn main() {
    let private: Jwk = serde_json::from_str(RECIPIENT).expect("private test recipient");
    let public = private.to_public_jwk();
    let started = Instant::now();
    for _ in 0..25 {
        round_trip(&private, &public);
    }
    println!("jwe-oracle: 25 round trips passed in {} us", started.elapsed().as_micros());
}
'''


class QualificationError(RuntimeError):
    """A qualification command or expected disposition failed."""


def nix_executable() -> str:
    discovered = shutil.which("nix")
    if discovered is not None:
        return discovered
    fallback = Path("/nix/var/nix/profiles/default/bin/nix")
    if fallback.is_file():
        return str(fallback)
    raise QualificationError("Nix is required for portable and audit evidence")


def run(
    command: list[str],
    *,
    cwd: Path,
    env: dict[str, str],
    emit_stdout: bool = True,
) -> subprocess.CompletedProcess[str]:
    started = time.monotonic()
    result = subprocess.run(command, cwd=cwd, env=env, text=True, capture_output=True, check=False)
    elapsed = time.monotonic() - started
    print(f"jwe-qualification: {' '.join(command)} ({elapsed:.2f}s)")
    if emit_stdout and result.stdout:
        print(result.stdout, end="")
    return result


def require_success(result: subprocess.CompletedProcess[str], label: str) -> None:
    if result.returncode != 0:
        if result.stderr:
            print(result.stderr, file=sys.stderr, end="")
        raise QualificationError(f"{label} failed with exit {result.returncode}")


def prepare(root: Path, work: Path, env: dict[str, str]) -> None:
    (work / "src").mkdir(parents=True)
    manifest = MANIFEST.replace("__IDENTUS_JOSE_PATH__", (root / "crates/jose").as_posix())
    (work / "Cargo.toml").write_text(manifest, encoding="utf-8")
    (work / "src/main.rs").write_text(SOURCE, encoding="utf-8")
    require_success(
        run(["cargo", "+1.89.0", "generate-lockfile"], cwd=work, env=env),
        "exact candidate lock",
    )
    actual_lock = hashlib.sha256((work / "Cargo.lock").read_bytes()).hexdigest()
    if actual_lock != LOCK_SHA256:
        raise QualificationError(
            f"candidate lock drifted: expected {LOCK_SHA256}, got {actual_lock}"
        )


def host(work: Path, env: dict[str, str]) -> None:
    require_success(
        run(["cargo", "+1.89.0", "run", "--locked", "--release"], cwd=work, env=env),
        "independent public-encrypt/private-decrypt oracle",
    )


def portable(root: Path, work: Path, env: dict[str, str]) -> None:
    for target, shell in TARGETS.items():
        command = [
            nix_executable(),
            "develop",
            f"{root}#{shell}",
            "--command",
            "env",
            f"CARGO_TARGET_DIR={env['CARGO_TARGET_DIR']}",
            "cargo",
            "check",
            "--locked",
            "--target",
            target,
        ]
        require_success(run(command, cwd=work, env=env), f"portable target {target}")


def exact_audit_blocker(report: object) -> bool:
    try:
        vulnerabilities = report["vulnerabilities"]
        findings = vulnerabilities["list"]
        finding = findings[0]
        return (
            vulnerabilities["found"] is True
            and vulnerabilities["count"] == 1
            and len(findings) == 1
            and finding["advisory"]["id"] == BLOCKING_ADVISORY
            and finding["package"]["name"] == "rsa"
            and finding["package"]["version"] == "0.10.0-rc.19"
            and report["warnings"] == {}
        )
    except (IndexError, KeyError, TypeError):
        return False


def audit(root: Path, work: Path, env: dict[str, str]) -> None:
    command = [
        nix_executable(),
        "develop",
        str(root),
        "--command",
        "cargo",
        "audit",
        "--file",
        str(work / "Cargo.lock"),
        "--deny",
        "warnings",
        "--format",
        "json",
    ]
    result = run(command, cwd=work, env=env, emit_stdout=False)
    if result.returncode == 0:
        raise QualificationError("candidate audit unexpectedly passed; open a new adoption review")
    try:
        report = json.loads(result.stdout)
    except json.JSONDecodeError:
        exact_blocker = False
    else:
        exact_blocker = exact_audit_blocker(report)
    if not exact_blocker:
        if result.stderr:
            print(result.stderr, file=sys.stderr, end="")
        raise QualificationError("candidate audit failed for an unexpected reason")
    print(
        "jwe-qualification: oracle-qualified / production-rejected "
        f"({BLOCKING_ADVISORY}, {BLOCKING_PACKAGE})"
    )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--mode", choices=("host", "portable", "audit", "all"), default="all")
    parser.add_argument("--keep", action="store_true", help="retain the disposable package for debugging")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = Path(__file__).resolve().parents[1]
    temporary = Path(tempfile.mkdtemp(prefix="sdk-rust-jwe-qualification-"))
    try:
        env = os.environ.copy()
        env["CARGO_TARGET_DIR"] = str(temporary / "target")
        env["CARGO_TERM_COLOR"] = "never"
        work = temporary / "oracle"
        prepare(root, work, env)
        if args.mode in {"host", "all"}:
            host(work, env)
        if args.mode in {"portable", "all"}:
            portable(root, work, env)
        if args.mode in {"audit", "all"}:
            audit(root, work, env)
        print(f"jwe-qualification: {CANDIDATE} {VERSION} disposition verified")
        return 0
    except QualificationError as error:
        print(f"jwe-qualification: {error}", file=sys.stderr)
        return 1
    finally:
        if args.keep:
            print(f"jwe-qualification: retained {temporary}")
        else:
            shutil.rmtree(temporary, ignore_errors=True)


if __name__ == "__main__":
    raise SystemExit(main())
