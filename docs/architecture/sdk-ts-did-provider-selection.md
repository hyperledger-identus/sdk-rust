# SDK-TS DID provider capability selection

The TypeScript DID facade preserves a three-method `DidProvider` seam:
`parseDid`, `parseDidUrl`, and `parseDidDocument`. Peer DID operations are an
explicit `PeerDidProvider` capability. Code that injected a core-only provider
before peer DID support therefore remains assignable to the React boundary and
to `DidProviderFactory`.

Browser and Node consumers choose a provider before initialization:

```ts
import { selectDidProvider } from "@hyperledger/identus-sdk/browser";

const provider = await selectDidProvider({
  kind: "injected",
  provider: coreProvider,
});
```

This route does not import or initialize the generated WASM artifact. Consumers
that need peer DID methods select the peer-WASM capability instead:

```ts
const provider = await selectDidProvider({ kind: "peer-wasm" });
provider.createPeerDidNumalgo4(inputDocument);
```

Selection is terminal. A rejected initialization or operation is returned to
the caller with the stable public error contract; the facade never switches to
an injected provider or another artifact automatically. Rollback is an explicit
deployment or application choice made by selecting the alternative before the
first operation.

React uses the same boundary for both capabilities. A core-only provider may be
passed as `initialProvider`, while `createDidProvider` remains strongly typed as
`PeerDidProvider` for applications that require peer methods. No second context
or lifecycle implementation exists.
