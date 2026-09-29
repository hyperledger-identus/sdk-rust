# DID candidate review

The `identus-did` and `identus-did-resolver-http` train is candidate-only. It
does not authorize publication or change the canonical workspace version and
publication denial.

Build deterministic candidate evidence from an exact clean revision with the
pinned Nix app:

```bash
nix run .#did-candidate -- \
  --output /absolute/external/new-directory \
  --revision <full-current-head-sha>
```

To review whether registry resolution would change the frozen staged lock, use
the explicit refresh mode. The output directory must not already exist and
must be outside every Git worktree:

```bash
nix run .#did-candidate -- \
  --refresh-staged-lock \
  --output /absolute/external/new-directory \
  --revision <full-current-head-sha>
```

Refresh emits proposed `did-candidate.lock` bytes and a bounded drift report.
It never edits the repository lock or descriptor. Review and merge any proposed
lock and descriptor digest update through a separate issue-linked pull request;
do not dispatch or rerun the weekly slow workflow as part of refresh.
