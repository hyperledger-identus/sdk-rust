import assert from "node:assert/strict";
import { createElement } from "react";
import { renderToString } from "react-dom/server";
import { test } from "node:test";

import { createDidProvider, createDidQueryProvider } from "../dist/node.js";
import { DidProviderBoundary, useDidProvider } from "../dist/react.js";
import { assertAllVectors } from "./vector-assertions.mjs";
import { loadVectors } from "./vector-loader.mjs";
import { assertAllDocumentVectors } from "./document-vector-assertions.mjs";
import { loadDocumentVectors } from "./document-vector-loader.mjs";
import { assertAllResolutionVectors } from "./resolution-vector-assertions.mjs";
import { loadResolutionVectors } from "./resolution-vector-loader.mjs";
import { assertAllPeerVectors } from "./peer-vector-assertions.mjs";
import { loadPeerVectors } from "./peer-vector-loader.mjs";

const reactDocument = '{"id":"did:example:react-ssr"}';

function DidText() {
  const state = useDidProvider();
  return createElement(
    "span",
    { "data-state": state.status },
    state.status === "ready"
      ? state.provider.parseDidDocument(reactDocument).serialized
      : state.status,
  );
}

test("React SSR crosses the Node provider without duplicated DID logic", async () => {
  const provider = await createDidProvider();
  assertAllVectors(provider, await loadVectors());
  assertAllDocumentVectors(provider, await loadDocumentVectors());
  assertAllPeerVectors(provider, await loadPeerVectors());
  await assertAllResolutionVectors(createDidQueryProvider, await loadResolutionVectors());
  const factory = async () => {
    throw new Error("SSR must use the explicit initial provider");
  };
  const markup = renderToString(createElement(
    DidProviderBoundary,
    { factory, initialProvider: provider },
    createElement(DidText),
  ));
  assert.equal(markup, '<span data-state="ready">{&quot;id&quot;:&quot;did:example:react-ssr&quot;}</span>');
});

test("React SSR renders a typed loading state before asynchronous initialization", () => {
  const markup = renderToString(createElement(
    DidProviderBoundary,
    { factory: async () => { throw new Error("not run during SSR"); } },
    createElement(DidText),
  ));
  assert.equal(markup, '<span data-state="loading">loading</span>');
});
