#!/usr/bin/env python3
"""Mutation tests for the cross-language vector catalog checker."""

from __future__ import annotations

import shutil
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-cross-language-vectors.py"
CATALOG = ROOT / "docs/conformance/cross-language-vector-catalog.toml"
PACKET = ROOT / "docs/conformance/fixtures/did/v1/vectors.json"


def replace_once(text: str, before: str, after: str) -> str:
    count = text.count(before)
    if count != 1:
        raise AssertionError(f"mutation source must occur exactly once: {before!r}; found {count}")
    return text.replace(before, after, 1)


def run(catalog: str, packet: str, should_pass: bool) -> None:
    with tempfile.TemporaryDirectory(prefix="cross-language-vectors-") as temporary:
        root = Path(temporary)
        catalog_target = root / "docs/conformance/cross-language-vector-catalog.toml"
        packet_target = root / "docs/conformance/fixtures/did/v1/vectors.json"
        catalog_target.parent.mkdir(parents=True)
        packet_target.parent.mkdir(parents=True)
        catalog_target.write_text(catalog, encoding="utf-8")
        packet_target.write_text(packet, encoding="utf-8")
        result = subprocess.run([str(CHECKER), str(root)], capture_output=True, text=True, check=False)
        if (result.returncode == 0) != should_pass:
            raise AssertionError(
                f"expected pass={should_pass}, got {result.returncode}\nstdout={result.stdout}\nstderr={result.stderr}"
            )


def main() -> None:
    catalog = CATALOG.read_text(encoding="utf-8")
    packet = PACKET.read_text(encoding="utf-8")
    run(catalog, packet, True)
    run(
        replace_once(
            catalog,
            'schema_version  = 1\ncatalog_version = "1.0.0"',
            'schema_version  = 1\nunknown         = true\ncatalog_version = "1.0.0"',
        ),
        packet,
        False,
    )
    run(replace_once(catalog, 'revision       = "edf03abcc963d37703daa61192940394bdc553ca"', 'revision       = "develop"'), packet, False)
    run(replace_once(catalog, 'public_data    = true', 'public_data    = false'), packet, False)
    run(
        replace_once(
            catalog,
            'id                 = "did.syntax.valid-basic"\npacket             = "did.syntax.v1"\ncase               = "did.syntax.valid-basic"\ncapability         = "did.syntax"\nauthority          = "normative"',
            'id                 = "did.syntax.valid-basic"\npacket             = "did.syntax.v1"\ncase               = "did.syntax.valid-basic"\ncapability         = "did.syntax"\nauthority          = "copied"',
        ),
        packet,
        False,
    )
    run(replace_once(catalog, 'case               = "did.syntax.valid-basic"', 'case               = "did.syntax.missing"'), packet, False)
    run(replace_once(catalog, 'sha256         = "72ebf6603594fe3b2b93f9d1a9cd9dc40bf8708f6bc8a7b4370ee9b9f173679a"', 'sha256         = "' + "0" * 64 + '"'), packet, False)
    run(catalog, replace_once(packet, '"schemaVersion": 1,', '"schemaVersion": 1,\n  "unknown": true,'), False)
    run(catalog, replace_once(packet, '"count": 2036', '"count": 10001'), False)
    run(catalog, packet + "\n", False)

    with tempfile.TemporaryDirectory(prefix="cross-language-vector-symlink-") as temporary:
        root = Path(temporary)
        catalog_target = root / "docs/conformance/cross-language-vector-catalog.toml"
        packet_target = root / "docs/conformance/fixtures/did/v1/vectors.json"
        catalog_target.parent.mkdir(parents=True)
        packet_target.parent.mkdir(parents=True)
        catalog_target.write_text(catalog, encoding="utf-8")
        external = root / "outside.json"
        shutil.copy2(PACKET, external)
        packet_target.symlink_to(external)
        result = subprocess.run([str(CHECKER), str(root)], capture_output=True, text=True, check=False)
        if result.returncode == 0:
            raise AssertionError("symlinked packets must fail validation")

    print("cross-language-vectors-tests: mutation suite passed")


if __name__ == "__main__":
    main()
