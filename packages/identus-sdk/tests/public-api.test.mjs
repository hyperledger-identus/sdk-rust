import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";

const packageRoot = new URL("../", import.meta.url);

test("runtime root exports only the public error value", async () => {
  const exports = Object.keys(await import("../dist/index.js"));
  assert.deepEqual(exports, ["DidError"]);
});

test("the reviewed public declaration names contain no historical components", async () => {
  const declaration = await readFile(new URL("dist/public.d.ts", packageRoot), "utf8");
  const reviewed = await readFile(new URL("api/public-api.d.ts", packageRoot), "utf8");
  const snapshot = await readFile(new URL("api/public-exports.txt", packageRoot), "utf8");
  const forbidden = ["apollo", "castor", "mercury", "pluto", "pollux", "edgeagent"];
  const publicText = `${declaration}\n${snapshot}`.toLowerCase().replaceAll(/[^a-z]/gu, "");

  for (const name of forbidden) {
    assert.equal(publicText.includes(name), false, `forbidden public name: ${name}`);
  }
  assert.equal(
    declaration.replace(/^\/\/# sourceMappingURL=.*$/gmu, "").trim(),
    reviewed.trim(),
  );
  assert.equal(snapshot.trim().split("\n").length, 12);
});

test("target entries match reviewed declarations and expose only their capability surface", async () => {
  for (const target of ["browser", "node", "react"]) {
    const declaration = await readFile(new URL(`dist/${target}.d.ts`, packageRoot), "utf8");
    const reviewed = await readFile(new URL(`api/${target}-api.d.ts`, packageRoot), "utf8");
    assert.equal(
      declaration.replace(/^\/\/# sourceMappingURL=.*$/gmu, "").trim(),
      reviewed.trim(),
      target,
    );
  }
  assert.deepEqual(Object.keys(await import("../dist/browser.js")), ["createDidProvider"]);
  assert.deepEqual(Object.keys(await import("../dist/node.js")), ["createDidProvider"]);
  assert.deepEqual(
    Object.keys(await import("../dist/react.js")),
    ["DidProviderBoundary", "useDidProvider"],
  );
});
