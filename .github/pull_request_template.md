## Linked Work

- Issue:
- Discussion:
- Spec Kit task:

## Summary

-

## Validation

- [ ] `node tools/check-agent-sdlc.mjs --check`
- [ ] `.specify/scripts/bash/check-prerequisites.sh --json --include-tasks`
- [ ] `node tools/generate-wrapper-api-parity.mjs --check`
- [ ] `node tools/generate-workspace-dependency-graph.mjs --check`
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`

## Security Impact

- [ ] No security-sensitive behavior changed.
- [ ] Security review required.

Notes:

## Docs Impact

- [ ] No docs update required.
- [ ] Architecture, testing, migration, or release docs updated.
- [ ] Docs review required.

Notes:

## Conformance Impact

- [ ] No fixture or conformance impact.
- [ ] Static-model fixture updated.
- [ ] Vector fixture updated.
- [ ] Transcript fixture updated.
- [ ] Interop fixture updated.
- [ ] Infrastructure fixture updated.
- [ ] Conformance review required.

Notes:

## Commit Hygiene

- [ ] Commits are GPG-signed.
- [ ] Commits are DCO-signed.
