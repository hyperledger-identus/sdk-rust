# Align peer DID measurement and parity evidence

## Why

The peer DID TypeScript facade has private functional target receipts, but its
benchmark declaration overstates the measured runtimes and its retained
measurement does not compare a pre-peer baseline with the candidate or time
import-inclusive initialization. The roadmap also labels private target proof
as delivered support before the adoption, rollback, and budget gates exist.

## What Changes

- Retain one revision-bound receipt that compares the pre-peer baseline and
  peer-enabled candidate using the same artifact and compression definitions.
- Measure import-inclusive cold and warm Node initialization separately from
  provider construction and peer operation latency.
- Restrict the benchmark declaration to the package and Node runtime actually
  measured.
- Mark browser, Node.js, and React web peer target states as partial while
  preserving their private functional receipts and React Native disposition.

## Capabilities

### Modified Capabilities

- `sdk-ts-peer-did-method`: require matched baseline/candidate size and Node
  initialization evidence, exact benchmark applicability, and honest parity
  labels for private target proof.

## Impact

This is measurement- and evidence-only. It changes no public API, peer DID
wire value, stable error, production dependency, supported target, publication
state, external repository, release decision, accepted performance budget, or
mdoc scope.
