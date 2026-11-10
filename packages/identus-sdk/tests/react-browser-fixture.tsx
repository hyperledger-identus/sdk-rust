import { createElement } from "react";
import { hydrateRoot } from "react-dom/client";

import { createDidProvider } from "../dist/browser.js";
import { DidProviderBoundary, useDidProvider } from "../dist/react.js";

declare global {
  interface Window {
    __identusReactCanary?: Readonly<{
      readonly status: "success" | "failure";
      readonly detail: string;
    }>;
  }
}

function DidText() {
  const state = useDidProvider();
  return createElement(
    "span",
    { "data-state": state.status },
    state.status === "ready"
      ? state.provider.parseDid("did:example:react-hydration").serialized
      : state.status,
  );
}

try {
  const provider = await createDidProvider();
  const root = document.getElementById("root");
  if (root === null) throw new Error("root missing");
  const recoverable: string[] = [];
  hydrateRoot(
    root,
    createElement(
      DidProviderBoundary,
      { factory: async () => provider, initialProvider: provider },
      createElement(DidText),
    ),
    { onRecoverableError: () => recoverable.push("recoverable") },
  );
  await new Promise((resolve) => window.setTimeout(resolve, 0));
  const text = root.textContent ?? "";
  if (text !== "did:example:react-hydration" || recoverable.length !== 0) {
    throw new Error("hydration mismatch");
  }
  window.__identusReactCanary = Object.freeze({ status: "success", detail: text });
} catch {
  window.__identusReactCanary = Object.freeze({ status: "failure", detail: "redacted" });
}
