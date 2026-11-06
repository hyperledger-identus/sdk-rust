#!/usr/bin/env python3
"""Verify the pinned PRISM protobuf snapshot and private Rust projection."""

from __future__ import annotations

import hashlib
import sys
from pathlib import Path


SOURCE_SHA256 = {
    "prism.proto": "412163eef867771e78418e25dd8a761c7a8ef2c390b190e82b413d0a0c7b8fdd",
    "prism-ssi.proto": "dd365637988fdf61ec80038ba6f6ab1dfafb6556288fa368e2d0607fdbb78609",
    "prism-storage.proto": "958e1c8d13caa06b9a27539a1b37995b96c55f7d3c4f5d7d8db6de5b20a1053e",
    "prism-version.proto": "481ef353a57345a06afca7928de94cd5fefa522d46c1000e66a4bceb0de31f4d",
}
PRIVATE_SCHEMA_SHA256 = "1afac92b85913c062e67ed395142336a5c7231fcceb73633297817b8c56a3db8"


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> int:
    root = Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else Path.cwd().resolve()
    proto_root = root / "crates" / "prism-did" / "proto"
    failures: list[str] = []

    for name, expected in SOURCE_SHA256.items():
        path = proto_root / name
        if not path.is_file():
            failures.append(f"missing pinned source: {path.relative_to(root)}")
        elif digest(path) != expected:
            failures.append(f"pinned source digest changed: {path.relative_to(root)}")

    private_schema = root / "crates" / "prism-did" / "src" / "proto.rs"
    if not private_schema.is_file():
        failures.append(f"missing private schema: {private_schema.relative_to(root)}")
    elif digest(private_schema) != PRIVATE_SCHEMA_SHA256:
        failures.append(
            "private schema changed without updating the reviewed source-to-Rust projection receipt"
        )

    if failures:
        for failure in failures:
            print(f"did-prism-proto: {failure}", file=sys.stderr)
        return 1

    print("did-prism-proto: 4 pinned sources and private schema passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
