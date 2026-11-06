#!/usr/bin/env python3
"""Validate the isolated JWE qualification contract without network access."""

from __future__ import annotations

import re
import sys
from pathlib import Path


RUNNER = Path("scripts/qualify-jwe-engine.py")
ADR = Path("docs/adr/0192-require-all-gates-before-adopting-a-jwe-engine.md")
KNOWLEDGE = Path("docs/architecture/oid4vp-encrypted-response-boundary.md")


def validate(root: Path) -> list[str]:
    errors: list[str] = []
    try:
        runner = (root / RUNNER).read_text(encoding="utf-8")
        adr = (root / ADR).read_text(encoding="utf-8")
        knowledge = (root / KNOWLEDGE).read_text(encoding="utf-8")
    except OSError as error:
        return [f"cannot read qualification contract: {error}"]

    required_runner = (
        'VERSION = "0.10.0"',
        'BLOCKING_ADVISORY = "RUSTSEC-2023-0071"',
        'BLOCKING_PACKAGE = "rsa 0.10.0-rc.19"',
        'LOCK_SHA256 = "2e893f1a3a046b55c26c9399e761d3ab812d65c9a2bf736862526b9cb47d4638"',
        'identus-jose = { path = "__IDENTUS_JOSE_PATH__" }',
        'jose-rs = { version = "=0.10.0", default-features = false }',
        '"cargo", "+1.89.0", "run", "--locked", "--release"',
        '"wasm32-unknown-unknown": "wasm"',
        '"aarch64-apple-ios": "bindings"',
        '"aarch64-linux-android": "bindings"',
        'candidate audit unexpectedly passed; open a new adoption review',
        'vulnerabilities["count"] == 1',
        'and report["warnings"] == {}',
        'oracle-qualified / production-rejected',
        'all qualification gates verified',
        'partial evidence complete: {args.mode} gate only',
    )
    for phrase in required_runner:
        if phrase not in runner:
            errors.append(f"runner is missing exact fail-closed contract: {phrase}")
    if len(re.findall(r"path\s*=", runner)) != 1:
        errors.append("oracle manifest may use only the isolated identus-jose path adapter")
    if "cargo audit" in runner:
        errors.append("runner must invoke cargo audit as argument tokens, not a shell command")
    if "--ignore" in runner or "--allow" in runner:
        errors.append("runner must not suppress advisory evidence")
    for phrase in ("#538", "RUSTSEC-2023-0071", "public-encrypt/private-decrypt"):
        if phrase not in adr + knowledge:
            errors.append(f"knowledge contract is missing: {phrase}")
    return errors


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    errors = validate(root)
    if errors:
        for error in errors:
            print(f"jwe-engine-qualification: {error}", file=sys.stderr)
        return 1
    print("jwe-engine-qualification: isolated oracle contract passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
