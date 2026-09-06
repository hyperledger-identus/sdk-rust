#!/usr/bin/env python3

from __future__ import annotations

import argparse
import hashlib
import re
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path


MAX_FILE_BYTES = 1024 * 1024
MAX_ACTIVE_CHANGES = 256
MAX_CAPABILITIES_PER_CHANGE = 256
MAX_REQUIREMENTS = 1024
CHANGE_NAME = re.compile(r"[a-z0-9][a-z0-9-]*\Z")
HASH = re.compile(r"[0-9a-f]{64}\Z")
SECTION = re.compile(r"^##\s+(.+?)\s*$")
REQUIREMENT = re.compile(r"^###\s*Requirement:\s*(.+?)\s*$", re.IGNORECASE)
RENAME_FROM = re.compile(
    r"^\s*-?\s*FROM:\s*`?###\s*Requirement:\s*(.+?)`?\s*$", re.IGNORECASE
)
RENAME_TO = re.compile(
    r"^\s*-?\s*TO:\s*`?###\s*Requirement:\s*(.+?)`?\s*$", re.IGNORECASE
)
INTENT_KEYS = {"capability", "requirement", "canonical_sha256", "reason"}


@dataclass(frozen=True)
class RequirementBlock:
    name: str
    raw: str


@dataclass(frozen=True)
class ReplacementIntent:
    capability: str
    requirement: str
    canonical_sha256: str
    reason: str

    @property
    def key(self) -> tuple[str, str]:
        return (normalize_name(self.capability), normalize_name(self.requirement))


def normalize_name(value: str) -> str:
    return value.strip().casefold()


def normalized_lines(value: str) -> list[str]:
    lines = value.replace("\r\n", "\n").replace("\r", "\n").split("\n")
    lines = [line.rstrip() for line in lines]
    while lines and not lines[0]:
        lines.pop(0)
    while lines and not lines[-1]:
        lines.pop()
    return lines


def block_hash(block: RequirementBlock) -> str:
    normalized = "\n".join(normalized_lines(block.raw))
    return hashlib.sha256(normalized.encode("utf-8")).hexdigest()


def has_symlink_component(path: Path, root: Path) -> bool:
    current = root
    for component in path.relative_to(root).parts:
        current /= component
        if current.is_symlink():
            return True
    return False


def read_bounded_text(path: Path, root: Path, failures: list[str]) -> str | None:
    try:
        relative = path.relative_to(root)
    except ValueError:
        failures.append(f"path escapes repository root: {path}")
        return None
    if has_symlink_component(path, root):
        failures.append(f"symlinked OpenSpec input is forbidden: {relative}")
        return None
    try:
        size = path.stat().st_size
    except OSError as error:
        failures.append(f"cannot stat {relative}: {error}")
        return None
    if size > MAX_FILE_BYTES:
        failures.append(f"OpenSpec input exceeds {MAX_FILE_BYTES} bytes: {relative}")
        return None
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError) as error:
        failures.append(f"cannot read UTF-8 OpenSpec input {relative}: {error}")
        return None


def top_level_sections(text: str, label: str, failures: list[str]) -> dict[str, str]:
    lines = text.replace("\r\n", "\n").replace("\r", "\n").split("\n")
    starts: list[tuple[str, int]] = []
    for index, line in enumerate(lines):
        match = SECTION.match(line)
        if match:
            starts.append((match.group(1).strip(), index))
    result: dict[str, str] = {}
    for index, (name, start) in enumerate(starts):
        key = normalize_name(name)
        if key in result:
            failures.append(f"duplicate level-two section in {label}: {name}")
            continue
        end = starts[index + 1][1] if index + 1 < len(starts) else len(lines)
        result[key] = "\n".join(lines[start + 1 : end])
    return result


def parse_requirements(text: str, label: str, failures: list[str]) -> list[RequirementBlock]:
    lines = text.replace("\r\n", "\n").replace("\r", "\n").split("\n")
    starts: list[tuple[str, int]] = []
    for index, line in enumerate(lines):
        match = REQUIREMENT.match(line)
        if match:
            starts.append((match.group(1).strip(), index))
    if len(starts) > MAX_REQUIREMENTS:
        failures.append(f"too many requirements in {label}: {len(starts)}")
        return []
    blocks: list[RequirementBlock] = []
    seen: set[str] = set()
    for index, (name, start) in enumerate(starts):
        key = normalize_name(name)
        if key in seen:
            failures.append(f"duplicate requirement in {label}: {name}")
            continue
        seen.add(key)
        end = starts[index + 1][1] if index + 1 < len(starts) else len(lines)
        blocks.append(RequirementBlock(name=name, raw="\n".join(lines[start:end]).rstrip()))
    return blocks


def parse_renames(text: str, label: str, failures: list[str]) -> dict[str, str]:
    result: dict[str, str] = {}
    source: str | None = None
    for line in text.replace("\r\n", "\n").replace("\r", "\n").split("\n"):
        from_match = RENAME_FROM.match(line)
        to_match = RENAME_TO.match(line)
        if from_match:
            if source is not None:
                failures.append(f"unpaired rename source in {label}: {source}")
            source = from_match.group(1).strip()
        elif to_match:
            destination = to_match.group(1).strip()
            if source is None:
                failures.append(f"rename destination has no source in {label}: {destination}")
                continue
            key = normalize_name(destination)
            if key in result:
                failures.append(f"duplicate rename destination in {label}: {destination}")
            else:
                result[key] = source
            source = None
    if source is not None:
        failures.append(f"unpaired rename source in {label}: {source}")
    return result


def load_intents(
    change_dir: Path, root: Path, failures: list[str]
) -> dict[tuple[str, str], ReplacementIntent]:
    path = change_dir / "archive-intent.toml"
    if not path.exists() and not path.is_symlink():
        return {}
    text = read_bounded_text(path, root, failures)
    if text is None:
        return {}
    try:
        document = tomllib.loads(text)
    except tomllib.TOMLDecodeError as error:
        failures.append(f"malformed archive intent {path.relative_to(root)}: {error}")
        return {}
    if set(document) != {"modified_requirement"}:
        failures.append(
            f"archive intent has unknown or missing top-level keys: {path.relative_to(root)}"
        )
        return {}
    entries = document.get("modified_requirement")
    if not isinstance(entries, list):
        failures.append(f"modified_requirement must be an array: {path.relative_to(root)}")
        return {}
    intents: dict[tuple[str, str], ReplacementIntent] = {}
    for index, entry in enumerate(entries):
        label = f"{path.relative_to(root)} modified_requirement[{index}]"
        if not isinstance(entry, dict) or set(entry) != INTENT_KEYS:
            failures.append(f"{label} must contain exactly {','.join(sorted(INTENT_KEYS))}")
            continue
        if not all(isinstance(entry[key], str) for key in INTENT_KEYS):
            failures.append(f"{label} fields must be strings")
            continue
        intent = ReplacementIntent(
            capability=entry["capability"].strip(),
            requirement=entry["requirement"].strip(),
            canonical_sha256=entry["canonical_sha256"],
            reason=entry["reason"].strip(),
        )
        if not intent.capability or not intent.requirement or not intent.reason:
            failures.append(f"{label} capability, requirement and reason must be nonempty")
            continue
        if not HASH.fullmatch(intent.canonical_sha256):
            failures.append(f"{label} canonical_sha256 must be 64 lowercase hexadecimal characters")
            continue
        if intent.key in intents:
            failures.append(
                f"duplicate archive intent: {intent.capability} / {intent.requirement}"
            )
            continue
        intents[intent.key] = intent
    return intents


def is_additive_replacement(canonical: RequirementBlock, candidate: RequirementBlock) -> bool:
    old = [line for line in normalized_lines(canonical.raw)[1:] if line]
    new = [line for line in normalized_lines(candidate.raw)[1:] if line]
    cursor = iter(new)
    return all(any(candidate_line == line for candidate_line in cursor) for line in old)


def check_change(root: Path, change_dir: Path, failures: list[str]) -> None:
    intents = load_intents(change_dir, root, failures)
    used_intents: set[tuple[str, str]] = set()
    specs_dir = change_dir / "specs"
    if not specs_dir.is_dir() or specs_dir.is_symlink():
        failures.append(f"active change has no regular specs directory: {change_dir.name}")
        return
    capability_dirs: list[Path] = []
    for path in sorted(specs_dir.iterdir()):
        if path.is_symlink():
            failures.append(
                f"symlinked capability input is forbidden: {path.relative_to(root)}"
            )
        elif path.is_dir():
            capability_dirs.append(path)
    if len(capability_dirs) > MAX_CAPABILITIES_PER_CHANGE:
        failures.append(
            f"too many capabilities in {change_dir.name}: {len(capability_dirs)}"
        )
        return
    for capability_dir in capability_dirs:
        capability = capability_dir.name
        delta_path = capability_dir / "spec.md"
        if not delta_path.exists() and not delta_path.is_symlink():
            continue
        delta_text = read_bounded_text(delta_path, root, failures)
        if delta_text is None:
            continue
        label = str(delta_path.relative_to(root))
        sections = top_level_sections(delta_text, label, failures)
        modified = parse_requirements(
            sections.get("modified requirements", ""), f"{label} MODIFIED", failures
        )
        renames = parse_renames(
            sections.get("renamed requirements", ""), f"{label} RENAMED", failures
        )
        if not modified:
            continue
        canonical_path = root / "openspec/specs" / capability / "spec.md"
        if not canonical_path.exists() and not canonical_path.is_symlink():
            failures.append(f"MODIFIED capability has no canonical spec: {capability}")
            continue
        canonical_text = read_bounded_text(canonical_path, root, failures)
        if canonical_text is None:
            continue
        canonical_sections = top_level_sections(
            canonical_text, str(canonical_path.relative_to(root)), failures
        )
        canonical_blocks = {
            normalize_name(block.name): block
            for block in parse_requirements(
                canonical_sections.get("requirements", ""),
                f"{canonical_path.relative_to(root)} Requirements",
                failures,
            )
        }
        for candidate in modified:
            candidate_key = normalize_name(candidate.name)
            source_name = renames.get(candidate_key, candidate.name)
            canonical = canonical_blocks.get(normalize_name(source_name))
            if canonical is None:
                failures.append(
                    f"canonical requirement not found for {capability} / {candidate.name}"
                )
                continue
            if is_additive_replacement(canonical, candidate):
                continue
            key = (normalize_name(capability), candidate_key)
            actual_hash = block_hash(canonical)
            intent = intents.get(key)
            if intent is None:
                failures.append(
                    "lossy MODIFIED requirement lacks exact intent: "
                    f"{capability} / {candidate.name}; canonical_sha256={actual_hash}"
                )
                continue
            used_intents.add(key)
            if intent.canonical_sha256 != actual_hash:
                failures.append(
                    f"stale canonical hash for {capability} / {candidate.name}: expected {actual_hash}"
                )
    for key, intent in sorted(intents.items()):
        if key not in used_intents:
            failures.append(
                f"unused archive intent: {intent.capability} / {intent.requirement}"
            )


def active_changes(root: Path, selected: str | None, failures: list[str]) -> list[Path]:
    changes_root = root / "openspec/changes"
    if selected is not None:
        if not CHANGE_NAME.fullmatch(selected):
            failures.append(f"invalid active change name: {selected}")
            return []
        path = changes_root / selected
        if not path.is_dir() or path.is_symlink():
            failures.append(f"active change not found as a regular directory: {selected}")
            return []
        return [path]
    if not changes_root.is_dir() or changes_root.is_symlink():
        failures.append("missing regular openspec/changes directory")
        return []
    entries = sorted(
        path
        for path in changes_root.iterdir()
        if path.name != "archive" and not path.name.startswith(".")
    )
    changes: list[Path] = []
    for path in entries:
        if path.is_symlink():
            failures.append(f"symlinked active change is forbidden: {path.relative_to(root)}")
        elif path.is_dir():
            changes.append(path)
    if len(changes) > MAX_ACTIVE_CHANGES:
        failures.append(f"too many active changes: {len(changes)}")
        return []
    return changes


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Fail closed on lossy OpenSpec MODIFIED requirement archives."
    )
    parser.add_argument("root", nargs="?", default=".", help="repository root")
    parser.add_argument("--change", help="check one active change")
    return parser.parse_args()


def main() -> int:
    arguments = parse_arguments()
    root = Path(arguments.root).resolve()
    failures: list[str] = []
    changes = active_changes(root, arguments.change, failures)
    for change in changes:
        check_change(root, change, failures)
    if failures:
        for failure in failures:
            print(f"openspec-archive-preservation: {failure}", file=sys.stderr)
        print(
            f"openspec-archive-preservation: {len(failures)} failure(s)", file=sys.stderr
        )
        return 1
    print(f"openspec-archive-preservation: {len(changes)} active change(s) passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
