import assert from "node:assert/strict";
import { createElement } from "react";
import { renderToString } from "react-dom/server";
import { test } from "node:test";

import { createDidProvider, createDidQueryProvider, createKeyOperationsProvider, selectDidProvider } from "../dist/node.js";
import { DidProviderBoundary, useDidProvider } from "../dist/react.js";
import { assertAllVectors } from "./vector-assertions.mjs";
import { loadVectors } from "./vector-loader.mjs";
import { assertAllDocumentVectors } from "./document-vector-assertions.mjs";
import { loadDocumentVectors } from "./document-vector-loader.mjs";
import { assertAllResolutionVectors } from "./resolution-vector-assertions.mjs";
import { loadResolutionVectors } from "./resolution-vector-loader.mjs";
import { assertAllPeerVectors } from "./peer-vector-assertions.mjs";
import { loadPeerVectors } from "./peer-vector-loader.mjs";
import {
  assertPrismHistoryVector,
  assertPrismStaticVectors,
} from "./prism-vector-assertions.mjs";
import {
  DID_ADOPTION_JOURNEY,
  createLegacyAdoptionFixture,
  runDidAdoptionRehearsal,
} from "./adoption-journey.mjs";
import { assertCryptoVectors } from "./crypto-vector-assertions.mjs";

const reactDocument = '{"id":"did:example:react-ssr"}';

function DidText({ document = reactDocument }) {
  const state = useDidProvider();
  return createElement(
    "span",
    { "data-state": state.status },
    state.status === "ready"
      ? state.provider.parseDidDocument(document).serialized
      : state.status,
  );
}

test("React SSR crosses the Node provider without duplicated DID logic", async () => {
  assert.equal(assertCryptoVectors(await createKeyOperationsProvider()), 2);
  const provider = await createDidProvider();
  assertAllVectors(provider, await loadVectors());
  assertAllDocumentVectors(provider, await loadDocumentVectors());
  assertAllPeerVectors(provider, await loadPeerVectors());
  assertPrismStaticVectors(provider);
  await assertPrismHistoryVector(createDidQueryProvider);
  await assertAllResolutionVectors(createDidQueryProvider, await loadResolutionVectors());
  const rehearsal = await runDidAdoptionRehearsal({
    target: "react-ssr",
    selectDidProvider,
    createDidQueryProvider,
  });
  assert.deepEqual(rehearsal.routes.map(({ route }) => route), ["legacy", "rust"]);
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

test("React SSR renders the same adoption document through both selected routes", async () => {
  const rust = await selectDidProvider({ kind: "peer-wasm" });
  const legacy = createLegacyAdoptionFixture().provider;
  const factory = async () => { throw new Error("SSR routes must be preselected"); };
  const render = (provider) => renderToString(createElement(
    DidProviderBoundary,
    { factory, initialProvider: provider },
    createElement(DidText, { document: DID_ADOPTION_JOURNEY.documentLiteral }),
  ));
  const expected = `<span data-state="ready">${DID_ADOPTION_JOURNEY.documentLiteral.replaceAll('"', "&quot;")}</span>`;
  assert.equal(render(legacy), expected);
  assert.equal(render(rust), expected);
});

test("React SSR renders a typed loading state before asynchronous initialization", () => {
  const markup = renderToString(createElement(
    DidProviderBoundary,
    { factory: async () => { throw new Error("not run during SSR"); } },
    createElement(DidText),
  ));
  assert.equal(markup, '<span data-state="loading">loading</span>');
});

test("React injects the original three-method provider without peer capability", () => {
  const peerless = Object.freeze({
    parseDid: (value) => Object.freeze({
      serialized: value,
      method: "example",
      methodSpecificId: "react",
    }),
    parseDidUrl: () => { throw new Error("unused peerless URL path"); },
    parseDidDocument: (value) => Object.freeze({
      serialized: value,
      id: Object.freeze({
        serialized: "did:example:react-ssr",
        method: "example",
        methodSpecificId: "react-ssr",
      }),
      context: undefined,
      controller: undefined,
      alsoKnownAs: undefined,
      verificationMethod: undefined,
      authentication: undefined,
      assertionMethod: undefined,
      keyAgreement: undefined,
      capabilityInvocation: undefined,
      capabilityDelegation: undefined,
      service: undefined,
      properties: Object.freeze({}),
    }),
  });
  const markup = renderToString(createElement(
    DidProviderBoundary,
    {
      factory: async () => { throw new Error("peerless provider is preselected"); },
      initialProvider: peerless,
    },
    createElement(DidText),
  ));
  assert.equal(markup, '<span data-state="ready">{&quot;id&quot;:&quot;did:example:react-ssr&quot;}</span>');
});
