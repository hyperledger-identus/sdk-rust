# Context

`identus-wallet` defines five least-authority asynchronous persistence ports.
`identus-wallet-conformance` verifies exact-key loading, insert-only writes,
compare-and-swap replacement/deletion, revision invalidation, scope isolation
and bounded pagination without selecting a runtime or backend. Aries Askar is
an established encrypted record store, but its public records expose no native
revision token and its top-level crate unconditionally includes its broad KMS
cryptography package even when only storage is used.

# Goals

- Determine whether Askar transactions can preserve the SDK exact-record
  contract behind an SDK-owned adapter.
- Exercise real encrypted storage through the public released API.
- Expose coupling, atomicity, diagnostics and target mismatches before adoption.
- Keep candidate dependencies outside release artifacts and ordinary fast CI.

# Decisions

## Use a separately locked SQLite fixture

The fixture is a nested workspace outside the root workspace. It pins exact
`aries-askar = 0.4.6`, disables defaults and enables only `sqlite`. It depends on
the local `identus-wallet` and `identus-wallet-conformance` crates for the
contract. No root feature or lock changes.

## Prove one exact-record slice

The fixture implements `SecretStore<Scope = String, Key = String, Value =
String>`. It opens a fresh encrypted `sqlite://:memory:` store and runs the
shared exact-store conformance suite. The adapter encodes an SDK-owned monotonic
revision alongside each value, performs conditional mutations in Askar
transactions, and maps candidate failures to closed `StorageError` classes.
This is enough to evaluate the hardest exact-record seam without implying the
list-capable ports or a complete production adapter.

## Keep candidate authority private

Askar categories and names are private adapter details. Generic scopes, keys,
values, revisions and errors stay SDK-owned. Candidate error messages are never
returned or logged. The test uses a fixed research pass key only inside the
ephemeral fixture; it is not a custody or key-management recommendation.

## Decide after executable evidence

The final ADR will distinguish technical feasibility from production fit. A
passing exact-store proof may justify a separate optional-adapter issue, but it
cannot admit Askar into core, choose SQLite for consumers, or activate support.

# Compatibility matrix

| Surface | Spike effect |
| --- | --- |
| Root workspace, features and lock | Unchanged |
| `identus-wallet` public contract | Unchanged |
| Default and fast CI | Unchanged |
| Research fixture | Exact locked Askar/SQLite graph; explicit execution only |
| Storage | Ephemeral encrypted in-memory SQLite only |
| WASM, iOS and Android | Compile/link feasibility measured; no runtime claim |

# Rollback

Remove the fixture, script, ADR and research capability specification. No public
API, persisted consumer data, release artifact or downstream repository needs
migration.
