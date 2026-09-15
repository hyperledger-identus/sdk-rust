# Local review

- **Review date:** 2026-09-15
- **Review angle:** roadmap semantics, network/offline separation, command and
  snapshot boundaries, diagnostic privacy, Nix source completeness and
  OpenSpec canonical consistency
- **Scope:** issue #285 implementation relative to planning commit `a8713d6`
- **Result:** passed after the findings below were resolved

## Resolved findings

1. The first evidence run passed locally but Nix omitted the new untracked test
   from its Git-derived flake source. Both executable scripts are now staged and
   explicitly listed by the repository/factory contract; the corrected Nix
   derivation contains and executes them.
2. Updating the CSV owner for IDR-023 without replacing the canonical
   requirement would have left OpenSpec requiring closed child #250. The delta
   now contains a complete, explicitly acknowledged replacement that returns
   ownership to open epic #7 without claiming engine completion.
3. The first strict snapshot validation checked only that an issue URL used
   GitHub HTTPS. It now binds the URL exactly to the tracked repository and
   issue number in both snapshot and live modes.
4. The evidence-only closure test initially closed a shared issue that also
   owned an `in_progress` row. It now excludes every active owner, proving the
   intended delivery-state distinction rather than weakening it.

## Architecture review

The network-free checker remains the only required CI dependency. The new
checker composes it before reading coordination state and is exposed through a
separate factory command. No GitHub model or mutable state enters SDK crates,
Nix gates or OpenSpec validation. The command has one responsibility: prove
issue visibility and open ownership for current `in_progress` rows.

## Security and privacy review

- Repository identity is loaded from tracked policy; validated issue numbers
  are passed to `gh` as argv rather than shell text.
- Only issue number, state and URL are requested. Bodies, comments, actors,
  tokens and credential output are not read or retained.
- GitHub stderr is not reflected. Command failures expose only the issue number
  and exit status.
- Snapshot size, schema, fields, repository, issue uniqueness, state vocabulary
  and exact URLs are bounded; symlink inputs fail.
- The command is read-only and cannot update the CSV, issues, settings or Pi
  policy.

## Remaining limitations

- Live selection depends on GitHub and the local `gh` authentication context;
  outages and visibility failures stop selection but do not break offline CI.
- Issue state proves coordination freshness only, not completeness, priority,
  conformance or security.
- The command performs one bounded lookup per unique ledger issue. The current
  ledger resolves nine unique issues; the fixed canonical ledger caps this at
  thirty.
- Nix still reports deprecated platform aliases from the external devshell
  framework while evaluating the fuzz shell. All five first-party
  `stdenv.isDarwin` uses were replaced; external input patching is outside this
  routine slice.

No unresolved blocking finding remains.
