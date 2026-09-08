# Distinct local review

## Scope reviewed

- SDK exact diff from `origin/develop@42ff92e` through the current branch.
- Upstream exact diff from signed-tag revision `6539dc9` to contribution
  `b9e5c78`.
- Issue #179, ADRs 0078 and 0089, material constraint `SDK-DEP-001`, the
  dependency-readiness delta, research record and verification evidence.

## Findings

No blocking finding remains.

1. The upstream patch does not touch derivation or signature mechanics. Its
   production diff is limited to manifest features, private formatting, drop
   zeroization and removal of the superseded unsafe helper.
2. Keeping both formatting traits avoids a source-level trait removal while
   intentionally changing secret-dependent text to one constant marker. The
   test proves both paths exactly.
3. The exact zeroize 1.8.2 selection is justified by the upstream Rust 1.81
   contract; the possible duplicate with SDK zeroize 1.9 is disclosed and is a
   mandatory measurement in the later immutable-release update.
4. The minimal cryptoxide feature set is supported by direct imports, the
   resolved feature tree, complete existing tests and portable compile checks.
5. The upstream contribution remains external coordination evidence. The SDK
   Cargo graph, runtime, public API and downstream repositories are unchanged.
6. Accepted ADR 0078 was not edited. ADR 0089 carries the new reusable rule and
   the machine constraint index links it with an objective fork fallback.

## Non-blocking observations

- Upstream strict Clippy has eleven existing findings on both base and patch;
  this is reported verbatim and not expanded into unrelated cleanup.
- Upstream GitHub Actions has not attached checks to the fork pull request at
  review time. The contribution's merge remains upstream-owned; the SDK
  evidence PR does not claim upstream merge or release.
- User-installed cargo-audit 0.20.1 is too old for a current CVSS 4.0 advisory;
  the Nix-pinned 0.22.2 audit is the recorded successful gate.

## Verdict

The change is cohesive, reversible and truthful about what is and is not
remediated. It is ready for factory archive and the issue-linked SDK evidence
pull request.
