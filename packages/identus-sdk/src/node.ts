import * as generated from "../artifacts/nodejs/identus_wasm_did.js";

import type {
  DidProvider,
  DidProviderOptions,
  DidQueryEffects,
  DidQueryProvider,
} from "./public.js";
import { adaptGeneratedBinding } from "./internal/generated-binding.js";
import {
  createRustDidProvider,
  createRustDidQueryProvider,
} from "./internal/provider.js";

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

/** Create the explicit Node.js-WASM DID query provider over injected effects. */
export async function createDidQueryProvider(
  effects: DidQueryEffects,
  options: DidProviderOptions = {},
): Promise<DidQueryProvider> {
  providerBinding ??= adaptGeneratedBinding(
    generated,
    (error): error is generated.DidParseError =>
      error instanceof generated.DidParseError,
  );
  return createRustDidQueryProvider(
    providerBinding,
    "node-wasm",
    effects,
    options,
  );
}
