# Design

## Direct digest attestation

`aggregate_matrix` reads the descriptor-owned digest once and rejects each
lane whose closed lock field differs before accepting its rows. Peer equality
remains a diagnostic invariant but cannot substitute for source identity.
Archive assembly returns the digest computed after copy; the first and second
passes must agree, and the candidate receipt records that returned value.

## Runtime capability plus AST policy

Remove `generate-lockfile` from ordinary allowed Cargo operations. One private
generation helper owns the exact command and accepts a closed purpose value.
Runtime checks allow it only for `extracted-closure` or `staged-refresh`; every
ordinary `run` and all `run_stdout` calls reject resolved generation commands,
including variable-built lists.

Offline AST policy requires exactly one privileged subprocess call in that
helper and permits helper calls only in `verify_closure` and
`refresh_staged_lock`. It validates closed literal purposes and exact
purpose-owned manifest paths. This combines execution-time completeness with
static review of the escape hatch.

## Explicit refresh mode

`--refresh-staged-lock` is mutually exclusive with build and matrix modes and
requires a new output directory. Under the existing pinned DID candidate Nix
app it verifies Rust/Cargo 1.98.1, clean exact HEAD, renders staged manifests in
VCS-independent scratch, generates one proposed lock, validates its size,
format, sources, checksums, and candidate identities, then writes:

- `did-candidate.lock`: proposed exact bytes; and
- `refresh-report.json`: source revision, tool identities, current/proposed
  hashes, status, and sorted added/removed closed dependency identities.

The mode reserves the absent output directory before network resolution, then
rejects any unexpected content before publishing its two files. It never
copies output into the repository or changes the descriptor.
No-drift output is still useful: it proves the current reviewed resolution can
be reproduced at that time.

## Locked staged commands

Archive package, profile checks/tests, matrix checks/tests, and Rustdoc JSON
operate with Cargo `--locked`. Cargo-cyclonedx 0.5.9 has no locked CLI option,
so the pinned binary runs directly with `CARGO` set to a generated
metadata-only wrapper. That wrapper accepts only `metadata`, injects
`--locked`, and the caller also proves the staged lock digest is unchanged.
`cargo-public-api` consumes an already generated JSON file and does not resolve
the staged workspace. Extracted closure checks/tests remain locked against
their distinct generated lock.

## Path and rollback behavior

The descriptor permits exactly `docs/release/did-candidate.lock`; exact path
equality already excludes absolute and parent traversal syntax. Regular-file,
symlink, size, digest, TOML, source, checksum, and identity checks remain.
Rollback is atomic and restores #483 behavior, including its known gaps.
