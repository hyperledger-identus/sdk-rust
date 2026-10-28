# UniFFI generator risk acceptance

Issue [#564](https://github.com/hyperledger-identus/sdk-rust/issues/564) and
[ADR 0191](../adr/0191-bound-uniffi-generator-maintenance-risk.md) authorize one
temporary exception for the unpublished native-binding generator. This page is
the operator knowledge base; it does not authorize another dependency or an
extension.

## Accepted fact

RUSTSEC-2026-0374 classifies `basic-toml 0.1.10` as unmaintained. It is accepted
only on this locked build-time path:

```text
identus-uniffi-bindgen 0.0.0
└── uniffi 0.32.0
    └── uniffi_bindgen 0.32.0
        └── askama 0.16.1
            └── askama_macros 0.16.1
                └── askama_derive 0.16.1
                    └── basic-toml 0.1.10
```

It is absent from production runtime crates and generated Swift/Kotlin
packages. The acceptance expires after **2027-01-31 UTC**. A vulnerability,
changed version/path, or expiry invalidates it immediately.

## Operator command

Run from the repository root inside the bindings shell:

```console
scripts/check-uniffi-generator-supply-chain.py
```

The command verifies the exact inverse tree, runs `cargo deny` for bans,
licenses and sources, then runs `cargo audit --deny warnings` as the single
advisory authority with only RUSTSEC-2026-0374 scoped out. It is
followed by the existing Swift, Kotlin/JVM, Apple and Android host/package E2E
scripts in the weekly macOS lane. Do not pass `--as-of` outside deterministic
tests.

## Exit and escalation

Monitor UniFFI and Askama releases. Remove the exception immediately when a
compatible release removes `basic-toml`. If no clean graph exists before the
deadline, open a new issue and research a UniFFI migration, a narrow reviewed
fork, replacement tooling, or an explicitly shorter renewal. Never edit the
date or expected tree as a drive-by CI fix.

The natural scheduled run—not a rerun or manual dispatch—is the immutable
weekly health receipt. A red natural run remains actionable evidence even when
an exact local reproduction passes.
