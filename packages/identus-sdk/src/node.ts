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
  HdDerivationProvider,
  HdDerivationProviderOptions,
  HdDerivationProviderSelection,
  InjectedHdDerivationProviderSelection,
  WasmHdDerivationProviderSelection,
} from "./public.js";
import type { DidParseError as GeneratedDidParseError } from "../artifacts/nodejs/identus_wasm_did.js";
import { adaptGeneratedBinding } from "./internal/generated-binding.js";
import { retryableSingleton } from "./internal/initialization.js";
import {
  createRustDidProvider,
  createRustDidQueryProvider,
} from "./internal/provider.js";
import { selectDidProviderWith } from "./internal/selection.js";
import { createRustKeyOperationsProvider } from "./internal/crypto-provider.js";
import { createRustHdDerivationProvider } from "./internal/hd-provider.js";

let providerBinding: ReturnType<typeof adaptGeneratedBinding> | undefined;
const initialize = retryableSingleton(() =>
  import("../artifacts/nodejs/identus_wasm_did.js"));
const initializeCrypto = retryableSingleton(() =>
  import("../artifacts/crypto/nodejs/identus_wasm_crypto.js"));

/** Create the explicit Node.js-WASM key-operations provider. */
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

/** Create the explicit Node.js-WASM HD-derivation provider. */
export async function createHdDerivationProvider(
  options: HdDerivationProviderOptions = {},
): Promise<HdDerivationProvider> {
  const generated = await initializeCrypto();
  return createRustHdDerivationProvider({
    hdDerivationBindingApiVersion: generated.hd_derivation_binding_api_version,
    createRoot: generated.HdPrivateHandle.fromSeed,
  }, options);
}

/** Select an injected HD provider or Node.js WASM before initialization. */
export function selectHdDerivationProvider<Provider extends HdDerivationProvider>(
  selection: InjectedHdDerivationProviderSelection<Provider>,
): Promise<Provider>;
export function selectHdDerivationProvider(
  selection: WasmHdDerivationProviderSelection,
): Promise<HdDerivationProvider>;
export function selectHdDerivationProvider(
  selection: HdDerivationProviderSelection,
): Promise<HdDerivationProvider> {
  return selection.kind === "injected"
    ? Promise.resolve(selection.provider)
    : createHdDerivationProvider(selection.options);
}

/** Select an injected key provider or Node.js WASM before initialization. */
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
): Promise<PrismDidProvider> {
  return createRustDidProvider(await loadBinding(), "node-wasm", options);
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

/** Create the explicit Node.js-WASM DID query provider over injected effects. */
export async function createDidQueryProvider(
  effects: DidQueryEffects & PrismDidEffects,
  options: DidProviderOptions = {},
): Promise<PrismDidQueryProvider> {
  return createRustDidQueryProvider(
    await loadBinding(),
    "node-wasm",
    effects,
    options,
  );
}
