# Exact-diff architecture, security and evidence review

Review date: 2026-10-27
Issue: #487
Develop base: `fe3a14b767a6b13f8aa2fb2acf92bc6e37d93c03`
Planning commit: `2a76c3f`
Preimplementation receipt commit: `a79413d`
Initial implementation commit: `83dc91a27dbffd9dac66a7c6e93e37c3ad390397`
Reviewed implementation commit: `18187dfae0cfc7195628f53c6c026598f6b0e2e4`
Evidence correction commit: `68f1d29`

## Resolved implementation findings

The Codex CLI fallback reviewed the complete diff after Claude CLI availability
ceased to be a delivery dependency. Four review rounds found and resolved:

1. JSON node/depth limits were initially checked after materialization. A
   streaming `DeserializeSeed` validator now applies node and container-depth
   budgets during parsing, before retaining presentation bytes.
2. Primitive leaves were initially charged as containers and object
   reserialization could normalize a number outside `u64`. Container depth is
   now counted independently and validated object bytes are whitespace-minified
   without changing numeric or string lexemes. The public depth limit is capped
   at the parser's enforceable ceiling.
3. Duplicate object member detection initially performed a quadratic scan.
   It now uses a deterministic ordered set whose owned names are zeroized on
   drop; a 4,000-member regression exercises the bounded path.

## Resolved evidence findings

The final review found no confirmed runtime defect but rejected three
overstated evidence claims. The declaration now:

- runs the complete direct-post integration test rather than naming negative
  cases that a filtered property test did not execute;
- limits the fuzz claim to arbitrary bounded presentation bytes placed in one
  fixed valid request-bound envelope, leaving collection cardinality to the
  integration suite; and
- describes the benchmark as an end-to-end ordinary/near-limit diagnostic,
  without claiming isolated serialization, allocation observation, a maximum
  profile or an established regression threshold.

## Final architecture and security review

- Production behavior remains cohesive inside `identus-oid4vp`; generic
  credential, product, wallet and transport policy remain outside.
- No new crate enters the resolved graph. The existing `serde` parser contract
  is declared directly rather than accessed transitively through `serde_json`.
- The transition consumes validated request/JAR/DCQL lineage and cannot accept
  destination, state or query rules from independent caller booleans.
- Presentation, collection, JSON node/depth, `vp_token` and form-body budgets
  are independent and positive. Checked growth precedes returned output.
- Retained presentation, state, query and JSON member bytes are zeroizing;
  public diagnostics expose static categories, counts and lengths only.
- The output is inert POST material. No HTTP, DNS, TLS, redirects, filesystem,
  clock, randomness, signing, encryption, persistence or product mutation was
  introduced.
- The synthetic factory fixture now retains the new vector and fuzz harness,
  so Git-less/Nix validation checks the same evidence graph as the repository.

Final disposition: all blocker/high/medium review findings are resolved. The
plain success profile is ready for protected exact-head CI; protocol-error
responses continue separately in issue #532.
