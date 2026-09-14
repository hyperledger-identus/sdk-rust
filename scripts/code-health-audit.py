#!/usr/bin/env python3
"""Generate and validate deterministic sdk-rust code-health evidence."""

from __future__ import annotations

import argparse
import hashlib
import io
import json
import re
import subprocess
import sys
import tarfile
import tempfile
import tomllib
from dataclasses import dataclass
from pathlib import Path
from typing import Iterator


CONFIG_PATH = Path("docs/architecture/code-health.toml")
EXPECTED_SCHEMA = 1
ALLOWED_DISPOSITIONS = {
    "decompose",
    "deduplicate",
    "document-exception",
    "defer-with-owner",
}


class AuditError(RuntimeError):
    """An invalid audit input or failed analysis."""


@dataclass(frozen=True)
class Span:
    start: int
    end: int


def canonical_json(value: object) -> str:
    return json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n"


def sanitize_rust(source: str) -> str:
    """Blank comments and literals while retaining offsets and newlines."""
    chars = list(source)

    def blank(start: int, end: int) -> None:
        for index in range(start, end):
            if chars[index] != "\n":
                chars[index] = " "

    index = 0
    length = len(source)
    while index < length:
        if source.startswith("//", index):
            end = source.find("\n", index)
            end = length if end < 0 else end
            blank(index, end)
            index = end
            continue
        if source.startswith("/*", index):
            depth = 1
            end = index + 2
            while end < length and depth:
                if source.startswith("/*", end):
                    depth += 1
                    end += 2
                elif source.startswith("*/", end):
                    depth -= 1
                    end += 2
                else:
                    end += 1
            blank(index, end)
            index = end
            continue

        raw = re.match(r"(?:br|r)(#{0,255})\"", source[index:])
        if raw:
            hashes = raw.group(1)
            closing = '"' + hashes
            content_start = index + raw.end()
            found = source.find(closing, content_start)
            end = length if found < 0 else found + len(closing)
            blank(index, end)
            index = end
            continue

        quote_start = index
        if source.startswith('b"', index):
            index += 1
        if source[index:index + 1] == '"':
            end = index + 1
            while end < length:
                if source[end] == "\\":
                    end += 2
                    continue
                end += 1
                if source[end - 1] == '"':
                    break
            blank(quote_start, min(end, length))
            index = min(end, length)
            continue

        char_start = index
        if source.startswith("b'", index):
            index += 1
        if source[index:index + 1] == "'":
            # A lifetime has no closing quote immediately after one token.
            match = re.match(r"'[A-Za-z_][A-Za-z0-9_]*", source[index:])
            if match and source[index + len(match.group(0)):index + len(match.group(0)) + 1] != "'":
                index += len(match.group(0))
                continue
            end = index + 1
            while end < length:
                if source[end] == "\\":
                    end += 2
                    continue
                end += 1
                if source[end - 1] == "'":
                    break
            blank(char_start, min(end, length))
            index = min(end, length)
            continue
        index += 1
    return "".join(chars)


def matching_delimiter(text: str, start: int, opening: str, closing: str) -> int:
    depth = 0
    for index in range(start, len(text)):
        char = text[index]
        if char == opening:
            depth += 1
        elif char == closing:
            depth -= 1
            if depth == 0:
                return index
    raise AuditError(f"unbalanced {opening}{closing} delimiter at byte {start}")


CFG_TOKEN = re.compile(
    r'\s*(?:(?P<ident>[A-Za-z_][A-Za-z0-9_-]*)|(?P<string>"(?:\\.|[^"\\])*")|(?P<punct>[(),=]))'
)
ITEM_BLOCK_HEADER = re.compile(
    r"^\s*(?:(?:pub(?:\s*\([^)]*\))?|unsafe|async|const|default)\s+)*"
    r"(?:(?:extern(?:\s+\"[^\"]*\")?)\s+)?"
    r"(?:(?:fn|mod|impl|trait|enum|struct|union)\b|macro_rules\s*!)"
)
EXTERN_BLOCK_HEADER = re.compile(
    r"^\s*(?:(?:pub(?:\s*\([^)]*\))?|unsafe)\s+)*extern(?:\s+\"[^\"]*\")?\s*$"
)


class CfgParser:
    def __init__(self, text: str):
        self.tokens: list[str] = []
        offset = 0
        while offset < len(text):
            match = CFG_TOKEN.match(text, offset)
            if not match:
                if text[offset:].strip():
                    raise AuditError(f"unsupported cfg syntax: {text!r}")
                break
            self.tokens.append(match.group("ident") or match.group("string") or match.group("punct"))
            offset = match.end()
        self.index = 0

    def peek(self) -> str | None:
        return self.tokens[self.index] if self.index < len(self.tokens) else None

    def take(self) -> str:
        token = self.peek()
        if token is None:
            raise AuditError("unexpected end of cfg expression")
        self.index += 1
        return token

    def parse(self) -> bool | None:
        value = self.expression()
        if self.peek() is not None:
            raise AuditError(f"trailing cfg token {self.peek()!r}")
        return value

    def expression(self) -> bool | None:
        name = self.take()
        if not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_-]*", name):
            raise AuditError(f"expected cfg predicate, found {name!r}")
        if self.peek() == "=":
            self.take()
            self.take()
            return None
        if self.peek() != "(":
            return False if name == "test" else None

        self.take()
        values: list[bool | None] = []
        if self.peek() != ")":
            while True:
                values.append(self.expression())
                if self.peek() != ",":
                    break
                self.take()
                if self.peek() == ")":
                    break
        if self.take() != ")":
            raise AuditError("cfg group is not closed")

        if name == "all":
            if any(value is False for value in values):
                return False
            return True if all(value is True for value in values) else None
        if name == "any":
            if any(value is True for value in values):
                return True
            return False if all(value is False for value in values) else None
        if name == "not" and len(values) == 1:
            value = values[0]
            return None if value is None else not value
        return None


def cfg_value(attribute_body: str) -> bool | None:
    match = re.fullmatch(r"\s*cfg\s*\((.*)\)\s*", attribute_body, re.DOTALL)
    if not match:
        return None
    return CfgParser(match.group(1)).parse()


def skip_space(text: str, index: int) -> int:
    while index < len(text) and text[index].isspace():
        index += 1
    return index


def item_end(clean: str, index: int) -> int:
    """Find the end of an attributed Rust item in sanitized source."""
    index = skip_space(clean, index)
    while clean.startswith("#[", index):
        close = matching_delimiter(clean, index + 1, "[", "]")
        index = skip_space(clean, close + 1)

    parens = brackets = braces = angles = 0
    cursor = index
    while cursor < len(clean):
        char = clean[cursor]
        if char == "(":
            parens += 1
        elif char == ")":
            parens = max(0, parens - 1)
        elif char == "[":
            brackets += 1
        elif char == "]":
            brackets = max(0, brackets - 1)
        elif char == "<" and parens == 0 and brackets == 0 and braces == 0:
            angles += 1
        elif char == ">" and angles:
            angles -= 1
        elif char == "{" and parens == 0 and brackets == 0 and angles == 0:
            header = clean[index:cursor]
            if ITEM_BLOCK_HEADER.search(header) or EXTERN_BLOCK_HEADER.fullmatch(header):
                return matching_delimiter(clean, cursor, "{", "}") + 1
            braces += 1
        elif char == "}" and braces:
            braces -= 1
        elif char == ";" and parens == 0 and brackets == 0 and braces == 0:
            return cursor + 1
        elif char == "," and parens == 0 and brackets == 0 and braces == 0 and angles == 0:
            return cursor + 1
        cursor += 1
    raise AuditError(f"could not find attributed item after byte {index}")


def merge_spans(spans: list[Span]) -> list[Span]:
    merged: list[Span] = []
    for span in sorted(spans, key=lambda item: (item.start, item.end)):
        if merged and span.start <= merged[-1].end:
            merged[-1] = Span(merged[-1].start, max(merged[-1].end, span.end))
        else:
            merged.append(span)
    return merged


def test_only_spans(source: str) -> list[Span]:
    clean = sanitize_rust(source)
    spans: list[Span] = []
    cursor = 0
    while cursor < len(clean):
        found = clean.find("#[", cursor)
        if found < 0:
            break
        group_start = found
        bodies: list[str] = []
        after = found
        while clean.startswith("#[", after):
            close = matching_delimiter(clean, after + 1, "[", "]")
            bodies.append(source[after + 2:close])
            after = skip_space(clean, close + 1)
        if any(cfg_value(body) is False for body in bodies):
            spans.append(Span(group_start, item_end(clean, after)))
        cursor = after
    return merge_spans(spans)


def span_lines(source: str, spans: list[Span]) -> set[int]:
    lines: set[int] = set()
    for span in spans:
        start = source.count("\n", 0, span.start) + 1
        end = source.count("\n", 0, max(span.start, span.end - 1)) + 1
        lines.update(range(start, end + 1))
    return lines


OUT_OF_LINE_MODULE = re.compile(
    r"(?:(?:pub(?:\s*\([^)]*\))?|unsafe)\s+)*mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;"
)


def out_of_line_modules(source: str, spans: list[Span] | None = None) -> list[str]:
    clean = sanitize_rust(source)
    regions = spans if spans is not None else [Span(0, len(clean))]
    names: list[str] = []
    for region in regions:
        names.extend(match.group(1) for match in OUT_OF_LINE_MODULE.finditer(clean, region.start, region.end))
    return names


def resolve_module_path(parent: Path, name: str, sources: dict[Path, str]) -> Path:
    if parent.name in {"lib.rs", "main.rs", "mod.rs"}:
        base = parent.parent
    else:
        base = parent.parent / parent.stem
    candidates = (base / f"{name}.rs", base / name / "mod.rs")
    matches = [candidate for candidate in candidates if candidate in sources]
    if len(matches) != 1:
        raise AuditError(
            f"out-of-line module {name!r} from {parent} must resolve to exactly one ordinary Rust module"
        )
    return matches[0]


def inherited_test_files(
    sources: dict[Path, str],
    production_paths: list[Path],
    external_test_paths: list[Path] | None = None,
) -> set[Path]:
    production = set(production_paths)
    external_tests = set(external_test_paths or [])
    queued: list[Path] = []
    for parent in production_paths:
        spans = test_only_spans(sources[parent])
        for name in out_of_line_modules(sources[parent], spans):
            queued.append(resolve_module_path(parent, name, sources))

    inherited: set[Path] = set()
    visited: set[Path] = set()
    while queued:
        path = queued.pop()
        if path in visited:
            continue
        visited.add(path)
        if path not in production and path not in external_tests:
            raise AuditError(f"test-only module resolved outside authored production paths: {path}")
        if path in production:
            inherited.add(path)
        for name in out_of_line_modules(sources[path]):
            queued.append(resolve_module_path(path, name, sources))
    return inherited


def working_tree_sources(root: Path) -> dict[Path, str]:
    return {
        path.relative_to(root): path.read_text(encoding="utf-8")
        for path in sorted((root / "crates").glob("*/**/*.rs"))
    }


def git_tree_sources(root: Path, revision: str) -> dict[Path, str]:
    archived = subprocess.run(
        ["git", "archive", "--format=tar", revision, "--", "crates"],
        cwd=root,
        capture_output=True,
        check=True,
    ).stdout
    sources: dict[Path, str] = {}
    with tarfile.open(fileobj=io.BytesIO(archived), mode="r:") as tree:
        for member in sorted(tree.getmembers(), key=lambda item: item.name):
            if not member.isfile() or not member.name.endswith(".rs"):
                continue
            handle = tree.extractfile(member)
            if handle is None:
                raise AuditError(f"could not read archived Rust source: {member.name}")
            sources[Path(member.name)] = handle.read().decode("utf-8")
    return sources


def classify_sources(
    sources: dict[Path, str], config: dict[str, object]
) -> tuple[list[Path], list[Path], list[Path]]:
    production: list[Path] = []
    external_tests: list[Path] = []
    generated: list[Path] = []
    exclusions = {
        Path(item["path"]): item["marker"]
        for item in config.get("generated_exclusions", [])
    }
    observed_exclusions: set[Path] = set()
    for path, source in sorted(sources.items()):
        parts = path.parts
        if path in exclusions:
            marker = exclusions[path]
            header = "\n".join(source.splitlines()[:10])
            if marker not in header:
                raise AuditError(f"generated exclusion marker is absent: {path}")
            observed_exclusions.add(path)
            generated.append(path)
        elif (
            parts[2] in {"tests", "benches"}
            or (
                parts[2] == "src"
                and len(parts) >= 4
                and (parts[3] == "tests.rs" or parts[3] == "tests")
            )
        ):
            external_tests.append(path)
        elif len(parts) >= 4 and parts[2] == "src":
            production.append(path)
    missing = sorted(set(exclusions) - observed_exclusions)
    if missing:
        raise AuditError(f"generated exclusion path is absent: {missing[0]}")
    return production, external_tests, generated


def engine_version(engine: str) -> str:
    completed = subprocess.run(
        [engine, "--version"], capture_output=True, text=True, check=False, timeout=10
    )
    if completed.returncode != 0:
        raise AuditError(f"{engine} --version failed: {completed.stderr.strip()}")
    return completed.stdout.strip()


def analyze_file(root: Path, path: Path, engine: str) -> dict[str, object]:
    completed = subprocess.run(
        [engine, "-p", str(path), "-m", "-O", "json"],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
        timeout=30,
    )
    if completed.returncode != 0:
        raise AuditError(f"analysis failed for {path}: {completed.stderr.strip()}")
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise AuditError(f"invalid analyzer JSON for {path}: {error}") from error


def function_spaces(space: dict[str, object]) -> Iterator[dict[str, object]]:
    if space.get("kind") == "function":
        yield space
    for child in space.get("spaces", []):
        yield from function_spaces(child)


def function_record(path: Path, item: dict[str, object], population: str) -> dict[str, object]:
    metrics = item["metrics"]
    loc = metrics["loc"]
    return {
        "cognitive": int(metrics["cognitive"]["sum"]),
        "cyclomatic": int(metrics["cyclomatic"]["sum"]),
        "end_line": int(item["end_line"]),
        "name": item["name"],
        "path": path.as_posix(),
        "population": population,
        "sloc": int(loc["sloc"]),
        "start_line": int(item["start_line"]),
    }


def load_config(root: Path) -> dict[str, object]:
    with (root / CONFIG_PATH).open("rb") as handle:
        config = tomllib.load(handle)
    expected_config_keys = {
        "schema_version",
        "contract_version",
        "engine",
        "engine_version",
        "baseline_revision",
        "baseline_source_fingerprint_sha256",
        "baseline_report_sha256",
        "generated_exclusions",
        "attention",
        "hotspots",
    }
    if set(config) != expected_config_keys:
        raise AuditError("code-health config has unknown or missing fields")
    if config.get("schema_version") != EXPECTED_SCHEMA:
        raise AuditError("unsupported code-health config schema")
    attention = config.get("attention")
    expected_attention = {
        "function_sloc",
        "cognitive",
        "cyclomatic",
        "module_authored_nonblank_lines",
    }
    if not isinstance(attention, dict) or set(attention) != expected_attention:
        raise AuditError("code-health attention policy has unknown or missing fields")
    if any(type(value) is not int or value < 0 for value in attention.values()):
        raise AuditError("code-health attention values must be non-negative integers")
    for field in ("baseline_revision",):
        if not re.fullmatch(r"[0-9a-f]{40}", str(config.get(field, ""))):
            raise AuditError(f"{field} must be an exact Git SHA")
    for field in ("baseline_source_fingerprint_sha256", "baseline_report_sha256"):
        if not re.fullmatch(r"[0-9a-f]{64}", str(config.get(field, ""))):
            raise AuditError(f"{field} must be a SHA-256 digest")
    generated_config = config.get("generated_exclusions", [])
    if not isinstance(generated_config, list):
        raise AuditError("generated_exclusions must be a list")
    generated_paths: set[str] = set()
    for exclusion in generated_config:
        if not isinstance(exclusion, dict) or set(exclusion) != {"path", "marker"}:
            raise AuditError("generated exclusions require only path and marker")
        path = exclusion["path"]
        marker = exclusion["marker"]
        if (
            not isinstance(path, str)
            or not re.fullmatch(r"crates/[^/]+/(?:src|tests|benches)/.+\.rs", path)
            or path in generated_paths
            or not isinstance(marker, str)
            or not marker
            or "\n" in marker
        ):
            raise AuditError("generated exclusion must have one exact unique Rust path and marker")
        generated_paths.add(path)
    seen: set[str] = set()
    for hotspot in config.get("hotspots", []):
        if not isinstance(hotspot, dict) or set(hotspot) != {
            "id",
            "locations",
            "disposition",
            "owner",
            "evidence",
        }:
            raise AuditError("hotspot has unknown or missing fields")
        identifier = hotspot.get("id")
        if not isinstance(identifier, str) or not identifier or identifier in seen:
            raise AuditError("hotspot ids must be unique non-empty strings")
        seen.add(identifier)
        if hotspot.get("disposition") not in ALLOWED_DISPOSITIONS:
            raise AuditError(f"hotspot {identifier} has invalid disposition")
        locations = hotspot.get("locations")
        if (
            not isinstance(hotspot.get("owner"), str)
            or not hotspot["owner"]
            or not isinstance(hotspot.get("evidence"), str)
            or not hotspot["evidence"]
            or not isinstance(locations, list)
            or not locations
            or any(not isinstance(location, str) or not location for location in locations)
        ):
            raise AuditError(f"hotspot {identifier} lacks owner, evidence, or locations")
    if not seen:
        raise AuditError("at least one hotspot classification is required")
    return config


def source_fingerprint(sources: dict[Path, str], paths: list[Path]) -> str:
    digest = hashlib.sha256()
    for path in sorted(paths):
        digest.update(path.as_posix().encode())
        digest.update(b"\0")
        digest.update(sources[path].encode("utf-8"))
        digest.update(b"\0")
    return digest.hexdigest()


def resolve_revision(root: Path, requested: str | None) -> str:
    candidate = requested or "HEAD"
    resolved = subprocess.run(
        ["git", "rev-parse", "--verify", f"{candidate}^{{commit}}"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.strip()
    if not re.fullmatch(r"[0-9a-f]{40}", resolved):
        raise AuditError(f"Git did not resolve an exact commit for {candidate!r}")
    return resolved


def exact_working_revision(root: Path, requested: str | None) -> str:
    resolved = resolve_revision(root, requested)
    changed = subprocess.run(
        ["git", "diff", "--name-only", resolved, "--", "crates"],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
    )
    changed_rust = [path for path in changed.stdout.splitlines() if path.endswith(".rs")]
    untracked = subprocess.run(
        ["git", "ls-files", "--others", "--exclude-standard", "--", "crates"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    ).stdout.splitlines()
    untracked_rust = [path for path in untracked if path.endswith(".rs")]
    if changed.returncode != 0 or changed_rust or untracked_rust:
        raise AuditError(
            "authored Rust does not match the requested revision; commit it or audit an exact clean worktree"
        )
    return resolved


def source_population_evidence(
    sources: dict[Path, str], config: dict[str, object]
) -> tuple[
    list[Path],
    list[Path],
    list[Path],
    dict[Path, set[int]],
    dict[str, dict[str, int]],
]:
    production_paths, test_paths, generated_paths = classify_sources(sources, config)
    inherited_inline_paths = inherited_test_files(sources, production_paths, test_paths)
    inline_lines_by_path: dict[Path, set[int]] = {}
    counts = {
        "production": {"authored_nonblank_lines": 0, "files": 0, "functions": 0},
        "external_test": {"authored_nonblank_lines": 0, "files": 0, "functions": 0},
        "inline_test": {"authored_nonblank_lines": 0, "files": 0, "functions": 0},
    }
    for path in production_paths:
        source = sources[path]
        lines = source.splitlines()
        inline_lines = (
            set(range(1, len(lines) + 1))
            if path in inherited_inline_paths
            else span_lines(source, test_only_spans(source))
        )
        inline_lines_by_path[path] = inline_lines
        production_lines = sum(
            1
            for number, line in enumerate(lines, 1)
            if line.strip() and number not in inline_lines
        )
        test_lines = sum(
            1
            for number, line in enumerate(lines, 1)
            if line.strip() and number in inline_lines
        )
        if production_lines:
            counts["production"]["files"] += 1
            counts["production"]["authored_nonblank_lines"] += production_lines
        if test_lines:
            counts["inline_test"]["files"] += 1
            counts["inline_test"]["authored_nonblank_lines"] += test_lines
    for path in test_paths:
        counts["external_test"]["files"] += 1
        counts["external_test"]["authored_nonblank_lines"] += sum(
            1 for line in sources[path].splitlines() if line.strip()
        )
    return (
        production_paths,
        test_paths,
        generated_paths,
        inline_lines_by_path,
        counts,
    )


def build_report(
    analysis_root: Path,
    sources: dict[Path, str],
    config: dict[str, object],
    revision: str,
    engine: str,
) -> dict[str, object]:
    (
        production_paths,
        test_paths,
        generated_paths,
        inline_lines_by_path,
        counts,
    ) = source_population_evidence(sources, config)
    all_authored = production_paths + test_paths
    functions: list[dict[str, object]] = []
    modules: list[dict[str, object]] = []

    for path in production_paths:
        source = sources[path]
        inline_lines = inline_lines_by_path[path]
        lines = source.splitlines()
        production_lines = sum(1 for number, line in enumerate(lines, 1) if line.strip() and number not in inline_lines)
        if production_lines:
            modules.append({"authored_nonblank_lines": production_lines, "path": path.as_posix()})

        result = analyze_file(analysis_root, path, engine)
        for item in function_spaces(result):
            line = int(item["start_line"])
            population = "inline_test" if line in inline_lines else "production"
            counts[population]["functions"] += 1
            functions.append(function_record(path, item, population))

    for path in test_paths:
        result = analyze_file(analysis_root, path, engine)
        for item in function_spaces(result):
            counts["external_test"]["functions"] += 1
            functions.append(function_record(path, item, "external_test"))

    attention = config["attention"]
    function_signals = [
        item
        for item in functions
        if item["population"] == "production"
        and (
            item["sloc"] > attention["function_sloc"]
            or item["cognitive"] > attention["cognitive"]
            or item["cyclomatic"] > attention["cyclomatic"]
        )
    ]
    function_signals.sort(key=lambda item: (item["path"], item["start_line"], item["name"]))
    module_signals = [
        item
        for item in modules
        if item["authored_nonblank_lines"] > attention["module_authored_nonblank_lines"]
    ]
    module_signals.sort(key=lambda item: item["path"])

    return {
        "analyzer": {
            "contract_version": config["contract_version"],
            "engine": engine,
            "engine_version": config["engine_version"],
        },
        "attention": attention,
        "generated_exclusions": [path.as_posix() for path in generated_paths],
        "hotspots": config["hotspots"],
        "populations": counts,
        "revision": revision,
        "schema_version": EXPECTED_SCHEMA,
        "signals": {
            "functions": function_signals,
            "modules": module_signals,
        },
        "source_fingerprint_sha256": source_fingerprint(sources, all_authored),
    }


def checked_engine(config: dict[str, object]) -> str:
    engine = str(config["engine"])
    expected = f"{engine} {config['engine_version']}"
    actual = engine_version(engine)
    if actual != expected:
        raise AuditError(f"expected exactly {expected}, found {actual}")
    return engine


def audit(root: Path, revision: str | None) -> dict[str, object]:
    config = load_config(root)
    engine = checked_engine(config)
    resolved = exact_working_revision(root, revision)
    sources = working_tree_sources(root)
    return build_report(root, sources, config, resolved, engine)


def regenerate_baseline(root: Path) -> dict[str, object]:
    config = load_config(root)
    engine = checked_engine(config)
    revision = resolve_revision(root, str(config["baseline_revision"]))
    sources = git_tree_sources(root, revision)
    with tempfile.TemporaryDirectory(prefix="sdk-rust-code-health-") as directory:
        analysis_root = Path(directory)
        for path, source in sources.items():
            target = analysis_root / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(source, encoding="utf-8")
        return build_report(analysis_root, sources, config, revision, engine)


def require_exact_keys(value: object, expected: set[str], context: str) -> dict[str, object]:
    if not isinstance(value, dict) or set(value) != expected:
        raise AuditError(f"{context} has unknown or missing fields")
    return value


def require_nonnegative_int(value: object, context: str) -> None:
    if type(value) is not int or value < 0:
        raise AuditError(f"{context} must be a non-negative integer")


def validate_report(root: Path, path: Path, policy_only: bool = False) -> dict[str, object]:
    raw = path.read_text(encoding="utf-8")
    try:
        report = json.loads(raw)
    except json.JSONDecodeError as error:
        raise AuditError(f"invalid report JSON: {error}") from error
    if raw != canonical_json(report):
        raise AuditError("report is not canonical sorted/indented JSON")
    config = load_config(root)
    if hashlib.sha256(raw.encode("utf-8")).hexdigest() != config["baseline_report_sha256"]:
        raise AuditError("report content does not match the policy-pinned digest")
    report = require_exact_keys(
        report,
        {
            "analyzer",
            "attention",
            "generated_exclusions",
            "hotspots",
            "populations",
            "revision",
            "schema_version",
            "signals",
            "source_fingerprint_sha256",
        },
        "report",
    )
    if report.get("schema_version") != EXPECTED_SCHEMA:
        raise AuditError("report schema version mismatch")
    analyzer = require_exact_keys(
        report["analyzer"], {"contract_version", "engine", "engine_version"}, "analyzer"
    )
    for field in ("contract_version", "engine", "engine_version"):
        if analyzer.get(field) != config.get(field):
            raise AuditError(f"report analyzer {field} does not match config")
    attention = require_exact_keys(
        report["attention"],
        {"function_sloc", "cognitive", "cyclomatic", "module_authored_nonblank_lines"},
        "attention",
    )
    if attention != config.get("attention"):
        raise AuditError("report attention policy does not match config")
    for field, value in attention.items():
        require_nonnegative_int(value, f"attention {field}")
    if not re.fullmatch(r"[0-9a-f]{40}", str(report.get("revision", ""))):
        raise AuditError("report revision must be an exact 40-character Git SHA")
    if report["revision"] != config["baseline_revision"]:
        raise AuditError("report revision does not match pinned baseline policy")
    if not re.fullmatch(r"[0-9a-f]{64}", str(report.get("source_fingerprint_sha256", ""))):
        raise AuditError("report source fingerprint must be SHA-256")
    if report["source_fingerprint_sha256"] != config["baseline_source_fingerprint_sha256"]:
        raise AuditError("report source fingerprint does not match pinned baseline policy")
    if report.get("hotspots") != config.get("hotspots"):
        raise AuditError("report hotspot classifications are stale")
    populations = require_exact_keys(
        report["populations"], {"production", "external_test", "inline_test"}, "populations"
    )
    for population in ("production", "external_test", "inline_test"):
        values = require_exact_keys(
            populations[population],
            {"files", "functions", "authored_nonblank_lines"},
            f"{population} population",
        )
        for field, value in values.items():
            require_nonnegative_int(value, f"{population} {field}")
    if not isinstance(report.get("generated_exclusions"), list):
        raise AuditError("generated exclusions must be a list")
    expected_generated = [item["path"] for item in config.get("generated_exclusions", [])]
    if report["generated_exclusions"] != expected_generated:
        raise AuditError("generated exclusions do not match the exact policy allowlist")
    signals = require_exact_keys(report["signals"], {"functions", "modules"}, "signals")
    if not isinstance(signals.get("functions"), list) or not isinstance(signals.get("modules"), list):
        raise AuditError("report signals must contain function and module lists")
    for item in signals["functions"]:
        item = require_exact_keys(
            item,
            {
                "cognitive",
                "cyclomatic",
                "end_line",
                "name",
                "path",
                "population",
                "sloc",
                "start_line",
            },
            "function signal",
        )
        for field in ("cognitive", "cyclomatic", "end_line", "sloc", "start_line"):
            require_nonnegative_int(item[field], f"function signal {field}")
        if not isinstance(item["name"], str) or not isinstance(item["path"], str):
            raise AuditError("function signal name and path must be strings")
        if item["population"] != "production":
            raise AuditError("attention function signals must be production")
    for item in signals["modules"]:
        item = require_exact_keys(
            item, {"authored_nonblank_lines", "path"}, "module signal"
        )
        require_nonnegative_int(item["authored_nonblank_lines"], "module signal lines")
        if not isinstance(item["path"], str):
            raise AuditError("module signal path must be a string")

    if not policy_only:
        sources = git_tree_sources(root, report["revision"])
        (
            production_paths,
            test_paths,
            generated_paths,
            _,
            source_counts,
        ) = source_population_evidence(sources, config)
        fingerprint = source_fingerprint(sources, production_paths + test_paths)
        if fingerprint != report["source_fingerprint_sha256"]:
            raise AuditError("report fingerprint does not match its pinned Git tree")
        if [path.as_posix() for path in generated_paths] != report["generated_exclusions"]:
            raise AuditError("report generated exclusions do not match its pinned Git tree")
        for population in ("production", "external_test", "inline_test"):
            for field in ("files", "authored_nonblank_lines"):
                if source_counts[population][field] != populations[population][field]:
                    raise AuditError(
                        f"report {population} {field} does not match its pinned Git tree"
                    )
    return report


def verify_baseline(root: Path, path: Path) -> None:
    report = validate_report(root, path)
    regenerated = regenerate_baseline(root)
    if report != regenerated:
        raise AuditError("pinned baseline does not match regenerated Git-tree analysis")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path("."))
    parser.add_argument("--output", type=Path)
    parser.add_argument("--revision")
    parser.add_argument("--policy-only", action="store_true")
    action = parser.add_mutually_exclusive_group()
    action.add_argument("--check-report", type=Path)
    action.add_argument("--verify-baseline", action="store_true")
    args = parser.parse_args()
    root = args.root.resolve()
    try:
        if args.check_report:
            validate_report(root, args.check_report, policy_only=args.policy_only)
            print(f"code-health: report contract passed: {args.check_report}")
            return 0
        if args.policy_only:
            raise AuditError("--policy-only requires --check-report")
        if args.verify_baseline:
            baseline = root / "docs/architecture/code-health-baseline.json"
            verify_baseline(root, baseline)
            print(f"code-health: pinned baseline regenerated exactly: {baseline}")
            return 0
        report = audit(root, args.revision)
        rendered = canonical_json(report)
        if args.output:
            args.output.write_text(rendered, encoding="utf-8")
            print(f"code-health: wrote {args.output}")
        else:
            sys.stdout.write(rendered)
        return 0
    except (AuditError, OSError, subprocess.SubprocessError) as error:
        print(f"code-health: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
