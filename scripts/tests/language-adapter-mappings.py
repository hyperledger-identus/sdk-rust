#!/usr/bin/env python3
"""Mutation tests for canonical language-adapter mapping validation."""

from __future__ import annotations

import re
import shutil
import subprocess
import tempfile
import tomllib
from collections.abc import Callable
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts/check-language-adapter-mappings.py"
REGISTRY = ROOT / "docs/architecture/language-adapter-mappings.toml"
RENDERED = ROOT / "docs/architecture/language-adapter-mappings.md"
CATALOG = ROOT / "docs/conformance/cross-language-vector-catalog.toml"


def replace_once(value: str, before: str, after: str) -> str:
    count = value.count(before)
    if count != 1:
        raise AssertionError(f"mutation source must occur exactly once: {before!r}; found {count}")
    return value.replace(before, after, 1)


def replace_first(value: str, before: str, after: str) -> str:
    if before not in value:
        raise AssertionError(f"mutation source is missing: {before!r}")
    return value.replace(before, after, 1)


def mapping_section(value: str, identifier: str) -> tuple[str, str, str]:
    marker = f'[[mappings]]\nid = "{identifier}"'
    start = value.index(marker)
    end = value.find("\n[[mappings]]", start + len(marker))
    if end == -1:
        end = len(value)
    return value[:start], value[start:end], value[end:]


def mutate_mapping(value: str, identifier: str, mutate: Callable[[str], str]) -> str:
    before, section, after = mapping_section(value, identifier)
    return before + mutate(section) + after


def remove_nested_tables(section: str, table: str) -> str:
    pattern = re.compile(
        rf"\n\[\[mappings\.{re.escape(table)}\]\]\n.*?(?=\n\[\[|\Z)", re.DOTALL
    )
    result, count = pattern.subn("", section)
    if count == 0:
        raise AssertionError(f"no nested {table} tables found")
    return result


def catalog_vector_section(value: str, identifier: str) -> tuple[str, str, str]:
    marker = f'[[vectors]]\nid                 = "{identifier}"'
    start = value.index(marker)
    end = value.find("\n[[vectors]]", start + len(marker))
    if end == -1:
        end = len(value)
    return value[:start], value[start:end], value[end:]


def materialize_bound_sources(root: Path, registry: str) -> None:
    try:
        document = tomllib.loads(registry)
    except tomllib.TOMLDecodeError:
        return
    mappings = document.get("mappings") if isinstance(document, dict) else None
    if not isinstance(mappings, list):
        return
    for mapping in mappings:
        if not isinstance(mapping, dict):
            continue
        raw = mapping.get("canonical_bound_path")
        if not isinstance(raw, str):
            continue
        relative = Path(raw)
        if relative.is_absolute() or ".." in relative.parts or "\\" in raw:
            continue
        source = ROOT / relative
        cursor = ROOT
        has_symlink = False
        for part in relative.parts:
            cursor /= part
            if cursor.is_symlink():
                has_symlink = True
                break
        if has_symlink:
            continue
        try:
            source.resolve(strict=True).relative_to(ROOT.resolve())
        except (OSError, ValueError):
            continue
        if not source.is_file() or source.is_symlink():
            continue
        target = root / relative
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, target)


def run(
    registry: str,
    rendered: str,
    should_pass: bool,
    *,
    catalog: str | None = None,
    check_rendered: bool = False,
    setup: Callable[[Path], None] | None = None,
) -> subprocess.CompletedProcess[str]:
    with tempfile.TemporaryDirectory(prefix="language-adapter-mappings-") as temporary:
        root = Path(temporary)
        registry_target = root / "docs/architecture/language-adapter-mappings.toml"
        rendered_target = root / "docs/architecture/language-adapter-mappings.md"
        catalog_target = root / "docs/conformance/cross-language-vector-catalog.toml"
        registry_target.parent.mkdir(parents=True)
        catalog_target.parent.mkdir(parents=True)
        registry_target.write_text(registry, encoding="utf-8")
        rendered_target.write_text(rendered, encoding="utf-8")
        catalog_target.write_text(
            CATALOG.read_text(encoding="utf-8") if catalog is None else catalog,
            encoding="utf-8",
        )
        materialize_bound_sources(root, registry)
        if setup is not None:
            setup(root)
        command = [str(CHECKER), str(root)]
        if not check_rendered:
            command.append("--render")
        result = subprocess.run(command, capture_output=True, text=True, check=False)
        if "Traceback (most recent call last)" in result.stderr:
            raise AssertionError(f"validator must return diagnostics, not a traceback:\n{result.stderr}")
        if (result.returncode == 0) != should_pass:
            raise AssertionError(
                f"expected pass={should_pass}, got {result.returncode}\n"
                f"stdout={result.stdout}\nstderr={result.stderr}"
            )
        return result


def write_source(root: Path, relative: str, content: str) -> None:
    target = root / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(content, encoding="utf-8")


def main() -> None:
    registry = REGISTRY.read_text(encoding="utf-8")
    rendered = RENDERED.read_text(encoding="utf-8")
    catalog = CATALOG.read_text(encoding="utf-8")

    run(registry, rendered, True)
    run(registry, rendered, True, check_rendered=True)

    # Closed schemas and malformed structures produce diagnostics, never tracebacks.
    run(
        replace_once(
            registry,
            'schema_version   = 1\nregistry_version = "1.0.0"',
            'schema_version   = 1\nunknown          = true\nregistry_version = "1.0.0"',
        ),
        rendered,
        False,
    )
    prefix = registry[: registry.index("[[mappings]]")]
    run(prefix + 'mappings = [ "invalid" ]\n', rendered, False)
    run("schema_version = [", rendered, False)
    run(replace_first(registry, "errors = [  ]", 'errors = [ "invalid" ]'), rendered, False)

    # Cargo identity, Rust API identity, and stable error codes remain distinct.
    run(
        replace_first(
            registry,
            'canonical_rust_path = "identus_did::Did"',
            'canonical_rust_path = "identus_did::identus_did::Did"',
        ),
        rendered,
        False,
    )
    run(
        replace_first(
            registry,
            'canonical_rust_path = "identus_did::Error::to_identus_error"',
            'canonical_rust_path = "did.invalid_did"',
        ),
        rendered,
        False,
    )

    # Exact-patch evidence and enclosing direction are non-weakenable.
    run(
        replace_first(registry, 'version_window = ">=8.1.4,<8.1.5"', 'version_window = ">=8.1.4,<10.0.0"'),
        rendered,
        False,
    )
    run(
        mutate_mapping(
            registry,
            "did-url.value.typescript.legacy-v1",
            lambda section: replace_first(
                section,
                'transform = "nested-did-mapping"\ndirection = "rust-to-language"',
                'transform = "nested-did-mapping"\ndirection = "both"',
            ),
        ),
        rendered,
        False,
    )
    run(
        mutate_mapping(
            registry,
            "did-url.value.typescript.legacy-v1",
            lambda section: replace_first(
                section,
                'transform = "nested-did-mapping"\ndirection = "rust-to-language"',
                'transform = "nested-did-mapping"\ndirection = "language-to-rust"',
            ),
        ),
        rendered,
        False,
    )

    # Losses are explicit, structured, and linked from unsupported cases.
    run(
        mutate_mapping(
            registry,
            "did.error.invalid-did.typescript.legacy-v1",
            lambda section: remove_nested_tables(section, "losses"),
        ),
        rendered,
        False,
    )
    run(
        replace_first(
            registry,
            'id          = "did-url.raw-query-round-trip"',
            'id          = "did-url.raw-query-renamed"',
        ),
        rendered,
        False,
    )
    run(
        replace_first(
            registry,
            'mitigation  = "Preserve the stable Rust error code beside the redacted legacy class."',
            'mitigation  = ""',
        ),
        rendered,
        False,
    )

    # Canonical bounds are literal, unique, bounded, and cannot escape via symlinks.
    run(replace_first(registry, "max_input_bytes = 2048", "max_input_bytes = 2049"), rendered, False)
    run(
        replace_first(registry, 'canonical_bound_path = "crates/did/src/did.rs"', 'canonical_bound_path = "../did.rs"'),
        rendered,
        False,
    )
    symlink_registry = replace_first(
        registry,
        'canonical_bound_path = "crates/did/src/did.rs"',
        'canonical_bound_path = "linked/did.rs"',
    )
    run(
        symlink_registry,
        rendered,
        False,
        setup=lambda root: (root / "linked").symlink_to(
            ROOT / "crates/did/src", target_is_directory=True
        ),
    )
    leaf_symlink_registry = replace_first(
        registry,
        'canonical_bound_path = "crates/did/src/did.rs"',
        'canonical_bound_path = "evidence/link.rs"',
    )

    def leaf_symlink(root: Path) -> None:
        target = root / "evidence/link.rs"
        target.parent.mkdir(parents=True)
        target.symlink_to(ROOT / "crates/did/src/did.rs")

    run(leaf_symlink_registry, rendered, False, setup=leaf_symlink)
    missing_registry = replace_first(
        registry,
        'canonical_bound_path = "crates/did/src/did.rs"',
        'canonical_bound_path = "evidence/missing.rs"',
    )
    run(missing_registry, rendered, False)
    duplicate_registry = replace_first(
        registry,
        'canonical_bound_path = "crates/did/src/did.rs"',
        'canonical_bound_path = "evidence/duplicate.rs"',
    )
    run(
        duplicate_registry,
        rendered,
        False,
        setup=lambda root: write_source(
            root,
            "evidence/duplicate.rs",
            "pub const MAX_DID_BYTES: usize = 2048;\npub const MAX_DID_BYTES: usize = 2048;\n",
        ),
    )
    computed_registry = replace_first(
        registry,
        'canonical_bound_path = "crates/did/src/did.rs"',
        'canonical_bound_path = "evidence/computed.rs"',
    )
    run(
        computed_registry,
        rendered,
        False,
        setup=lambda root: write_source(
            root, "evidence/computed.rs", "pub const MAX_DID_BYTES: usize = 2 * 1024;\n"
        ),
    )
    invalid_utf8_registry = replace_first(
        registry,
        'canonical_bound_path = "crates/did/src/did.rs"',
        'canonical_bound_path = "evidence/invalid-utf8.rs"',
    )

    def invalid_utf8(root: Path) -> None:
        target = root / "evidence/invalid-utf8.rs"
        target.parent.mkdir(parents=True)
        target.write_bytes(b"pub const MAX_DID_BYTES: usize = 2048;\n\xff")

    run(invalid_utf8_registry, rendered, False, setup=invalid_utf8)
    oversized_registry = replace_first(
        registry,
        'canonical_bound_path = "crates/did/src/did.rs"',
        'canonical_bound_path = "evidence/oversized.rs"',
    )

    def oversized(root: Path) -> None:
        target = root / "evidence/oversized.rs"
        target.parent.mkdir(parents=True)
        target.write_bytes(b"pub const MAX_DID_BYTES: usize = 2048;\n" + b" " * (2 * 1024 * 1024))

    run(oversized_registry, rendered, False, setup=oversized)

    # Vector references resolve exactly once with matching capability and target language.
    run(replace_first(registry, '"did.syntax.valid-basic"', '"did.syntax.unknown"'), rendered, False)
    ambiguous_catalog = replace_first(
        catalog,
        'id                 = "did.syntax.valid-colon-segments"',
        'id                 = "did.syntax.valid-basic"',
    )
    run(registry, rendered, False, catalog=ambiguous_catalog)
    run(registry, rendered, False, catalog="schema_version = [")
    wrong_capability_catalog = replace_first(
        catalog,
        'id                 = "did.syntax.valid-basic"\npacket             = "did.syntax.v1"\ncase               = "did.syntax.valid-basic"\ncapability         = "did.syntax"',
        'id                 = "did.syntax.valid-basic"\npacket             = "did.syntax.v1"\ncase               = "did.syntax.valid-basic"\ncapability         = "credential.core"',
    )
    run(registry, rendered, False, catalog=wrong_capability_catalog)
    before, vector, after = catalog_vector_section(catalog, "did.syntax.valid-basic")
    vector = replace_once(
        vector,
        'targets            = [ "rust", "typescript", "swift", "kotlin" ]',
        'targets            = [ "rust", "swift", "kotlin" ]',
    )
    run(registry, rendered, False, catalog=before + vector + after)

    # Shared catalog evidence and a second canonical source path are additive.
    first_mapping = registry.index("[[mappings]]")
    second_mapping = registry.index("[[mappings]]", first_mapping + 1)
    additional = registry[first_mapping:second_mapping]
    additional = replace_once(additional, 'id = "did.value.typescript.legacy-v1"', 'id = "did.value.swift.legacy-v1"')
    additional = replace_once(additional, 'language = "typescript"', 'language = "swift"')
    additional = replace_once(additional, 'language_target = "node-and-browser"', 'language_target = "ios"')
    additional = replace_once(additional, 'consumers = [ "sdk-ts" ]', 'consumers = [ "sdk-swift" ]')
    additional = replace_once(
        additional,
        'canonical_bound_path = "crates/did/src/did.rs"\ncanonical_bound_symbol = "MAX_DID_BYTES"',
        'canonical_bound_path = "crates/did/src/uri.rs"\ncanonical_bound_symbol = "MAX_URI_BYTES"',
    )
    run(registry.rstrip() + "\n\n" + additional, rendered, True)

    result = subprocess.run([str(CHECKER), str(ROOT), "--render"], capture_output=True, text=True, check=False)
    if result.returncode != 0 or result.stdout != rendered:
        raise AssertionError("render output must be deterministic and equal the checked-in Markdown")
    for required in ("Cargo package", "Rust API", "### Losses", "Rust code"):
        if required not in rendered:
            raise AssertionError(f"rendered review surface is missing {required!r}")
    if "identus-did::identus_did" in rendered:
        raise AssertionError("rendered Rust API must not duplicate the Cargo package")
    print("language-adapter-mappings-tests: mutation suite passed")


if __name__ == "__main__":
    main()
