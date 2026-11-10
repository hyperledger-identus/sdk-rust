import {
  DidError,
  type DidErrorCode,
  type DidMappingId,
  type DidObservation,
  type DidOperation,
  type DidProvider,
  type DidProviderOptions,
  type DidProviderRoute,
  type DidUrlValue,
  type DidValue,
} from "../public.js";
import type {
  RawDidBinding,
  RawDidParseError,
  RawDidUrlView,
  RawDidView,
  RawOwnedValue,
} from "./raw-binding.js";

const BINDING_API_VERSION = 1;
const MAX_OBSERVED_DURATION_MS = 2_147_483_647;

interface ProviderInternals {
  readonly now?: () => number;
}

function bindingFailure(): Error {
  const error = new Error("DID binding contract failed");
  error.name = "DidBindingError";
  return error;
}

function requireString(value: unknown): string {
  if (typeof value !== "string") {
    throw bindingFailure();
  }
  return value;
}

function requireOptionalString(value: unknown): string | undefined {
  if (value === undefined || typeof value === "string") {
    return value;
  }
  throw bindingFailure();
}

function dispose(value: RawOwnedValue): void {
  try {
    value.free();
  } catch {
    throw bindingFailure();
  }
}

function projectDid(raw: RawDidView): DidValue {
  try {
    return Object.freeze({
      serialized: requireString(raw.value),
      method: requireString(raw.method),
      methodSpecificId: requireString(raw.methodSpecificId),
    });
  } finally {
    dispose(raw);
  }
}

function projectDidUrl(raw: RawDidUrlView): DidUrlValue {
  try {
    return Object.freeze({
      serialized: requireString(raw.value),
      did: requireString(raw.did),
      method: requireString(raw.method),
      methodSpecificId: requireString(raw.methodSpecificId),
      path: requireString(raw.path),
      query: requireOptionalString(raw.query),
      fragment: requireOptionalString(raw.fragment),
    });
  } finally {
    dispose(raw);
  }
}

function readErrorCode(raw: RawDidParseError): DidErrorCode {
  let code: unknown;
  try {
    code = raw.code;
  } finally {
    dispose(raw);
  }
  if (code === "did.invalid_did" || code === "did.invalid_did_url") {
    return code;
  }
  throw bindingFailure();
}

function mappedError(binding: RawDidBinding, error: unknown): Error {
  try {
    if (!binding.isParseError(error)) {
      return bindingFailure();
    }
    return new DidError(readErrorCode(error));
  } catch {
    return bindingFailure();
  }
}

function boundedDuration(start: number, end: number): number {
  if (!Number.isFinite(start) || !Number.isFinite(end)) {
    return 0;
  }
  return Math.min(
    MAX_OBSERVED_DURATION_MS,
    Math.max(0, Math.round(end - start)),
  );
}

function notify(
  observer: DidProviderOptions["observer"],
  observation: DidObservation,
): void {
  if (observer === undefined) {
    return;
  }
  try {
    observer(Object.freeze(observation));
  } catch {
    // Observation is deliberately isolated from parsing and ownership.
  }
}

function successMapping(operation: DidOperation): DidMappingId {
  return operation === "did.parse"
    ? "did.value.typescript.legacy-v1"
    : "did-url.value.typescript.legacy-v1";
}

function errorMapping(code: DidErrorCode): DidMappingId {
  return code === "did.invalid_did"
    ? "did.error.invalid-did.typescript.legacy-v1"
    : "did.error.invalid-did-url.typescript.legacy-v1";
}

function requireInput(value: string): void {
  if (typeof value !== "string") {
    throw new TypeError("DID input must be a string");
  }
}

export function createRustDidProvider(
  binding: RawDidBinding,
  route: Exclude<DidProviderRoute, "legacy">,
  options: DidProviderOptions = {},
  internals: ProviderInternals = {},
): DidProvider {
  if (binding.bindingApiVersion() !== BINDING_API_VERSION) {
    throw bindingFailure();
  }
  const now = internals.now ?? Date.now;

  function run<T>(
    operation: DidOperation,
    invoke: () => T,
  ): T {
    const start = now();
    try {
      const result = invoke();
      notify(options.observer, {
        operation,
        route,
        result: "success",
        mappingId: successMapping(operation),
        errorCode: undefined,
        durationMs: boundedDuration(start, now()),
      });
      return result;
    } catch (error) {
      const mapped = error instanceof DidError ? error : mappedError(binding, error);
      notify(options.observer, {
        operation,
        route,
        result: "error",
        mappingId: mapped instanceof DidError
          ? errorMapping(mapped.code)
          : successMapping(operation),
        errorCode: mapped instanceof DidError ? mapped.code : undefined,
        durationMs: boundedDuration(start, now()),
      });
      throw mapped;
    }
  }

  return Object.freeze({
    parseDid(value: string): DidValue {
      requireInput(value);
      return run("did.parse", () => projectDid(binding.parseDid(value)));
    },
    parseDidUrl(value: string): DidUrlValue {
      requireInput(value);
      return run("did-url.parse", () => projectDidUrl(binding.parseDidUrl(value)));
    },
  });
}
