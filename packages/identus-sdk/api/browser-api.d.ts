import type { DidProvider, DidProviderOptions, DidQueryEffects, DidQueryProvider } from "./public.js";
/** Create the explicit browser-WASM DID provider. */
export declare function createDidProvider(options?: DidProviderOptions): Promise<DidProvider>;
/** Create the explicit browser-WASM DID query provider over injected effects. */
export declare function createDidQueryProvider(effects: DidQueryEffects, options?: DidProviderOptions): Promise<DidQueryProvider>;
