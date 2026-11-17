import initWasm, * as generated from "../artifacts/web/identus_wasm_did.js";

import type { DidProvider, DidProviderOptions } from "./public.js";
import { adaptGeneratedBinding } from "./internal/generated-binding.js";
import { retryableSingleton } from "./internal/initialization.js";
import { createRustDidProvider } from "./internal/provider.js";

const initialize = retryableSingleton(() => initWasm().then(() => undefined));

/** Create the explicit browser-WASM DID provider. */
export async function createDidProvider(
  options: DidProviderOptions = {},
): Promise<DidProvider> {
  await initialize();
  return createRustDidProvider(
    adaptGeneratedBinding(
      generated,
      (error): error is generated.DidParseError =>
        error instanceof generated.DidParseError,
    ),
    "browser-wasm",
    options,
  );
}
