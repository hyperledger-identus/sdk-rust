import * as generated from "../artifacts/nodejs/identus_wasm_did.js";

import type { DidProvider, DidProviderOptions } from "./public.js";
import { adaptGeneratedBinding } from "./internal/generated-binding.js";
import { createRustDidProvider } from "./internal/provider.js";

let providerBinding: ReturnType<typeof adaptGeneratedBinding> | undefined;

/** Create the explicit Node.js-WASM DID provider. */
export async function createDidProvider(
  options: DidProviderOptions = {},
): Promise<DidProvider> {
  providerBinding ??= adaptGeneratedBinding(
    generated,
    (error): error is generated.DidParseError =>
      error instanceof generated.DidParseError,
  );
  return createRustDidProvider(providerBinding, "node-wasm", options);
}
