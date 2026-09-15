#!/usr/bin/env python3
"""Mutation tests for the offline SDK input-resource boundary checker."""

from __future__ import annotations

import re
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-input-resource-boundaries.py"
SOURCE = ROOT / "docs/architecture/sdk-input-resource-boundaries.toml"


def invoke(content: str) -> subprocess.CompletedProcess[str]:
    with tempfile.TemporaryDirectory() as temporary:
        candidate = Path(temporary) / "inventory.toml"
        candidate.write_text(content, encoding="utf-8")
        return subprocess.run(
            [str(CHECKER), str(ROOT), "--inventory", str(candidate)],
            check=False,
            capture_output=True,
            text=True,
        )


def expect_failure(name: str, content: str, marker: str) -> None:
    result = invoke(content)
    output = result.stdout + result.stderr
    if result.returncode == 0 or marker not in output:
        raise AssertionError(
            f"{name} did not fail with {marker!r}: rc={result.returncode}\n{output}"
        )


def remove_package_blocks(content: str, package: str) -> str:
    blocks = content.split("[[boundaries]]")
    retained = [blocks[0]]
    for block in blocks[1:]:
        if re.search(
            rf'(?m)^package\s*=\s*"{re.escape(package)}"\s*$', block
        ) is None:
            retained.append("[[boundaries]]" + block)
    return "".join(retained)


def rewrite_first_block(
    content: str, marker: str, pattern: str, replacement: str
) -> str:
    blocks = content.split("[[boundaries]]")
    for index, block in enumerate(blocks[1:], start=1):
        if re.search(marker, block, flags=re.MULTILINE):
            rewritten, count = re.subn(
                pattern, replacement, block, count=1, flags=re.MULTILINE
            )
            if count != 1:
                raise AssertionError(f"mutation pattern missing from block: {pattern}")
            blocks[index] = rewritten
            return "[[boundaries]]".join(blocks)
    raise AssertionError(f"mutation marker missing from inventory: {marker}")


def main() -> None:
    canonical = SOURCE.read_text(encoding="utf-8")
    valid = invoke(canonical)
    if valid.returncode != 0:
        raise AssertionError(valid.stdout + valid.stderr)

    expect_failure(
        "missing package coverage",
        remove_package_blocks(canonical, "identus-wasm-did"),
        "implemented packages missing boundary coverage: identus-wasm-did",
    )
    expect_failure(
        "placeholder package",
        rewrite_first_block(
            canonical,
            r'^package\s*=\s*"identus-wasm-did"\s*$',
            r'^package\s*=.*$',
            'package = "identus-trust"',
        ),
        "references non-implemented package: identus-trust",
    )
    expect_failure(
        "unknown disposition",
        rewrite_first_block(
            canonical,
            r'^disposition\s*=\s*"sdk-enforced"\s*$',
            r'^disposition\s*=.*$',
            'disposition = "magic"',
        ),
        "unknown disposition: magic",
    )
    expect_failure(
        "enforced without limit",
        rewrite_first_block(
            canonical,
            r'^disposition\s*=\s*"sdk-enforced"\s*$',
            r'^limits\s*=.*$',
            "limits = []",
        ),
        "sdk-enforced boundary requires explicit limits",
    )
    expect_failure(
        "missing evidence",
        rewrite_first_block(
            canonical,
            r'^id\s*=\s*"core-fixed-values"\s*$',
            r'^evidence\s*=.*$',
            "evidence = []",
        ),
        "evidence must not be empty",
    )
    first_id = 'id = "core-fixed-values"'
    expect_failure(
        "duplicate identifier",
        rewrite_first_block(
            canonical,
            r'^id\s*=\s*"core-url"\s*$',
            r'^id\s*=.*$',
            first_id,
        ),
        "duplicate boundary id: core-fixed-values",
    )
    expect_failure(
        "unknown boundary field",
        rewrite_first_block(
            canonical,
            r'^id\s*=\s*"core-fixed-values"\s*$',
            r'^id\s*=.*$',
            first_id + '\nunknown = "value"',
        ),
        "must contain exactly",
    )
    expect_failure(
        "missing evidence path",
        canonical.replace("crates/core/src/time.rs", "crates/core/src/not-present.rs", 1),
        "evidence is missing or symlinked",
    )
    expect_failure(
        "oversized inventory",
        canonical + ("# padding\n" * 40_000),
        "exceeds 262144 bytes",
    )
    print("input-resource-boundaries mutations: passed")


if __name__ == "__main__":
    main()
