#!/usr/bin/env python3
"""Mutation tests for canonical language-adapter mapping validation."""

from __future__ import annotations

import hashlib
import os
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
os.environ["GIT_CONFIG_GLOBAL"] = os.devnull
os.environ["GIT_CONFIG_SYSTEM"] = os.devnull


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


def replace_error_tables_with_scalar(section: str) -> str:
    without_errors = remove_nested_tables(section, "errors")
    return replace_once(
        without_errors,
        "fields = [  ]\nunsupported = [  ]",
        "fields = [  ]\nerrors = 5\nunsupported = [  ]",
    )


def replace_error_tables_with_empty(section: str) -> str:
    without_errors = remove_nested_tables(section, "errors")
    return replace_once(
        without_errors,
        "fields = [  ]\nunsupported = [  ]",
        "fields = [  ]\nerrors = [  ]\nunsupported = [  ]",
    )


def replace_loss_tables_with_empty(section: str) -> str:
    without_losses = remove_nested_tables(section, "losses")
    return replace_once(
        without_losses,
        "fields = [  ]\nunsupported = [  ]",
        "fields = [  ]\nlosses = [  ]\nunsupported = [  ]",
    )


def replace_field_tables_with_empty(section: str) -> str:
    without_fields = remove_nested_tables(section, "fields")
    return replace_once(
        without_fields,
        "errors = [  ]\nlosses = [  ]",
        "fields = [  ]\nerrors = [  ]\nlosses = [  ]",
    )


def duplicate_first_nested_table(section: str, table: str) -> str:
    marker = f"[[mappings.{table}]]"
    start = section.index(marker)
    end = section.find("\n[[", start + len(marker))
    if end == -1:
        end = len(section)
    block = section[start:end].rstrip()
    return section.rstrip() + "\n\n" + block + "\n"


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
    sources = document.get("canonical_sources") if isinstance(document, dict) else None
    if not isinstance(sources, list):
        return
    for source_record in sources:
        if not isinstance(source_record, dict):
            continue
        raw = source_record.get("path")
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
    expect_diagnostic: str | None = None,
    argument_factory: Callable[[Path], list[str]] | None = None,
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
        if argument_factory is not None:
            command.extend(argument_factory(root))
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
        if expect_diagnostic is not None and expect_diagnostic not in result.stderr:
            raise AssertionError(
                f"expected diagnostic {expect_diagnostic!r}\n"
                f"stdout={result.stdout}\nstderr={result.stderr}"
            )
        return result


def write_source(root: Path, relative: str, content: str) -> None:
    target = root / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(content, encoding="utf-8")


def bind_source_evidence(
    registry: str, identifier: str, relative: str, payload: bytes, mode: str = "100644"
) -> str:
    marker = f'[[canonical_sources]]\nid                 = "{identifier}"'
    start = registry.index(marker)
    end = registry.find("\n[[canonical_sources]]", start + len(marker))
    mapping_start = registry.find("\n[[mappings]]", start + len(marker))
    if end == -1 or (mapping_start != -1 and mapping_start < end):
        end = mapping_start
    if end == -1:
        end = len(registry)
    section = registry[start:end]
    blob = b"blob " + str(len(payload)).encode("ascii") + b"\0" + payload
    replacements = {
        "path": relative,
        "file_mode": mode,
        "byte_length": str(len(payload)),
        "sha256": hashlib.sha256(payload).hexdigest(),
        "git_blob_oid": hashlib.sha1(blob).hexdigest(),
    }
    for field, value in replacements.items():
        quote = field != "byte_length"
        rendered = f'"{value}"' if quote else value
        section, count = re.subn(
            rf"(?m)^{field}\s*=\s*.*$", f"{field:<19}= {rendered}", section, count=1
        )
        if count != 1:
            raise AssertionError(f"source evidence field missing: {field}")
    return registry[:start] + section + registry[end:]


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
            'schema_version   = 2\nregistry_version = "2.0.0"',
            'schema_version   = 2\nunknown          = true\nregistry_version = "2.0.0"',
        ),
        rendered,
        False,
    )
    for mutation in (
        replace_once(
            registry,
            'id                 = "did.source.identus-did.v1"',
            'id                 = "invalid source id"',
        ),
        replace_once(registry, "owner_issue        = 512", "owner_issue        = 999"),
        replace_once(registry, 'state              = "active"', 'state              = "planned"'),
    ):
        run(mutation, rendered, False)
    prefix = registry[: registry.index("[[mappings]]")]
    run(prefix + 'mappings = [ "invalid" ]\n', rendered, False)
    run("schema_version = [", rendered, False)
    run(replace_first(registry, "errors = [  ]", 'errors = [ "invalid" ]'), rendered, False)
    run(
        mutate_mapping(
            registry,
            "did.error.invalid-did.typescript.legacy-v1",
            replace_error_tables_with_scalar,
        ),
        rendered,
        False,
        expect_diagnostic="fields/errors/losses/unsupported must be arrays",
    )
    run(
        mutate_mapping(
            registry,
            "did.value.typescript.legacy-v1",
            replace_field_tables_with_empty,
        ),
        rendered,
        False,
        expect_diagnostic="value mapping requires fields and forbids error mappings",
    )
    run(
        mutate_mapping(
            registry,
            "did.error.invalid-did.typescript.legacy-v1",
            replace_error_tables_with_empty,
        ),
        rendered,
        False,
        expect_diagnostic="error mapping requires errors and forbids field mappings",
    )
    for mutation in (
        replace_once(registry, 'canonical_owner  = "sdk-rust"', 'canonical_owner  = "sdk-ts"'),
        replace_first(
            registry,
            'canonical_revision = "5da04223c1c4e029cb368870877e56d699d40388"',
            'canonical_revision = "develop"',
        ),
        replace_first(registry, 'language_version = "8.1.4"', 'language_version = "8.1"'),
        replace_first(registry, 'language_license = "Apache-2.0"', 'language_license = "MIT"'),
        replace_first(registry, 'kind = "value"', 'kind = "error"'),
        replace_first(
            registry,
            'id = "did-url.value.typescript.legacy-v1"',
            'id = "did.value.typescript.legacy-v1"',
        ),
        replace_first(
            registry,
            'id = "did.value.typescript.legacy-v1"',
            'id = "did.value.typescript.renamed-v1"',
        ),
        replace_first(registry, "max_input_bytes = 2048", "max_input_bytes = 0"),
        replace_first(
            registry,
            'canonical_bound_symbol = "MAX_DID_BYTES"',
            'canonical_bound_symbol = "MAX_UNKNOWN_BYTES"',
        ),
        replace_first(registry, 'redaction = "caller-input"', 'redaction = "caller-output"'),
        replace_first(
            registry,
            'async_ownership = "not-applicable"',
            'async_ownership = "typescript"',
        ),
        replace_first(
            registry,
            'language_path = "packages/shared/domain/src/models/DID.ts"',
            'language_path = "../DID.ts"',
        ),
        replace_first(
            registry,
            'fallback = "No parser fallback; reject values refused by identus-did."',
            'fallback = ""',
        ),
        replace_first(
            registry,
            'removal_gate = "All supported SDK-TS consumers use the Rust-backed additive DID facade and its migration window has elapsed."',
            'removal_gate = ""',
        ),
        replace_first(
            registry,
            'rust_code               = "did.invalid_did"',
            'rust_code               = "legacy invalid did"',
        ),
        replace_first(registry, "preserve_rust_code      = true", "preserve_rust_code      = false"),
        replace_first(registry, "redact_input            = true", "redact_input            = false"),
    ):
        run(mutation, rendered, False)
    run(replace_once(registry, "schema_version   = 2", "schema_version   = true"), rendered, False)
    run(replace_once(registry, "owner_issue      = 505", "owner_issue      = 505.0"), rendered, False)

    # Canonical sources are closed, content-addressed, shared, and fully consumed.
    source_start = registry.index("[[canonical_sources]]")
    source_end = registry.index("[[mappings]]")
    source_block = registry[source_start:source_end]
    run(
        replace_once(
            registry,
            'file_mode          = "100644"',
            'unknown            = true\nfile_mode          = "100644"',
        ),
        rendered,
        False,
    )
    run(
        registry,
        rendered,
        False,
        setup=lambda root: (root / "crates/did/src/did.rs").write_bytes(
            (root / "crates/did/src/did.rs").read_bytes() + b"\n"
        ),
        expect_diagnostic="byte_length does not match exact source bytes",
    )
    run(
        registry,
        rendered,
        False,
        setup=lambda root: os.chmod(root / "crates/did/src/did.rs", 0o755),
        expect_diagnostic="file_mode does not match the source executable bit",
    )
    run(
        replace_once(registry, 'file_mode          = "100644"', 'file_mode          = "100755"'),
        rendered,
        False,
        expect_diagnostic="file_mode does not match the source executable bit",
    )
    run(replace_once(registry, "byte_length        = 12528", "byte_length        = 12527"), rendered, False)
    run(
        replace_once(
            registry,
            'sha256             = "8d85e03603c7e1d8d114681e6820b49266beed5110f5983d076e4fa57508ad79"',
            'sha256             = "0d85e03603c7e1d8d114681e6820b49266beed5110f5983d076e4fa57508ad79"',
        ),
        rendered,
        False,
    )
    run(replace_once(registry, 'git_object_format  = "sha1"', 'git_object_format  = "sha256"'), rendered, False)
    run(
        replace_once(
            registry,
            'git_blob_oid       = "6cbdc95ffdc04adf349ccb675688799ab6ef9c83"',
            'git_blob_oid       = "0cbdc95ffdc04adf349ccb675688799ab6ef9c83"',
        ),
        rendered,
        False,
    )
    run(registry[:source_end] + source_block + registry[source_end:], rendered, False)
    unused_source = replace_once(
        source_block,
        'id                 = "did.source.identus-did.v1"',
        'id                 = "did.source.identus-did.unused-v1"',
    )
    unused_registry = registry[:source_end] + unused_source + registry[source_end:]
    unused_payload = b"pub const UNUSED_BOUND: usize = 1;\n"
    unused_registry = bind_source_evidence(
        unused_registry,
        "did.source.identus-did.unused-v1",
        "crates/did/src/evidence/unused.rs",
        unused_payload,
    )
    run(
        unused_registry,
        rendered,
        False,
        setup=lambda root: write_source(
            root, "crates/did/src/evidence/unused.rs", unused_payload.decode()
        ),
        expect_diagnostic="canonical sources are unused",
    )
    run(
        replace_first(
            registry,
            'canonical_source_id = "did.source.identus-did.v1"',
            'canonical_source_id = "did.source.missing.v1"',
        ),
        rendered,
        False,
    )
    run(
        replace_once(
            registry,
            'canonical_crate    = "identus-did"',
            'canonical_crate    = "identus-core"',
        ),
        rendered,
        False,
    )
    run(
        mutate_mapping(
            registry,
            "did.value.typescript.legacy-v1",
            lambda section: duplicate_first_nested_table(section, "fields"),
        ),
        rendered,
        False,
    )
    run(
        mutate_mapping(
            registry,
            "did.error.invalid-did.typescript.legacy-v1",
            lambda section: duplicate_first_nested_table(section, "errors"),
        ),
        rendered,
        False,
    )
    run(replace_first(registry, 'state = "active"', 'state = "removed"'), rendered, False)
    for mutation in (
        replace_once(registry, "schema_version   = 2", "schema_version   = 1"),
        replace_once(registry, 'registry_version = "2.0.0"', 'registry_version = "v1"'),
        replace_once(registry, 'status           = "active"', 'status           = "planned"'),
        replace_once(registry, "owner_issue      = 505", "owner_issue      = 999"),
        replace_first(registry, 'capability = "did.syntax"', 'capability = "DID syntax"'),
        replace_first(registry, 'owner_issue = 505', 'owner_issue = 999'),
        replace_first(registry, 'state = "active"', 'state = "unknown"'),
        replace_first(registry, 'canonical_crate = "identus-did"', 'canonical_crate = "did"'),
        replace_first(
            registry,
            'canonical_rust_path = "identus_did::Did"',
            'canonical_rust_path = "other_crate::Did"',
        ),
        replace_first(registry, 'language = "typescript"', 'language = "javascript"'),
        replace_first(
            registry,
            'language_revision = "4bf86ebf69d5e96616a148e4c973f831f95fa38e"',
            'language_revision = "main"',
        ),
        replace_first(registry, 'direction = "bidirectional"', 'direction = "sideways"'),
        replace_first(
            registry,
            'compatibility_class = "deprecated"',
            'compatibility_class = "unknown"',
        ),
        replace_first(registry, 'fidelity = "lossless"', 'fidelity = "unknown"'),
        replace_first(
            registry,
            'deprecation_phase = "transitional"',
            'deprecation_phase = "unknown"',
        ),
        replace_first(registry, 'consumers = [ "sdk-ts" ]', 'consumers = [ "other-sdk" ]'),
        replace_first(registry, '"did.syntax.valid-basic"', '"invalid vector id"'),
        replace_first(
            registry,
            'rust_selectors = [ "identus_did::Did",',
            'rust_selectors = [ "identus did",',
        ),
        replace_first(registry, 'sensitive_fields = [  ]', 'sensitive_fields = [ "x", "x" ]'),
        replace_first(
            registry,
            'cancellation_ownership = "not-applicable"',
            'cancellation_ownership = "typescript"',
        ),
        replace_first(
            registry,
            'rust      = "serialized"\nlanguage  = "uuid"',
            'rust      = "serialized"\nunknown   = true\nlanguage  = "uuid"',
        ),
        replace_first(registry, 'transform = "identity"', 'transform = ""'),
        replace_first(registry, 'direction = "both"', 'direction = "sideways"'),
        replace_first(registry, "required  = true", 'required  = "true"'),
        replace_first(
            registry,
            'rust_code               = "did.invalid_did"\nlanguage_class',
            'rust_code               = "did.invalid_did"\nunknown                 = true\nlanguage_class',
        ),
        replace_first(
            registry,
            'language_message_stable = false',
            'language_message_stable = true',
        ),
        replace_first(
            registry,
            'id          = "did-url.raw-query-round-trip"\ndistinction',
            'id          = "did-url.raw-query-round-trip"\nunknown     = true\ndistinction',
        ),
        replace_first(
            registry,
            'id          = "did-url.empty-query-presence"',
            'id          = "did-url.raw-query-round-trip"',
        ),
        replace_first(
            registry,
            'id           = "did-url.raw-query-round-trip"\nreason',
            'id           = "did-url.raw-query-round-trip"\nunknown      = true\nreason',
        ),
        replace_first(
            registry,
            'id           = "did-url.empty-query-presence"',
            'id           = "did-url.raw-query-round-trip"',
        ),
        replace_first(
            registry,
            'stable_error = "adapter.unsupported_lossy_mapping"',
            'stable_error = "invalid error"',
        ),
        replace_first(
            registry,
            'deprecation_phase = "transitional"',
            'deprecation_phase = "removed"',
        ),
    ):
        run(mutation, rendered, False)

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
        replace_first(registry, 'version_window = ">=8.1.4,<8.1.5"', 'version_window = ">=8.1.4,<8.01.5"'),
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
            replace_loss_tables_with_empty,
        ),
        rendered,
        False,
        expect_diagnostic="lossy mapping requires at least one loss record",
    )
    run(
        mutate_mapping(
            registry,
            "did-url.value.typescript.legacy-v1",
            lambda section: replace_first(
                section, 'fidelity = "lossy"', 'fidelity = "lossless"'
            ),
        ),
        rendered,
        False,
        expect_diagnostic="lossless mapping cannot declare losses or unsupported cases",
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
        replace_first(registry, 'path               = "crates/did/src/did.rs"', 'path               = "../did.rs"'),
        rendered,
        False,
    )
    symlink_registry = replace_first(
        registry,
        'path               = "crates/did/src/did.rs"',
        'path               = "crates/did/src/linked/did.rs"',
    )
    def directory_symlink(root: Path) -> None:
        target = root / "crates/did/src/linked"
        target.parent.mkdir(parents=True)
        target.symlink_to(ROOT / "crates/did/src", target_is_directory=True)

    run(symlink_registry, rendered, False, setup=directory_symlink)
    leaf_symlink_registry = replace_first(
        registry,
        'path               = "crates/did/src/did.rs"',
        'path               = "crates/did/src/evidence/link.rs"',
    )

    def leaf_symlink(root: Path) -> None:
        target = root / "crates/did/src/evidence/link.rs"
        target.parent.mkdir(parents=True)
        target.symlink_to(ROOT / "crates/did/src/did.rs")

    run(leaf_symlink_registry, rendered, False, setup=leaf_symlink)
    missing_registry = replace_first(
        registry,
        'path               = "crates/did/src/did.rs"',
        'path               = "crates/did/src/evidence/missing.rs"',
    )
    run(missing_registry, rendered, False)
    duplicate_payload = (
        b"pub const MAX_DID_BYTES: usize = 2048;\n"
        b"pub const MAX_DID_BYTES: usize = 2048;\n"
    )
    duplicate_registry = bind_source_evidence(
        registry,
        "did.source.identus-did.v1",
        "crates/did/src/evidence/duplicate.rs",
        duplicate_payload,
    )
    run(
        duplicate_registry,
        rendered,
        False,
        setup=lambda root: write_source(
            root,
            "crates/did/src/evidence/duplicate.rs",
            duplicate_payload.decode(),
        ),
        expect_diagnostic="must resolve exactly once to a literal public usize constant",
    )
    computed_payload = b"pub const MAX_DID_BYTES: usize = 2 * 1024;\n"
    computed_registry = bind_source_evidence(
        registry,
        "did.source.identus-did.v1",
        "crates/did/src/evidence/computed.rs",
        computed_payload,
    )
    run(
        computed_registry,
        rendered,
        False,
        setup=lambda root: write_source(
            root,
            "crates/did/src/evidence/computed.rs",
            computed_payload.decode(),
        ),
        expect_diagnostic="must resolve exactly once to a literal public usize constant",
    )
    invalid_utf8_registry = replace_first(
        registry,
        'path               = "crates/did/src/did.rs"',
        'path               = "crates/did/src/evidence/invalid-utf8.rs"',
    )

    def invalid_utf8(root: Path) -> None:
        target = root / "crates/did/src/evidence/invalid-utf8.rs"
        target.parent.mkdir(parents=True)
        target.write_bytes(b"pub const MAX_DID_BYTES: usize = 2048;\n\xff")

    invalid_utf8_result = run(
        invalid_utf8_registry, rendered, False, setup=invalid_utf8
    )
    if "not valid UTF-8" not in invalid_utf8_result.stderr:
        raise AssertionError("invalid UTF-8 must be classified as unreadable evidence")
    oversized_registry = replace_first(
        registry,
        'path               = "crates/did/src/did.rs"',
        'path               = "crates/did/src/evidence/oversized.rs"',
    )

    def oversized(root: Path) -> None:
        target = root / "crates/did/src/evidence/oversized.rs"
        target.parent.mkdir(parents=True)
        target.write_bytes(b"pub const MAX_DID_BYTES: usize = 2048;\n" + b" " * (2 * 1024 * 1024))

    run(oversized_registry, rendered, False, setup=oversized)
    wrong_crate_registry = replace_first(
        registry,
        'path               = "crates/did/src/did.rs"',
        'path               = "crates/core/src/url.rs"',
    )
    wrong_crate_registry = replace_first(
        wrong_crate_registry, "max_input_bytes = 2048", "max_input_bytes = 8192"
    )
    run(wrong_crate_registry, rendered, False)
    same_crate_registry = replace_first(
        registry,
        'canonical_bound_symbol = "MAX_DID_BYTES"',
        'canonical_bound_symbol = "MAX_DID_URL_BYTES"',
    )
    same_crate_registry = replace_first(
        same_crate_registry, "max_input_bytes = 2048", "max_input_bytes = 4096"
    )
    run(same_crate_registry, rendered, False)

    # Vector references resolve exactly once with matching capability and target language.
    run(replace_first(registry, '"did.syntax.valid-basic"', '"did.syntax.unknown"'), rendered, False)
    ambiguous_catalog = replace_first(
        catalog,
        'id                 = "did.syntax.valid-colon-segments"',
        'id                 = "did.syntax.valid-basic"',
    )
    run(registry, rendered, False, catalog=ambiguous_catalog)
    run(registry, rendered, False, catalog="schema_version = [")

    def symlink_catalog(root: Path) -> None:
        target = root / "docs/conformance/cross-language-vector-catalog.toml"
        target.unlink()
        target.symlink_to(CATALOG)

    run(registry, rendered, False, setup=symlink_catalog)
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
    run(
        replace_first(
            registry,
            '"did.syntax.valid-basic",',
            '"did.syntax.valid-basic",\n  "did.syntax.over-max-bytes",',
        ),
        rendered,
        False,
    )
    run(
        mutate_mapping(
            registry,
            "did.error.invalid-did.typescript.legacy-v1",
            lambda section: replace_first(
                section,
                '"did.syntax.invalid-prefix",',
                '"did.syntax.invalid-prefix",\n  "did.syntax.valid-basic",',
            ),
        ),
        rendered,
        False,
    )

    # A second mapping may bind a distinct, fully evidenced canonical source.
    first_mapping = registry.index("[[mappings]]")
    second_mapping = registry.index("[[mappings]]", first_mapping + 1)
    additional = registry[first_mapping:second_mapping]
    additional = replace_once(
        additional,
        'id = "did.value.typescript.legacy-v1"',
        'id = "uri.value.swift.legacy-v1"',
    )
    additional = replace_once(
        additional,
        'canonical_rust_path = "identus_did::Did"',
        'canonical_rust_path = "identus_did::Uri"',
    )
    additional = replace_once(
        additional,
        'canonical_source_id = "did.source.identus-did.v1"',
        'canonical_source_id = "uri.source.identus-did.v1"',
    )
    additional = replace_once(
        additional, 'canonical_bound_symbol = "MAX_DID_BYTES"', 'canonical_bound_symbol = "MAX_URI_BYTES"'
    )
    additional = replace_once(additional, 'language = "typescript"', 'language = "swift"')
    additional = replace_once(
        additional, 'language_target = "node-and-browser"', 'language_target = "ios"'
    )
    additional = replace_once(additional, 'consumers = [ "sdk-ts" ]', 'consumers = [ "sdk-swift" ]')
    additional, vector_count = re.subn(
        r'vector_ids = \[\n.*?\n\]',
        'vector_ids = [ "uri.syntax.max-bytes" ]',
        additional,
        count=1,
        flags=re.DOTALL,
    )
    if vector_count != 1:
        raise AssertionError("additive mapping vector fixture must be replaced once")
    additional = replace_once(
        additional,
        'rust_selectors = [ "identus_did::Did",',
        'rust_selectors = [ "identus_did::Uri",',
    )
    second_source = source_block.replace(
        'id                 = "did.source.identus-did.v1"',
        'id                 = "uri.source.identus-did.v1"',
        1,
    )
    with_second_source = registry[:source_end] + second_source + registry[source_end:]
    uri_payload = (ROOT / "crates/did/src/uri.rs").read_bytes()
    with_second_source = bind_source_evidence(
        with_second_source,
        "uri.source.identus-did.v1",
        "crates/did/src/uri.rs",
        uri_payload,
    )
    synthetic_vector = """

[[vectors]]
id             = "uri.syntax.max-bytes"
capability     = "did.syntax"
targets        = [ "rust", "swift" ]
expected       = "success"
source_locator = "MAX_URI_BYTES; additive second-source fixture"
"""
    run(
        with_second_source.rstrip() + "\n\n" + additional,
        rendered,
        True,
        catalog=catalog.rstrip() + synthetic_vector,
    )

    # A source move must rotate to a revision where the new path resolves to the blob.
    moved_registry = replace_once(
        registry,
        'path               = "crates/did/src/did.rs"',
        'path               = "crates/did/src/evidence/moved.rs"',
    )

    def moved_source(root: Path) -> None:
        original = root / "crates/did/src/did.rs"
        original.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ROOT / "crates/did/src/did.rs", original)
        subprocess.run(["git", "init", "-q", "-b", "master"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.name", "fixture"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.email", "fixture@example.invalid"], cwd=root, check=True)
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "fixture"], cwd=root, check=True)
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=root, capture_output=True, text=True, check=True
        ).stdout.strip()
        target = root / "crates/did/src/evidence/moved.rs"
        target.parent.mkdir(parents=True)
        shutil.copy2(ROOT / "crates/did/src/did.rs", target)
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "move source"], cwd=root, check=True)
        registry_path = root / "docs/architecture/language-adapter-mappings.toml"
        registry_path.write_text(
            replace_once(
                registry_path.read_text(encoding="utf-8"),
                'canonical_revision = "5da04223c1c4e029cb368870877e56d699d40388"',
                f'canonical_revision = "{revision}"',
            ),
            encoding="utf-8",
        )

    run(
        moved_registry,
        rendered,
        False,
        setup=moved_source,
        expect_diagnostic="canonical_revision path must resolve to git_blob_oid",
    )
    pending_moved_registry = replace_once(
        moved_registry, 'state              = "active"', 'state              = "pending-revision"'
    )
    first_commit_args = lambda root: [
        "--pr-base",
        subprocess.run(
            ["git", "rev-list", "--max-parents=0", "HEAD"],
            cwd=root,
            capture_output=True,
            text=True,
            check=True,
        ).stdout.strip(),
    ]
    run(
        moved_registry,
        rendered,
        False,
        setup=moved_source,
        argument_factory=first_commit_args,
        expect_diagnostic="changed from PR base and must use pending-revision",
    )
    run(
        pending_moved_registry,
        rendered,
        True,
        setup=moved_source,
        argument_factory=first_commit_args,
    )
    run(
        pending_moved_registry,
        rendered,
        False,
        setup=moved_source,
        argument_factory=lambda root: [
            "--pr-base",
            subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=root,
                capture_output=True,
                text=True,
                check=True,
            ).stdout.strip(),
        ],
        expect_diagnostic="pending revision must equal the PR base",
    )
    run(
        pending_moved_registry,
        rendered,
        True,
        setup=moved_source,
        argument_factory=lambda _root: ["--develop-head"],
    )

    def pending_head_mismatch(root: Path) -> None:
        moved_source(root)
        target = root / "crates/did/src/evidence/moved.rs"
        payload = target.read_bytes() + b"\n"
        target.write_bytes(payload)
        registry_path = root / "docs/architecture/language-adapter-mappings.toml"
        registry_path.write_text(
            bind_source_evidence(
                registry_path.read_text(encoding="utf-8"),
                "did.source.identus-did.v1",
                "crates/did/src/evidence/moved.rs",
                payload,
            ),
            encoding="utf-8",
        )

    run(
        pending_moved_registry,
        rendered,
        False,
        setup=pending_head_mismatch,
        expect_diagnostic="pending revision must match the HEAD path blob",
    )

    def expired_pending(root: Path) -> None:
        moved_source(root)
        marker = root / "after-squash.txt"
        marker.write_text("next change\n", encoding="utf-8")
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "later change"], cwd=root, check=True)

    run(
        pending_moved_registry,
        rendered,
        False,
        setup=expired_pending,
        argument_factory=lambda _root: ["--develop-head"],
        expect_diagnostic="pending revision expired",
    )
    def unchanged_pending(root: Path) -> None:
        subprocess.run(["git", "init", "-q", "-b", "master"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.name", "fixture"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.email", "fixture@example.invalid"], cwd=root, check=True)
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "fixture"], cwd=root, check=True)
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=root, capture_output=True, text=True, check=True
        ).stdout.strip()
        registry_path = root / "docs/architecture/language-adapter-mappings.toml"
        registry_path.write_text(
            replace_once(
                registry_path.read_text(encoding="utf-8"),
                'canonical_revision = "5da04223c1c4e029cb368870877e56d699d40388"',
                f'canonical_revision = "{revision}"',
            ),
            encoding="utf-8",
        )

    run(
        replace_once(
            registry, 'state              = "active"', 'state              = "pending-revision"'
        ),
        rendered,
        False,
        setup=unchanged_pending,
        expect_diagnostic="pending revision must represent changed source bytes",
    )

    def unrelated_git_history(root: Path) -> None:
        subprocess.run(["git", "init", "-q", "-b", "master"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.name", "fixture"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.email", "fixture@example.invalid"], cwd=root, check=True)
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "fixture"], cwd=root, check=True)

    run(
        registry,
        rendered,
        False,
        setup=unrelated_git_history,
        argument_factory=lambda root: [
            "--pr-base",
            subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=root,
                capture_output=True,
                text=True,
                check=True,
            ).stdout.strip(),
        ],
        expect_diagnostic="canonical_revision must name an ancestor commit",
    )

    def feature_only_active_revision(root: Path) -> None:
        subprocess.run(["git", "init", "-q", "-b", "master"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.name", "fixture"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.email", "fixture@example.invalid"], cwd=root, check=True)
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "base"], cwd=root, check=True)
        marker = root / "feature-only.txt"
        marker.write_text("feature\n", encoding="utf-8")
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "feature"], cwd=root, check=True)
        revision = subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=root, capture_output=True, text=True, check=True
        ).stdout.strip()
        registry_path = root / "docs/architecture/language-adapter-mappings.toml"
        registry_path.write_text(
            replace_once(
                registry_path.read_text(encoding="utf-8"),
                'canonical_revision = "5da04223c1c4e029cb368870877e56d699d40388"',
                f'canonical_revision = "{revision}"',
            ),
            encoding="utf-8",
        )

    run(
        registry,
        rendered,
        False,
        setup=feature_only_active_revision,
        argument_factory=first_commit_args,
        expect_diagnostic="active revision must be reachable from the PR base",
    )

    def non_ancestor_revision(root: Path) -> None:
        subprocess.run(["git", "init", "-q", "-b", "master"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.name", "fixture"], cwd=root, check=True)
        subprocess.run(["git", "config", "user.email", "fixture@example.invalid"], cwd=root, check=True)
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "base"], cwd=root, check=True)
        subprocess.run(["git", "checkout", "-qb", "side"], cwd=root, check=True)
        marker = root / "side.txt"
        marker.write_text("side\n", encoding="utf-8")
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "side"], cwd=root, check=True)
        side = subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=root, capture_output=True, text=True, check=True
        ).stdout.strip()
        subprocess.run(["git", "checkout", "-q", "master"], cwd=root, check=True)
        marker = root / "main.txt"
        marker.write_text("main\n", encoding="utf-8")
        subprocess.run(["git", "add", "."], cwd=root, check=True)
        subprocess.run(["git", "commit", "-qm", "main"], cwd=root, check=True)
        registry_path = root / "docs/architecture/language-adapter-mappings.toml"
        registry_path.write_text(
            replace_once(
                registry_path.read_text(encoding="utf-8"),
                'canonical_revision = "5da04223c1c4e029cb368870877e56d699d40388"',
                f'canonical_revision = "{side}"',
            ),
            encoding="utf-8",
        )

    run(
        registry,
        rendered,
        False,
        setup=non_ancestor_revision,
        expect_diagnostic="canonical_revision must name an ancestor commit",
    )

    result = subprocess.run([str(CHECKER), str(ROOT), "--render"], capture_output=True, text=True, check=False)
    if result.returncode != 0 or result.stdout != rendered:
        raise AssertionError("render output must be deterministic and equal the checked-in Markdown")
    for required in ("Cargo package", "Rust API", "### Losses", "Rust code"):
        if required not in rendered:
            raise AssertionError(f"rendered review surface is missing {required!r}")
    if "identus-did::identus_did" in rendered:
        raise AssertionError("rendered Rust API must not duplicate the Cargo package")
    run(registry, rendered + "\n", False, check_rendered=True)
    run(
        registry,
        rendered.replace(
            "Validated immutable DID preserving the exact serialized value",
            "Validated immutable DID with changed review semantics",
            1,
        ),
        False,
        check_rendered=True,
    )
    print("language-adapter-mappings-tests: mutation suite passed")


if __name__ == "__main__":
    main()
