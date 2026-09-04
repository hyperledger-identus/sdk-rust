#!/usr/bin/env python3
"""Validate the machine-readable SDK compatibility policy offline."""

from __future__ import annotations

import json
import re
import sys
from functools import cache
from pathlib import Path
from typing import Any

import tomllib

POLICY_PATH = Path("docs/architecture/sdk-support-policy.toml")
GATE_MANIFEST_PATH = Path("nix/checks/gates.toml")
GATE_FIELDS = {
    "name",
    "operation",
    "toolchain",
    "source",
    "artifacts",
    "locked",
    "workspace",
    "packages",
    "exclude_packages",
    "lib",
    "all_targets",
    "no_default_features",
    "all_features",
    "features",
    "target",
    "extra_args",
}
ALLOWED_OPERATIONS = {
    "cargoAudit",
    "cargoBuild",
    "cargoClippy",
    "cargoDeny",
    "cargoDoc",
    "cargoFmt",
    "cargoNextest",
}
OPERATIONS_WITH_CARGO_SELECTION = {
    "cargoBuild",
    "cargoClippy",
    "cargoDoc",
    "cargoNextest",
}
OPERATION_EXTRA_ARGS = {
    "cargoAudit": [],
    "cargoBuild": [],
    "cargoClippy": ["--", "-D", "warnings"],
    "cargoDeny": [],
    "cargoDoc": ["--no-deps"],
    "cargoFmt": [],
    "cargoNextest": ["--no-fail-fast", "--no-tests=pass"],
}
REQUIRED_HOSTS = {
    "x86_64-linux": "x86_64-unknown-linux-gnu",
    "aarch64-darwin": "aarch64-apple-darwin",
}
REQUIRED_TARGETS = {
    "wasm32-unknown-unknown": "compile-checked",
    "aarch64-linux-android": "compile-checked",
    "aarch64-apple-ios": "compile-checked",
    "x86_64-pc-windows-msvc": "planned",
    "wasm32-wasip1": "planned",
}
REQUIRED_COMPILE_PACKAGES = {
    "identus-core",
    "identus-crypto",
    "identus-did",
    "identus-adapters-entropy",
}
REQUIRED_FEATURE_SURFACES = {
    "workspace-default",
    "crypto-minimal",
    "crypto-kmp-compat",
    "entropy-minimal",
    "entropy-deterministic",
    "entropy-system-random",
    "entropy-all",
}
ALLOWED_TIERS = {"host-tested", "compile-checked", "planned", "not-supported"}
NIX_URI_PREFIX = re.compile(r"[A-Za-z][A-Za-z0-9+.-]*:[A-Za-z0-9%/?@&=+$,_.!~*'-]")
NIX_UNPREFIXED_PATH_PREFIX = re.compile(r"[A-Za-z0-9._+?-]+/")


def load_toml(path: Path, failures: list[str]) -> dict[str, Any]:
    try:
        with path.open("rb") as source:
            value = tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        failures.append(f"cannot read {path}: {error}")
        return {}
    if not isinstance(value, dict):
        failures.append(f"{path} is not a TOML table")
        return {}
    return value


def require_table(
    data: dict[str, Any], name: str, failures: list[str]
) -> dict[str, Any]:
    value = data.get(name)
    if not isinstance(value, dict):
        failures.append(f"policy is missing [{name}]")
        return {}
    return value


def require_nonempty_string(
    table: dict[str, Any], field: str, context: str, failures: list[str]
) -> str:
    value = table.get(field)
    if not isinstance(value, str) or not value.strip():
        failures.append(f"{context} is missing non-empty {field}")
        return ""
    return value


def workspace_packages(
    root: Path, cargo: dict[str, Any], failures: list[str]
) -> tuple[set[str], dict[str, Path]]:
    dependencies = cargo.get("workspace", {}).get("dependencies", {})
    if not isinstance(dependencies, dict):
        failures.append("Cargo.toml is missing [workspace.dependencies]")
        return set(), {}

    packages: set[str] = set()
    manifests: dict[str, Path] = {}
    for alias, value in dependencies.items():
        if not isinstance(value, dict) or "path" not in value:
            continue
        manifest_path = root / str(value["path"]) / "Cargo.toml"
        manifest = load_toml(manifest_path, failures)
        package_name = manifest.get("package", {}).get("name")
        if not isinstance(package_name, str):
            failures.append(f"{manifest_path} has no package.name")
            continue
        packages.add(package_name)
        manifests[package_name] = manifest_path
        if alias != package_name:
            failures.append(
                f"workspace dependency alias {alias} does not match package {package_name}"
            )
    return packages, manifests


def nix_block_comment_end(text: str, index: int) -> int:
    """Return the exclusive end of a possibly nested Nix block comment."""
    depth = 1
    index += 2
    while index < len(text) and depth:
        if text.startswith("/*", index):
            depth += 1
            index += 2
        elif text.startswith("*/", index):
            depth -= 1
            index += 2
        else:
            index += 1
    return index


@cache
def nix_path_or_uri_end(text: str, index: int) -> int | None:
    """Return the end of a path or URI token beginning at index."""
    character = text[index]
    plausible_unprefixed_start = character.isalnum() or character in "._+?-"
    if character not in "./~<" and not plausible_unprefixed_start:
        return None
    uri_boundary = character.isalpha() and (
        index == 0 or not (text[index - 1].isalnum() or text[index - 1] in "+.-")
    )
    token_boundary = index == 0 or not (
        text[index - 1].isalnum() or text[index - 1] in "_'.+-"
    )
    unprefixed_path = (
        plausible_unprefixed_start
        and token_boundary
        and NIX_UNPREFIXED_PATH_PREFIX.match(text, index) is not None
    )
    if character not in "./~<" and not uri_boundary and not unprefixed_path:
        return None
    relative_path = text.startswith(("./", "../", "~/"), index)
    absolute_path = (
        text.startswith("/", index)
        and not text.startswith(("/*", "//"), index)
        and index + 1 < len(text)
        and not text[index + 1].isspace()
        and (index == 0 or text[index - 1].isspace() or text[index - 1] in "=([{;,")
    )
    uri = uri_boundary and NIX_URI_PREFIX.match(text, index) is not None
    if text.startswith("<", index):
        end = text.find(">", index + 1)
        if end != -1 and not any(character.isspace() for character in text[index:end]):
            return end + 1
    if not (relative_path or absolute_path or uri or unprefixed_path):
        return None

    cursor = index + 1
    while cursor < len(text):
        if text.startswith("${", cursor):
            cursor = nix_interpolation_end(text, cursor + 2)
            continue
        if text[cursor].isspace() or text[cursor] in "#;,()[]{}":
            break
        cursor += 1
    return cursor


def nix_interpolation_end(text: str, index: int) -> int:
    """Return the exclusive end of a Nix interpolation after its opening `${`."""
    depth = 1
    while index < len(text) and depth:
        path_end = nix_path_or_uri_end(text, index)
        if path_end is not None:
            index = path_end
            continue
        string_end = nix_string_end(text, index)
        if string_end is not None:
            index = string_end
            continue
        if text[index] == "#":
            newline = text.find("\n", index)
            index = len(text) if newline == -1 else newline
            continue
        if text.startswith("/*", index):
            index = nix_block_comment_end(text, index)
            continue
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
        index += 1
    return index


def nix_string_end(text: str, index: int) -> int | None:
    """Return a Nix string end, including nested interpolation expressions."""
    if text[index] == '"':
        cursor = index + 1
        while cursor < len(text):
            if text[cursor] == "\\":
                cursor += 2
                continue
            if text.startswith("${", cursor):
                cursor = nix_interpolation_end(text, cursor + 2)
                continue
            if text[cursor] == '"':
                return cursor + 1
            cursor += 1
        return len(text)
    if text.startswith("''", index):
        if index and (text[index - 1].isalnum() or text[index - 1] in "_-'"):
            return None
        cursor = index + 2
        while cursor < len(text):
            if text.startswith("''", cursor):
                escaped = cursor + 2 < len(text) and text[cursor + 2] in "$'\\"
                if escaped:
                    cursor += 4 if text[cursor + 2] == "\\" else 3
                    continue
                return cursor + 2
            if text.startswith("${", cursor):
                cursor = nix_interpolation_end(text, cursor + 2)
                continue
            cursor += 1
        return len(text)
    return None


@cache
def nix_string_mask(text: str) -> str:
    """Mask Nix strings while preserving offsets for bounded source scanning."""
    masked = list(text)
    index = 0
    while index < len(text):
        path_end = nix_path_or_uri_end(text, index)
        if path_end is not None:
            index = path_end
            continue
        end = nix_string_end(text, index)
        if end is not None:
            masked[index:end] = " " * (end - index)
            index = end
        else:
            index += 1
    return "".join(masked)


@cache
def nix_without_comments(text: str) -> str:
    """Mask comments outside Nix strings while preserving source shape."""
    without_comments = list(text)
    index = 0
    while index < len(text):
        path_end = nix_path_or_uri_end(text, index)
        if path_end is not None:
            index = path_end
            continue
        string_end = nix_string_end(text, index)
        if string_end is not None:
            index = string_end
            continue
        if text[index] == "#":
            end = text.find("\n", index)
            end = len(text) if end == -1 else end
            without_comments[index:end] = " " * (end - index)
            index = end
            continue
        if text.startswith("/*", index):
            start = index
            index = nix_block_comment_end(text, index)
            for position in range(start, index):
                if without_comments[position] != "\n":
                    without_comments[position] = " "
            continue
        index += 1
    return "".join(without_comments)


@cache
def nix_static_string_value(literal: str) -> str | None:
    """Return the value of a simple static Nix string expression."""
    if re.fullmatch(r'"[A-Za-z_][A-Za-z0-9_\'-]*"', literal):
        return literal[1:-1]
    if re.fullmatch(r"''[A-Za-z_][A-Za-z0-9_'-]*''", literal):
        return literal[2:-2]
    payload = None
    indented = False
    if literal.startswith('"') and literal.endswith('"'):
        payload = literal[1:-1]
    elif literal.startswith("''") and literal.endswith("''"):
        payload = literal[2:-2]
        indented = True
    if payload is None:
        return None
    if indented:
        normalized = payload.strip()
        if re.fullmatch(r"[A-Za-z_][A-Za-z0-9_'-]*", normalized):
            return normalized
        payload = normalized
    interpolated = re.fullmatch(
        r"^\$\{\s*(?:\"(?P<double>[A-Za-z_][A-Za-z0-9_'-]*)\"|''(?P<indented>[A-Za-z_][A-Za-z0-9_'-]*)'')\s*\}$",
        payload,
    )
    if interpolated is None:
        return None
    return interpolated.group("double") or interpolated.group("indented")


@cache
def nix_attribute_assignment_ends(text: str, name: str) -> tuple[int, ...]:
    """Return offsets after static or directly dynamic attribute assignments."""
    source = nix_without_comments(text)
    masked = nix_string_mask(source)
    assignments = {
        match.end()
        for match in re.finditer(rf"(?<![A-Za-z0-9_'])\b{re.escape(name)}\s*=", masked)
    }

    index = 0
    while index < len(source):
        path_end = nix_path_or_uri_end(source, index)
        if path_end is not None:
            index = path_end
            continue
        string_end = nix_string_end(source, index)
        if string_end is None:
            index += 1
            continue
        if nix_static_string_value(source[index:string_end]) == name:
            direct = re.match(r"\s*=", source[string_end:])
            if direct is not None:
                assignments.add(string_end + direct.end())
            prefix = source[:index].rstrip()
            dynamic = re.match(r"\s*}\s*=", source[string_end:])
            if prefix.endswith("${") and dynamic is not None:
                assignments.add(string_end + dynamic.end())
        index = string_end
    return tuple(sorted(assignments))


@cache
def nix_binds_attribute(text: str, name: str) -> bool:
    """Return whether executable Nix source binds an attribute name."""
    return bool(nix_attribute_assignment_ends(text, name))


@cache
def nix_uses_computed_attribute(text: str) -> bool:
    """Return whether executable Nix source contains a computed attribute."""
    source = nix_without_comments(text)
    if "${" in nix_string_mask(source):
        return True

    index = 0
    while index < len(source):
        path_end = nix_path_or_uri_end(source, index)
        if path_end is not None:
            index = path_end
            continue
        string_end = nix_string_end(source, index)
        if string_end is None:
            index += 1
            continue
        literal = source[index:string_end]
        ambiguous_interpolation = (
            "${" in literal and nix_static_string_value(literal) is None
        )
        indented_control_escape = literal.startswith("''") and "''\\" in literal
        if ambiguous_interpolation or indented_control_escape:
            follows_attribute = (
                re.match(r"\s*(?:=|\.)", source[string_end:]) is not None
            )
            follows_selection = source[:index].rstrip().endswith(".")
            if follows_attribute or follows_selection:
                return True
        index = string_end
    return False


@cache
def nix_executable_names(text: str) -> frozenset[str]:
    """Return bare and statically selected executable names in source."""
    source = nix_without_comments(text)
    masked = nix_string_mask(source)
    names = set(re.findall(r"(?<![A-Za-z0-9_'])\b[A-Za-z_][A-Za-z0-9_'-]*\b", masked))

    index = 0
    while index < len(source):
        path_end = nix_path_or_uri_end(source, index)
        if path_end is not None:
            index = path_end
            continue
        string_end = nix_string_end(source, index)
        if string_end is None:
            index += 1
            continue
        selected = nix_static_string_value(source[index:string_end])
        if selected is not None and source[:index].rstrip().endswith("."):
            names.add(selected)
        index = string_end
    return frozenset(names)


def nix_uses_executable_name(text: str, names: tuple[str, ...]) -> bool:
    """Return whether source executes a bare or statically selected name."""
    return not nix_executable_names(text).isdisjoint(names)


@cache
def nix_uses_reflective_attribute_access(text: str) -> bool:
    """Return whether executable Nix source retrieves an attribute by name."""
    return nix_uses_executable_name(text, ("attrByPath", "getAttr", "getAttrFromPath"))


@cache
def nix_constructs_attributes_dynamically(text: str) -> bool:
    """Return whether executable Nix source constructs attribute names at runtime."""
    constructors = (
        "fromJSON",
        "fromTOML",
        "genAttrs",
        "groupBy",
        "listToAttrs",
        "mapAttrs",
        "zipAttrsWith",
    )
    return nix_uses_executable_name(text, constructors)


@cache
def nix_uses_import_expression(text: str) -> bool:
    """Return whether executable Nix source evaluates the import primitive."""
    return nix_uses_executable_name(text, ("import", "scopedImport"))


@cache
def nix_inherits_attribute(text: str, name: str) -> bool:
    """Return whether executable Nix source inherits an attribute name."""
    source = nix_string_mask(nix_without_comments(text))
    return any(
        re.search(rf"(?<![A-Za-z0-9_'])\b{re.escape(name)}\b", match.group(1))
        is not None
        for match in re.finditer(r"\binherit\b(.*?);", source, re.DOTALL)
    )


@cache
def nix_uses_priority_override(text: str) -> bool:
    """Return whether Nix source contains an effective module override value."""
    source = nix_without_comments(text)
    if nix_binds_attribute(source, "_type") or nix_inherits_attribute(source, "_type"):
        return True
    priority_constructor = (
        r"(?:mkForce|mkDefault|mkOptionDefault|mk[A-Za-z0-9_]*Override)"
    )
    if re.search(rf"\b{priority_constructor}\b", nix_string_mask(source)):
        return True

    index = 0
    while index < len(source):
        path_end = nix_path_or_uri_end(source, index)
        if path_end is not None:
            index = path_end
            continue
        string_end = nix_string_end(source, index)
        if string_end is None:
            index += 1
            continue

        literal = source[index:string_end]
        literal_name = nix_static_string_value(literal) or ""
        if re.fullmatch(priority_constructor, literal_name):
            prefix = source[:index].rstrip()
            static_selection = prefix.endswith(".")
            dynamic_selection = prefix.endswith("${") and prefix[:-2].rstrip().endswith(
                "."
            )
            if static_selection or dynamic_selection:
                return True
        if literal == '"override"':
            prefix = source[:index].rstrip()
            if re.search(r'(?:\b_type|"_type")\s*=\s*$', prefix):
                return True
        index = string_end
    return False


@cache
def nix_delimited_end(text: str, index: int) -> int | None:
    """Return the exclusive end of a balanced Nix delimiter expression."""
    pairs = {")": "(", "]": "[", "}": "{"}
    if index >= len(text) or text[index] not in "([{":
        return None
    delimiters = [text[index]]
    index += 1
    while index < len(text):
        path_end = nix_path_or_uri_end(text, index)
        if path_end is not None:
            index = path_end
            continue
        character = text[index]
        if character in "([{":
            delimiters.append(character)
        elif character in pairs:
            if not delimiters or delimiters.pop() != pairs[character]:
                return None
            if not delimiters:
                return index + 1
        index += 1
    return None


@cache
def nix_local_imports(
    text: str,
    parent: Path,
    allowed_external_flake_modules: frozenset[str] = frozenset(),
    module_result: bool = True,
) -> tuple[frozenset[Path], bool]:
    """Resolve literal child- and parent-relative imports from a Nix module."""
    imports: set[Path] = set()
    sources = (text,)
    if module_result:
        statements = nix_module_result_statements(text)
        if statements is None:
            return frozenset(), True
        sources = tuple(
            statement
            for statement in statements
            if nix_statement_binds(statement, "imports")
        )
    unresolved = any(nix_inherits_attribute(source, "imports") for source in sources)
    for source in sources:
        masked = nix_string_mask(nix_without_comments(source))
        assignment_ends = nix_attribute_assignment_ends(source, "imports")
        if (
            module_result
            and len(assignment_ends) != 1
            and not nix_inherits_attribute(source, "imports")
        ):
            unresolved = True
        for assignment_end in assignment_ends:
            list_start = assignment_end
            while list_start < len(masked) and masked[list_start].isspace():
                list_start += 1
            if list_start == len(masked) or masked[list_start] != "[":
                unresolved = True
                continue
            list_end = nix_delimited_end(masked, list_start)
            if list_end is None:
                unresolved = True
                continue
            terminator = list_end
            while terminator < len(masked) and masked[terminator].isspace():
                terminator += 1
            if terminator == len(masked) or masked[terminator] != ";":
                unresolved = True
            body = masked[list_start + 1 : list_end - 1]
            residue = list(body)
            index = 0
            while index < len(body):
                path_end = nix_path_or_uri_end(body, index)
                if path_end is None:
                    index += 1
                    continue
                relative = body[index:path_end]
                if relative.startswith(("./", "../")):
                    residue[index:path_end] = " " * (path_end - index)
                    if "${" in relative:
                        unresolved = True
                    else:
                        imported = (parent / relative).resolve()
                        if imported.is_dir():
                            imported = imported / "default.nix"
                        imports.add(imported)
                index = path_end
            residue_text = "".join(residue)
            for external_module in allowed_external_flake_modules:
                residue_text = re.sub(
                    rf"(?<![A-Za-z0-9_']){re.escape(external_module)}"
                    r"(?![A-Za-z0-9_'])",
                    "",
                    residue_text,
                )
            if residue_text.strip():
                unresolved = True
    return frozenset(imports), unresolved


def nix_statement_binds(statement: str, name: str) -> bool:
    """Return whether an immediate let statement binds the given identifier."""
    stripped = statement.lstrip()
    if stripped.startswith(('"', "''")):
        string_end = nix_string_end(stripped, 0)
        if (
            string_end is not None
            and nix_static_string_value(stripped[:string_end]) == name
            and re.match(r"\s*(?:=|\.)", stripped[string_end:]) is not None
        ):
            return True
    escaped_name = re.escape(name)
    binding_name = (
        rf'(?:{escaped_name}|"{escaped_name}"|\'\'{escaped_name}\'\'|'
        rf'\$\{{\s*(?:"{escaped_name}"|\'\'{escaped_name}\'\')\s*\}})'
    )
    if re.match(rf"\s*{binding_name}\s*(?:=|\.)", statement):
        return True
    inherited = re.fullmatch(
        r"\s*inherit(?:\s*\([^)]*\))?\s+(.*?)\s*;\s*", statement, re.DOTALL
    )
    return inherited is not None and name in re.findall(
        r"\b[A-Za-z_][A-Za-z0-9_']*\b", inherited.group(1)
    )


def outer_per_system_let(text: str) -> tuple[str, str, str] | None:
    """Return perSystem's immediate let body, result, and function formals."""
    masked = nix_string_mask(text)
    header = re.match(
        r"\s*\{\s*inputs\s*,\s*\.\.\.\s*\}\s*:\s*\{\s*"
        r"perSystem\s*=\s*\{(?P<formals>[^{}]*)\}\s*:\s*let\b",
        masked,
        re.DOTALL,
    )
    if header is None:
        return None

    body_start = header.end()
    formals = text[header.start("formals") : header.end("formals")]
    delimiters: list[str] = []
    nested_lets = 0
    index = body_start
    pairs = {")": "(", "]": "[", "}": "{"}
    while index < len(masked):
        path_end = nix_path_or_uri_end(masked, index)
        if path_end is not None:
            index = path_end
            continue
        character = masked[index]
        if character in "([{":
            delimiters.append(character)
            index += 1
            continue
        if character in pairs:
            if not delimiters or delimiters.pop() != pairs[character]:
                return None
            index += 1
            continue
        if character.isalpha() or character == "_":
            end = index + 1
            while end < len(masked) and (masked[end].isalnum() or masked[end] in "_-'"):
                end += 1
            token = masked[index:end]
            if token == "let":
                nested_lets += 1
            elif token == "in":
                if nested_lets:
                    nested_lets -= 1
                elif not delimiters:
                    return text[body_start:index], text[end:], formals
            index = end
            continue
        index += 1
    return None


def top_level_nix_statements(text: str) -> list[str] | None:
    """Split a let body at semicolons belonging to its immediate scope."""
    masked = nix_string_mask(text)
    delimiters: list[str] = []
    nested_lets = 0
    statements: list[str] = []
    start = 0
    index = 0
    pairs = {")": "(", "]": "[", "}": "{"}
    while index < len(masked):
        path_end = nix_path_or_uri_end(masked, index)
        if path_end is not None:
            index = path_end
            continue
        character = masked[index]
        if character in "([{":
            delimiters.append(character)
            index += 1
            continue
        if character in pairs:
            if not delimiters or delimiters.pop() != pairs[character]:
                return None
            index += 1
            continue
        if character.isalpha() or character == "_":
            end = index + 1
            while end < len(masked) and (masked[end].isalnum() or masked[end] in "_-'"):
                end += 1
            token = masked[index:end]
            if token == "let":
                nested_lets += 1
            elif token == "in":
                if not nested_lets:
                    return None
                nested_lets -= 1
            index = end
            continue
        if character == ";" and not delimiters and not nested_lets:
            statements.append(text[start : index + 1])
            start = index + 1
        index += 1

    if delimiters or nested_lets or text[start:].strip():
        return None
    return statements


@cache
def nix_module_result_statements(text: str) -> tuple[str, ...] | None:
    """Return immediate bindings from a direct Nix module result attribute set."""
    source = nix_without_comments(text)
    masked = nix_string_mask(source)
    start = 0
    while start < len(masked) and masked[start].isspace():
        start += 1
    if start == len(masked) or masked[start] != "{":
        return None

    first_end = nix_delimited_end(masked, start)
    if first_end is None:
        return None
    after_first = first_end
    while after_first < len(masked) and masked[after_first].isspace():
        after_first += 1
    if after_first < len(masked) and masked[after_first] == ":":
        start = after_first + 1
        while start < len(masked) and masked[start].isspace():
            start += 1
        if start == len(masked) or masked[start] != "{":
            return None

    result_end = nix_delimited_end(masked, start)
    if result_end is None or masked[result_end:].strip():
        return None
    statements = top_level_nix_statements(source[start + 1 : result_end - 1])
    return tuple(statements) if statements is not None else None


def nix_outer_let_result_start(text: str, index: int) -> int | None:
    """Return the start of an outer let expression's result after `in`."""
    if re.match(r"let\b", text[index:]) is None:
        return None
    index += len("let")
    delimiters: list[str] = []
    nested_lets = 0
    pairs = {")": "(", "]": "[", "}": "{"}
    while index < len(text):
        path_end = nix_path_or_uri_end(text, index)
        if path_end is not None:
            index = path_end
            continue
        character = text[index]
        if character in "([{":
            delimiters.append(character)
            index += 1
            continue
        if character in pairs:
            if not delimiters or delimiters.pop() != pairs[character]:
                return None
            index += 1
            continue
        if character.isalpha() or character == "_":
            end = index + 1
            while end < len(text) and (text[end].isalnum() or text[end] in "_-'"):
                end += 1
            token = text[index:end]
            if token == "let":
                nested_lets += 1
            elif token == "in":
                if nested_lets:
                    nested_lets -= 1
                elif not delimiters:
                    return end
            index = end
            continue
        index += 1
    return None


@cache
def nix_per_system_result_module(text: str) -> tuple[str | None, bool]:
    """Isolate a reachable module's deferred perSystem result module."""
    statements = nix_module_result_statements(text)
    if statements is None:
        return None, False
    per_system = tuple(
        statement
        for statement in statements
        if nix_statement_binds(statement, "perSystem")
    )
    if not per_system:
        return None, False
    if len(per_system) != 1:
        return None, True

    source = nix_without_comments(per_system[0])
    masked = nix_string_mask(source)
    assignment_ends = nix_attribute_assignment_ends(source, "perSystem")
    if len(assignment_ends) != 1:
        return None, True
    index = assignment_ends[0]
    while index < len(masked) and masked[index].isspace():
        index += 1
    if index == len(masked) or masked[index] != "{":
        return None, True
    formals_end = nix_delimited_end(masked, index)
    if formals_end is None:
        return None, True
    index = formals_end
    while index < len(masked) and masked[index].isspace():
        index += 1
    if index == len(masked) or masked[index] != ":":
        return None, True
    index += 1
    while index < len(masked) and masked[index].isspace():
        index += 1
    if re.match(r"let\b", masked[index:]) is not None:
        result_start = nix_outer_let_result_start(masked, index)
        if result_start is None:
            return None, True
        index = result_start
        while index < len(masked) and masked[index].isspace():
            index += 1
    if index == len(masked) or masked[index] != "{":
        return None, True
    result_end = nix_delimited_end(masked, index)
    if result_end is None or re.fullmatch(r"\s*;\s*", masked[result_end:]) is None:
        return None, True
    return source[index:result_end], False


@cache
def nix_module_binds_attribute(text: str, name: str) -> bool:
    """Return whether a direct Nix module result immediately binds an attribute."""
    statements = nix_module_result_statements(text)
    return statements is not None and any(
        nix_statement_binds(statement, name) for statement in statements
    )


def validate_gate_wiring(root: Path, failures: list[str]) -> None:
    checks_root = (root / "nix/checks").resolve()
    checks_entry = checks_root / "default.nix"
    generator_path = checks_root / "rust-gates.nix"
    flake_path = root / "flake.nix"
    flake = nix_without_comments(flake_path.read_text(encoding="utf-8"))
    flake_masked = nix_string_mask(flake)
    flake_statements = nix_module_result_statements(flake)
    inputs_statement = next(
        (
            statement
            for statement in flake_statements or ()
            if nix_statement_binds(statement, "inputs")
        ),
        None,
    )
    inputs_module = None
    if inputs_statement is not None:
        inputs_match = re.fullmatch(
            r"\s*inputs\s*=\s*(\{.*\})\s*;\s*", inputs_statement, re.DOTALL
        )
        if inputs_match is not None:
            inputs_module = inputs_match.group(1)
    input_statements = nix_module_result_statements(inputs_module or "")

    def compact_statement(value: str) -> str:
        return re.sub(r"\s+", " ", value).strip()

    canonical_input_statements = {
        'flake-parts.url = "github:hercules-ci/flake-parts";',
        'nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";',
        'devshell.url = "github:numtide/devshell";',
        'rust-overlay.url = "github:oxalica/rust-overlay";',
        'crane.url = "github:ipetkov/crane";',
        compact_statement(
            """advisory-db = {
              url = "github:rustsec/advisory-db";
              flake = false;
            };"""
        ),
        'openspec.url = "github:Fission-AI/OpenSpec";',
    }
    actual_input_statements = {
        compact_statement(statement) for statement in input_statements or ()
    }
    if actual_input_statements != canonical_input_statements:
        failures.append("flake.nix does not bind canonical trusted input sources")

    canonical_input_provenance: dict[str, dict[str, str]] = {
        "flake-parts": {"owner": "hercules-ci", "repo": "flake-parts"},
        "nixpkgs": {
            "owner": "NixOS",
            "repo": "nixpkgs",
            "ref": "nixpkgs-unstable",
        },
        "devshell": {"owner": "numtide", "repo": "devshell"},
        "rust-overlay": {"owner": "oxalica", "repo": "rust-overlay"},
        "crane": {"owner": "ipetkov", "repo": "crane"},
        "advisory-db": {"owner": "rustsec", "repo": "advisory-db"},
        "openspec": {"owner": "Fission-AI", "repo": "OpenSpec"},
    }
    canonical_transitive_input_provenance: dict[str, dict[str, dict[str, str]]] = {
        "flake-parts": {
            "nixpkgs-lib": {"owner": "nix-community", "repo": "nixpkgs.lib"}
        },
        "devshell": {
            "nixpkgs": {
                "owner": "NixOS",
                "repo": "nixpkgs",
                "ref": "nixpkgs-unstable",
            }
        },
        "rust-overlay": {
            "nixpkgs": {
                "owner": "NixOS",
                "repo": "nixpkgs",
                "ref": "nixpkgs-unstable",
            }
        },
        "openspec": {
            "nixpkgs": {
                "owner": "NixOS",
                "repo": "nixpkgs",
                "ref": "nixos-unstable",
            }
        },
    }

    def canonical_locked_inputs() -> bool:
        try:
            lock = json.loads((root / "flake.lock").read_text(encoding="utf-8"))
            nodes = lock["nodes"]
            root_inputs = nodes[lock["root"]]["inputs"]
        except (FileNotFoundError, json.JSONDecodeError, KeyError, TypeError):
            return False
        if not isinstance(nodes, dict) or not isinstance(root_inputs, dict):
            return False
        if set(root_inputs) != set(canonical_input_provenance):
            return False

        def canonical_node(
            node_name: object, source: dict[str, str], *, flake: bool = True
        ) -> dict[str, Any] | None:
            if not isinstance(node_name, str):
                return None
            node = nodes.get(node_name)
            if not isinstance(node, dict):
                return None
            expected_original = {"type": "github", **source}
            if node.get("original") != expected_original:
                return None
            locked = node.get("locked")
            if not isinstance(locked, dict) or set(locked) != {
                "lastModified",
                "narHash",
                "owner",
                "repo",
                "rev",
                "type",
            }:
                return None
            if any(locked.get(key) != source[key] for key in ("owner", "repo")):
                return None
            if locked.get("type") != "github":
                return None
            if not isinstance(locked.get("lastModified"), int):
                return None
            if re.fullmatch(r"[0-9a-f]{40}", str(locked.get("rev", ""))) is None:
                return None
            if (
                re.fullmatch(
                    r"sha256-[A-Za-z0-9+/]{43}=", str(locked.get("narHash", ""))
                )
                is None
            ):
                return None
            if node.get("flake", True) is not flake:
                return None
            return node

        for name, source in canonical_input_provenance.items():
            node = canonical_node(
                root_inputs.get(name), source, flake=name != "advisory-db"
            )
            if node is None:
                return False
            expected_inputs = canonical_transitive_input_provenance.get(name, {})
            node_inputs = node.get("inputs", {})
            if not isinstance(node_inputs, dict) or set(node_inputs) != set(
                expected_inputs
            ):
                return False
            for input_name, input_source in expected_inputs.items():
                target = canonical_node(node_inputs.get(input_name), input_source)
                if target is None or target.get("inputs", {}) != {}:
                    return False
        return True

    if not canonical_locked_inputs():
        failures.append("flake.lock does not bind canonical trusted inputs")
    canonical_outputs = re.search(
        r"\boutputs\s*=\s*inputs\s*@\s*\{\s*flake-parts\s*,\s*"
        r"nixpkgs\s*,\s*rust-overlay\s*,\s*\.\.\.\s*\}\s*:\s*"
        r"flake-parts\.lib\.mkFlake\b",
        flake_masked,
        re.DOTALL,
    )
    plain_root = re.match(r"\s*\{", flake_masked) is not None
    outputs_bindings = len(re.findall(r"\boutputs\s*=", flake_masked))
    if not plain_root or outputs_bindings != 1 or canonical_outputs is None:
        failures.append("flake.nix does not expose canonical unshadowed outputs")
    flake_imports: frozenset[Path] = frozenset()
    root_module: str | None = None
    root_statements: tuple[str, ...] | None = None
    canonical_mk_flake_inputs = False
    outputs_expression_complete = False
    if canonical_outputs is not None:
        argument_start = canonical_outputs.end()
        while (
            argument_start < len(flake_masked)
            and flake_masked[argument_start].isspace()
        ):
            argument_start += 1
        inputs_end = nix_delimited_end(flake_masked, argument_start)
        if inputs_end is not None:
            inputs_argument = flake[argument_start:inputs_end]
            inputs_argument_statements = nix_module_result_statements(inputs_argument)
            canonical_mk_flake_inputs = (
                inputs_argument_statements is not None
                and len(inputs_argument_statements) == 1
                and re.fullmatch(
                    r"\s*inherit\s+inputs\s*;\s*", inputs_argument_statements[0]
                )
                is not None
            )
            module_start = inputs_end
            while (
                module_start < len(flake_masked)
                and flake_masked[module_start].isspace()
            ):
                module_start += 1
            module_end = nix_delimited_end(flake_masked, module_start)
            if module_end is not None:
                outputs_expression_complete = (
                    re.fullmatch(r"\s*;\s*}\s*", flake_masked[module_end:]) is not None
                )
                if outputs_expression_complete:
                    root_module = flake[module_start:module_end]
    if canonical_outputs is not None and not outputs_expression_complete:
        failures.append("flake.nix does not expose canonical unshadowed outputs")
    if not canonical_mk_flake_inputs:
        failures.append("flake.nix does not pass canonical inputs to mkFlake")
    root_imports_unresolved = root_module is None
    if root_module is not None:
        root_statements = nix_module_result_statements(root_module)
        flake_imports, root_imports_unresolved = nix_local_imports(
            root_module,
            flake_path.parent,
            allowed_external_flake_modules=frozenset({"inputs.devshell.flakeModule"}),
        )
    if root_imports_unresolved:
        failures.append("root Nix module graph has unresolved imports")
    canonical_root_bindings = ("imports", "systems", "perSystem")
    canonical_root_statements = (
        root_statements is not None
        and len(root_statements) == len(canonical_root_bindings)
        and all(
            sum(nix_statement_binds(statement, name) for statement in root_statements)
            == 1
            for name in canonical_root_bindings
        )
    )
    if not canonical_root_statements:
        failures.append("root Nix module has non-canonical statements")
    if root_module is not None and nix_module_binds_attribute(root_module, "config"):
        failures.append("root Nix module composes explicit config")
    if checks_entry not in flake_imports:
        failures.append("flake.nix does not import the nix/checks module")
        return

    repository_root = root.resolve()
    local_module_graph: set[Path] = {flake_path.resolve()}
    pending_modules = list(flake_imports)
    unresolved_import_modules: set[Path] = set()
    while pending_modules:
        module_path = pending_modules.pop()
        try:
            module_path.relative_to(repository_root)
        except ValueError:
            continue
        if module_path in local_module_graph or not module_path.is_file():
            continue
        local_module_graph.add(module_path)
        module_source = module_path.read_text(encoding="utf-8")
        module_imports, unresolved_imports = nix_local_imports(
            module_source, module_path.parent
        )
        deferred_module, deferred_unresolved = nix_per_system_result_module(
            module_source
        )
        if deferred_module is not None:
            deferred_imports, deferred_imports_unresolved = nix_local_imports(
                deferred_module, module_path.parent
            )
            module_imports = module_imports | deferred_imports
            deferred_unresolved = deferred_unresolved or deferred_imports_unresolved
        pending_modules.extend(module_imports)
        if unresolved_imports or deferred_unresolved:
            unresolved_import_modules.add(module_path)
    if unresolved_import_modules:
        failures.append(
            "local Nix module graph has unresolved imports: "
            + ", ".join(
                str(path.relative_to(repository_root))
                for path in sorted(unresolved_import_modules)
            )
        )
    disabled_modules = sorted(
        str(path.relative_to(repository_root))
        for path in local_module_graph
        if (
            nix_binds_attribute(path.read_text(encoding="utf-8"), "disabledModules")
            or nix_inherits_attribute(
                path.read_text(encoding="utf-8"), "disabledModules"
            )
        )
    )
    if disabled_modules:
        failures.append(
            "local Nix module graph uses disabledModules: "
            + ", ".join(disabled_modules)
        )
    computed_attribute_modules = sorted(
        str(path.relative_to(repository_root))
        for path in local_module_graph
        if path != generator_path
        and nix_uses_computed_attribute(path.read_text(encoding="utf-8"))
    )
    if computed_attribute_modules:
        failures.append(
            "local Nix module graph uses computed attributes: "
            + ", ".join(computed_attribute_modules)
        )
    reflective_attribute_modules = sorted(
        str(path.relative_to(repository_root))
        for path in local_module_graph
        if path != generator_path
        and nix_uses_reflective_attribute_access(path.read_text(encoding="utf-8"))
    )
    if reflective_attribute_modules:
        failures.append(
            "local Nix module graph uses reflective attributes: "
            + ", ".join(reflective_attribute_modules)
        )
    dynamic_attribute_modules = sorted(
        str(path.relative_to(repository_root))
        for path in local_module_graph
        if path != generator_path
        and nix_constructs_attributes_dynamically(path.read_text(encoding="utf-8"))
    )
    if dynamic_attribute_modules:
        failures.append(
            "local Nix module graph constructs attributes dynamically: "
            + ", ".join(dynamic_attribute_modules)
        )
    import_expression_modules = sorted(
        str(path.relative_to(repository_root))
        for path in local_module_graph
        if path != flake_path
        and nix_uses_import_expression(path.read_text(encoding="utf-8"))
    )
    if import_expression_modules:
        failures.append(
            "local Nix module graph uses import expressions: "
            + ", ".join(import_expression_modules)
        )
    explicit_config_modules = sorted(
        str(path.relative_to(repository_root))
        for path in local_module_graph
        if path != flake_path
        and nix_module_binds_attribute(path.read_text(encoding="utf-8"), "config")
    )
    if explicit_config_modules:
        failures.append(
            "local Nix module graph composes explicit config: "
            + ", ".join(explicit_config_modules)
        )
    competing_check_modules = sorted(
        str(path.relative_to(repository_root))
        for path in local_module_graph
        if path not in {checks_entry, generator_path}
        and (
            nix_binds_attribute(path.read_text(encoding="utf-8"), "checks")
            or nix_inherits_attribute(path.read_text(encoding="utf-8"), "checks")
        )
    )
    if competing_check_modules:
        failures.append(
            "local Nix module graph contributes competing checks: "
            + ", ".join(competing_check_modules)
        )
    priority_modules = sorted(
        str(path.relative_to(repository_root))
        for path in local_module_graph
        if nix_uses_priority_override(path.read_text(encoding="utf-8"))
    )
    if priority_modules:
        failures.append(
            "local Nix module graph uses priority overrides: "
            + ", ".join(priority_modules)
        )

    provider_pattern = (
        r"\s*perSystem\s*=\s*\{\s*system\s*,\s*\.\.\.\s*\}\s*:\s*\{\s*"
        r"_module\.args\.pkgs\s*=\s*import\s+nixpkgs\s*\{\s*"
        r"inherit\s+system\s*;\s*overlays\s*=\s*\[\s*"
        r"\(\s*import\s+rust-overlay\s*\)\s*\]\s*;\s*\}\s*;\s*"
        r"\}\s*;\s*"
    )
    provider_statements = [
        statement
        for statement in root_statements or ()
        if nix_statement_binds(statement, "perSystem")
    ]
    canonical_pkgs_provider = (
        len(provider_statements) == 1
        and re.fullmatch(
            provider_pattern,
            nix_string_mask(provider_statements[0]),
            re.DOTALL,
        )
        is not None
    )
    if not canonical_pkgs_provider:
        failures.append("flake.nix does not provide canonical pkgs to perSystem")

    default_nix = nix_without_comments(checks_entry.read_text(encoding="utf-8"))
    default_masked = nix_string_mask(default_nix)
    wrapper_check_bindings = len(re.findall(r"\bchecks\s*=", default_masked))
    plain_wrapper_checks = re.search(r"\bchecks\s*=\s*\{", default_masked)
    priority_override = nix_uses_priority_override(default_nix)
    if wrapper_check_bindings != 1 or plain_wrapper_checks is None or priority_override:
        failures.append("nix/checks/default.nix does not safely compose checks")
    check_imports, _ = nix_local_imports(default_nix, checks_entry.parent)
    if generator_path not in check_imports:
        failures.append("nix/checks/default.nix does not import rust-gates.nix")

    if not generator_path.is_file():
        failures.append("nix/checks/rust-gates.nix does not exist")
        return
    generator = nix_without_comments(generator_path.read_text(encoding="utf-8"))
    outer_scope = outer_per_system_let(generator)
    if outer_scope is None:
        failures.append(
            "rust-gates.nix does not expose perSystem as the direct canonical module result"
        )
    statements = (
        top_level_nix_statements(outer_scope[0]) if outer_scope is not None else None
    )
    statements = statements or []
    formal_entries = (
        [entry.strip() for entry in outer_scope[2].split(",") if entry.strip()]
        if outer_scope is not None
        else []
    )
    malformed_formals = [
        entry
        for entry in formal_entries
        if entry != "..." and re.fullmatch(r"[A-Za-z_][A-Za-z0-9_'-]*", entry) is None
    ]
    expected_formals = {
        "pkgs",
        "craneLib",
        "msrvCraneLib",
        "cargoArtifacts",
        "msrvCargoArtifacts",
        "rustSrc",
        "...",
    }
    invalid_formals = bool(malformed_formals) or set(formal_entries) != expected_formals
    shadows_global_builtins = "builtins" in formal_entries
    if shadows_global_builtins:
        failures.append("rust-gates.nix binds builtins in perSystem formals")
    if invalid_formals:
        failures.append("rust-gates.nix uses non-canonical perSystem formals")
    shadowed_trusted_roots = sorted(
        root
        for root in (
            "builtins",
            "pkgs",
            "inputs",
            "craneLib",
            "msrvCraneLib",
            "cargoArtifacts",
            "msrvCargoArtifacts",
            "rustSrc",
        )
        if any(nix_statement_binds(statement, root) for statement in statements)
    )
    if shadowed_trusted_roots:
        failures.append(
            "rust-gates.nix shadows trusted root(s) in perSystem let: "
            + ", ".join(shadowed_trusted_roots)
        )
    ambiguous_binding_root = any(
        re.match(r'\s*(?:"|\$\{)', statement) is not None for statement in statements
    )
    if ambiguous_binding_root:
        failures.append(
            "rust-gates.nix uses a quoted or dynamic immediate let binding root"
        )
    library_inherit = next(
        (
            match
            for statement in statements
            if (
                match := re.fullmatch(
                    r"\s*inherit\s*\(\s*pkgs\.lib\s*\)(.*?)\s*;\s*",
                    statement,
                    re.DOTALL,
                )
            )
            is not None
        ),
        None,
    )
    inherited_helpers = (
        set(re.findall(r"\b[A-Za-z_][A-Za-z0-9_']*\b", library_inherit.group(1)))
        if library_inherit is not None
        else set()
    )
    expected_helpers = {
        "concatMap",
        "concatStringsSep",
        "escapeShellArgs",
        "getAttr",
        "listToAttrs",
        "map",
        "optionalAttrs",
        "optionals",
    }
    if inherited_helpers != expected_helpers:
        failures.append(
            "rust-gates.nix does not inherit map and listToAttrs from pkgs.lib"
        )
    manifest_pattern = (
        r"\s*manifest\s*=\s*builtins\.fromTOML\s*"
        r"\(builtins\.readFile\s+\./gates\.toml\)\s*;\s*"
    )
    manifest_binding = next(
        (
            statement
            for statement in statements
            if re.fullmatch(manifest_pattern, statement, re.DOTALL)
        ),
        None,
    )
    canonical_gate_statements = {
        (
            "cargoArgumentAttribute = {\n"
            '        cargoBuild = "cargoExtraArgs";\n'
            '        cargoClippy = "cargoClippyExtraArgs";\n'
            '        cargoDoc = "cargoDocExtraArgs";\n'
            '        cargoNextest = "cargoNextestExtraArgs";\n'
            "      };"
        ),
        (
            "cargoArgs =\n"
            "        gate:\n"
            "        escapeShellArgs (\n"
            '          optionals gate.locked [ "--locked" ]\n'
            '          ++ optionals gate.workspace [ "--workspace" ]\n'
            "          ++ concatMap (package: [\n"
            '            "--package"\n'
            "            package\n"
            "          ]) gate.packages\n"
            "          ++ concatMap (package: [\n"
            '            "--exclude"\n'
            "            package\n"
            "          ]) gate.exclude_packages\n"
            '          ++ optionals gate.lib [ "--lib" ]\n'
            '          ++ optionals gate.all_targets [ "--all-targets" ]\n'
            '          ++ optionals gate.no_default_features [ "--no-default-features" ]\n'
            '          ++ optionals gate.all_features [ "--all-features" ]\n'
            "          ++ optionals (gate.features != [ ]) [\n"
            '            "--features"\n'
            '            (concatStringsSep "," gate.features)\n'
            "          ]\n"
            '          ++ optionals (gate.target != "") [\n'
            '            "--target"\n'
            "            gate.target\n"
            "          ]\n"
            "          ++ gate.extra_args\n"
            "        );"
        ),
        (
            "makeGate =\n"
            "        gate:\n"
            "        let\n"
            '          selectedCrane = if gate.toolchain == "msrv" then msrvCraneLib else craneLib;\n'
            "          operation = getAttr gate.operation selectedCrane;\n"
            "          argumentAttribute = cargoArgumentAttribute.${gate.operation} or null;\n"
            '          selectedArtifacts = if gate.artifacts == "msrv" then msrvCargoArtifacts else cargoArtifacts;\n'
            "        in\n"
            "        operation (\n"
            "          {\n"
            '            src = if gate.source == "repository" then ./../.. else rustSrc;\n'
            "          }\n"
            '          // optionalAttrs (gate.artifacts != "none") {\n'
            "            cargoArtifacts = selectedArtifacts;\n"
            "          }\n"
            "          // optionalAttrs (argumentAttribute != null) {\n"
            "            ${argumentAttribute} = cargoArgs gate;\n"
            "          }\n"
            '          // optionalAttrs (gate.operation == "cargoBuild") {\n'
            "            doCheck = false;\n"
            "          }\n"
            '          // optionalAttrs (gate.operation == "cargoAudit") {\n'
            "            inherit (inputs) advisory-db;\n"
            "          }\n"
            "        );"
        ),
    }
    actual_gate_statements = {
        statement.strip()
        for statement in statements
        if any(
            nix_statement_binds(statement, name)
            for name in ("cargoArgumentAttribute", "cargoArgs", "makeGate")
        )
    }
    canonical_gate_construction = actual_gate_statements == canonical_gate_statements
    generated_pattern = (
        r"\s*generatedChecks\s*=\s*listToAttrs\s*\(\s*map\s*\("
        r"\s*gate\s*:\s*\{\s*inherit\s*\(\s*gate\s*\)\s*name\s*;"
        r"\s*value\s*=\s*makeGate\s+gate\s*;\s*\}\s*\)"
        r"\s*manifest\.gates\s*\)\s*;\s*"
    )
    published_pattern = r"\s*\{\s*checks\s*=\s*generatedChecks\s*;\s*\}\s*;\s*\}\s*"
    generated_binding = next(
        (
            statement
            for statement in statements
            if re.fullmatch(generated_pattern, statement, re.DOTALL)
        ),
        None,
    )
    published_result = (
        re.fullmatch(published_pattern, outer_scope[1], re.DOTALL)
        if outer_scope is not None
        else None
    )
    connected_result = (
        all(
            value is not None
            for value in (
                library_inherit,
                manifest_binding,
                generated_binding,
                published_result,
            )
        )
        and canonical_gate_construction
        and not shadowed_trusted_roots
        and not ambiguous_binding_root
        and not shadows_global_builtins
        and not invalid_formals
    )
    if manifest_binding is None:
        failures.append("rust-gates.nix does not parse gates.toml as manifest")
    if not canonical_gate_construction:
        failures.append("rust-gates.nix does not bind canonical gate construction")
    if generated_binding is None:
        failures.append(
            "rust-gates.nix does not map gate names and values from manifest entries"
        )
    if published_result is None:
        failures.append(
            "rust-gates.nix does not return generatedChecks as top-level checks"
        )
    if not connected_result:
        failures.append(
            "rust-gates.nix does not directly return its manifest-mapped generatedChecks"
        )

    duplicates = sorted(
        path.relative_to(root)
        for path in checks_root.glob("rust-*.nix")
        if path.name != "rust-gates.nix"
    )
    if duplicates:
        failures.append(
            f"hand-written Rust gate modules duplicate the manifest: {duplicates}"
        )


def string_list(value: Any, context: str, failures: list[str]) -> list[str]:
    if not isinstance(value, list) or not all(
        isinstance(item, str) and item for item in value
    ):
        failures.append(f"{context} must be a list of non-empty strings")
        return []
    if len(value) != len(set(value)):
        failures.append(f"{context} must not contain duplicates")
    return value


def load_gate_manifest(
    root: Path,
    workspace: set[str],
    manifests: dict[str, Path],
    failures: list[str],
) -> dict[str, dict[str, Any]]:
    path = root / GATE_MANIFEST_PATH
    manifest = load_toml(path, failures)
    if manifest.get("schema_version") != 1:
        failures.append("gate manifest schema_version must be 1")
    if set(manifest) != {"schema_version", "gates"}:
        failures.append(
            f"gate manifest has unknown top-level fields {sorted(set(manifest) - {'schema_version', 'gates'})}"
        )
    raw_gates = manifest.get("gates")
    if not isinstance(raw_gates, list) or not raw_gates:
        failures.append("gate manifest is missing [[gates]] entries")
        return {}

    gates: dict[str, dict[str, Any]] = {}
    for index, gate in enumerate(raw_gates):
        context = f"gate manifest entry {index}"
        if not isinstance(gate, dict):
            failures.append(f"{context} must be a table")
            continue
        unknown = set(gate) - GATE_FIELDS
        missing = GATE_FIELDS - set(gate)
        if unknown or missing:
            failures.append(
                f"{context} fields do not match schema: missing={sorted(missing)}, unknown={sorted(unknown)}"
            )
        name = gate.get("name")
        if not isinstance(name, str) or re.fullmatch(r"rust-[a-z0-9-]+", name) is None:
            failures.append(f"{context} has invalid name {name!r}")
            continue
        if name in gates:
            failures.append(f"gate manifest contains duplicate gate {name!r}")
            continue
        gates[name] = {}

        operation = gate.get("operation")
        if not isinstance(operation, str) or operation not in ALLOWED_OPERATIONS:
            failures.append(f"gate {name} has unsupported operation {operation!r}")
            operation = ""
        gate["operation"] = operation
        toolchain = gate.get("toolchain")
        if not isinstance(toolchain, str) or toolchain not in {"etalon", "msrv"}:
            failures.append(f"gate {name} has invalid toolchain {toolchain!r}")
            toolchain = ""
        gate["toolchain"] = toolchain
        source = gate.get("source")
        if not isinstance(source, str) or source not in {"rust", "repository"}:
            failures.append(f"gate {name} has invalid source {source!r}")
            source = ""
        gate["source"] = source
        artifacts = gate.get("artifacts")
        if not isinstance(artifacts, str) or artifacts not in {
            "none",
            "etalon",
            "msrv",
        }:
            failures.append(f"gate {name} has invalid artifacts {artifacts!r}")
            artifacts = ""
        gate["artifacts"] = artifacts

        for field in (
            "locked",
            "workspace",
            "lib",
            "all_targets",
            "no_default_features",
            "all_features",
        ):
            value = gate.get(field)
            if not isinstance(value, bool):
                failures.append(f"gate {name}.{field} must be a boolean")
                gate[field] = False
        packages = string_list(gate.get("packages"), f"gate {name}.packages", failures)
        excluded = string_list(
            gate.get("exclude_packages"), f"gate {name}.exclude_packages", failures
        )
        features = string_list(gate.get("features"), f"gate {name}.features", failures)
        extra_args = string_list(
            gate.get("extra_args"), f"gate {name}.extra_args", failures
        )
        target = gate.get("target")
        if not isinstance(target, str):
            failures.append(f"gate {name}.target must be a string")
            target = ""
        gate["packages"] = packages
        gate["exclude_packages"] = excluded
        gate["features"] = features
        gate["extra_args"] = extra_args
        gate["target"] = target
        gates[name] = gate

        unknown_packages = (set(packages) | set(excluded)) - workspace
        if unknown_packages:
            failures.append(
                f"gate {name} names unknown packages {sorted(unknown_packages)}"
            )
        if gate.get("workspace") and packages:
            failures.append(
                f"gate {name} cannot select workspace and explicit packages"
            )
        if (
            operation in OPERATIONS_WITH_CARGO_SELECTION
            and not gate.get("workspace")
            and not packages
        ):
            failures.append(
                f"gate {name} operation {operation} requires explicit workspace or packages"
            )
        if excluded and not gate.get("workspace"):
            failures.append(f"gate {name} exclusions require workspace=true")
        if gate.get("all_features") and features:
            failures.append(
                f"gate {name} cannot select all_features and named features"
            )
        if gate.get("lib") and gate.get("all_targets"):
            failures.append(f"gate {name} cannot select lib and all_targets")
        if gate.get("no_default_features") and gate.get("all_features"):
            failures.append(
                f"gate {name} cannot select no_default_features and all_features"
            )
        if target and operation != "cargoBuild":
            failures.append(f"gate {name} target selection requires cargoBuild")

        has_selection = bool(
            gate.get("locked")
            or gate.get("workspace")
            or packages
            or excluded
            or gate.get("lib")
            or gate.get("all_targets")
            or gate.get("no_default_features")
            or gate.get("all_features")
            or features
            or target
            or gate.get("extra_args")
        )
        if operation not in OPERATIONS_WITH_CARGO_SELECTION and has_selection:
            failures.append(
                f"gate {name} operation {operation} cannot carry Cargo selection"
            )
        if (
            operation in OPERATION_EXTRA_ARGS
            and extra_args != OPERATION_EXTRA_ARGS[operation]
        ):
            failures.append(
                f"gate {name} operation {operation} requires extra_args={OPERATION_EXTRA_ARGS[operation]!r}"
            )
        if (
            operation in OPERATIONS_WITH_CARGO_SELECTION
            and gate.get("source") != "rust"
        ):
            failures.append(f"gate {name} operation {operation} requires rust source")
        if operation in OPERATIONS_WITH_CARGO_SELECTION and gate.get(
            "artifacts"
        ) != gate.get("toolchain"):
            failures.append(
                f"gate {name} operation {operation} artifacts must match its toolchain"
            )
        if operation in {"cargoAudit", "cargoDeny"} and (
            gate.get("source") != "repository" or gate.get("artifacts") != "none"
        ):
            failures.append(
                f"gate {name} operation {operation} requires repository source and no artifacts"
            )
        if operation == "cargoFmt" and (
            gate.get("source") != "rust" or gate.get("artifacts") != "none"
        ):
            failures.append(
                f"gate {name} cargoFmt requires rust source and no artifacts"
            )
        if gate.get("toolchain") == "msrv" and (
            operation != "cargoBuild" or gate.get("artifacts") != "msrv"
        ):
            failures.append(
                f"gate {name} MSRV toolchain requires cargoBuild and MSRV artifacts"
            )
        if gate.get("toolchain") == "etalon" and gate.get("artifacts") == "msrv":
            failures.append(f"gate {name} etalon toolchain cannot use MSRV artifacts")
        if gate.get("artifacts") == "etalon" and gate.get("toolchain") != "etalon":
            failures.append(f"gate {name} etalon artifacts require etalon toolchain")

        for feature in features:
            package_name, separator, feature_name = feature.partition("/")
            if separator:
                candidate_package = package_name
            elif len(packages) == 1:
                candidate_package = packages[0]
                feature_name = feature
            else:
                failures.append(
                    f"gate {name} feature {feature!r} requires one package or package/feature syntax"
                )
                continue
            manifest_path = manifests.get(candidate_package)
            if manifest_path is None:
                continue
            package_manifest = load_toml(manifest_path, failures)
            available = package_manifest.get("features", {})
            if not isinstance(available, dict) or feature_name not in available:
                failures.append(f"gate {name} references missing feature {feature!r}")
    return gates


def validate_gate(
    gate: Any,
    evidence_token: str,
    gates: dict[str, dict[str, Any]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or not gate:
        failures.append(f"{context} has an invalid gate")
        return
    definition = gates.get(gate)
    if definition is None:
        failures.append(f"{context} references undefined Nix gate {gate}")
        return
    evidence = " ".join(
        [
            *definition.get("packages", []),
            *definition.get("features", []),
            str(definition.get("target", "")),
            *definition.get("extra_args", []),
            "--no-default-features" if definition.get("no_default_features") else "",
            "--all-features" if definition.get("all_features") else "",
        ]
    )
    if evidence_token and evidence_token not in evidence:
        failures.append(
            f"{context} gate {gate} does not contain evidence token {evidence_token!r} in structured execution data"
        )


def referenced_policy_gates(policy: dict[str, Any]) -> set[str]:
    gates: set[str] = set()
    for host in policy.get("hosts", []):
        if isinstance(host, dict):
            gates.update(
                gate for gate in host.get("gates", []) if isinstance(gate, str)
            )
    for target in policy.get("targets", []):
        if isinstance(target, dict) and isinstance(target.get("gate"), str):
            gates.add(target["gate"])
    for surface in policy.get("features", []):
        if not isinstance(surface, dict):
            continue
        gates.update(gate for gate in surface.get("gates", []) if isinstance(gate, str))
        if isinstance(surface.get("msrv_gate"), str):
            gates.add(surface["msrv_gate"])
    return gates


def validate_gate_operations(
    policy: dict[str, Any],
    gates: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    raw_operations = require_table(policy, "gate_operations", failures)
    operations: dict[str, str] = {}
    for gate, operation in raw_operations.items():
        if not isinstance(operation, str) or not operation:
            failures.append(f"gate_operations.{gate} must be a non-empty string")
            continue
        operations[gate] = operation

    referenced = referenced_policy_gates(policy)
    if set(operations) != referenced:
        failures.append(
            "gate_operations keys do not match policy gates: "
            f"missing={sorted(referenced - set(operations))}, "
            f"extra={sorted(set(operations) - referenced)}"
        )
    if set(gates) != referenced:
        failures.append(
            "gate manifest names do not match policy gates: "
            f"missing={sorted(referenced - set(gates))}, "
            f"extra={sorted(set(gates) - referenced)}"
        )
    for gate in sorted(referenced & set(operations)):
        definition = gates.get(gate)
        if definition is None:
            continue
        if definition.get("operation") != operations[gate]:
            failures.append(
                f"gate {gate} uses Crane operation {definition.get('operation')}, "
                f"expected {operations[gate]} in {GATE_MANIFEST_PATH}"
            )


def validate_gate_target(
    gate: Any,
    expected_target: str,
    gates: dict[str, dict[str, Any]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or gate not in gates:
        return
    target = gates[gate].get("target")
    if target != expected_target:
        failures.append(
            f"{context} gate {gate} uses Cargo target {target!r}, "
            f"expected {expected_target!r} in {GATE_MANIFEST_PATH}"
        )


def validate_gate_cargo_selection(
    gate: Any,
    declared_packages: set[str],
    expects_workspace: bool,
    no_default_features: bool,
    declared_features: set[str],
    available_features: set[str] | None,
    workspace_packages: set[str],
    gates: dict[str, dict[str, Any]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or gate not in gates:
        return
    definition = gates[gate]
    selects_workspace = definition.get("workspace") is True
    excluded_packages = set(definition.get("exclude_packages", []))
    selected_packages = (
        set(workspace_packages)
        if selects_workspace
        else set(definition.get("packages", []))
    ) - excluded_packages
    actual_no_default = definition.get("no_default_features") is True
    selected_features = set(definition.get("features", []))
    if definition.get("all_features") is True:
        selected_features = (
            set(available_features)
            if available_features is not None
            else {"<all-features>"}
        )
    if (
        selected_packages != declared_packages
        or selects_workspace != expects_workspace
        or bool(excluded_packages)
        or actual_no_default != no_default_features
        or selected_features != declared_features
    ):
        failures.append(
            f"{context} gate {gate} selects packages {sorted(selected_packages)}, "
            f"workspace={selects_workspace}, excludes={sorted(excluded_packages)}, "
            f"no_default_features={actual_no_default}, "
            f"features={sorted(selected_features)}; "
            f"expected packages {sorted(declared_packages)}, "
            f"workspace={expects_workspace}, "
            f"no_default_features={no_default_features}, features={sorted(declared_features)} in {GATE_MANIFEST_PATH}"
        )


def index_unique_policy_entries(
    entries: list[Any],
    key: str,
    identity_name: str,
    failures: list[str],
) -> dict[str, dict[str, Any]]:
    indexed: dict[str, dict[str, Any]] = {}
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        identity = entry.get(key)
        if not isinstance(identity, str):
            continue
        if identity in indexed:
            failures.append(f"policy contains duplicate {identity_name} {identity!r}")
            continue
        indexed[identity] = entry
    return indexed


def validate_msrv_builder(
    gate: Any,
    gates: dict[str, dict[str, Any]],
    context: str,
    failures: list[str],
) -> None:
    if not isinstance(gate, str) or gate not in gates:
        return
    if gates[gate].get("toolchain") != "msrv":
        failures.append(
            f"{context} gate {gate} is not built with the MSRV toolchain in {GATE_MANIFEST_PATH}"
        )


def validate_toolchains(
    root: Path,
    policy: dict[str, Any],
    cargo: dict[str, Any],
    gates: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    toolchains = require_table(policy, "toolchains", failures)
    expected = {
        "msrv": "1.85.0",
        "etalon": "nightly-2026-03-18",
        "neoprism_revision": "8becb225132efb1d9302b2c5f6ed4d87b84e8685",
        "nixpkgs_revision": "c27cdad491a991b11ed731760aa2ef8db0cb0410",
        "rust_overlay_revision": "f17186f52e82ec5cf40920b58eac63b78692ac7c",
        "nix_version": "2.34.8",
    }
    for field, expected_value in expected.items():
        actual = require_nonempty_string(toolchains, field, "[toolchains]", failures)
        if actual and actual != expected_value:
            failures.append(
                f"toolchains.{field} must be {expected_value}, found {actual}"
            )

    cargo_msrv = cargo.get("workspace", {}).get("package", {}).get("rust-version")
    if cargo_msrv != toolchains.get("msrv"):
        failures.append(
            f"Cargo workspace rust-version {cargo_msrv!r} does not match policy MSRV {toolchains.get('msrv')!r}"
        )

    rust_nix = nix_without_comments(
        (root / "nix/rust-toolchain.nix").read_text(encoding="utf-8")
    )
    rust_module_statements = nix_module_result_statements(rust_nix)
    rust_per_system_statements = [
        statement
        for statement in rust_module_statements or ()
        if nix_statement_binds(statement, "perSystem")
    ]
    canonical_rust_module_formals = (
        re.match(
            r"\s*\{\s*inputs\s*,\s*\.\.\.\s*\}\s*:\s*\{",
            nix_string_mask(rust_nix),
        )
        is not None
    )
    toolchain_scope = (
        outer_per_system_let("{ inputs, ... }: {" + rust_per_system_statements[0] + "}")
        if canonical_rust_module_formals and len(rust_per_system_statements) == 1
        else None
    )
    toolchain_statements = (
        top_level_nix_statements(toolchain_scope[0])
        if toolchain_scope is not None
        else None
    )

    def compact(value: str) -> str:
        return re.sub(r"\s+", " ", value).strip()

    etalon_date = str(toolchains.get("etalon", "")).removeprefix("nightly-")
    expected_toolchain_statements = {
        compact(
            f'''toolchain = pkgs.rust-bin.nightly."{etalon_date}".default.override {{
              extensions = [ "rust-src" "rust-analyzer" ];
              targets = [
                "aarch64-apple-ios"
                "aarch64-linux-android"
                "wasm32-unknown-unknown"
              ];
            }};'''
        ),
        "craneLib = (inputs.crane.mkLib pkgs).overrideToolchain toolchain;",
        f'msrvToolchain = pkgs.rust-bin.stable."{toolchains.get("msrv", "")}".minimal;',
        "msrvCraneLib = (inputs.crane.mkLib pkgs).overrideToolchain msrvToolchain;",
    }
    protected_toolchain_names = (
        "inputs",
        "pkgs",
        "toolchain",
        "craneLib",
        "msrvToolchain",
        "msrvCraneLib",
    )
    actual_toolchain_statements = {
        compact(statement)
        for statement in toolchain_statements or ()
        if any(
            nix_statement_binds(statement, name) for name in protected_toolchain_names
        )
    }
    expected_provider_publication = compact(
        """{
          inherit craneLib msrvCraneLib msrvToolchain toolchain ;
        };"""
    )
    result_statements: list[str] | None = None
    if toolchain_scope is not None:
        result_source = toolchain_scope[1]
        result_masked = nix_string_mask(result_source)
        result_start = len(result_masked) - len(result_masked.lstrip())
        result_end = nix_delimited_end(result_masked, result_start)
        result_is_complete = result_end is not None and re.fullmatch(
            r"\s*;\s*}\s*", result_masked[result_end:]
        )
        if result_is_complete:
            result_statements = top_level_nix_statements(
                result_source[result_start + 1 : result_end - 1]
            )
    provider_publications = {
        compact(re.sub(r"^\s*_module\.args\s*=\s*", "", statement))
        for statement in result_statements or ()
        if re.match(r"\s*_module\.args\s*=", statement) is not None
    }
    direct_provider_overrides = any(
        re.match(
            r"\s*_module\.args\."
            r"(?:craneLib|msrvCraneLib|msrvToolchain|toolchain)\s*=",
            statement,
        )
        is not None
        for statement in result_statements or ()
    )
    toolchain_formals = (
        {entry.strip() for entry in toolchain_scope[2].split(",") if entry.strip()}
        if toolchain_scope is not None
        else set()
    )
    if (
        actual_toolchain_statements != expected_toolchain_statements
        or len(result_statements or ()) != 1
        or provider_publications != {expected_provider_publication}
        or direct_provider_overrides
        or toolchain_formals != {"pkgs", "..."}
    ):
        failures.append(
            "nix/rust-toolchain.nix does not bind canonical Crane providers"
        )
    for token in [
        f'nightly."{str(toolchains.get("etalon", "")).removeprefix("nightly-")}"',
        f'stable."{toolchains.get("msrv", "")}"',
    ]:
        if token not in rust_nix:
            failures.append(f"nix/rust-toolchain.nix does not select {token}")

    msrv = re.escape(str(toolchains.get("msrv", "")))
    if not re.search(
        rf'msrvToolchain\s*=\s*pkgs\.rust-bin\.stable\."{msrv}"\.minimal\s*;',
        rust_nix,
    ):
        failures.append(
            "nix/rust-toolchain.nix does not bind msrvToolchain to the policy stable toolchain"
        )
    if not re.search(
        r"msrvCraneLib\s*=\s*\(inputs\.crane\.mkLib\s+pkgs\)"
        r"\.overrideToolchain\s+msrvToolchain\s*;",
        rust_nix,
    ):
        failures.append(
            "nix/rust-toolchain.nix does not wire msrvCraneLib to msrvToolchain"
        )

    for target, tier in REQUIRED_TARGETS.items():
        if tier == "compile-checked" and f'"{target}"' not in rust_nix:
            failures.append(f"Nix Rust toolchain is missing target component {target}")

    flake = nix_without_comments((root / "flake.nix").read_text(encoding="utf-8"))
    systems_match = re.search(r"systems\s*=\s*\[(.*?)\];", flake, re.DOTALL)
    systems = (
        set(re.findall(r'"([^"]+)"', systems_match.group(1)))
        if systems_match
        else set()
    )
    if systems != set(REQUIRED_HOSTS):
        failures.append(
            f"flake systems {sorted(systems)} do not match policy hosts {sorted(REQUIRED_HOSTS)}"
        )

    lock_path = root / "flake.lock"
    try:
        lock = json.loads(lock_path.read_text(encoding="utf-8"))
        nodes = lock["nodes"]
        root_inputs = nodes["root"]["inputs"]
        nixpkgs_rev = nodes[root_inputs["nixpkgs"]]["locked"]["rev"]
        overlay_rev = nodes[root_inputs["rust-overlay"]]["locked"]["rev"]
        if nixpkgs_rev != toolchains.get("nixpkgs_revision"):
            failures.append("root nixpkgs lock revision does not match support policy")
        if overlay_rev != toolchains.get("rust_overlay_revision"):
            failures.append(
                "root rust-overlay lock revision does not match support policy"
            )
    except (OSError, KeyError, TypeError, json.JSONDecodeError) as error:
        failures.append(f"cannot validate flake.lock root inputs: {error}")

    adr = (root / "docs/adr/0002-neoprism-toolchain-alignment.md").read_text(
        encoding="utf-8"
    )
    for field in ("neoprism_revision", "nix_version"):
        value = str(toolchains.get(field, ""))
        if value and value not in adr:
            failures.append(f"ADR 0002 does not record policy {field} {value}")

    validate_gate("rust-msrv", "", gates, "MSRV policy", failures)


def validate_hosts(
    policy: dict[str, Any],
    gates: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    hosts = policy.get("hosts")
    if not isinstance(hosts, list):
        failures.append("policy is missing [[hosts]] entries")
        return
    by_system = index_unique_policy_entries(
        hosts, "nix_system", "host system", failures
    )
    if set(by_system) != set(REQUIRED_HOSTS):
        failures.append(
            f"policy hosts {sorted(str(value) for value in by_system)} do not match required hosts {sorted(REQUIRED_HOSTS)}"
        )
    for system, target in REQUIRED_HOSTS.items():
        host = by_system.get(system, {})
        if host.get("rust_target") != target:
            failures.append(f"host {system} must map to Rust target {target}")
        if host.get("tier") != "host-tested":
            failures.append(f"host {system} must be host-tested")
        require_nonempty_string(host, "limitation", f"host {system}", failures)
        gate_names = host.get("gates")
        if not isinstance(gate_names, list) or not gate_names:
            failures.append(f"host {system} must declare gates")
            continue
        for gate in gate_names:
            validate_gate(gate, "", gates, f"host {system}", failures)


def validate_targets(
    policy: dict[str, Any],
    packages: set[str],
    manifests: dict[str, Path],
    gates: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    targets = policy.get("targets")
    if not isinstance(targets, list):
        failures.append("policy is missing [[targets]] entries")
        return
    by_triple = index_unique_policy_entries(
        targets, "triple", "target triple", failures
    )
    if set(by_triple) != set(REQUIRED_TARGETS):
        failures.append(
            f"policy targets {sorted(str(value) for value in by_triple)} do not match required targets {sorted(REQUIRED_TARGETS)}"
        )
    for triple, required_tier in REQUIRED_TARGETS.items():
        target = by_triple.get(triple, {})
        tier = target.get("tier")
        if tier not in ALLOWED_TIERS:
            failures.append(f"target {triple} has invalid tier {tier!r}")
        elif tier != required_tier:
            failures.append(f"target {triple} must be {required_tier}, found {tier}")
        require_nonempty_string(target, "surface", f"target {triple}", failures)
        require_nonempty_string(target, "limitation", f"target {triple}", failures)
        declared_packages = target.get("packages")
        if not isinstance(declared_packages, list):
            failures.append(f"target {triple} has no package list")
            continue
        unknown = set(declared_packages) - packages
        if unknown:
            failures.append(f"target {triple} names unknown packages {sorted(unknown)}")
        if tier == "compile-checked":
            if set(declared_packages) != REQUIRED_COMPILE_PACKAGES:
                failures.append(
                    f"target {triple} compile package set does not match {sorted(REQUIRED_COMPILE_PACKAGES)}"
                )
            declared_features = target.get("features")
            if not isinstance(declared_features, list) or not all(
                isinstance(item, str) for item in declared_features
            ):
                failures.append(f"target {triple} has an invalid features list")
                declared_features = []
            for qualified_feature in declared_features:
                package_name, separator, feature_name = qualified_feature.partition("/")
                if not separator or not package_name or not feature_name:
                    failures.append(
                        f"target {triple} feature {qualified_feature!r} must use package/feature syntax"
                    )
                    continue
                if package_name not in declared_packages:
                    failures.append(
                        f"target {triple} feature {qualified_feature!r} names an unselected package"
                    )
                    continue
                manifest_path = manifests.get(package_name)
                if manifest_path is None:
                    continue
                manifest = load_toml(manifest_path, failures)
                available = manifest.get("features", {})
                if not isinstance(available, dict) or feature_name not in available:
                    failures.append(
                        f"target {triple} references missing feature {qualified_feature}"
                    )
            no_default_features = target.get("no_default_features")
            if not isinstance(no_default_features, bool):
                failures.append(f"target {triple} must declare no_default_features")
                no_default_features = False
            evidence_token = require_nonempty_string(
                target, "evidence_token", f"target {triple}", failures
            )
            validate_gate(
                target.get("gate"), evidence_token, gates, f"target {triple}", failures
            )
            validate_gate_target(
                target.get("gate"), triple, gates, f"target {triple}", failures
            )
            validate_gate_cargo_selection(
                target.get("gate"),
                set(declared_packages),
                False,
                no_default_features,
                set(declared_features),
                None,
                packages,
                gates,
                f"target {triple}",
                failures,
            )
        elif declared_packages:
            failures.append(f"planned target {triple} must not claim eligible packages")
        elif "gate" in target:
            failures.append(f"planned target {triple} must not claim a required gate")


def validate_features(
    policy: dict[str, Any],
    manifests: dict[str, Path],
    gates: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    features = policy.get("features")
    if not isinstance(features, list):
        failures.append("policy is missing [[features]] entries")
        return
    by_name = index_unique_policy_entries(features, "name", "feature surface", failures)
    if set(by_name) != REQUIRED_FEATURE_SURFACES:
        failures.append(
            f"policy feature surfaces {sorted(str(value) for value in by_name)} do not match required surfaces {sorted(REQUIRED_FEATURE_SURFACES)}"
        )

    for name in REQUIRED_FEATURE_SURFACES:
        surface = by_name.get(name, {})
        package = surface.get("package")
        if package != "*" and package not in manifests:
            failures.append(f"feature surface {name} names unknown package {package!r}")
            continue
        declared = surface.get("features")
        if not isinstance(declared, list) or not all(
            isinstance(item, str) for item in declared
        ):
            failures.append(f"feature surface {name} has an invalid features list")
            continue
        if not isinstance(surface.get("no_default_features"), bool):
            failures.append(f"feature surface {name} must declare no_default_features")
        available_features: set[str] | None = None
        if package != "*":
            manifest = load_toml(manifests[package], failures)
            available = manifest.get("features", {})
            if not isinstance(available, dict):
                failures.append(f"package {package} has an invalid features table")
                available = {}
            available_features = set(available) - {"default"}
            for feature in declared:
                if feature not in available:
                    failures.append(
                        f"feature surface {name} references missing {package} feature {feature}"
                    )
        gate_names = surface.get("gates")
        token = require_nonempty_string(
            surface, "evidence_token", f"feature {name}", failures
        )
        if not isinstance(gate_names, list) or not gate_names:
            failures.append(f"feature surface {name} must declare gates")
            continue
        for gate in gate_names:
            validate_gate(gate, token, gates, f"feature {name}", failures)
            validate_gate_cargo_selection(
                gate,
                set(manifests) if package == "*" else {package},
                package == "*",
                surface.get("no_default_features"),
                set(declared),
                available_features,
                set(manifests),
                gates,
                f"feature {name}",
                failures,
            )

        msrv_gate = require_nonempty_string(
            surface, "msrv_gate", f"feature {name}", failures
        )
        validate_gate(msrv_gate, "", gates, f"feature {name} MSRV", failures)
        validate_msrv_builder(msrv_gate, gates, f"feature {name} MSRV", failures)
        validate_gate_cargo_selection(
            msrv_gate,
            set(manifests) if package == "*" else {package},
            package == "*",
            surface.get("no_default_features"),
            set(declared),
            available_features,
            set(manifests),
            gates,
            f"feature {name} MSRV",
            failures,
        )


def validate_deferred_dimensions(
    policy: dict[str, Any], packages: set[str], failures: list[str]
) -> None:
    ffi = require_table(policy, "ffi", failures)
    if ffi.get("status") != "not-supported":
        failures.append("ffi.status must remain not-supported in this bootstrap policy")
    if ffi.get("package") not in packages:
        failures.append("ffi.package must name an existing workspace placeholder")
    require_nonempty_string(ffi, "reason", "[ffi]", failures)

    for dimension in ("binary_size", "build_time"):
        table = require_table(policy, dimension, failures)
        if table.get("status") != "measurement-only":
            failures.append(f"{dimension}.status must be measurement-only")
        require_nonempty_string(table, "measurement_point", f"[{dimension}]", failures)
        require_nonempty_string(table, "reason", f"[{dimension}]", failures)


def validate(root: Path) -> list[str]:
    failures: list[str] = []
    policy = load_toml(root / POLICY_PATH, failures)
    cargo = load_toml(root / "Cargo.toml", failures)
    if policy.get("schema_version") != 1:
        failures.append("policy schema_version must be 1")
    require_nonempty_string(policy, "policy_revision", "policy", failures)

    packages, manifests = workspace_packages(root, cargo, failures)
    gates = load_gate_manifest(root, packages, manifests, failures)
    validate_gate_wiring(root, failures)
    validate_gate_operations(policy, gates, failures)
    validate_toolchains(root, policy, cargo, gates, failures)
    validate_hosts(policy, gates, failures)
    validate_targets(policy, packages, manifests, gates, failures)
    validate_features(policy, manifests, gates, failures)
    validate_deferred_dimensions(policy, packages, failures)
    return sorted(set(failures))


def main() -> int:
    root = (
        Path(sys.argv[1]).resolve()
        if len(sys.argv) > 1
        else Path(__file__).resolve().parents[1]
    )
    failures = validate(root)
    if failures:
        for failure in failures:
            print(f"sdk-support-policy: {failure}", file=sys.stderr)
        print(f"sdk-support-policy: {len(failures)} failure(s)", file=sys.stderr)
        return 1
    print("sdk-support-policy: contract passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
