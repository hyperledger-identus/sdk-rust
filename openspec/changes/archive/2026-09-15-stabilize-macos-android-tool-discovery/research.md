# Research readiness

Research class: routine
Research status: ready
Decision date: 2026-09-15
Source retrieval date: 2026-09-15
Research blockers: none

## Problem and existing implementation

Manual slow run
[`34951058164`](https://github.com/hyperledger-identus/sdk-rust/actions/runs/34951058164)
used GitHub image `macos-26-arm64` version `20260907.0351.1`. The macOS matrix
completed code-health, complete Clippy, and `nix flake check`, then failed at
`Install exact Android slow-lane inputs` with `sdkmanager: command not found`.

The workflow invokes a bare command while the downstream Android verifier
already treats `ANDROID_SDK_ROOT` or `ANDROID_HOME` as the authoritative SDK
location. The exact runner-image inventory confirms Android Command Line Tools
16.0 and `/Users/runner/Library/Android/sdk` environment bindings, so the tool
exists below the SDK but is not promised on the shell path.

## Normative sources

- The exact hosted-image inventory linked from the failed job records Android
  Command Line Tools 16.0 and the Android SDK environment variables:
  `https://github.com/actions/runner-images/blob/macos-26-arm64/20260907.0351/images/macos/macos-26-arm64-Readme.md`.
- Failed job `104321891117` is the execution evidence for the missing ambient
  command.
- `scripts/check-uniffi-did-android.sh` already resolves `avdmanager`, emulator,
  and platform tools beneath the selected Android SDK and validates exact input
  versions before runtime evidence.

## Candidate decisions

| Candidate | Decision | Reason | Reconsideration trigger |
| --- | --- | --- | --- |
| Resolve `cmdline-tools/latest/bin/sdkmanager` below the selected SDK | `adopt` | Uses the runner's declared SDK authority and matches downstream tool discovery. | Hosted images remove the stable `latest` command-line-tools link. |
| Add the SDK tool directory to global workflow `PATH` | `not-adopt` | Broadens ambient authority for later steps and obscures which SDK is mutated. | Multiple later commands need the same exact tool directory. |
| Install command-line tools with Homebrew | `not-adopt` | Adds an unpinned package-manager bootstrap and a second Android SDK authority. | GitHub stops providing Android Command Line Tools. |
| Remove the Android runtime evidence | `not-adopt` | Hides a portability regression and violates the accepted native binding milestone. | A separately approved support-policy decision replaces the gate. |

## Compatibility and dependency evidence

No library, crate, workflow action, or package version changes. The workflow
continues installing NDK 27.0.12077973, platform API 35, and the Play Store API
35 arm64 system image through the preinstalled Android command-line tools.

## Security, privacy and maintenance evidence

The SDK root comes only from GitHub's predefined Android environment and is
quoted before execution. The workflow checks both the root and executable and
fails closed. It does not print environment contents, credentials, tokens, or
untrusted values. Network behavior remains the existing `sdkmanager --install`
operation inside a weekly/manual read-only-repository job.

## Rejected or deferred candidates

Global `PATH` mutation, Homebrew bootstrap, runner-label changes, and removal of
Android evidence are rejected above. Pinning a different macOS image or newer
Android package set remains a separate researched change.

## Open questions and blockers

There are no research blockers. Hosted execution is required to prove the
exact image path and the complete emulator-backed evidence after merge.

## Evidence commands

- `gh run view 34951058164 --job 104321891117 --log-failed` returned
  `sdkmanager: command not found`.
- The exact runner image inventory was retrieved through the GitHub API and
  confirmed Command Line Tools 16.0 plus `ANDROID_SDK_ROOT` and `ANDROID_HOME`.
- The local workflow-policy mutation suite is the offline regression surface.

## Reconsideration triggers

- GitHub changes the macOS Android SDK root contract or removes the `latest`
  command-line-tools link.
- The repository provisions Android SDK inputs reproducibly through Nix.
- Native Android runtime evidence moves to a dedicated runner/image contract.
