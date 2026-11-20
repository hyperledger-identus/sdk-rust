import {
  CryptoError,
  type CryptoErrorCode,
  type Ed25519PrivateKeyHandle,
  type Ed25519PublicKeyValue,
  type KeyOperationsProvider,
  type KeyOperation,
  type KeyOperationObserver,
  type KeyOperationsProviderOptions,
  type X25519PrivateKeyHandle,
  type X25519PublicKeyValue,
} from "../public.js";
import type {
  RawEd25519PrivateHandle,
  RawKeyOperationsBinding,
  RawPrivateKeyHandle,
  RawX25519PrivateHandle,
} from "./crypto-binding.js";

const BINDING_API_VERSION = 1;
const MAX_KEY_OPERATION_MESSAGE_BYTES = 1_048_576;
const ERROR_CODES: ReadonlySet<string> = new Set<CryptoErrorCode>([
  "crypto.handle_disposed",
  "crypto.invalid_private_key",
  "crypto.invalid_public_key",
  "crypto.invalid_signature",
  "crypto.message_too_large",
  "crypto.non_contributory_peer",
  "crypto.entropy_unavailable",
  "crypto.observer_failure",
  "adapter.binding_contract",
]);

function bindingFailure(): CryptoError {
  return new CryptoError("adapter.binding_contract");
}

function mapFailure(error: unknown): CryptoError {
  if (error instanceof CryptoError) {
    return error;
  }
  if (error instanceof Error && ERROR_CODES.has(error.message)) {
    return new CryptoError(error.message as CryptoErrorCode);
  }
  return bindingFailure();
}

function requireBytes(value: unknown, length?: number): Uint8Array {
  if (!(value instanceof Uint8Array) || (length !== undefined && value.length !== length)) {
    throw bindingFailure();
  }
  return new Uint8Array(value);
}

function requireMessage(value: unknown): Uint8Array {
  const message = requireBytes(value);
  if (message.length > MAX_KEY_OPERATION_MESSAGE_BYTES) {
    throw new CryptoError("crypto.message_too_large");
  }
  return message;
}

function runObserved<T>(
  operation: KeyOperation,
  curve: "Ed25519" | "X25519",
  observer: KeyOperationObserver | undefined,
  action: () => T,
  onObserverFailure?: (result: T | undefined) => void,
): T {
  const started = performance.now();
  let result: "success" | "error" = "success";
  let failure: CryptoError | undefined;
  let value: T | undefined;
  try {
    value = action();
    return value;
  } catch (error) {
    result = "error";
    failure = mapFailure(error);
    throw failure;
  } finally {
    if (observer !== undefined) {
      try {
        observer(Object.freeze({
          operation,
          curve,
          result,
          errorCode: failure?.code,
          durationMs: Math.max(0, performance.now() - started),
        }));
      } catch {
        onObserverFailure?.(value);
        throw new CryptoError("crypto.observer_failure");
      }
    }
  }
}

function destroyRaw(raw: RawPrivateKeyHandle | undefined): void {
  if (raw === undefined) return;
  try {
    raw.dispose();
  } catch {
    // Cleanup continues because generated ownership is terminal.
  } finally {
    try {
      raw.free();
    } catch {
      // Cleanup cannot be retried safely after an observer failure.
    }
  }
}

const privateHandleFinalizer = new FinalizationRegistry<RawPrivateKeyHandle>((raw) => {
  try {
    raw.dispose();
  } catch {
    // Finalization is defense in depth after explicit lifecycle authority is lost.
  } finally {
    try {
      raw.free();
    } catch {
      // Generated cleanup is terminal and cannot be retried safely here.
    }
  }
});

abstract class PrivateHandleBase<Raw extends RawPrivateKeyHandle> {
  protected raw: Raw | undefined;
  protected readonly observer: KeyOperationObserver | undefined;
  private readonly finalizerToken = Object.freeze({});

  protected constructor(raw: Raw, observer: KeyOperationObserver | undefined) {
    this.raw = raw;
    this.observer = observer;
    privateHandleFinalizer.register(this, raw, this.finalizerToken);
  }

  protected requireRaw(): Raw {
    if (this.raw === undefined) {
      throw new CryptoError("crypto.handle_disposed");
    }
    return this.raw;
  }

  dispose(): void {
    const raw = this.raw;
    if (raw === undefined) {
      return;
    }
    this.raw = undefined;
    privateHandleFinalizer.unregister(this.finalizerToken);
    try {
      raw.dispose();
    } catch {
      throw bindingFailure();
    } finally {
      try {
        raw.free();
      } catch {
        // The binding-owned object is terminal even if generated cleanup fails.
      }
    }
  }
}

class Ed25519Handle extends PrivateHandleBase<RawEd25519PrivateHandle>
  implements Ed25519PrivateKeyHandle {
  constructor(raw: RawEd25519PrivateHandle, observer: KeyOperationObserver | undefined) {
    super(raw, observer);
  }

  publicKey(): Ed25519PublicKeyValue {
    return runObserved("ed25519.public-key", "Ed25519", this.observer, () => {
      return Object.freeze({
        algorithm: "Ed25519" as const,
        bytes: requireBytes(this.requireRaw().publicKeyBytes(), 32),
      });
    });
  }

  sign(message: Uint8Array): Uint8Array {
    return runObserved("ed25519.sign", "Ed25519", this.observer, () => {
      return requireBytes(this.requireRaw().sign(requireMessage(message)), 64);
    });
  }

  override dispose(): void {
    runObserved("ed25519.dispose", "Ed25519", this.observer, () => super.dispose());
  }
}

class X25519Handle extends PrivateHandleBase<RawX25519PrivateHandle>
  implements X25519PrivateKeyHandle {
  constructor(raw: RawX25519PrivateHandle, observer: KeyOperationObserver | undefined) {
    super(raw, observer);
  }

  publicKey(): X25519PublicKeyValue {
    return runObserved("x25519.public-key", "X25519", this.observer, () => {
      return Object.freeze({
        algorithm: "X25519" as const,
        bytes: requireBytes(this.requireRaw().publicKeyBytes(), 32),
      });
    });
  }

  deriveSharedSecret(peerPublicKey: Uint8Array): Uint8Array {
    return runObserved("x25519.derive-shared-secret", "X25519", this.observer, () => {
      return requireBytes(
        this.requireRaw().deriveSharedSecret(requireBytes(peerPublicKey, 32)),
        32,
      );
    });
  }

  override dispose(): void {
    runObserved("x25519.dispose", "X25519", this.observer, () => super.dispose());
  }
}

export function createRustKeyOperationsProvider(
  binding: RawKeyOperationsBinding,
  options: KeyOperationsProviderOptions = {},
): KeyOperationsProvider {
  if (binding.cryptoBindingApiVersion() !== BINDING_API_VERSION) {
    throw bindingFailure();
  }
  return Object.freeze({
    generateEd25519: () => new Ed25519Handle(
      runObserved(
        "ed25519.generate",
        "Ed25519",
        options.observer,
        () => binding.generateEd25519(),
        destroyRaw,
      ),
      options.observer,
    ),
    importEd25519: (privateKey: Uint8Array) => {
      try {
        return new Ed25519Handle(
          runObserved(
            "ed25519.import",
            "Ed25519",
            options.observer,
            () => binding.importEd25519(requireBytes(privateKey, 32)),
            destroyRaw,
          ),
          options.observer,
        );
      } finally {
        privateKey.fill(0);
      }
    },
    verifyEd25519: (
      publicKey: Uint8Array,
      message: Uint8Array,
      signature: Uint8Array,
    ) => {
      return runObserved("ed25519.verify", "Ed25519", options.observer, () => {
        const result = binding.verifyEd25519(
          requireBytes(publicKey, 32),
          requireMessage(message),
          requireBytes(signature, 64),
        );
        if (typeof result !== "boolean") {
          throw bindingFailure();
        }
        return result;
      });
    },
    generateX25519: () => new X25519Handle(
      runObserved(
        "x25519.generate",
        "X25519",
        options.observer,
        () => binding.generateX25519(),
        destroyRaw,
      ),
      options.observer,
    ),
    importX25519: (privateKey: Uint8Array) => {
      try {
        return new X25519Handle(
          runObserved(
            "x25519.import",
            "X25519",
            options.observer,
            () => binding.importX25519(requireBytes(privateKey, 32)),
            destroyRaw,
          ),
          options.observer,
        );
      } finally {
        privateKey.fill(0);
      }
    },
  });
}
