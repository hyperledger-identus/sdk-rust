# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/166
Constraint blockers: none

## Existing entries affected

No effective product or compatibility entry changes. Existing MSRV, platform,
repository-boundary, security and release facts are indexed without replacing
their canonical sources.

## Introduced or changed constraints

The change introduces the governance constraint that every qualifying change
must disclose constraint impact and cannot activate a material consumer-facing
constraint from an unapproved target. This exact process outcome was directed
by the sponsor in issue #166.

## Introduced or changed limitations

No SDK product limitation is introduced. The checker limitation is explicit:
it validates schema, references and declared state but semantic sufficiency
still requires review.

## Consumer and product impact

Consumers receive clearer visibility into effective constraints, future
targets and unsupported surfaces. No consumer must change Rust, dependencies,
features, targets, data or integration behavior in this change.

## Activation and rollback

The governance and factory checks activate when this issue-linked PR merges to
`develop`. Rollback reverts that single PR; it does not change any SDK runtime
or downstream repository.

## Evidence

Focused negative tests will prove missing records, unresolved material
direction, invalid states, missing sources and MSRV projection drift fail
closed. Factory and Nix gates provide integration evidence.
