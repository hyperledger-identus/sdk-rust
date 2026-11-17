import {
  DidError,
  type DidDereferencingRequestValue,
  type DidDereferencingOptions,
  type DidQueryEffects,
  type DidQueryProvider,
  type DidDocumentValue,
  type DidErrorCode,
  type DidMappingId,
  type DidObservation,
  type DidOperation,
  type DidProvider,
  type DidProviderOptions,
  type DidProviderRoute,
  type DidOperationContext,
  type DidResolutionOptions,
  type DidResolutionRequestValue,
  type DidResolutionResultValue,
  type DidUrlDereferencingResultValue,
  type DidUrlValue,
  type DidValue,
} from "../public.js";
import { projectDidDocument as projectDidDocumentValue } from "./document-projection.js";
import {
  projectDereferencingOptions,
  projectDidDereferencingResult,
  projectDidResolutionResult,
  projectResolutionOptions,
} from "./resolution-projection.js";
import {
  serializeDereferencingOptions,
  serializeResolutionOptions,
} from "./resolution-options.js";
import type {
  RawDidBinding,
  RawDidDereferencingRequestView,
  RawDidDereferencingResultView,
  RawDidDocumentView,
  RawDidParseError,
  RawDidResolutionRequestView,
  RawDidResolutionResultView,
  RawDidUrlView,
  RawDidView,
  RawOwnedValue,
} from "./raw-binding.js";

const BINDING_API_VERSION = 3;
const MAX_OBSERVED_DURATION_MS = 2_147_483_647;

interface ProviderInternals {
  readonly now?: () => number;
}

function bindingFailure(): Error {
  const error = new Error("DID binding contract failed");
  error.name = "DidBindingError";
  Object.defineProperty(error, "stack", {
    configurable: false,
    enumerable: false,
    value: undefined,
    writable: false,
  });
  return Object.freeze(error);
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

function projectDidDocument(raw: RawDidDocumentView): DidDocumentValue {
  try {
    return projectDidDocumentValue(
      requireString(raw.serialized),
      requireString(raw.id),
    );
  } finally {
    dispose(raw);
  }
}

function projectResolutionRequest(
  raw: RawDidResolutionRequestView,
  signal: AbortSignal | undefined,
): DidResolutionRequestValue {
  try {
    return Object.freeze({
      did: Object.freeze({
        serialized: requireString(raw.did),
        method: requireString(raw.method),
        methodSpecificId: requireString(raw.methodSpecificId),
      }),
      options: projectResolutionOptions(requireString(raw.options)),
      signal,
    });
  } finally {
    dispose(raw);
  }
}

function projectDereferencingRequest(
  raw: RawDidDereferencingRequestView,
  signal: AbortSignal | undefined,
): DidDereferencingRequestValue {
  try {
    return Object.freeze({
      didUrl: Object.freeze({
        serialized: requireString(raw.didUrl),
        did: requireString(raw.did),
        method: requireString(raw.method),
        methodSpecificId: requireString(raw.methodSpecificId),
        path: requireString(raw.path),
        query: requireOptionalString(raw.query),
        fragment: requireOptionalString(raw.fragment),
      }),
      options: projectDereferencingOptions(requireString(raw.options)),
      signal,
    });
  } finally {
    dispose(raw);
  }
}

function projectResolutionResult(
  raw: RawDidResolutionResultView,
): DidResolutionResultValue {
  try {
    return projectDidResolutionResult(
      requireString(raw.serialized),
      requireString(raw.kind),
    );
  } finally {
    dispose(raw);
  }
}

function projectDereferencingResult(
  raw: RawDidDereferencingResultView,
): DidUrlDereferencingResultValue {
  try {
    return projectDidDereferencingResult(
      requireString(raw.serialized),
      requireString(raw.kind),
    );
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
  if (
    code === "did.invalid_did"
    || code === "did.invalid_did_url"
    || code === "did.invalid_document"
    || code === "did.invalid_resolution_options"
    || code === "did.invalid_dereferencing_options"
    || code === "did.invalid_resolution_result"
    || code === "did.invalid_dereferencing_result"
    || code === "adapter.unsupported_lossy_mapping"
  ) {
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
  switch (operation) {
    case "did.parse":
      return "did.value.typescript.legacy-v1";
    case "did-url.parse":
      return "did-url.value.typescript.legacy-v1";
    case "did-document.parse":
      return "did.document.typescript.legacy-v1";
    case "did.resolve":
    case "did-url.dereference":
      return "did.resolution.typescript.host-v1";
  }
}

function errorMapping(code: DidErrorCode): DidMappingId {
  switch (code) {
    case "did.invalid_did":
      return "did.error.invalid-did.typescript.legacy-v1";
    case "did.invalid_did_url":
      return "did.error.invalid-did-url.typescript.legacy-v1";
    case "did.invalid_document":
      return "did.document.typescript.legacy-v1";
    case "adapter.unsupported_lossy_mapping":
      return "did.document.typescript.legacy-v1";
    case "did.invalid_resolution_options":
    case "did.invalid_dereferencing_options":
    case "did.invalid_resolution_result":
    case "did.invalid_dereferencing_result":
      return "did.resolution.typescript.host-v1";
    case "did.resolution_host_failure":
    case "did.dereferencing_host_failure":
    case "did.resolution_provider_unavailable":
    case "did.dereferencing_provider_unavailable":
    case "did.operation_aborted":
    case "did.observer_failure":
    case "adapter.binding_contract":
      return "did.resolution.typescript.host-v1";
  }
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
    parseDidDocument(value: string): DidDocumentValue {
      requireInput(value);
      return run(
        "did-document.parse",
        () => projectDidDocument(binding.parseDidDocument(value)),
      );
    },
  });
}

function strictNotify(
  observer: DidProviderOptions["observer"],
  observation: DidObservation,
): void {
  if (observer === undefined) {
    return;
  }
  try {
    observer(Object.freeze(observation));
  } catch {
    throw new DidError("did.observer_failure");
  }
}

function isAborted(signal: AbortSignal | undefined): boolean {
  if (signal === undefined) {
    return false;
  }
  try {
    return signal.aborted === true;
  } catch {
    throw new DidError("adapter.binding_contract");
  }
}

function mapQueryBindingError(binding: RawDidBinding, error: unknown): DidError {
  const mapped = mappedError(binding, error);
  return mapped instanceof DidError
    ? mapped
    : new DidError("adapter.binding_contract");
}

export function createRustDidQueryProvider(
  binding: RawDidBinding,
  route: Exclude<DidProviderRoute, "legacy">,
  effects: DidQueryEffects,
  options: DidProviderOptions = {},
  internals: ProviderInternals = {},
): DidQueryProvider {
  const base = createRustDidProvider(binding, route, options, internals);
  const now = internals.now ?? Date.now;

  async function runQuery<T>(
    operation: Extract<DidOperation, "did.resolve" | "did-url.dereference">,
    invoke: () => Promise<T>,
  ): Promise<T> {
    const start = now();
    let result: T;
    try {
      result = await invoke();
    } catch (error) {
      const mapped = error instanceof DidError
        ? error
        : new DidError("adapter.binding_contract");
      strictNotify(options.observer, {
        operation,
        route,
        result: "error",
        mappingId: successMapping(operation),
        errorCode: mapped.code,
        durationMs: boundedDuration(start, now()),
      });
      throw mapped;
    }
    strictNotify(options.observer, {
      operation,
      route,
      result: "success",
      mappingId: successMapping(operation),
      errorCode: undefined,
      durationMs: boundedDuration(start, now()),
    });
    return result;
  }

  return Object.freeze({
    ...base,
    resolveDid(
      did: string,
      resolutionOptions?: DidResolutionOptions,
      context?: DidOperationContext,
    ) {
      requireInput(did);
      return runQuery("did.resolve", async () => {
        if (isAborted(context?.signal)) {
          throw new DidError("did.operation_aborted");
        }
        if (effects.resolver === undefined) {
          throw new DidError("did.resolution_provider_unavailable");
        }
        let request: DidResolutionRequestValue;
        try {
          request = projectResolutionRequest(
            binding.prepareDidResolution(
              did,
              serializeResolutionOptions(resolutionOptions),
            ),
            context?.signal,
          );
        } catch (error) {
          throw mapQueryBindingError(binding, error);
        }
        let raw: string;
        try {
          raw = await effects.resolver.resolve(request);
        } catch {
          throw new DidError("did.resolution_host_failure");
        }
        if (typeof raw !== "string") {
          throw new DidError("did.resolution_host_failure");
        }
        try {
          return projectResolutionResult(
            binding.parseDidResolutionResult(request.did.serialized, raw),
          );
        } catch (error) {
          throw mapQueryBindingError(binding, error);
        }
      });
    },
    dereferenceDidUrl(
      didUrl: string,
      dereferencingOptions?: DidDereferencingOptions,
      context?: DidOperationContext,
    ) {
      requireInput(didUrl);
      return runQuery("did-url.dereference", async () => {
        if (isAborted(context?.signal)) {
          throw new DidError("did.operation_aborted");
        }
        if (effects.dereferencer === undefined) {
          throw new DidError("did.dereferencing_provider_unavailable");
        }
        let request: DidDereferencingRequestValue;
        try {
          request = projectDereferencingRequest(
            binding.prepareDidDereferencing(
              didUrl,
              serializeDereferencingOptions(dereferencingOptions),
            ),
            context?.signal,
          );
        } catch (error) {
          throw mapQueryBindingError(binding, error);
        }
        let raw: string;
        try {
          raw = await effects.dereferencer.dereference(request);
        } catch {
          throw new DidError("did.dereferencing_host_failure");
        }
        if (typeof raw !== "string") {
          throw new DidError("did.dereferencing_host_failure");
        }
        try {
          return projectDereferencingResult(
            binding.parseDidDereferencingResult(raw),
          );
        } catch (error) {
          throw mapQueryBindingError(binding, error);
        }
      });
    },
  });
}
