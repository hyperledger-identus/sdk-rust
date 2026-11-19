import { createElement, useEffect } from "react";
import { hydrateRoot } from "react-dom/client";

import { createDidProvider, createDidQueryProvider } from "../dist/browser.js";
import { DidProviderBoundary, useDidProvider } from "../dist/react.js";
import { assertAllVectors } from "./vector-assertions.mjs";
import { assertAllDocumentVectors } from "./document-vector-assertions.mjs";
import { assertAllResolutionVectors } from "./resolution-vector-assertions.mjs";
import { assertAllPeerVectors } from "./peer-vector-assertions.mjs";

declare global {
  interface Window {
    __identusReactCanary?: Readonly<{
      readonly status: "success" | "failure";
      readonly detail: string;
    }>;
    __identusDidVectors?: readonly unknown[];
    __identusDidDocumentVectors?: readonly unknown[];
    __identusDidResolutionVectors?: readonly unknown[];
    __identusPeerDidVectors?: readonly unknown[];
  }
}

function DidText({ onHydrated }: Readonly<{ onHydrated: () => void }>) {
  const state = useDidProvider();
  useEffect(onHydrated, [onHydrated]);
  return createElement(
    "span",
    { "data-state": state.status },
    state.status === "ready"
      ? state.provider.parseDidDocument('{"id":"did:example:react-hydration"}').serialized
      : state.status,
  );
}

try {
  const provider = await createDidProvider();
  const vectors = window.__identusDidVectors;
  const documentVectors = window.__identusDidDocumentVectors;
  const resolutionVectors = window.__identusDidResolutionVectors;
  const peerVectors = window.__identusPeerDidVectors;
  if (vectors === undefined) throw new Error("vectors missing");
  if (documentVectors === undefined) throw new Error("document vectors missing");
  if (resolutionVectors === undefined) throw new Error("resolution vectors missing");
  if (peerVectors === undefined) throw new Error("peer vectors missing");
  const root = document.getElementById("root");
  if (root === null) throw new Error("root missing");
  const recoverable: string[] = [];
  let resolveHydrated: (() => void) | undefined;
  const hydrated = new Promise<void>((resolve) => { resolveHydrated = resolve; });
  const onHydrated = () => resolveHydrated?.();
  hydrateRoot(
    root,
    createElement(
      DidProviderBoundary,
      { factory: async () => provider, initialProvider: provider },
      createElement(DidText, { onHydrated }),
    ),
    { onRecoverableError: () => recoverable.push("recoverable") },
  );
  await hydrated;
  assertAllVectors(provider, vectors);
  assertAllDocumentVectors(provider, documentVectors);
  assertAllPeerVectors(provider, peerVectors);
  await assertAllResolutionVectors(createDidQueryProvider, resolutionVectors);
  const text = root.textContent ?? "";
  if (text !== '{"id":"did:example:react-hydration"}' || recoverable.length !== 0) {
    throw new Error("hydration mismatch");
  }
  window.__identusReactCanary = Object.freeze({ status: "success", detail: text });
} catch {
  window.__identusReactCanary = Object.freeze({ status: "failure", detail: "redacted" });
}
