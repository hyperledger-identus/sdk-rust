# Design

## Inventory corrections

The DID method registry is an SDK-enforced retained collection with a 64-entry
ceiling. DID resolution caching has two owners: the SDK enforces encoded-key,
declared-capacity, and TTL policy limits; injected cache and monotonic-clock
adapters own storage allocation, eviction, synchronization, execution time,
cancellation, and backend behavior. Three rows preserve those distinctions.

## Non-recursive rejected-value cleanup

`PublicKeyJwk::from_parts` takes ownership of its extension map before profile,
reserved-member, budget, or coordinate validation. A private guard owns that
map across every early return. Its destructor takes the map and dismantles
arrays and objects with an explicit `Vec<Value>` work stack. Success takes the
validated map from the guard into `PublicKeyJwk`, where accepted depth and node
budgets make ordinary destruction safe.

The guard changes no public signature or error. A hostile-depth native test
combines deep JSON with an earlier profile error so ordering cannot bypass the
cleanup path. Existing exact budget and serde/native parity tests remain.

## Risks

- Iterative cleanup may allocate a work stack for already allocated hostile
  breadth. This is caller-owned preallocation, but it avoids process-stack
  exhaustion and releases every owned node deterministically.
- A future recursive public carrier needs the same ownership review; the
  inventory review trigger remains authoritative.
