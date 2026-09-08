#!/usr/bin/env python3
"""Validate the measurement-only crypto benchmark artifact."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

SHA = re.compile(r"^[0-9a-f]{40}$")
OPERATIONS = {
    "sha-256", "sha-512", "ed25519-sign", "ed25519-verify",
    "x25519-agreement", "secp256k1-sign", "secp256k1-verify",
    "p256-sign", "p256-verify", "bip39-seed",
    "secp256k1-bip32-hardened-child", "cardano-v2-private-child",
    "cardano-v2-public-child",
}
TOP_LEVEL_KEYS = {
    "schema_version", "status", "apollo_comparison", "revision", "compiler",
    "os", "arch", "cpu", "environment", "feature_profile", "samples",
    "warmup_batches", "operations",
}
OPERATION_KEYS = {
    "name", "iterations_per_sample", "p50_ns", "p95_ns", "min_ns", "max_ns",
}


def validate(data: Any) -> list[str]:
    errors: list[str] = []
    if not isinstance(data, dict):
        return ["artifact must be a JSON object"]
    if set(data) != TOP_LEVEL_KEYS:
        errors.append("top-level fields differ from schema")
    if data.get("schema_version") != 1:
        errors.append("schema_version must be 1")
    if data.get("status") != "measurement-only":
        errors.append("status must be measurement-only")
    if data.get("apollo_comparison") != "unavailable":
        errors.append("apollo_comparison must be unavailable")
    if not SHA.fullmatch(str(data.get("revision", ""))):
        errors.append("revision must be a full lowercase Git SHA")
    for field in ("compiler", "os", "arch", "cpu", "environment"):
        value = data.get(field)
        if not isinstance(value, str) or not value.strip() or len(value) > 512:
            errors.append(f"{field} must be non-empty and at most 512 characters")
    if data.get("feature_profile") != "default":
        errors.append("feature_profile must be default")
    samples = data.get("samples")
    if not isinstance(samples, int) or isinstance(samples, bool) or not 20 <= samples <= 10_000:
        errors.append("samples must be an integer from 20 through 10000")
    if data.get("warmup_batches") != 1:
        errors.append("warmup_batches must be 1")

    operations = data.get("operations")
    if not isinstance(operations, list):
        return errors + ["operations must be an array"]
    names: list[str] = []
    for index, operation in enumerate(operations):
        label = f"operation[{index}]"
        if not isinstance(operation, dict):
            errors.append(f"{label} must be an object")
            continue
        if set(operation) != OPERATION_KEYS:
            errors.append(f"{label} fields differ from schema")
        name = operation.get("name")
        if isinstance(name, str):
            names.append(name)
        for field in ("iterations_per_sample", "p50_ns", "p95_ns", "min_ns", "max_ns"):
            value = operation.get(field)
            minimum = 1 if field == "iterations_per_sample" else 0
            if not isinstance(value, int) or isinstance(value, bool) or value < minimum:
                errors.append(f"{label}.{field} must be an integer of at least {minimum}")
        minimum = operation.get("min_ns")
        p50 = operation.get("p50_ns")
        p95 = operation.get("p95_ns")
        maximum = operation.get("max_ns")
        if all(isinstance(value, int) and not isinstance(value, bool) for value in (minimum, p50, p95, maximum)):
            if not minimum <= p50 <= p95 <= maximum:
                errors.append(f"{label} timing order must be min <= p50 <= p95 <= max")
    if set(names) != OPERATIONS or len(names) != len(OPERATIONS):
        errors.append("operation inventory differs from the canonical matrix")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("artifact", type=Path)
    args = parser.parse_args()
    try:
        data = json.loads(args.artifact.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        print(f"crypto-benchmark: cannot load artifact: {error}", file=sys.stderr)
        return 1
    errors = validate(data)
    if errors:
        for error in errors:
            print(f"crypto-benchmark: {error}", file=sys.stderr)
        print(f"crypto-benchmark: {len(errors)} failure(s)", file=sys.stderr)
        return 1
    print(f"crypto-benchmark: {len(data['operations'])} operations passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
