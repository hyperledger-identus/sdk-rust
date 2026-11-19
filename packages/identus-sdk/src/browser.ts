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
import type { DidParseError as GeneratedDidParseError } from "../artifacts/web/identus_wasm_did.js";
import { adaptGeneratedBinding } from "./internal/generated-binding.js";
import { retryableSingleton } from "./internal/initialization.js";
import {
  createRustDidProvider,
  createRustDidQueryProvider,
} from "./internal/provider.js";
import { selectDidProviderWith } from "./internal/selection.js";

const initialize = retryableSingleton(async () => {
  const generated = await import("../artifacts/web/identus_wasm_did.js");
  await generated.default();
  return generated;
});

/** Create the explicit browser-WASM DID provider. */
export async function createDidProvider(
  options: DidProviderOptions = {},
): Promise<PeerDidProvider> {
  const generated = await initialize();
  return createRustDidProvider(
    adaptGeneratedBinding(
      generated,
      (error): error is GeneratedDidParseError =>
        error instanceof generated.DidParseError,
    ),
    "browser-wasm",
    options,
  );
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

/** Create the explicit browser-WASM DID query provider over injected effects. */
export async function createDidQueryProvider(
  effects: DidQueryEffects,
  options: DidProviderOptions = {},
): Promise<PeerDidQueryProvider> {
  const generated = await initialize();
  return createRustDidQueryProvider(
    adaptGeneratedBinding(
      generated,
      (error): error is GeneratedDidParseError =>
        error instanceof generated.DidParseError,
    ),
    "browser-wasm",
    effects,
    options,
  );
}
