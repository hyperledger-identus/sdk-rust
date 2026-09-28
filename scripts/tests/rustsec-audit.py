#!/usr/bin/env python3
"""Mutation tests for the RustSec evidence classifier and compatibility probe."""

from __future__ import annotations

import json
import os
import stat
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CHECKER = ROOT / "scripts" / "check-rustsec-audit.py"


def report(count: int, advisory_id: str | None = None) -> bytes:
    entries = []
    if advisory_id is not None:
        entries.append({"advisory": {"id": advisory_id}})
    return json.dumps(
        {
            "database": {"advisory-count": 1},
            "vulnerabilities": {
                "found": count > 0,
                "count": count,
                "list": entries,
            },
            "warnings": {},
        }
    ).encode()


def classify(raw: bytes, command_exit: int) -> tuple[subprocess.CompletedProcess[bytes], dict]:
    with tempfile.TemporaryDirectory() as temporary:
        output = Path(temporary) / "evidence.json"
        result = subprocess.run(
            [
                sys.executable,
                str(CHECKER),
                "classify",
                "--command-exit",
                str(command_exit),
                "--expected-tool-version",
                "0.22.2",
                "--advisory-db-revision",
                "fixture-revision",
                "--output",
                str(output),
            ],
            input=raw,
            check=False,
            capture_output=True,
        )
        return result, json.loads(output.read_text(encoding="utf-8"))


def fake_audit(path: Path, *, version: str, probe_exit: int) -> Path:
    executable = path / "cargo-audit"
    executable.write_text(
        "#!/usr/bin/env python3\n"
        "import json, sys\n"
        f"version = {version!r}\n"
        f"probe_exit = {probe_exit!r}\n"
        "if '--version' in sys.argv:\n"
        "    print(f'cargo-audit-audit {version}')\n"
        "    raise SystemExit(0)\n"
        "required = {'--no-fetch', '--no-yanked', '--format', 'json'}\n"
        "if not required.issubset(set(sys.argv)):\n"
        "    raise SystemExit(9)\n"
        "print(json.dumps({'database': {'advisory-count': 1}, 'vulnerabilities': {'found': False, 'count': 0, 'list': []}, 'warnings': {}}))\n"
        "raise SystemExit(probe_exit)\n",
        encoding="utf-8",
    )
    executable.chmod(executable.stat().st_mode | stat.S_IXUSR)
    return executable


def main() -> int:
    clean_result, clean = classify(report(0), 0)
    assert clean_result.returncode == 0
    assert clean["advisories"]["status"] == "success"
    assert clean["yanked"] == {
        "status": "unavailable",
        "reason": "registry-index-not-provided",
    }

    vulnerable_result, vulnerable = classify(
        report(1, "RUSTSEC-2099-0001"), 1
    )
    assert vulnerable_result.returncode == 1
    assert vulnerable["advisories"]["status"] == "vulnerability"
    assert vulnerable["advisories"]["advisoryIds"] == ["RUSTSEC-2099-0001"]

    malformed_result, malformed = classify(b"not-json", 1)
    assert malformed_result.returncode == 1
    assert malformed["advisories"]["status"] == "incompatible-tool"

    inconsistent_result, inconsistent = classify(
        json.dumps(
            {
                "database": {"advisory-count": 0},
                "vulnerabilities": {"found": False, "count": 0, "list": []},
                "warnings": {},
            }
        ).encode(),
        0,
    )
    assert inconsistent_result.returncode == 1
    assert inconsistent["advisories"]["status"] == "incompatible-tool"

    invalid_count_result, invalid_count = classify(
        json.dumps(
            {
                "database": {"advisory-count": 1},
                "vulnerabilities": {
                    "found": False,
                    "count": "0",
                    "list": [],
                },
                "warnings": {},
            }
        ).encode(),
        0,
    )
    assert invalid_count_result.returncode == 1
    assert invalid_count["advisories"]["status"] == "incompatible-tool"

    with tempfile.TemporaryDirectory() as temporary:
        root = Path(temporary)
        database = root / "db"
        database.mkdir()
        lockfile = root / "Cargo.lock"
        lockfile.write_text("version = 4\n", encoding="utf-8")

        compatible = fake_audit(root, version="0.22.2", probe_exit=0)
        result = subprocess.run(
            [
                sys.executable,
                str(CHECKER),
                "probe",
                "--cargo-audit",
                str(compatible),
                "--expected-tool-version",
                "0.22.2",
                "--advisory-db",
                str(database),
                "--lockfile",
                str(lockfile),
            ],
            check=False,
            capture_output=True,
            env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"},
        )
        assert result.returncode == 0

        incompatible = fake_audit(root, version="0.20.1", probe_exit=0)
        result = subprocess.run(
            [
                sys.executable,
                str(CHECKER),
                "probe",
                "--cargo-audit",
                str(incompatible),
                "--expected-tool-version",
                "0.22.2",
                "--advisory-db",
                str(database),
                "--lockfile",
                str(lockfile),
            ],
            check=False,
            capture_output=True,
        )
        assert result.returncode == 1
        assert b"incompatible-tool" in result.stderr

        parser_failure = fake_audit(root, version="0.22.2", probe_exit=7)
        result = subprocess.run(
            [
                sys.executable,
                str(CHECKER),
                "probe",
                "--cargo-audit",
                str(parser_failure),
                "--expected-tool-version",
                "0.22.2",
                "--advisory-db",
                str(database),
                "--lockfile",
                str(lockfile),
            ],
            check=False,
            capture_output=True,
        )
        assert result.returncode == 1
        assert b"incompatible-tool" in result.stderr

    print("rustsec-audit: mutation contract passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
