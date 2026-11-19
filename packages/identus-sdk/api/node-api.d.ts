import type { DidProvider, DidProviderOptions, DidQueryEffects, PrismDidEffects, InjectedDidProviderSelection, PrismDidProvider, PrismDidQueryProvider, PeerWasmDidProviderSelection } from "./public.js";
/** Create the explicit Node.js-WASM DID provider. */
export declare function createDidProvider(options?: DidProviderOptions): Promise<PrismDidProvider>;
/** Select an injected core provider or the peer-WASM capability before initialization. */
export declare function selectDidProvider<Provider extends DidProvider>(selection: InjectedDidProviderSelection<Provider>): Promise<Provider>;
export declare function selectDidProvider(selection: PeerWasmDidProviderSelection): Promise<PrismDidProvider>;
/** Create the explicit Node.js-WASM DID query provider over injected effects. */
export declare function createDidQueryProvider(effects: DidQueryEffects & PrismDidEffects, options?: DidProviderOptions): Promise<PrismDidQueryProvider>;
