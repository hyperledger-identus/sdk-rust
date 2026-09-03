# Design: safe chain-neutral DID Registration lifecycle

## Standards and immutable evidence

The design uses the DIF DID Registration draft at commit
`ec1bf38f7860b361eb6692c02f742b9dfc48291b` (3 January 2025, Apache-2.0).
The repository still labels it Draft and publishes no ratified version. The
portable contract adopts its create/update/deactivate operations and
finished/failed/action/wait vocabulary while isolating unstable wire details.

Read-only implementation evidence:

| Repository | Revision | Relevant evidence |
| --- | --- | --- |
| NeoPRISM | `d6ad1ecade80757f08da4f9101d14c2fb1a4d02b` | bounded ordered PRISM create/update/deactivate operations, previous-operation hashes, submission and finality identifiers |
| midnight-identity | `427f8571950c42967a18726cbcbefecc19ef8d79` | narrow generic registrar, ordered method calls, private-state custody and finalized transaction results |
| Lace ID Portal | `804de0a9e58cf48ece3cc6c24b2245bb70bc80f1` | idempotency, partial-commit/orphan, serialization and encrypted-custody hazards |
| Oxid | `bfe3b481568dc738f0732c2b27548fab8721fd95` | confirmation, managed-key, wallet-policy and chain-finality boundaries |
| Apollo | `ccee22bcd693e618b9b8ff3e15ed6f9c9156c27c` | no registration component; legacy crypto evidence only |

No source or fixture is copied. Consumer trees had pre-existing local changes
at inspection and remain untouched.

## Draft divergence profile

| DIF draft surface | SDK decision | Reason |
| --- | --- | --- |
| raw `secret` input/output maps | opaque custody handles and public action data only | private material must not enter serialization, errors, debug, metadata, or FFI-shaped values |
| `storeSecrets=false` and `returnSecrets=false` | invalid internal-secret policy | silently losing generated controller capability is an unsafe default |
| terminal result may omit job; non-terminal must carry job | enforce this invariant for every constructor | the prose is coherent even though draft examples contradict it |
| repeated operation call with `jobId` | explicit `Continue` request with method-scoped job | makes state transition and registry routing unambiguous |
| no idempotency contract | required idempotency key on every request | mutation retries otherwise duplicate irreversible operations |
| no cancellation operation | explicit best-effort `Cancel` request | dropping a Rust future cannot imply rollback of external work |
| redirect and decrypt actions | deferred | callback integrity and plaintext handling need separate transport/secret contracts |
| `execute` and DID URL resource operations | deferred | experimental/method-specific and not required by the first two adapters |
| draft JSON structures | no direct wire binding | keeps this safer Rust profile replaceable as the draft evolves |

The divergences are deliberate. A future DIF adapter translates only after its
own profile, transport, callback, and secret review.

## Public composition model

`RegistrationRequest` is a closed operation enum:

- `Create` carries a method, optional requested DID/initial document, options,
  secret mode, and idempotency key;
- `Update` carries an exact DID and non-empty ordered document operations;
- `Deactivate` carries an exact DID;
- `Continue` carries a method-scoped job and either the exact outstanding
  action response or no response for a wait;
- `Cancel` carries the method-scoped job.

Every variant exposes its exact method without parsing opaque job text. This
lets `DidMethodRegistry` route all operations with the same exact-match rule as
resolution. A binding may contain a resolver, dereferencer, and registrar
independently. A missing method returns `methodNotSupported`; a known method
without a registrar returns `featureNotSupported`.

The one-method object-safe `DidRegistrar` port accepts a borrowed request and
returns a boxed `Send` future containing a `DidRegistrationResult`. Method
failures are typed result states rather than a second adapter-specific error
channel. Invalid native construction is rejected before the port is called.

## Jobs and state invariants

`RegistrationJob` contains a validated `DidMethod`, opaque bounded job id, and
either an expected action id or a wait continuation. Its debug output contains
only the method and continuation kind, never the opaque token.

`DidRegistrationResult` contains exactly one state plus bounded public
registration and DID-document metadata:

- `Finished`: no job; create has a DID, and update/deactivate preserve the
  request DID. A public document is optional because finality can precede
  indexing.
- `Failed`: no job and a bounded public failure code; cancellation is a normal
  failure code and does not claim rollback.
- `Action`: a job whose expected action id exactly matches the one public
  action request.
- `Wait`: a wait job and optional retry hint no greater than 24 hours.

Result validation rejects terminal jobs, non-terminal missing jobs, action/job
correlation mismatches, and DID/method mismatches. `validate_for_request`
checks create method and update/deactivate identity before a result is accepted
by a caller or registry.

## Idempotency, retry, cancellation, and concurrency

Requests are immutable and equality-comparable. An adapter must bind an
idempotency key to the canonical request and return the same logical job or
terminal outcome for an identical replay. The same key with different input is
a conflict. Storage and retention are adapter decisions; the generic crate
does not persist an idempotency table.

Continuation requires exact action-response correlation. Wait continuation
contains no invented response. Retry hints are advisory values only: the SDK
does not sleep or spawn work. Callers set backoff, attempt, deadline, consent,
and cancellation policy.

Dropping the returned future cancels only local observation. It makes no claim
about already-submitted external work. `Cancel` is explicit and best effort;
adapters return the actual terminal or continuing state and never report an
irreversible transaction as rolled back.

The port and registry are `Send + Sync`. The generic layer owns no lock or
mutable job state, so cancellation cannot poison shared SDK state.

## Updates and public action data

Ordered `DidDocumentOperation` values distinguish:

- complete `SetDidDocument` using the validated `DidDocument` model;
- standard `AddToDidDocument` and `RemoveFromDidDocument` using bounded public
  JSON objects;
- a bounded method-specific operation name plus optional public JSON object.

The core preserves order and never calculates a diff, resolves a prior
document, or claims atomicity. Method adapters declare which operations and
combinations they support.

Client-managed work uses a bounded `RegistrationAction` and matching
`RegistrationActionResponse`. Their open payloads are explicitly public
protocol data. Recursive validation rejects private-key/secret-shaped member
names and applies the same JSON limits as document extensions. Payload values
and custody handles have redacted debug output even when they are public enough
to cross the port.

## Secret and custody boundary

`RegistrationSecretMode` represents:

- internal generation with at least one of store or return-opaque-handle;
- external custody through a `RegistrationSecretHandle` understood only by the
  adapter;
- client-managed public actions.

A handle is a reference, not secret bytes and not a guarantee that a wallet is
unlocked or authorized. The contract has no private JWK, multibase private key,
seed, password, decrypted payload, signing implementation, or export function.
Adapters must keep private material behind their custody boundary and may use
more restrictive policy.

Open registration options, update data, action data, and metadata pass one
recursive public-data validator. It rejects reserved collisions, excessive
maps/arrays/depth/nodes/strings, control-bearing names, and private-material
field names. Errors expose only stable reason variants.

## Resource bounds and serialization

The slice uses existing document JSON budgets: 64 properties per map, 128
items per collection, depth 32, and 4,096 nodes. Public-data strings and action
payloads are at most 64 KiB. Raw public JSON entry points are at most 512 KiB.
Idempotency/action identifiers are at most 256 bytes; method-scoped job and
custody identifiers are at most 1 KiB. Wait hints are at most 86,400,000 ms.

Only bounded value objects that have an intentional representation implement
Serde. The request/result Rust enums do not establish the DIF wire format.
Native and JSON construction of public open data use the same validator.

## Error and observability contract

Construction failures bridge to `did.invalid_registration` and contain no
caller-controlled value. Standard lifecycle failures use stable open codes such
as `methodNotSupported`, `featureNotSupported`, `conflict`, `cancelled`, and
`internalError`; method codes remain bounded. Debug output reports variant,
counts, and presence only. It omits full DIDs, opaque ids, handles, public-data
contents, documents, and payload bytes.

Successful update and deactivate results expose the affected DID. An outer
coordinator can then call existing DID-wide cache invalidation. The registrar
does not depend on or mutate a cache automatically.

## Alternatives rejected

- **Copy midnight-identity's generic trait:** it cannot model jobs, actions,
  idempotency, safe custody, or partial finality.
- **Expose NeoPRISM or Midnight operation enums:** introduces method/chain types
  into the shared layer.
- **Mirror the DIF JSON exactly:** freezes draft contradictions and permits raw
  private material through generic maps.
- **Put persistence in the port:** couples every method to one runtime/storage
  transaction model and cannot make ledgers atomic with local custody.
- **Treat future cancellation as operation cancellation:** external submission
  can outlive the observing future and would make retries unsafe.
- **Automatically invalidate resolver caches:** adds a hidden dependency and
  cannot know whether a non-terminal or partially committed result is visible.

## Rollback

Revert issue #47's pull request. The capability is additive, unpublished,
stores no state, performs no I/O, and leaves existing resolver, dereferencer,
cache, and registry behavior unchanged unless a registrar is explicitly bound.
