import type { DidProvider, DidProviderOptions } from "./public.js";
/** Create the explicit Node.js-WASM DID provider. */
export declare function createDidProvider(options?: DidProviderOptions): Promise<DidProvider>;
