# Enforce contribution-policy monotonicity

## Why

Issue #341 made the protected base the executable trust root. A proposed
`.github/contribution-policy.json` is now inert data during its own PR, but it
could still relax the baseline for every later contribution after merge. The
base-owned checker needs a closed partial order that permits identical or
strictly stronger policy and fails every relaxation.

## What changes

- Parse base and proposed policies with bounded duplicate-rejecting JSON.
- Validate a closed version-1 schema before comparison.
- Permit only subset allowlists/exemptions, non-increasing numeric ceilings,
  and false-to-true enforcement flags.
- Read the proposed policy directly from the exact imported head Git object;
  never execute proposed code.
- Add mutation tests, ADR 0167, workflow integration, and governance guidance.

## What does not change

No contribution invariant is relaxed. No approval waiver, repository setting,
secret, write permission, Rust code, SDK API, or signature-envelope decision is
introduced.

## Capabilities

### Modified capabilities

- `ai-software-factory`: require base-owned monotonic comparison of proposed
  contribution policy data.

## Authority

Issue #339, ADR 0166, and the completed base-owned workflow trust boundary.
