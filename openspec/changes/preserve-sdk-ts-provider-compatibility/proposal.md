# Preserve SDK-TS DID provider compatibility

## Why

The private TypeScript peer DID facade added required peer methods directly to
`DidProvider`. That makes previously valid three-method injected providers
unassignable and leaves rollback as a unit-test sketch rather than a real
pre-initialization browser/Node provider choice.

## What Changes

- Restore `DidProvider` as the stable three-method core seam.
- Add an explicit `PeerDidProvider` capability and retain strong peer typing on
  the Rust/WASM factories.
- Add injected-versus-peer-WASM selection before generated artifact
  initialization, with no automatic fallback after selection.
- Prove core-only injection through browser, Node ESM/CommonJS, React, and
  reviewed declarations.

## Capabilities

### Modified Capabilities

- `sdk-ts-peer-did-method`: require backward-assignable core providers and an
  explicit terminal capability choice before initialization.

## Impact

This is a compatible correction to the private, unpublished TypeScript facade.
It changes TypeScript declarations and target entry points but no Rust API,
wire format, peer DID identifier, document, stable error, or external SDK-TS
repository. React Native, publication, default activation, release, and mdoc
remain excluded.
