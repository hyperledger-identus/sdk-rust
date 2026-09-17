# Dependency direction

![Dependency direction from consumers to the three crate foundation](../diagrams/release-train.svg)

Arrows mean “depends on.” Dependency direction always points toward the more
stable, generic center:

1. wallets, identity products, and chain families select SDK components;
2. protocol/domain components select primitive foundations;
3. `identus-crypto` depends on `identus-core`;
4. `identus-core` and `identus-crypto` use `identus-derive` at compile time.

The reverse direction is forbidden. In particular, the three-crate release
train cannot depend on Midnight, Compact, Cardano/PRISM, consumer repositories,
wallet storage implementations, or product trust/custody policy.

This allows downstreams to evolve independently while sharing evidence-backed
foundations.
