# Verification

## Exact-head focused evidence

The following checks passed at implementation head
`5d32d87338dc1c969f079fba21f81b54c352b850` unless noted otherwise:

```text
node --test scripts/tests/factory-operations.mjs
  47 passed, 0 failed
git diff --check
  passed
```

The complete factory check and bootstrap check passed at
`d5b481632fa3dc274ca17f76dbb41bff1378212a`, before the
review-only closure-regex correction:

```text
scripts/factory check
./bootstrap.sh --check
  passed
```

## Compatible slow closure

At `d5b481632fa3dc274ca17f76dbb41bff1378212a`, the following command completed
successfully:

```text
nix flake check --no-write-lock-file --print-build-logs
  all checks passed
  31 compatible aarch64-darwin checks evaluated
  workspace nextest: 764 passed, 22 skipped
  focused crypto nextest: 148 passed, 1 skipped
  strict Clippy, docs, fmt, dependency policy, text and TOML lint passed
```

The final review-only JavaScript change does not alter Rust, Nix, dependency,
target, public API, or wire behavior. Hosted exact-head fast CI remains the
authoritative integration gate for the PR head.

## Pi canary

The accepted Pi handoff and privacy-bounded harvest are recorded in
`docs/factory/canary-320-delivery-closeout.md`:

```text
sessions: 1
turns: 27
tool calls: 67
input tokens: 130459
output tokens: 21762
cache-read tokens: 2156288
cache-write tokens: 0
worker exit: 0
handoff: accepted
```

No prompt, transcript, secret, path content, or generated response is included
in the public canary evidence.
