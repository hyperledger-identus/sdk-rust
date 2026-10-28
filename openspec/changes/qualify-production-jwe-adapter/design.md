# Design

## Evidence boundary

```text
isolated exact candidate -> public-key encrypt -> compact JWE -> private-key decrypt
          |                                                       |
          +-- target checks + dependency/audit evidence ---------+

production workspace -> identus-jose SDK port -> caller-provided adapter
```

The qualification route is deliberately outside the Cargo workspace. It proves
that an exact upstream engine can implement the port, while ensuring a rejected
candidate cannot enter released dependency metadata or the workspace lock.

## Reproduction route

A repository script owns a small exact manifest and program as inert text,
creates a private temporary directory, runs the round-trip, portable compile
checks and authoritative audit, and emits a bounded disposition. The expected
current outcome is `oracle-qualified / production-rejected` only when the
round-trip and target checks pass and audit reports the exact known candidate
blocker. Any other audit result fails closed for human reassessment.

The script does not run in the fast pull-request lane because it resolves an
external graph and cross-compiles multiple targets. Its source and validation
tests run in the repository factory; execution belongs to manual/weekly
evidence and this issue's delivery receipt.

## Knowledge and debt

ADR 0192 and the encrypted-response knowledge page record the decision. The
quality declaration remains open and points to the exact deferral. A future
adoption changes the declaration to satisfied only after the production adapter
and independent decryptability test run from the supported repository graph.

## Failure and privacy behavior

The reproduction route uses a published test key and non-sensitive plaintext.
It emits no private operational key, product credential, state or presentation.
Temporary files are removed unless an explicit debugging option retains them.
Unexpected success, advisory changes, target failure or candidate drift is a
failure requiring a new review rather than automatic adoption.
