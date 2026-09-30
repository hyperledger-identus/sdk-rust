#!/usr/bin/env python3
"""Mutation tests for canonical language-adapter mapping validation."""

from __future__ import annotations

import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-language-adapter-mappings.py"
REGISTRY = ROOT / "docs/architecture/language-adapter-mappings.toml"
RENDERED = ROOT / "docs/architecture/language-adapter-mappings.md"


def replace_once(value: str, before: str, after: str) -> str:
    count = value.count(before)
    if count != 1:
        raise AssertionError(f"mutation source must occur exactly once: {before!r}; found {count}")
    return value.replace(before, after, 1)


def run(registry: str, rendered: str, should_pass: bool) -> None:
    with tempfile.TemporaryDirectory(prefix="language-adapter-mappings-") as temporary:
        root = Path(temporary)
        registry_target = root / "docs/architecture/language-adapter-mappings.toml"
        rendered_target = root / "docs/architecture/language-adapter-mappings.md"
        registry_target.parent.mkdir(parents=True)
        registry_target.write_text(registry, encoding="utf-8")
        rendered_target.write_text(rendered, encoding="utf-8")
        result = subprocess.run([str(CHECKER), str(root)], capture_output=True, text=True, check=False)
        if (result.returncode == 0) != should_pass:
            raise AssertionError(
                f"expected pass={should_pass}, got {result.returncode}\nstdout={result.stdout}\nstderr={result.stderr}"
            )


def main() -> None:
    registry = REGISTRY.read_text(encoding="utf-8")
    rendered = RENDERED.read_text(encoding="utf-8")
    run(registry, rendered, True)
    run(
        replace_once(
            registry,
            'schema_version   = 1\nregistry_version = "1.0.0"',
            'schema_version   = 1\nunknown          = true\nregistry_version = "1.0.0"',
        ),
        rendered,
        False,
    )
    run(replace_once(registry, 'canonical_owner  = "sdk-rust"', 'canonical_owner  = "sdk-ts"'), rendered, False)
    run(
        replace_once(
            registry,
            'canonical_symbol = "Did"\ncanonical_version_origin = "unreleased-workspace-0.0.0"\ncanonical_revision = "edf03abcc963d37703daa61192940394bdc553ca"',
            'canonical_symbol = "Did"\ncanonical_version_origin = "unreleased-workspace-0.0.0"\ncanonical_revision = "develop"',
        ),
        rendered,
        False,
    )
    run(
        replace_once(
            registry,
            'language_version = "8.1.4"\nlanguage_revision = "4bf86ebf69d5e96616a148e4c973f831f95fa38e"\nlanguage_path = "packages/shared/domain/src/models/DID.ts"',
            'language_version = "8.1"\nlanguage_revision = "4bf86ebf69d5e96616a148e4c973f831f95fa38e"\nlanguage_path = "packages/shared/domain/src/models/DID.ts"',
        ),
        rendered,
        False,
    )
    run(
        replace_once(
            registry,
            'id = "did-url.value.typescript.legacy-v1"\ncapability = "did.syntax"\nowner_issue = 505\nstate = "active"\nkind = "value"',
            'id = "did-url.value.typescript.legacy-v1"\ncapability = "did.syntax"\nowner_issue = 505\nstate = "active"\nkind = "error"',
        ),
        rendered,
        False,
    )
    run(
        replace_once(
            registry,
            'id = "did-url.value.typescript.legacy-v1"\ncapability = "did.syntax"',
            'id = "did.value.typescript.legacy-v1"\ncapability = "did.syntax"',
        ),
        rendered,
        False,
    )
    run(
        replace_once(
            registry,
            '"did-url.syntax.valid-components",',
            '"did.syntax.valid-basic",',
        ),
        rendered,
        False,
    )
    run(
        replace_once(
            registry,
            'rust_code               = "did.invalid_did"\nlanguage_class          = "CastorError.InvalidDIDString"\nlanguage_message_stable = false\npreserve_rust_code      = true',
            'rust_code               = "did.invalid_did"\nlanguage_class          = "CastorError.InvalidDIDString"\nlanguage_message_stable = false\npreserve_rust_code      = false',
        ),
        rendered,
        False,
    )
    run(
        replace_once(
            registry,
            'id           = "did-url.raw-query-round-trip"\nreason       = "A TypeScript Map cannot preserve raw query ordering or duplicate names."',
            'id           = "did-url.raw-query-round-trip"\nunknown      = true\nreason       = "A TypeScript Map cannot preserve raw query ordering or duplicate names."',
        ),
        rendered,
        False,
    )
    run(registry, rendered + "\n", False)

    result = subprocess.run([str(CHECKER), str(ROOT), "--render"], capture_output=True, text=True, check=False)
    if result.returncode != 0 or result.stdout != rendered:
        raise AssertionError("render output must be deterministic and equal the checked-in Markdown")
    print("language-adapter-mappings-tests: mutation suite passed")


if __name__ == "__main__":
    main()
