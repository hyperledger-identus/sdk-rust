export function initializationFailure(): Error {
  const error = new Error("DID binding initialization failed");
  error.name = "DidBindingInitializationError";
  Object.defineProperty(error, "stack", {
    configurable: false,
    enumerable: false,
    value: undefined,
    writable: false,
  });
  return Object.freeze(error);
}

/** Keep one target initializer in flight and permit a retry only after failure. */
export function retryableSingleton<Value>(
  initialize: () => Promise<Value>,
): () => Promise<Value> {
  let current: Promise<Value> | undefined;
  return () => {
    current ??= initialize().catch(() => {
      current = undefined;
      throw initializationFailure();
    });
    return current;
  };
}
