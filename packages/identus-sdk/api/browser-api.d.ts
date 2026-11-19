import type { DidProvider, DidProviderOptions, DidQueryEffects, InjectedDidProviderSelection, PeerDidProvider, PeerDidQueryProvider, PeerWasmDidProviderSelection } from "./public.js";
/** Create the explicit browser-WASM DID provider. */
export declare function createDidProvider(options?: DidProviderOptions): Promise<PeerDidProvider>;
/** Select an injected core provider or the peer-WASM capability before initialization. */
export declare function selectDidProvider<Provider extends DidProvider>(selection: InjectedDidProviderSelection<Provider>): Promise<Provider>;
export declare function selectDidProvider(selection: PeerWasmDidProviderSelection): Promise<PeerDidProvider>;
/** Create the explicit browser-WASM DID query provider over injected effects. */
export declare function createDidQueryProvider(effects: DidQueryEffects, options?: DidProviderOptions): Promise<PeerDidQueryProvider>;
