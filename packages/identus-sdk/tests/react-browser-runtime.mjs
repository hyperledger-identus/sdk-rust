#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { rm, writeFile } from "node:fs/promises";
import path from "node:path";
import { createElement } from "react";
import { renderToString } from "react-dom/server";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright-core";
import { createServer } from "vite";

import { createDidProvider as createNodeDidProvider } from "../dist/node.js";
import { DidProviderBoundary, useDidProvider } from "../dist/react.js";
import { loadVectors } from "./vector-loader.mjs";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const fixture = path.join(root, "tests/.react-runtime.html");

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

const nodeProvider = await createNodeDidProvider();
const vectors = await loadVectors();
const markup = renderToString(createElement(
  DidProviderBoundary,
  { factory: async () => nodeProvider, initialProvider: nodeProvider },
  createElement(DidText),
));
const serializedVectors = JSON.stringify(vectors).replaceAll("<", "\\u003c");
await writeFile(
  fixture,
  `<!doctype html><html><body><div id="root">${markup}</div><script>window.__identusDidVectors=${serializedVectors}</script><script type="module" src="/tests/react-browser-fixture.tsx"></script></body></html>\n`,
  { flag: "wx" },
);

let vite;
let browser;
try {
  vite = await createServer({ root, server: { host: "127.0.0.1", port: 0 }, logLevel: "error" });
  await vite.listen();
  const address = vite.httpServer?.address();
  if (address === null || address === undefined || typeof address === "string") {
    throw new Error("Vite address unavailable");
  }
  browser = await chromium.launch({ executablePath: browserExecutable(), headless: true });
  const page = await browser.newPage();
  await page.goto(`http://127.0.0.1:${address.port}/tests/.react-runtime.html`);
  await page.waitForFunction(() => window.__identusReactCanary !== undefined);
  const result = await page.evaluate(() => window.__identusReactCanary);
  if (result?.status !== "success" || result.detail !== "did:example:react-hydration") {
    throw new Error("React hydration canary failed");
  }
  process.stdout.write("React Node SSR/browser hydration: passed\n");
} finally {
  await browser?.close();
  await vite?.close();
  await rm(fixture, { force: true });
}
