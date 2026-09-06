# Review: fuzz the JWS Compact boundary

## Pre-implementation review

- **Date:** 2026-09-06
- **Issue:** #100 under #8 / `IDR-004`
- **Develop base:** `d6368db836793fd39c545ee90694315e43491a71`
- **Result:** ready to implement; no unresolved blocker

### Architecture and cohesion

- Assurance stays with the generic codec in sdk-rust. Apollo has no parser to
  port, while NeoPRISM, Oxid and Lace supply consumer needs without owning the
  reusable boundary.
- One target is cohesive because segment, header, limit, exact-input and staged
  encoding invariants converge on the same parser result.
- Reusing the independent fuzz workspace, pinned runner and campaign envelope
  is the smallest design. No production crate, feature or root lock changes.

### API and compatibility

- The target consumes only public `identus-jose` types plus an independent
  base64 implementation. It changes no Rust item, wire form, parser limit,
  error, algorithm support or explicit unverified state.
- Rebuilding accepted values compares semantics rather than requiring protected
  JSON member order/whitespace to survive canonical staged encoding.
- A finding may justify a compatible parser fix only with a minimized named
  regression; weakening the target is not an acceptable resolution.

### Security and resources

- Non-UTF-8 is rejected before the `&str` boundary rather than decoded lossily.
- The documented `text:` seed transport removes only the repository line ending
  from committed examples; arbitrary unprefixed bytes remain raw.
  Default parsing sees the whole input; five raw `u16` values also exercise a
  capped positive limit tuple against the suffix.
- The 128-KiB generator ceiling crosses the 64-KiB public complete-input bound.
  Derived limits are capped, executions at five seconds and RSS at one GiB.
- Errors are validated by variant-to-code mapping, avoiding unreliable checks
  that compare arbitrary attacker text with static diagnostics.
- Custom assertion text contains no token, header, payload, signature or key
  identifier; failure artifacts remain untrusted and failure-only.

### Automation, provenance and performance

- RFC 7515 is the immutable representation source; RFC 8725 informs defensive
  handling. Oxid/Lace seeds are reconstructed from SDK tests, not copied.
- Fixed 4,096-run smoke is reproducible. Five-minute soak is separate and the
  hosted job remains bounded at 20 minutes.
- Campaign elapsed time is measurement evidence only. The scope deliberately
  stops at useful 70–80 percent maturity; structured generators, OSS-Fuzz and
  additional JOSE families are follow-ups.
- Downstream repositories remain unchanged in this slice. The newly authorized
  NeoPRISM integration branch belongs after sdk-rust functional PRISM/VDR APIs,
  not inside this assurance-only change.

## Post-implementation review

- **Date:** 2026-09-06
- **Result:** accepted locally after one lint-only correction; no unresolved
  correctness, security, compatibility, provenance or performance finding

### Review findings and resolution

- The first strict fuzz Clippy pass found `sliced_string_as_bytes` in the exact
  signing-input assertion. It was corrected to slice `compact.as_bytes()`;
  strict Clippy then passed. This did not change semantics.
- The first hosted EditorConfig gate correctly found that newline-terminated
  `text:` seeds conflicted with the generic raw-corpus rule. A target-specific
  final-newline exception, matching the existing JWK/COSE text transports, was
  added; the harness still removes that one line ending before parsing.
- Review confirmed `text:` normalization is opt-in, removes at most LF plus a
  preceding CR, and is applied only to the default whole-input route. The raw
  prefix/suffix limit route remains byte-exact.
- Review confirmed rejection is ordinary, accepted segments are independently
  decoded/re-encoded, the second separator defines the exact signing input,
  and staged reconstruction compares semantic fields rather than JSON text.
- Review confirmed the parser-only error allowlist is closed at the harness
  boundary and each accepted variant is checked against its stable `jose.*`
  bridge and `jose` capability.
- Review confirmed fuzz dependencies and the recalculated lock remain confined
  to `fuzz/`; production APIs, root lock, targets and downstream repositories
  are unchanged.
- Automated PR review found two P2 corpus gaps: accepted derived-limit parsing
  was unseeded, and decoded `jwk`/`x5c` key-reference paths were not reachable
  from raw dictionary words. The `limits:000` transport now seeds an accepted
  suffix under a derived tuple, and six complete encoded fixtures cover valid
  public JWK/X.509 plus ambiguous, private, empty and invalid rejection paths.

### Verification receipt

- JWS corpus replay: 19 files, 20 executions, no finding.
- Deterministic JWS smoke: seed `424242`, 4,096 executions, 1,359 covered
  edges, 3,583 feature edges, peak RSS 52 MiB, no finding, one second after
  compilation.
- Existing DID and crypto corpus replay: all four targets passed.
- Fuzz workspace: formatting, strict Clippy, cargo-deny and RustSec passed.
  Cargo-deny emitted only repository-wide unmatched-allowance warnings.
- `identus-jose --all-features`: 45 passed, 4 diagnostics ignored.
- Workspace `--all-features`: passed in full.
- `nix flake check --print-build-logs`: all 31 host/MSRV/target/lint/docs/test/
  supply-chain checks passed; the primary nextest lane ran 411 tests with 21
  diagnostics skipped. Existing Nix evaluation/dependency warnings were not
  branch-owned failures.
- No crash artifact was created and the curated corpus stayed clean because
  generated growth ran in a temporary copy.

### Scope conclusion

The campaign reaches the intended useful assurance point without adding a
structured generator, cryptographic verification, OSS-Fuzz, or downstream
integration. Those remain independently prioritizable follow-ups.
