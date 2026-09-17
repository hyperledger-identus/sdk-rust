# Architecture

SDK-Rust uses an inward dependency direction: products and chain adapters may
depend on generic components, while generic components cannot import product,
chain, runtime, deployment, or UI policy.

The first release candidate intentionally covers only the lowest cohesive
three-crate closure.

- [Dependency direction](dependency-direction.md)
- [Ownership boundaries](ownership-boundaries.md)
- [Evidence and release flow](evidence-flow.md)
