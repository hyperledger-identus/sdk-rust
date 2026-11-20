import type {
  DidProvider,
  DidProviderOptions,
  DidProviderSelection,
  DidQueryEffects,
  PrismDidEffects,
  InjectedDidProviderSelection,
  PrismDidProvider,
  PrismDidQueryProvider,
  PeerWasmDidProviderSelection,
  KeyOperationsProvider,
  KeyOperationsProviderOptions,
  KeyOperationsProviderSelection,
  InjectedKeyOperationsProviderSelection,
  WasmKeyOperationsProviderSelection,
} from "./public.js";
import type { DidParseError as GeneratedDidParseError } from "../artifacts/web/identus_wasm_did.js";
import { adaptGeneratedBinding } from "./internal/generated-binding.js";
import { retryableSingleton } from "./internal/initialization.js";
import {
  createRustDidProvider,
  createRustDidQueryProvider,
} from "./internal/provider.js";
import { selectDidProviderWith } from "./internal/selection.js";
import { createRustKeyOperationsProvider } from "./internal/crypto-provider.js";

const initialize = retryableSingleton(async () => {
  const generated = await import("../artifacts/web/identus_wasm_did.js");
  await generated.default();
  return generated;
});
const initializeCrypto = retryableSingleton(async () => {
  const generated = await import("../artifacts/crypto/web/identus_wasm_crypto.js");
  await generated.default();
  return generated;
});

/** Create the explicit browser-WASM key-operations provider. */
export async function createKeyOperationsProvider(
  options: KeyOperationsProviderOptions = {},
): Promise<KeyOperationsProvider> {
  const generated = await initializeCrypto();
  return createRustKeyOperationsProvider({
    cryptoBindingApiVersion: generated.crypto_binding_api_version,
    generateEd25519: generated.Ed25519PrivateHandle.generate,
    importEd25519: generated.Ed25519PrivateHandle.importBytes,
    verifyEd25519: generated.verifyEd25519,
    generateX25519: generated.X25519PrivateHandle.generate,
    importX25519: generated.X25519PrivateHandle.importBytes,
  }, options);
}

/** Select an injected key provider or browser WASM before initialization. */
export function selectKeyOperationsProvider<Provider extends KeyOperationsProvider>(
  selection: InjectedKeyOperationsProviderSelection<Provider>,
): Promise<Provider>;
export function selectKeyOperationsProvider(
  selection: WasmKeyOperationsProviderSelection,
): Promise<KeyOperationsProvider>;
export function selectKeyOperationsProvider(
  selection: KeyOperationsProviderSelection,
): Promise<KeyOperationsProvider> {
  return selection.kind === "injected"
    ? Promise.resolve(selection.provider)
    : createKeyOperationsProvider(selection.options);
}

/** Create the explicit browser-WASM DID provider. */
export async function createDidProvider(
  options: DidProviderOptions = {},
): Promise<PrismDidProvider> {
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
): Promise<PrismDidProvider>;
export function selectDidProvider(
  selection: DidProviderSelection,
): Promise<DidProvider> {
  return selectDidProviderWith(selection, createDidProvider);
}

/** Create the explicit browser-WASM DID query provider over injected effects. */
export async function createDidQueryProvider(
  effects: DidQueryEffects & PrismDidEffects,
  options: DidProviderOptions = {},
): Promise<PrismDidQueryProvider> {
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
