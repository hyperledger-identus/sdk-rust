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
  const result = await page.evaluate(async (vectors) => {
    const { createDidProvider } = await import("/dist/browser.js");
    const { assertAllVectors } = await import("/tests/vector-assertions.mjs");
    const provider = await createDidProvider();
    try {
      assertAllVectors(provider, vectors);
      return { status: "success", count: vectors.length };
    } catch {
      return { status: "failure", count: vectors.length };
    }
  }, await loadVectors());
  if (result.count !== 22 || result.status !== "success") {
    throw new Error("browser DID vector canary failed");
  }
  process.stdout.write("browser DID vectors: 22 passed\n");
} finally {
  await browser?.close();
  await new Promise((resolve, reject) => server.close((error) => error ? reject(error) : resolve()));
}
