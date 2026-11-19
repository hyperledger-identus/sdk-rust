# Design

## Matched artifact comparison

The measurement command receives two package roots and their immutable Git
revisions. Each root must contain the same assembly-manifest layout. For every
target, artifacts are sorted by manifest path. The receipt records the sum of
raw bytes and the sum of deterministic gzip bytes for the same files, then
records candidate-minus-baseline deltas. Assembly and target digests prevent
mixing revisions or inventories.

## Initialization boundaries

Cold samples execute an isolated Node process that starts its timer immediately
before dynamic import of `dist/node.js`, awaits the import, and creates the DID
provider. Warm samples repeat the same dynamic import and provider creation in
one process after a warm-up, so the module cache and initialized WASM are in
the measured state. Provider construction is also measured independently
after import. Existing peer creation, parse, document, and resolution timings
remain separate.

The receipt names sample and warm-up counts, p50/p95 aggregation, Node/OS/CPU,
exact revisions, artifact definitions, and limitations. Child output is
strictly parsed; missing roots, mismatched inventories, invalid revisions, or
failed samples fail closed.

## Evidence labels

The benchmark declaration names only `package:identus-typescript-sdk` and
`runtime:nodejs`. Browser, Node.js, and React web parity target states become
`partial`: their target receipts remain linked, but none represents external
adoption, rollback rehearsal, accepted budgets, publication, or support.
React Native remains not applicable to this WASM slice.

## Receipt lifecycle

The repository owns the executable schema and validators. A local final run
uses PR #604's exact pre-peer base as baseline and the implementation commit as
candidate, stores the private receipt outside source, and publishes its digest
and summary as immutable PR evidence. Source documentation records definitions
and exact revisions without treating host-specific timing as a product budget.
