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

Pending implementation and verification.
