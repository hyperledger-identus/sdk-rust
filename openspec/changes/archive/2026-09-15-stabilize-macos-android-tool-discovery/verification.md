# Verification

## Exact-head local evidence

Reviewed implementation head `432ace441d6f978aa2f277ba1797c260c99d998c`:

- `python3 scripts/tests/support-policy.py`: 191 tests passed.
- `scripts/tests/factory-contract.sh`: passed after the review hardening.
- `python3 scripts/check-support-policy.py .`: passed.
- `git diff --check`: passed.
- `/nix/var/nix/profiles/default/bin/nix flake check`: all 31 compatible
  `aarch64-darwin` checks passed; Nix truthfully omitted incompatible
  `x86_64-linux` evaluation.
- The exhaustive Nix run included 721 passing workspace tests, complete
  all-target/all-feature Clippy, documentation, dependency policy, MSRV and
  etalon gates, plus Android, iOS, and WebAssembly cross-builds.

## Behavioral contract evidence

The policy suite rejects removal of either SDK variable fallback, the nonempty
root guard, the exact SDK-relative command path, the executable guard, or the
absolute invocation. It independently rejects drift in the exact NDK,
platform, and system-image package identifiers. It also rejects an ambient
`command -v sdkmanager` replacement even when the expected path text remains in
a comment.

## Deliberately outstanding

Only the GitHub-hosted macOS image can prove package installation and emulator
execution. The protected PR gates remain authoritative before merge. After
merge, issue #276 owns the exact-`develop` slow canary receipt and remains open
until the first natural weekly schedule is observed. Because the current
contribution gate mandates `Closes #276`, the merge operator must reopen it
immediately before dispatching the canary.
