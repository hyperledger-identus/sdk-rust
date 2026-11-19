import type {
  DidProvider,
  DidProviderOptions,
  DidProviderSelection,
  DidQueryEffects,
  InjectedDidProviderSelection,
  PeerDidProvider,
  PeerDidQueryProvider,
  PeerWasmDidProviderSelection,
} from "./public.js";
import type { DidParseError as GeneratedDidParseError } from "../artifacts/nodejs/identus_wasm_did.js";
import { adaptGeneratedBinding } from "./internal/generated-binding.js";
import { retryableSingleton } from "./internal/initialization.js";
import {
  createRustDidProvider,
  createRustDidQueryProvider,
} from "./internal/provider.js";
import { selectDidProviderWith } from "./internal/selection.js";

let providerBinding: ReturnType<typeof adaptGeneratedBinding> | undefined;
const initialize = retryableSingleton(() =>
  import("../artifacts/nodejs/identus_wasm_did.js"));

async function loadBinding(): Promise<ReturnType<typeof adaptGeneratedBinding>> {
  const generated = await initialize();
  providerBinding ??= adaptGeneratedBinding(
    generated,
    (error): error is GeneratedDidParseError =>
      error instanceof generated.DidParseError,
  );
  return providerBinding;
}

/** Create the explicit Node.js-WASM DID provider. */
export async function createDidProvider(
  options: DidProviderOptions = {},
): Promise<PeerDidProvider> {
  return createRustDidProvider(await loadBinding(), "node-wasm", options);
}

/** Select an injected core provider or the peer-WASM capability before initialization. */
export function selectDidProvider<Provider extends DidProvider>(
  selection: InjectedDidProviderSelection<Provider>,
): Promise<Provider>;
export function selectDidProvider(
  selection: PeerWasmDidProviderSelection,
): Promise<PeerDidProvider>;
export function selectDidProvider(
  selection: DidProviderSelection,
): Promise<DidProvider> {
  return selectDidProviderWith(selection, createDidProvider);
}

/** Create the explicit Node.js-WASM DID query provider over injected effects. */
export async function createDidQueryProvider(
  effects: DidQueryEffects,
  options: DidProviderOptions = {},
): Promise<PeerDidQueryProvider> {
  return createRustDidQueryProvider(
    await loadBinding(),
    "node-wasm",
    effects,
    options,
  );
}
