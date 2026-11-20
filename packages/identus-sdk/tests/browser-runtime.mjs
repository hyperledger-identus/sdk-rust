#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { createReadStream, existsSync } from "node:fs";
import { stat } from "node:fs/promises";
import { createServer } from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright-core";

import { loadVectors } from "./vector-loader.mjs";
import { loadDocumentVectors } from "./document-vector-loader.mjs";
import { loadResolutionVectors } from "./resolution-vector-loader.mjs";
import { loadPeerVectors } from "./peer-vector-loader.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function browserExecutable() {
  const configured = process.env.CHROMIUM_PATH;
  if (configured && existsSync(configured)) return configured;
  for (const command of ["chromium", "chromium-browser", "google-chrome"]) {
    try {
      return execFileSync("which", [command], { encoding: "utf8" }).trim();
    } catch {
      // Try the next explicit executable name.
    }
  }
  const mac = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
  if (existsSync(mac)) return mac;
  throw new Error("Chromium executable is unavailable");
}

const mime = new Map([
  [".html", "text/html; charset=utf-8"],
  [".js", "text/javascript; charset=utf-8"],
  [".mjs", "text/javascript; charset=utf-8"],
  [".wasm", "application/wasm"],
]);

const server = createServer(async (request, response) => {
  try {
    const url = new URL(request.url ?? "/", "http://localhost");
    const relative = url.pathname === "/" ? "tests/browser-fixture.html" : url.pathname.slice(1);
    const file = path.resolve(root, relative);
    if (!file.startsWith(`${root}${path.sep}`) || !(await stat(file)).isFile()) {
      response.writeHead(404).end();
      return;
    }
    response.setHeader("content-type", mime.get(path.extname(file)) ?? "application/octet-stream");
    createReadStream(file).pipe(response);
  } catch {
    response.writeHead(404).end();
  }
});

await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
const address = server.address();
if (address === null || typeof address === "string") throw new Error("server address unavailable");

let browser;
try {
  browser = await chromium.launch({ executablePath: browserExecutable(), headless: true });
  const page = await browser.newPage();
  await page.goto(`http://127.0.0.1:${address.port}/`);
  const result = await page.evaluate(async ({ didVectors, documentVectors, resolutionVectors, peerVectors }) => {
    const {
      createDidProvider,
      createDidQueryProvider,
      selectDidProvider,
    } = await import("/dist/browser.js");
    const { assertAllVectors } = await import("/tests/vector-assertions.mjs");
    const { assertAllDocumentVectors } = await import("/tests/document-vector-assertions.mjs");
    const { assertAllResolutionVectors } = await import("/tests/resolution-vector-assertions.mjs");
    const { assertAllPeerVectors } = await import("/tests/peer-vector-assertions.mjs");
    const { assertPrismHistoryVector, assertPrismStaticVectors } = await import("/tests/prism-vector-assertions.mjs");
    const { runDidAdoptionRehearsal } = await import("/tests/adoption-journey.mjs");
    const peerless = Object.freeze({
      parseDid: (value) => Object.freeze({
        serialized: value,
        method: "example",
        methodSpecificId: "selected",
      }),
      parseDidUrl: () => { throw new Error("unused peerless URL path"); },
      parseDidDocument: () => { throw new Error("unused peerless document path"); },
    });
    const selectedPeerless = await selectDidProvider({
      kind: "injected",
      provider: peerless,
    });
    if (selectedPeerless !== peerless) {
      return { status: "failure", didCount: didVectors.length, documentCount: documentVectors.length, resolutionCount: resolutionVectors.length, peerCount: peerVectors.length };
    }
    const provider = await createDidProvider();
    try {
      assertAllVectors(provider, didVectors);
      assertAllDocumentVectors(provider, documentVectors);
      assertAllPeerVectors(provider, peerVectors);
      assertPrismStaticVectors(provider);
      await assertPrismHistoryVector(createDidQueryProvider);
      await assertAllResolutionVectors(createDidQueryProvider, resolutionVectors);
      const rehearsal = await runDidAdoptionRehearsal({
        target: "browser",
        selectDidProvider,
        createDidQueryProvider,
      });
      return { status: "success", didCount: didVectors.length, documentCount: documentVectors.length, resolutionCount: resolutionVectors.length, peerCount: peerVectors.length, adoptionRoutes: rehearsal.routes.length };
    } catch {
      return { status: "failure", didCount: didVectors.length, documentCount: documentVectors.length, resolutionCount: resolutionVectors.length, peerCount: peerVectors.length };
    }
  }, {
    didVectors: await loadVectors(),
    documentVectors: await loadDocumentVectors(),
    resolutionVectors: await loadResolutionVectors(),
    peerVectors: await loadPeerVectors(),
  });
  if (result.didCount !== 22 || result.documentCount < 1 || result.resolutionCount < 1 || result.peerCount !== 13 || result.adoptionRoutes !== 2 || result.status !== "success") {
    throw new Error("browser DID, DID Document, and DID Resolution vector canary failed");
  }
  process.stdout.write(`browser DID vectors: ${result.didCount} passed; DID Document vectors: ${result.documentCount} passed; DID Resolution vectors: ${result.resolutionCount} passed; peer DID vectors: ${result.peerCount} passed\n`);
} finally {
  await browser?.close();
  await new Promise((resolve, reject) => server.close((error) => error ? reject(error) : resolve()));
}
