# siros-dcql 0.3.0 compatibility spike

This unpublished, separately locked fixture is executable evidence for
[ADR 0156](../../adr/0156-retain-oid4vci-and-spike-a-private-dcql-engine.md).
It tests whether a bounded Identus-owned adapter can isolate the narrow DCQL
engine while preserving known semantic differences. It is not a production
dependency, public API, portable-runtime claim, or OID4VP implementation.

Run the host evidence from the repository root:

```text
./scripts/check-siros-dcql-spike.sh
```

The checker performs no network or protocol request. Cross-target commands are
recorded separately and only count when their required toolchains are present.
The [completed results](results.md) contain the capability matrix, clean-room
vector inventory, measured payoff, resource analysis, and acceptance mapping.
