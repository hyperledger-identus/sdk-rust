import { createElement, useEffect } from "react";
import { hydrateRoot } from "react-dom/client";

import { createDidProvider, createDidQueryProvider, selectDidProvider } from "../dist/browser.js";
import { DidProviderBoundary, useDidProvider } from "../dist/react.js";
import { assertAllVectors } from "./vector-assertions.mjs";
import { assertAllDocumentVectors } from "./document-vector-assertions.mjs";
import { assertAllResolutionVectors } from "./resolution-vector-assertions.mjs";
import { assertAllPeerVectors } from "./peer-vector-assertions.mjs";
import {
  DID_ADOPTION_JOURNEY,
  createLegacyAdoptionFixture,
  runDidAdoptionRehearsal,
} from "./adoption-journey.mjs";

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
      ? state.provider.parseDidDocument(DID_ADOPTION_JOURNEY.documentLiteral).serialized
      : state.status,
  );
}

try {
  const provider = await createDidProvider();
  const legacy = createLegacyAdoptionFixture().provider;
  const vectors = window.__identusDidVectors;
  const documentVectors = window.__identusDidDocumentVectors;
  const resolutionVectors = window.__identusDidResolutionVectors;
  const peerVectors = window.__identusPeerDidVectors;
  if (vectors === undefined) throw new Error("vectors missing");
  if (documentVectors === undefined) throw new Error("document vectors missing");
  if (resolutionVectors === undefined) throw new Error("resolution vectors missing");
  if (peerVectors === undefined) throw new Error("peer vectors missing");
  const rustRoot = document.getElementById("rust-root");
  const legacyRoot = document.getElementById("legacy-root");
  if (rustRoot === null || legacyRoot === null) throw new Error("root missing");
  const recoverable: string[] = [];
  const hydrate = (root: HTMLElement, initialProvider: typeof provider | typeof legacy) => new Promise<void>((resolve) => {
    const onHydrated = () => resolve();
    hydrateRoot(
      root,
      createElement(
        DidProviderBoundary,
        { factory: async () => initialProvider, initialProvider },
        createElement(DidText, { onHydrated }),
      ),
      { onRecoverableError: () => recoverable.push("recoverable") },
    );
  });
  await Promise.all([hydrate(rustRoot, provider), hydrate(legacyRoot, legacy)]);
  const rehearsal = await runDidAdoptionRehearsal({
    target: "react-hydrated-client",
    selectDidProvider,
    createDidProvider,
    createDidQueryProvider,
  });
  assertAllVectors(provider, vectors);
  assertAllDocumentVectors(provider, documentVectors);
  assertAllPeerVectors(provider, peerVectors);
  await assertAllResolutionVectors(createDidQueryProvider, resolutionVectors);
  const text = rustRoot.textContent ?? "";
  if (
    text !== DID_ADOPTION_JOURNEY.documentLiteral
    || legacyRoot.textContent !== text
    || rehearsal.routes.length !== 2
    || recoverable.length !== 0
  ) {
    throw new Error("hydration mismatch");
  }
  window.__identusReactCanary = Object.freeze({ status: "success", detail: text });
} catch {
  window.__identusReactCanary = Object.freeze({ status: "failure", detail: "redacted" });
}
