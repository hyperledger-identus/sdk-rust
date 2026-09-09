# Constraint and limitation impact

Impact class: material
Decision status: directed
Decision reference: https://github.com/hyperledger-identus/sdk-rust/issues/226
Constraint blockers: none

## Existing entries affected

- `SDK-SEC-001` continues to prohibit authored unsafe Rust; UniFFI's
  dependency-owned unsafe/native reach is pinned and reviewed rather than
  treated as a first-party exception.
- `SDK-SEC-002` prohibits raw secret material across FFI; this slice exports
  public identifiers only.
- `SDK-SEC-003` requires explicit untrusted-input bounds; the facade preserves
  `MAX_DID_BYTES` and `MAX_DID_URL_BYTES`.
- `SDK-COMPAT-002`, `SDK-COMPAT-004` and `SDK-COMPAT-005` require Rust 1.98.1.
- `SDK-ARCH-001` keeps chain and product policy out of this generic adapter.
- `SDK-LIM-002` remains effective: the new experimental host-tested component
  is not a supported or distributed mobile FFI surface.
- `SDK-LIM-003` remains effective: host execution and existing target
  compilation do not prove mobile packaging or device runtime.

## Introduced or changed constraints

The new outer-boundary crate owns a cross-language ABI version, SDK-owned value
records and closed errors. Generic domain crates must remain UniFFI-free. Every
ABI value is owned; no secret, Rust/domain/dependency type, borrowed pointer,
handle, callback or future may cross. Unexpected wrapper unwind maps to a
constant internal code without exposing its payload. Generator and runtime must
use exact matching UniFFI versions, and generator-only dependencies remain
outside the root runtime workspace cone.

## Introduced or changed limitations

- Host Swift and Kotlin/JVM tests are verification evidence, not a supported
  installable SDK package or semantic-version commitment.
- No iOS/Android runtime, packaging, keychain/keystore, store or certification
  claim is introduced.
- The ABI covers only DID/DID URL parsing and components; objects, async,
  callbacks, storage, networking and cryptography are unsupported.
- React Native, browser and Node remain separate unsupported surfaces.

## Consumer and product impact

Rust consumers are unaffected except for an unpublished workspace package.
Future native consumers gain a stable experimental call shape that can be
packaged by #222 without changing the domain crate. Oxid, Midnight,
midnight-identity, NeoPRISM, Lace and Apollo remain unchanged.

## Activation and rollback

After merge, the crate and its API snapshots are experimental implementation
evidence on `develop`; `SDK-LIM-002` still controls external support claims.
Activation as supported native packages requires #222 to update the machine
policy atomically with iOS/Android package and runtime receipts. Rollback removes
the new crate, tool and host tests; no published artifact, persisted data or
downstream migration exists.

## Evidence

Evidence includes exact dependency locks/licenses/advisories, Rust 1.98.1
build/tests, authored unsafe prohibition, panic/redaction/boundary tests,
deterministic complete generation, normalized API snapshots, executable Swift
and Kotlin/JVM host calls, inventory/support-policy checks, full Nix gates and a
distinct architecture/security review.
