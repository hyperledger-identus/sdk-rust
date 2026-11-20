// SPDX-License-Identifier: Apache-2.0

import type {
  DidProvider,
  DidProviderOptions,
  DidProviderSelection,
  InjectedDidProviderSelection,
  PrismDidProvider,
  PeerWasmDidProviderSelection,
} from "../public.js";

export type PeerDidProviderInitializer = (
  options?: DidProviderOptions,
) => Promise<PrismDidProvider>;

export function selectDidProviderWith<Provider extends DidProvider>(
  selection: InjectedDidProviderSelection<Provider>,
  initializePeerProvider: PeerDidProviderInitializer,
): Promise<Provider>;

export function selectDidProviderWith(
  selection: PeerWasmDidProviderSelection,
  initializePeerProvider: PeerDidProviderInitializer,
): Promise<PrismDidProvider>;

export function selectDidProviderWith(
  selection: DidProviderSelection,
  initializePeerProvider: PeerDidProviderInitializer,
): Promise<DidProvider>;

export async function selectDidProviderWith(
  selection: DidProviderSelection,
  initializePeerProvider: PeerDidProviderInitializer,
): Promise<DidProvider> {
  if (selection.kind === "injected") {
    return selection.provider;
  }
  return initializePeerProvider(selection.options);
}
