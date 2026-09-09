# Factory canary #246: `identus-core` input boundaries

Date: 2026-09-10

This is the first bounded SDK slice executed after the operational factory
merged in PR #244. It validates the OpenSpec-first handoff and records harness
evidence without treating the factory vault as upgrade authority.

## Immutable handoff

- Issue: [#246](https://github.com/hyperledger-identus/sdk-rust/issues/246)
- Integration base: `b5d1a4c52d31f3d37564eee9c6edc890fc7b1295`
- Planning head: `de5c613c47bda5afc4aff39dbd2e477b5a33c0bf`
- OpenSpec change: `audit-core-input-boundaries`
- Receipt: planning, research, constraints and strict validation passed before
  the worker changed implementation or evidence paths.

## Pi invocation and result

The worker was launched only through the repository bootstrap with the pinned
Nix Pi 0.84.2 and Node 24.19.0. The invocation used non-interactive print mode,
disabled session persistence and allowlisted read, search, edit and shell
tools. The ephemeral task text and any provider credentials, messages,
transcript, session or billing data are not retained.

The measured wall duration was 471 seconds. Pi changed the focused core time
test, crate input-boundary inventory and two `SDK-LIM-007` evidence surfaces.
It did not change dependencies, the lockfile, public/wire behavior, the factory
harness, receipt, checklist, downstream repositories or branch state. Its
focused `identus-core` run passed 24 tests; independent supervisor reruns of
the tests, strict Clippy, formatting and constraint validation also passed.

## Findings

| Class | Evidence | Disposition |
| --- | --- | --- |
| Specification | The first draft described a factory-only review, while the canonical contract requires an issue-backed SDK slice. It was corrected to the core security audit before the planning commit. | Resolved before implementation; no spec change required. |
| SDK implementation | No defect found. Generated serde already enforces the intended `u64` range; focused tests now protect it. | Accept the evidence-only change. |
| Harness | A fresh Pi launch installed 19,386 files in 1,797 directories under worktree-local `.pi/npm/`, consumed 168,177,664 bytes, and dirtied Git. It reproduced in two worktrees. | Follow-up issue [#247](https://github.com/hyperledger-identus/sdk-rust/issues/247); do not change versions. The generated cache was moved intact to `/tmp/sdk-rust-pi-npm-issue-246` for local inspection. |
| Harness | Print mode emitted no worker progress during the 471-second run. | Record only; one run does not justify a protocol/output-mode change. Reconsider after repeated latency or operator-observability evidence. |
| Agent ergonomics | The worker first called preflight without the required `--write` or `--validate-receipt` mode, then corrected itself and passed. | Record one retry; reconsider agent-prompt/docs tuning only if repeated. |
| Supply chain | npm reported three unapproved install scripts while resolving the exact project package set; the run completed without approving them. | Preserve fail-safe non-approval. A package/version change requires separate dependency research. |
| Environment | Nix reports the existing `stdenv.isDarwin` deprecation during evaluation. | Pre-existing and unrelated to the slice; no bundled edit. |

## Limitations

`SDK-LIM-007` remains effective for unaudited SDK crates and for hostile input
allocation/scanning before typed SDK validation. The canary does not activate
`main`, publish artifacts, prove downstream adoption or authorize any package,
toolchain, provider or model upgrade.

The raw metrics record is stored outside Git under the repository Git common
directory. Public evidence is limited to the bounded aggregates above.
