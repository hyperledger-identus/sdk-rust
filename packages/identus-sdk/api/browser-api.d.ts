import type { DidProvider, DidProviderOptions } from "./public.js";
/** Create the explicit browser-WASM DID provider. */
export declare function createDidProvider(options?: DidProviderOptions): Promise<DidProvider>;
