# identus-prism-vdr

Bounded, effect-neutral PRISM verifiable-data-registry primitives.

This crate owns storage-entry wire validation, exact-byte authorization,
current-entry state, and finite ancestry traversal. It depends on
`identus-prism-did` and exposes caller-implemented lookup ports.

Cardano sources, persistence, scheduling, retry, HTTP, CLI, runtime, and
deployment remain outside this crate. See the
[portable PRISM stack](../../site/src/architecture/prism-portable-stack.md) for
the complete ownership model and executable E2E command.
