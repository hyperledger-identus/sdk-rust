# Design: fail closed before a lossy OpenSpec archive

## Operating record

- Repository: `hyperledger-identus/sdk-rust`
- Worktree: `sdk-rust-worktrees/fail-closed-openspec-archive`
- Branch: `codex/fail-closed-openspec-archive`
- Base: `origin/develop@f3e7219c97604fadc4478ad466c36749df6907dd`
- Issue: #102, parent #20
- Owner: repository factory tooling; no runtime crate
- Consumers inspected or changed: none
- OpenSpec: 1.5.0 from
  `Fission-AI/OpenSpec@546224e00db26bd1be69874be465d5d6f5e4a851`
- Relevant upstream contract: `MODIFIED Requirements` MUST contain the full
  updated requirement; upstream explicitly warns that partial content loses
  detail at archive time.

## Failure model

OpenSpec applies a modified block by replacing the canonical requirement with
the delta block. Structural validation checks that the block exists and has a
scenario, not that unrelated canonical content survived. The issue #98 review
demonstrated this exact gap for the crate-ring layer rulebook.

The repository needs a mechanical answer to one narrow question: is a
replacement purely additive, or has the author explicitly acknowledged the
exact canonical block being rewritten? Correctness of new semantics remains a
semantic-review responsibility.

## Decisions

### 1. Normalize and hash requirement blocks, not whole files

The checker reads only UTF-8 Markdown beneath `openspec/specs/` and active
`openspec/changes/*/specs/`. It normalizes CRLF/CR to LF, strips trailing
whitespace from each line, removes leading/trailing blank lines, and hashes the
normalized complete canonical block without a terminal newline. The block
starts at `### Requirement:` and ends before the next requirement or level-two
section.

This makes the acknowledgement stable across line-ending and trailing-space
noise while binding it to the exact prose and scenarios reviewed.

### 2. Additive replacement is the safe default

For each `MODIFIED` block, every nonblank normalized line of the canonical
block must appear in the candidate block in the same order. New lines may be
inserted anywhere. This admits a complete copied block with additions and
rejects the known abbreviated replacement, scenario loss, and unacknowledged
prose rewrites.

Blank lines are ignored only for the subsequence comparison. Requirement and
scenario structure remains OpenSpec's strict-validation responsibility.

### 3. Intentional rewrites use an archived sidecar acknowledgement

When the additive test fails, the active change may contain
`archive-intent.toml`:

```toml
[[modified_requirement]]
capability = "example-capability"
requirement = "Existing requirement"
canonical_sha256 = "<64 lowercase hexadecimal characters>"
reason = "Why rewriting or removing canonical behavior is intentional"
```

The entry must match the capability and normalized requirement name, carry the
exact current canonical block hash, and provide a nonempty reason. Duplicate,
malformed, stale and unused entries fail. The sidecar moves into the archive
but never enters the living canonical spec. An acknowledgement makes deletion
explicit and reviewable; it does not prove the replacement semantics correct.

### 4. Rename-aware comparison follows OpenSpec ordering

OpenSpec applies `RENAMED` before `MODIFIED` and requires the modified block to
use the new name. The checker maps each renamed destination back to its current
canonical source, changes only the header for comparison, and binds any
intentional-replacement hash to the original canonical block. Ambiguous or
missing sources fail closed.

### 5. One standard-library checker owns the policy

`scripts/check-openspec-archive.py` exposes a repository-root positional
argument plus an optional active change name. `scripts/check-factory.sh` runs
it across every active change, so `scripts/factory check` and Nix enforce the
same policy. No network, OpenSpec internals, Markdown package, or donor code is
required.

`scripts/factory archive <change>` runs readiness, the scoped preservation
check, `openspec archive <change> --yes`, and the post-archive factory check.
Documentation directs agents to this guarded command. Raw OpenSpec remains a
pinned dependency, but it is not the repository's supported archive entrypoint.

### 6. Fixture tests operate only in temporary trees

The mutation suite creates bounded temporary OpenSpec trees and never invokes
archive against the repository. It proves:

- the issue #102 partial replacement is rejected;
- a full additive modified block succeeds;
- an intentional rewrite with the exact hash and reason succeeds;
- missing, stale, duplicate and unused acknowledgements fail;
- rename plus modification resolves the canonical source; and
- an added requirement in a new capability succeeds without intent metadata.

The existing factory-contract shell suite invokes this focused Python suite.
The Nix `factory-contract` already includes Python and therefore needs no new
dependency.

## Threats and limits

- Inputs are repository-controlled UTF-8 files, never network or user data.
- Symlinked specs, files over 1 MiB, more than 256 active changes, or more than
  1,024 requirements per file fail closed.
- Diagnostics contain repository-relative paths, requirement names and hashes,
  not secrets or arbitrary file bodies.
- Parsing recognizes exact level-two delta sections and level-three
  requirement headers case-insensitively, matching the pinned OpenSpec grammar.
- The checker cannot judge whether an acknowledged rewrite is wise; the exact
  hash and reason make that protected judgment visible to semantic review.

## Alternatives rejected

- **Patch OpenSpec:** outside this repository's ownership and slower to deploy.
- **Only compare scenario names:** would still permit silent loss of unrelated
  normative prose.
- **Require byte-identical old blocks forever:** prevents legitimate rewrites.
- **Preview archive and inspect Git diff manually:** repeats the failure mode
  and is not an executable CI gate.
- **Embed acknowledgement comments in the delta:** those comments would leak
  into the living canonical specification.

## Pre-implementation semantic review

The contract reproduces the actual replacement semantics, rejects silent loss,
provides an exact and auditable path for intentional changes, keeps semantic
review authoritative, and introduces no runtime, consumer, network, release or
governance mutation. No unresolved blocker remains. Implementation may begin
after strict structural validation passes.
