# Design

## Single target evidence receipt

The Apollo parity TOML gains one `target_evidence` table. It records the common
closing SDK SHA, successful slow workflow run, Rust toolchain, support-policy
path, portable package/feature lists, exact target list and compile-only
limitation. The three target rows continue to provide per-target gate and
limitation detail.

## Policy binding

The parity checker parses the repository support policy. It requires the
receipt toolchain to equal the primary compiler and requires all three
compile-checked policy targets to have the same declared package/feature shape.
For each portable target, the row ID, gate, tier, closing SHA, run link and
limitation must equal the corresponding policy/receipt value. Thus `fast`, a
host build or a stale target label cannot substitute for cross-target evidence.

The Linux host row remains bound to the green fast baseline. The unsupported
language-binding row remains issue evidence and is deliberately outside the
portable compile closure.

## Report rendering

The generated report includes a closing-receipt section with revision,
toolchain, packages, features and slow-run link. The target table includes each
row's revision and evidence link so reviewers can distinguish observed target
evidence from future binding work.

## Failure and rollback

Missing policy, malformed policy, a non-success conclusion, revision mismatch,
non-Actions URI, gate substitution, target inventory drift or policy-shape
drift fails validation. Rollback is documentation/checker-only and leaves the
support policy and Rust artifacts unchanged.
