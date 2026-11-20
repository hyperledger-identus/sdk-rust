import type { DidProvider, DidProviderOptions, DidQueryEffects, PrismDidEffects, InjectedDidProviderSelection, PrismDidProvider, PrismDidQueryProvider, PeerWasmDidProviderSelection, KeyOperationsProvider, KeyOperationsProviderOptions, InjectedKeyOperationsProviderSelection, WasmKeyOperationsProviderSelection, HdDerivationProvider, HdDerivationProviderOptions, InjectedHdDerivationProviderSelection, WasmHdDerivationProviderSelection } from "./public.js";
/** Create the explicit browser-WASM key-operations provider. */
export declare function createKeyOperationsProvider(options?: KeyOperationsProviderOptions): Promise<KeyOperationsProvider>;
/** Create the explicit browser-WASM HD-derivation provider. */
export declare function createHdDerivationProvider(options?: HdDerivationProviderOptions): Promise<HdDerivationProvider>;
/** Select an injected HD provider or browser WASM before initialization. */
export declare function selectHdDerivationProvider<Provider extends HdDerivationProvider>(selection: InjectedHdDerivationProviderSelection<Provider>): Promise<Provider>;
export declare function selectHdDerivationProvider(selection: WasmHdDerivationProviderSelection): Promise<HdDerivationProvider>;
/** Select an injected key provider or browser WASM before initialization. */
export declare function selectKeyOperationsProvider<Provider extends KeyOperationsProvider>(selection: InjectedKeyOperationsProviderSelection<Provider>): Promise<Provider>;
export declare function selectKeyOperationsProvider(selection: WasmKeyOperationsProviderSelection): Promise<KeyOperationsProvider>;
/** Create the explicit browser-WASM DID provider. */
export declare function createDidProvider(options?: DidProviderOptions): Promise<PrismDidProvider>;
/** Select an injected core provider or the peer-WASM capability before initialization. */
export declare function selectDidProvider<Provider extends DidProvider>(selection: InjectedDidProviderSelection<Provider>): Promise<Provider>;
export declare function selectDidProvider(selection: PeerWasmDidProviderSelection): Promise<PrismDidProvider>;
/** Create the explicit browser-WASM DID query provider over injected effects. */
export declare function createDidQueryProvider(effects: DidQueryEffects & PrismDidEffects, options?: DidProviderOptions): Promise<PrismDidQueryProvider>;
