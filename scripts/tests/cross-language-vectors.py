#!/usr/bin/env python3
"""Mutation tests for the cross-language vector catalog checker."""

from __future__ import annotations

import hashlib
import shutil
import subprocess
import tempfile
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-cross-language-vectors.py"
CATALOG = ROOT / "docs/conformance/cross-language-vector-catalog.toml"
PACKET = ROOT / "docs/conformance/fixtures/did/v1/vectors.json"
CRYPTO_PACKET = ROOT / "crates/crypto/tests/fixtures/crypto/v1/vectors.json"
RESOLUTION_PACKET = ROOT / "docs/conformance/fixtures/did-resolution/v1/vectors.json"
PACKET_PATH = PACKET.relative_to(ROOT)
CRYPTO_PACKET_PATH = CRYPTO_PACKET.relative_to(ROOT)
RESOLUTION_PACKET_PATH = RESOLUTION_PACKET.relative_to(ROOT)


def replace_once(text: str, before: str, after: str) -> str:
    count = text.count(before)
    if count != 1:
        raise AssertionError(f"mutation source must occur exactly once: {before!r}; found {count}")
    return text.replace(before, after, 1)


def replace_first(text: str, before: str, after: str) -> str:
    if before not in text:
        raise AssertionError(f"mutation source is missing: {before!r}")
    return text.replace(before, after, 1)


def stage_packets(root: Path, catalog: str, overrides: dict[Path, str] | None = None) -> None:
    """Materialize every packet referenced by the catalog under a fixture root."""
    replacements = overrides or {}
    document = tomllib.loads(catalog)
    for packet_record in document.get("packets", []):
        relative = Path(packet_record["path"])
        target = root / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        if relative in replacements:
            target.write_text(replacements[relative], encoding="utf-8")
        else:
            shutil.copy2(ROOT / relative, target)


def run(
    catalog: str,
    packet: str,
    should_pass: bool,
    *,
    crypto_packet: str | None = None,
    resolution_packet: str | None = None,
) -> None:
    with tempfile.TemporaryDirectory(prefix="cross-language-vectors-") as temporary:
        root = Path(temporary)
        catalog_target = root / "docs/conformance/cross-language-vector-catalog.toml"
        catalog_target.parent.mkdir(parents=True)
        catalog_target.write_text(catalog, encoding="utf-8")
        stage_packets(
            root,
            catalog,
            {
                PACKET_PATH: packet,
                CRYPTO_PACKET_PATH: (
                    CRYPTO_PACKET.read_text(encoding="utf-8")
                    if crypto_packet is None
                    else crypto_packet
                ),
                RESOLUTION_PACKET_PATH: (
                    RESOLUTION_PACKET.read_text(encoding="utf-8")
                    if resolution_packet is None
                    else resolution_packet
                ),
            },
        )
        result = subprocess.run([str(CHECKER), str(root)], capture_output=True, text=True, check=False)
        if (result.returncode == 0) != should_pass:
            raise AssertionError(
                f"expected pass={should_pass}, got {result.returncode}\nstdout={result.stdout}\nstderr={result.stderr}"
            )


def catalog_for_packet(catalog: str, packet: str) -> str:
    old_digest = hashlib.sha256(PACKET.read_bytes()).hexdigest()
    new_digest = hashlib.sha256(packet.encode()).hexdigest()
    return replace_once(catalog, f'sha256         = "{old_digest}"', f'sha256         = "{new_digest}"')


def run_packet_mutation(catalog: str, packet: str) -> None:
    run(catalog_for_packet(catalog, packet), packet, False)


def run_resolution_packet_mutation(catalog: str, packet: str) -> None:
    old_digest = hashlib.sha256(RESOLUTION_PACKET.read_bytes()).hexdigest()
    new_digest = hashlib.sha256(packet.encode()).hexdigest()
    mutated_catalog = replace_once(
        catalog,
        f'sha256         = "{old_digest}"',
        f'sha256         = "{new_digest}"',
    )
    run(mutated_catalog, PACKET.read_text(encoding="utf-8"), False, resolution_packet=packet)


def catalog_for_crypto_packet(catalog: str, packet: str) -> str:
    old_digest = hashlib.sha256(CRYPTO_PACKET.read_bytes()).hexdigest()
    new_digest = hashlib.sha256(packet.encode()).hexdigest()
    return replace_once(catalog, f'sha256         = "{old_digest}"', f'sha256         = "{new_digest}"')


def main() -> None:
    catalog = CATALOG.read_text(encoding="utf-8")
    packet = PACKET.read_text(encoding="utf-8")
    crypto_packet = CRYPTO_PACKET.read_text(encoding="utf-8")
    resolution_packet = RESOLUTION_PACKET.read_text(encoding="utf-8")
    run(catalog, packet, True)
    negative_crypto_packet = replace_once(
        crypto_packet,
        '"kind": "success",\n        "errorCode": null,\n        "redacted": null,\n        "fields": {\n          "operationKind": "verified"\n        }',
        '"kind": "error",\n        "errorCode": "crypto.signature_invalid",\n        "redacted": true,\n        "fields": {}',
    )
    negative_crypto_catalog = replace_once(
        catalog_for_crypto_packet(catalog, negative_crypto_packet),
        'operation          = "crypto.secp256k1.verify"\nexpected           = "success"',
        'operation          = "crypto.secp256k1.verify"\nexpected           = "crypto.signature_invalid"',
    )
    run(
        negative_crypto_catalog,
        packet,
        True,
        crypto_packet=negative_crypto_packet,
    )
    run(
        replace_first(
            catalog,
            'schema_version  = 1\ncatalog_version = "1.0.0"',
            'schema_version  = 1\nunknown         = true\ncatalog_version = "1.0.0"',
        ),
        packet,
        False,
    )
    run(replace_once(catalog, 'revision       = "edf03abcc963d37703daa61192940394bdc553ca"', 'revision       = "develop"'), packet, False)
    run(replace_first(catalog, 'public_data    = true', 'public_data    = false'), packet, False)
    run(
        replace_first(
            catalog,
            'id                 = "did.syntax.valid-basic"\npacket             = "did.syntax.v1"\ncase               = "did.syntax.valid-basic"\ncapability         = "did.syntax"\nauthority          = "normative"',
            'id                 = "did.syntax.valid-basic"\npacket             = "did.syntax.v1"\ncase               = "did.syntax.valid-basic"\ncapability         = "did.syntax"\nauthority          = "copied"',
        ),
        packet,
        False,
    )
    run(replace_once(catalog, 'case               = "did.syntax.valid-basic"', 'case               = "did.syntax.missing"'), packet, False)
    packet_digest = hashlib.sha256(PACKET.read_bytes()).hexdigest()
    run(
        replace_once(
            catalog,
            f'sha256         = "{packet_digest}"',
            'sha256         = "' + "0" * 64 + '"',
        ),
        packet,
        False,
    )
    run(
        replace_first(
            catalog,
            'source_ids         = [ "sdk-ts.did-parser", "sdk-swift.did-parser", "sdk-kmp.did-parser" ]',
            'source_ids         = [ "sdk-rust.did-syntax" ]',
        ),
        packet,
        False,
    )
    run(
        replace_first(
            catalog,
            'language_selectors = [ "DIDParser.test.ts:should test valid DIDs", "DIDParserTests.swift:testValidDIDs", "DIDParserTest.kt:it_should_test_valid_DIDs" ]',
            'language_selectors = [ "DIDParser.test.ts:missing selector" ]',
        ),
        packet,
        False,
    )
    run(
        catalog.replace('supersedes         = [  ]', 'supersedes         = [ "did.syntax.valid-colon-segments" ]', 1),
        packet,
        False,
    )
    run_packet_mutation(catalog, replace_once(packet, '"schemaVersion": 1,', '"schemaVersion": 1,\n  "unknown": true,'))
    run_packet_mutation(
        catalog,
        replace_once(
            packet,
            '"generated": { "prefix": "did:example:", "repeat": "a", "count": 131060, "suffix": "" }',
            '"generated": { "prefix": "did:example:", "repeat": "a", "count": 262145, "suffix": "" }',
        ),
    )
    run_resolution_packet_mutation(
        catalog,
        replace_once(resolution_packet, '"resultKind": "success"', '"unknown": "success"'),
    )
    run_packet_mutation(
        catalog,
        replace_once(
            packet,
            '"methodSpecificId": "123456789abcdefghi"',
            '"methodSpecificId": "123456789abcdefghi",\n        "did": "did:example:123456789abcdefghi"',
        ),
    )
    run_packet_mutation(catalog, replace_once(packet, '"literal": "DID:example:123"', '"literal": ""'))
    run_packet_mutation(
        catalog,
        replace_once(
            packet,
            '"method": "example",\n          "methodSpecificId": "123456789abcdefghi"',
            '"method": null,\n          "methodSpecificId": "123456789abcdefghi"',
        ),
    )
    run(catalog, packet + "\n", False)

    with tempfile.TemporaryDirectory(prefix="cross-language-vector-symlink-") as temporary:
        root = Path(temporary)
        catalog_target = root / "docs/conformance/cross-language-vector-catalog.toml"
        packet_target = root / "docs/conformance/fixtures/did/v1/vectors.json"
        catalog_target.parent.mkdir(parents=True)
        catalog_target.write_text(catalog, encoding="utf-8")
        stage_packets(root, catalog)
        external = root / "outside.json"
        shutil.copy2(PACKET, external)
        packet_target.unlink()
        packet_target.symlink_to(external)
        result = subprocess.run([str(CHECKER), str(root)], capture_output=True, text=True, check=False)
        if result.returncode == 0:
            raise AssertionError("symlinked packets must fail validation")
        if "did.syntax.v1.path must not resolve through a symlink" not in result.stderr:
            raise AssertionError(f"missing symlink diagnostic: {result.stderr}")

    print("cross-language-vectors-tests: mutation suite passed")


if __name__ == "__main__":
    main()
