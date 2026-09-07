# Verification evidence

**Date:** 2026-09-08

## Passed

- `scripts/factory doctor` on `develop@ff924db61452d4a1fd2a8bd4a1bfd3d702519604`
- `scripts/factory research-ready decide-ssi-repository-dispositions`
- `scripts/factory constraints-ready decide-ssi-repository-dispositions`
- `scripts/factory check`
- `nix build .#checks.aarch64-darwin.lint-text --print-build-logs`
- `git diff --check`
- independent semantic/exact-diff review and correction pass

## Not run and why

- Cargo build/test and `nix flake check`: no Rust, Cargo, Nix, workflow, or
  generated implementation changed; factory and text gates are proportional.
- Candidate repository compilation, dependency-cone, advisory, unsafe/native,
  and conformance probes: this change authorizes or rejects future work but
  adds no dependency. Each conditional/spike integration issue owns those
  commands before production adoption.
- External link crawler: repository-owned sources were queried directly during
  research; Markdown and relative-path structure passed, but remote URL
  availability is not represented as a permanent guarantee.

No unrun command is represented as passing.
