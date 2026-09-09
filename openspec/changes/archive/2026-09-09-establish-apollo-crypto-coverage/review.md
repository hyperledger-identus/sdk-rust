# Local review

Review status: complete

The distinct post-implementation review inspected denominator integrity,
feature-profile execution, artifact determinism, path confinement, vector
mapping, secret exposure, Nix pinning and fast/slow workflow separation.

## Findings

1. **Resolved -- raw LCOV contained out-of-scope first-party files.** The first
   hosted artifact correctly calculated its JSON denominator from
   `crates/crypto/src`, but its raw LCOV also contained `identus-core` and
   `identus-derive` records reached by compile tests. The normalizer now filters
   LCOV to the exact executable-file inventory accepted from LLVM JSON, rejects
   inventory drift, and rewrites every source path to a repository-relative
   value. Mutation tests cover mismatched inventories and the hosted artifact
   contains exactly 16 crypto source records.
2. **Resolved -- TOML formatting was omitted from the first local command set.**
   The first exact-head Linux slow run rejected alignment in the new parity
   tables. The manifest was formatted with the repository-pinned Taplo and the
   focused Nix lint derivation then passed.
3. **No blocker -- line coverage is not semantic assurance.** The manifest keeps
   published vectors plus negative, redaction, resource-bound and error-bridge
   selectors independently mandatory. The report states that coverage makes no
   branch, mutation, side-channel, runtime, certification or release claim.
4. **No blocker -- the tool is maintainer-only.** `cargo-llvm-cov` and
   `llvm-tools-preview` are pinned in Nix, do not enter Cargo metadata or the
   consumer dependency graph, and execute only in the manual/weekly slow
   workflow. The pull-request fast workflow is unchanged.
5. **No blocker -- evidence contains no secret material.** Normalized JSON,
   Markdown and LCOV contain revision/tool/profile metadata, aggregate counts
   and source paths only. The runner does not capture test stdout or values.

The final review found no unresolved correctness, security, tooling or scope
issue.
