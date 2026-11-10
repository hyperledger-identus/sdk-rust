import assert from "node:assert/strict";
import { createElement } from "react";
import { renderToString } from "react-dom/server";
import { test } from "node:test";

import { createDidProvider } from "../dist/node.js";
import { DidProviderBoundary, useDidProvider } from "../dist/react.js";
import { assertAllVectors } from "./vector-assertions.mjs";
import { loadVectors } from "./vector-loader.mjs";

function DidText() {
  const state = useDidProvider();
  return createElement(
    "span",
    { "data-state": state.status },
    state.status === "ready"
      ? state.provider.parseDid("did:example:react-ssr").serialized
      : state.status,
  );
}

test("React SSR crosses the Node provider without duplicated DID logic", async () => {
  const provider = await createDidProvider();
  assertAllVectors(provider, await loadVectors());
  const factory = async () => {
    throw new Error("SSR must use the explicit initial provider");
  };
  const markup = renderToString(createElement(
    DidProviderBoundary,
    { factory, initialProvider: provider },
    createElement(DidText),
  ));
  assert.equal(markup, '<span data-state="ready">did:example:react-ssr</span>');
});

test("React SSR renders a typed loading state before asynchronous initialization", () => {
  const markup = renderToString(createElement(
    DidProviderBoundary,
    { factory: async () => { throw new Error("not run during SSR"); } },
    createElement(DidText),
  ));
  assert.equal(markup, '<span data-state="loading">loading</span>');
});
