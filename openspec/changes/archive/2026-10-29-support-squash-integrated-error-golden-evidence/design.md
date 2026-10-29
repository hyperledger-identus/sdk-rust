# Design

## Evidence states

```text
active / ancestry-preserving
  base ──► contract ──► HEAD
             │
             └── planning-only diff + exact receipt + exact golden

protected squash archive
  base ──► squash integration ──► HEAD
             │
             ├── HEAD blob = working receipt = exact receipt hash
             ├── HEAD blob = working archived golden = exact golden hash
             └── HEAD blob = working stable golden = exact golden hash

  contract object reachable? ── yes ──► validate base/diff/blob as extra proof
                             └─ no  ──► canonical snapshot remains sufficient
```

## Receipt binding

`GoldenBinding` gains one fixed `receipt_sha256`. The checker reads the receipt
as a bounded regular non-symlink file, verifies the full byte hash before JSON
interpretation, then retains the current closed schema and identity checks.
This prevents mutation of the contract head, timestamp, readiness flags, or
any other byte after the planning boundary.

## Ancestry decision

The checker first proves `base -> contract`. A reachable contract that fails
that proof is invalid. If `contract -> HEAD` holds, current planning-only diff
and contract-blob checks remain unchanged.

If `contract -> HEAD` does not hold, the fallback is allowed only for a regular
archived change, never an active change. It requires `base -> HEAD`, the exact
receipt hash, and byte equality between each working-tree evidence file and its
exact `HEAD:<path>` blob before applying the existing archived/stable/fixed-hash
equality. This makes canonical history, not a copied working tree,
authoritative. When the contract object is reachable, its planning-only diff
and golden blob are still validated even though squash discarded its parent
relationship to `HEAD`. When the object is unreachable, no network fetch or
branch retention is required and the canonical `HEAD` snapshot is
authoritative.

The implementation distinguishes an unreachable object from a reachable but
invalid object; malformed reachable history cannot force the fallback.

## Mutation coverage

The synthetic repository suite covers:

- the existing ancestry-preserving success path;
- a single-parent canonical squash containing the archived receipt and exact
  stable/planning golden;
- the same state after deleting the source planning ref and pruning objects;
- active-change ancestry loss;
- receipt-byte, base, contract-head, golden and path mutations;
- exact working-tree evidence copied onto a `HEAD` that does not contain it;
- a reachable contract that is not planning-only; and
- no weakening of the first four bindings.

Quality declaration `factory.error-golden.quality.v1` binds the deterministic
mutation suite as its required on-change property evidence. Its other three
closed quality classes record issue-owned not-applicable decisions rather than
inventing fuzz, latency, or cross-implementation claims for repository policy.

## Documentation and rollback

The factory handbook explains why source-branch ancestry is an optimization,
not a protected squash invariant, and names the exact durable evidence. No
public SDK documentation changes. Rollback is a checker/test/docs revert with
no data or API migration.
