#!/usr/bin/env node
// SPDX-License-Identifier: Apache-2.0

import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import {
  chmod,
  copyFile,
  lstat,
  mkdir,
  readFile,
  readdir,
  writeFile,
} from "node:fs/promises";
import path from "node:path";
import process from "node:process";
import { fileURLToPath, pathToFileURL } from "node:url";
import { parseJsonWithoutDuplicates } from "./factory-tools/strict-json.mjs";

const bindingApiVersion = 1;
const artifactBase = "identus_wasm_did";
const artifactFiles = Object.freeze([
  `${artifactBase}.d.ts`,
  `${artifactBase}.js`,
  `${artifactBase}_bg.wasm`,
  `${artifactBase}_bg.wasm.d.ts`,
  "package.json",
]);
const generatedFiles = Object.freeze([".gitignore", ...artifactFiles].sort());
const legacyNames = /apollo|castor|mercury|pluto|pollux|edge[ _-]?agent/iu;
const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function fail(message) {
  throw new Error(message);
}

function parseArguments(argv) {
  if (argv[0] === "--verify") {
    if (argv.length !== 2 || !argv[1] || argv[1].startsWith("--")) {
      fail("usage: assemble-sdk-ts-did-artifacts.mjs --verify DIR");
    }
    return Object.freeze({ operation: "verify", output: path.resolve(argv[1]) });
  }
  const values = new Map();
  for (let index = 0; index < argv.length; index += 2) {
    const flag = argv[index];
    const value = argv[index + 1];
    if (!flag?.startsWith("--") || !value || value.startsWith("--")) {
      fail("usage: assemble-sdk-ts-did-artifacts.mjs --source DIR --output DIR --wasm-pack VERSION --wasm-bindgen VERSION");
    }
    if (values.has(flag)) fail(`duplicate argument: ${flag}`);
    values.set(flag, value);
  }
  const allowed = new Set(["--source", "--output", "--wasm-pack", "--wasm-bindgen"]);
  for (const flag of values.keys()) if (!allowed.has(flag)) fail(`unknown argument: ${flag}`);
  for (const flag of allowed) if (!values.has(flag)) fail(`missing argument: ${flag}`);
  return Object.freeze({
    operation: "assemble",
    source: path.resolve(values.get("--source")),
    output: path.resolve(values.get("--output")),
    wasmPack: values.get("--wasm-pack"),
    wasmBindgen: values.get("--wasm-bindgen"),
  });
}

function requireExactKeys(value, expected, context) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) fail(`${context} must be an object`);
  const observed = Object.keys(value).sort();
  const wanted = [...expected].sort();
  if (JSON.stringify(observed) !== JSON.stringify(wanted)) {
    fail(`${context} fields mismatch: ${observed.join(", ")}`);
  }
}

function digest(value) {
  return createHash("sha256").update(value).digest("hex");
}

async function requireRegularFile(file) {
  const info = await lstat(file);
  if (!info.isFile() || info.isSymbolicLink()) fail(`artifact must be a regular file: ${file}`);
  return info;
}

async function requireDirectory(directory) {
  const info = await lstat(directory);
  if (!info.isDirectory() || info.isSymbolicLink()) fail(`artifact directory must not be a symlink: ${directory}`);
}

async function inspectTarget(source, target) {
  const directory = path.join(source, target);
  const observed = (await readdir(directory)).sort();
  if (JSON.stringify(observed) !== JSON.stringify(generatedFiles)) {
    fail(`${target} artifact set mismatch: ${observed.join(", ")}`);
  }

  const declarationPath = path.join(directory, `${artifactBase}.d.ts`);
  const javaScriptPath = path.join(directory, `${artifactBase}.js`);
  const wasmPath = path.join(directory, `${artifactBase}_bg.wasm`);
  const packagePath = path.join(directory, "package.json");
  const declaration = await readFile(declarationPath, "utf8");
  const javaScript = await readFile(javaScriptPath, "utf8");
  const packageDocument = JSON.parse(await readFile(packagePath, "utf8"));

  if (legacyNames.test(declaration) || legacyNames.test(javaScript)) {
    fail(`${target} artifact contains a forbidden legacy public name`);
  }
  for (const symbol of [
    "DidParseError",
    "DidUrlView",
    "DidView",
    "bindingApiVersion",
    "parseDid",
    "parseDidUrl",
  ]) {
    if (!declaration.includes(symbol)) fail(`${target} declaration is missing ${symbol}`);
  }
  if (!javaScript.includes(`${artifactBase}_bg.wasm`)) {
    fail(`${target} loader does not bind the paired WASM filename`);
  }
  if (packageDocument.name !== "identus-wasm-did") {
    fail(`${target} package name is not identus-wasm-did`);
  }
  if (packageDocument.types !== `${artifactBase}.d.ts`) {
    fail(`${target} package does not select the generated declaration`);
  }
  if (target === "web") {
    if (packageDocument.type !== "module" || packageDocument.main !== `${artifactBase}.js`) {
      fail("web package is not an explicit ESM module");
    }
    if (!declaration.includes("export function initSync") || !declaration.includes("export default function __wbg_init")) {
      fail("web declaration is missing explicit initialization");
    }
  } else if (target === "nodejs") {
    if (packageDocument.main !== `${artifactBase}.js`) fail("nodejs package is missing its CommonJS entry");
    if (declaration.includes("export function initSync") || declaration.includes("export default function __wbg_init")) {
      fail("nodejs declaration unexpectedly exposes web initialization");
    }
  } else {
    fail(`unsupported target: ${target}`);
  }

  const artifacts = [];
  for (const relative of artifactFiles) {
    const file = path.join(directory, relative);
    const info = await requireRegularFile(file);
    const bytes = await readFile(file);
    artifacts.push(Object.freeze({
      path: `${target}/${relative}`,
      bytes: info.size,
      sha256: digest(bytes),
    }));
  }
  const javaScriptArtifact = artifacts.find((artifact) => artifact.path.endsWith(`/${artifactBase}.js`));
  const wasmArtifact = artifacts.find((artifact) => artifact.path.endsWith(`/${artifactBase}_bg.wasm`));
  return Object.freeze({
    target,
    packageVersion: packageDocument.version,
    artifacts,
    entry: `${target}/${artifactBase}.js`,
    wasm: `${target}/${artifactBase}_bg.wasm`,
    declaration: `${target}/${artifactBase}.d.ts`,
    pairSha256: digest(`${javaScriptArtifact.sha256}:${wasmArtifact.sha256}`),
    targetSha256: digest(artifacts.map((artifact) => `${artifact.path}\0${artifact.bytes}\0${artifact.sha256}`).join("\n")),
  });
}

async function copyTarget(source, output, target) {
  const targetOutput = path.join(output, target);
  await mkdir(targetOutput, { recursive: true, mode: 0o755 });
  for (const relative of artifactFiles) {
    const sourceFile = path.join(source, target, relative);
    const outputFile = path.join(targetOutput, relative);
    await copyFile(sourceFile, outputFile);
    await chmod(outputFile, 0o644);
  }
}

async function verifyRuntime(output) {
  const webDirectory = path.join(output, "web");
  const webModule = await import(pathToFileURL(path.join(webDirectory, `${artifactBase}.js`)).href);
  const webWasm = await readFile(path.join(webDirectory, `${artifactBase}_bg.wasm`));
  webModule.initSync({ module: webWasm });
  if (webModule.bindingApiVersion() !== bindingApiVersion) fail("web binding API version mismatch");
  const webValue = webModule.parseDid("did:example:artifact-check");
  try {
    if (webValue.value !== "did:example:artifact-check") fail("web runtime projection mismatch");
  } finally {
    webValue.free();
  }

  const nodeDirectory = path.join(output, "nodejs");
  const require = createRequire(import.meta.url);
  const nodeModule = require(path.join(nodeDirectory, `${artifactBase}.js`));
  if (nodeModule.bindingApiVersion() !== bindingApiVersion) fail("nodejs binding API version mismatch");
  const nodeValue = nodeModule.parseDid("did:example:artifact-check");
  try {
    if (nodeValue.value !== "did:example:artifact-check") fail("nodejs runtime projection mismatch");
  } finally {
    nodeValue.free();
  }
}

async function verifyAssembly(output) {
  await requireDirectory(output);
  const rootEntries = (await readdir(output)).sort();
  if (JSON.stringify(rootEntries) !== JSON.stringify(["manifest.json", "nodejs", "web"])) {
    fail(`assembly root inventory mismatch: ${rootEntries.join(", ")}`);
  }
  const manifestPath = path.join(output, "manifest.json");
  await requireRegularFile(manifestPath);
  const manifest = parseJsonWithoutDuplicates(await readFile(manifestPath, "utf8"), {
    maximumDepth: 8,
    maximumNodes: 256,
  });
  requireExactKeys(manifest, [
    "schemaVersion",
    "layout",
    "bindingApiVersion",
    "package",
    "generator",
    "entrypoints",
    "targets",
    "assemblySha256",
  ], "manifest");
  if (manifest.schemaVersion !== 1 || manifest.layout !== "identus.sdk-ts-did.private-artifacts.v1") {
    fail("manifest schema identity mismatch");
  }
  if (manifest.bindingApiVersion !== bindingApiVersion) fail("manifest binding API version mismatch");
  requireExactKeys(manifest.package, ["name", "version"], "manifest package");
  if (manifest.package.name !== "identus-wasm-did" || typeof manifest.package.version !== "string") {
    fail("manifest package identity mismatch");
  }
  requireExactKeys(manifest.generator, ["wasmPack", "wasmBindgen"], "manifest generator");
  if (manifest.generator.wasmPack !== "wasm-pack 0.15.0" || manifest.generator.wasmBindgen !== "wasm-bindgen 0.2.121") {
    fail("manifest generator identity mismatch");
  }
  requireExactKeys(manifest.entrypoints, ["browserEsm", "nodeImport", "nodeRequire"], "manifest entrypoints");
  const expectedEntrypoints = {
    browserEsm: `web/${artifactBase}.js`,
    nodeImport: `nodejs/${artifactBase}.js`,
    nodeRequire: `nodejs/${artifactBase}.js`,
  };
  if (JSON.stringify(manifest.entrypoints) !== JSON.stringify(expectedEntrypoints)) fail("manifest entrypoint contract mismatch");
  requireExactKeys(manifest.targets, ["web", "nodejs"], "manifest targets");

  const targetDigests = [];
  for (const target of ["web", "nodejs"]) {
    const targetDirectory = path.join(output, target);
    await requireDirectory(targetDirectory);
    const targetEntries = (await readdir(targetDirectory)).sort();
    if (JSON.stringify(targetEntries) !== JSON.stringify(artifactFiles)) {
      fail(`${target} assembly inventory mismatch: ${targetEntries.join(", ")}`);
    }
    const declaration = await readFile(path.join(targetDirectory, `${artifactBase}.d.ts`), "utf8");
    const javaScript = await readFile(path.join(targetDirectory, `${artifactBase}.js`), "utf8");
    const packageDocument = parseJsonWithoutDuplicates(await readFile(path.join(targetDirectory, "package.json"), "utf8"), {
      maximumDepth: 6,
      maximumNodes: 128,
    });
    if (legacyNames.test(declaration) || legacyNames.test(javaScript)) fail(`${target} artifact contains a forbidden legacy public name`);
    for (const symbol of ["DidParseError", "DidUrlView", "DidView", "bindingApiVersion", "parseDid", "parseDidUrl"]) {
      if (!declaration.includes(symbol)) fail(`${target} assembled declaration is missing ${symbol}`);
    }
    if (packageDocument.name !== "identus-wasm-did" || packageDocument.types !== `${artifactBase}.d.ts`) {
      fail(`${target} assembled package identity mismatch`);
    }
    if (!javaScript.includes(`${artifactBase}_bg.wasm`)) fail(`${target} assembled loader lost its paired WASM filename`);
    if (target === "web") {
      const snapshot = await readFile(path.join(repositoryRoot, "crates/wasm-did/api/typescript-api.d.ts"), "utf8");
      if (declaration !== snapshot) fail("web declaration differs from the reviewed binding snapshot");
      if (packageDocument.type !== "module" || packageDocument.main !== `${artifactBase}.js`) fail("web assembled package is not explicit ESM");
    } else if (packageDocument.main !== `${artifactBase}.js` || packageDocument.type === "module") {
      fail("nodejs assembled package is not explicit CommonJS");
    }
    const record = manifest.targets[target];
    requireExactKeys(record, ["entry", "wasm", "declaration", "pairSha256", "targetSha256", "artifacts"], `${target} record`);
    if (record.entry !== `${target}/${artifactBase}.js`
      || record.wasm !== `${target}/${artifactBase}_bg.wasm`
      || record.declaration !== `${target}/${artifactBase}.d.ts`) {
      fail(`${target} path contract mismatch`);
    }
    if (!Array.isArray(record.artifacts) || record.artifacts.length !== artifactFiles.length) {
      fail(`${target} artifact inventory mismatch`);
    }
    const observedArtifacts = [];
    for (let index = 0; index < artifactFiles.length; index += 1) {
      const artifact = record.artifacts[index];
      requireExactKeys(artifact, ["path", "bytes", "sha256"], `${target} artifact ${index}`);
      const expectedPath = `${target}/${artifactFiles[index]}`;
      if (artifact.path !== expectedPath || !Number.isSafeInteger(artifact.bytes) || artifact.bytes < 1 || !/^[0-9a-f]{64}$/u.test(artifact.sha256)) {
        fail(`${target} artifact ${index} schema mismatch`);
      }
      const file = path.join(output, expectedPath);
      const info = await requireRegularFile(file);
      const actualDigest = digest(await readFile(file));
      if (artifact.bytes !== info.size || artifact.sha256 !== actualDigest) fail(`${expectedPath} identity mismatch`);
      observedArtifacts.push(artifact);
    }
    const javaScriptArtifact = observedArtifacts.find((artifact) => artifact.path.endsWith(`/${artifactBase}.js`));
    const wasmArtifact = observedArtifacts.find((artifact) => artifact.path.endsWith(`/${artifactBase}_bg.wasm`));
    if (record.pairSha256 !== digest(`${javaScriptArtifact.sha256}:${wasmArtifact.sha256}`)) fail(`${target} pair digest mismatch`);
    const targetSha256 = digest(observedArtifacts.map((artifact) => `${artifact.path}\0${artifact.bytes}\0${artifact.sha256}`).join("\n"));
    if (record.targetSha256 !== targetSha256) fail(`${target} digest mismatch`);
    targetDigests.push(targetSha256);
  }
  if (manifest.assemblySha256 !== digest(targetDigests.join(":"))) fail("assembly digest mismatch");
  await verifyRuntime(output);
  return manifest;
}

async function assemble(options) {
  try {
    await lstat(options.output);
    fail(`output already exists: ${options.output}`);
  } catch (error) {
    if (error?.code !== "ENOENT") throw error;
  }

  const inspected = await Promise.all(["web", "nodejs"].map((target) => inspectTarget(options.source, target)));
  if (inspected[0].packageVersion !== inspected[1].packageVersion) {
    fail("web and nodejs package versions differ");
  }

  await mkdir(options.output, { recursive: false, mode: 0o755 });
  for (const target of ["web", "nodejs"]) await copyTarget(options.source, options.output, target);
  const manifest = Object.freeze({
    schemaVersion: 1,
    layout: "identus.sdk-ts-did.private-artifacts.v1",
    bindingApiVersion,
    package: Object.freeze({ name: "identus-wasm-did", version: inspected[0].packageVersion }),
    generator: Object.freeze({ wasmPack: options.wasmPack, wasmBindgen: options.wasmBindgen }),
    entrypoints: Object.freeze({
      browserEsm: `web/${artifactBase}.js`,
      nodeImport: `nodejs/${artifactBase}.js`,
      nodeRequire: `nodejs/${artifactBase}.js`,
    }),
    targets: Object.freeze(Object.fromEntries(inspected.map((target) => [target.target, Object.freeze({
      entry: target.entry,
      wasm: target.wasm,
      declaration: target.declaration,
      pairSha256: target.pairSha256,
      targetSha256: target.targetSha256,
      artifacts: target.artifacts,
    })]))),
  });
  const assemblySha256 = digest([manifest.targets.web.targetSha256, manifest.targets.nodejs.targetSha256].join(":"));
  const document = Object.freeze({ ...manifest, assemblySha256 });
  await writeFile(path.join(options.output, "manifest.json"), `${JSON.stringify(document, null, 2)}\n`, { flag: "wx", mode: 0o644 });
  await verifyAssembly(options.output);
  process.stdout.write(`${JSON.stringify(document)}\n`);
}

async function main() {
  const options = parseArguments(process.argv.slice(2));
  if (options.operation === "verify") {
    const manifest = await verifyAssembly(options.output);
    process.stdout.write(`${JSON.stringify(manifest)}\n`);
  } else {
    await assemble(options);
  }
}

main().catch((error) => {
  process.stderr.write(`[sdk-ts-did-artifacts] ${error.message}\n`);
  process.exitCode = 1;
});
