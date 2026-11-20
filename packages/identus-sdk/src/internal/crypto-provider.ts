import {
  CryptoError,
  type CryptoErrorCode,
  type Ed25519PrivateKeyHandle,
  type Ed25519PublicKeyValue,
  type KeyOperationsProvider,
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
const ERROR_CODES: ReadonlySet<string> = new Set<CryptoErrorCode>([
  "crypto.handle_disposed",
  "crypto.invalid_private_key",
  "crypto.invalid_public_key",
  "crypto.invalid_signature",
  "crypto.message_too_large",
  "crypto.non_contributory_peer",
  "crypto.entropy_unavailable",
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

abstract class PrivateHandleBase<Raw extends RawPrivateKeyHandle> {
  protected raw: Raw | undefined;

  protected constructor(raw: Raw) {
    this.raw = raw;
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
  constructor(raw: RawEd25519PrivateHandle) {
    super(raw);
  }

  publicKey(): Ed25519PublicKeyValue {
    try {
      return Object.freeze({
        algorithm: "Ed25519" as const,
        bytes: requireBytes(this.requireRaw().publicKeyBytes(), 32),
      });
    } catch (error) {
      throw mapFailure(error);
    }
  }

  sign(message: Uint8Array): Uint8Array {
    try {
      return requireBytes(this.requireRaw().sign(requireBytes(message)), 64);
    } catch (error) {
      throw mapFailure(error);
    }
  }
}

class X25519Handle extends PrivateHandleBase<RawX25519PrivateHandle>
  implements X25519PrivateKeyHandle {
  constructor(raw: RawX25519PrivateHandle) {
    super(raw);
  }

  publicKey(): X25519PublicKeyValue {
    try {
      return Object.freeze({
        algorithm: "X25519" as const,
        bytes: requireBytes(this.requireRaw().publicKeyBytes(), 32),
      });
    } catch (error) {
      throw mapFailure(error);
    }
  }

  deriveSharedSecret(peerPublicKey: Uint8Array): Uint8Array {
    try {
      return requireBytes(
        this.requireRaw().deriveSharedSecret(requireBytes(peerPublicKey, 32)),
        32,
      );
    } catch (error) {
      throw mapFailure(error);
    }
  }
}

export function createRustKeyOperationsProvider(
  binding: RawKeyOperationsBinding,
): KeyOperationsProvider {
  if (binding.cryptoBindingApiVersion() !== BINDING_API_VERSION) {
    throw bindingFailure();
  }
  return Object.freeze({
    generateEd25519: () => {
      try {
        return new Ed25519Handle(binding.generateEd25519());
      } catch (error) {
        throw mapFailure(error);
      }
    },
    importEd25519: (privateKey: Uint8Array) => {
      try {
        return new Ed25519Handle(binding.importEd25519(requireBytes(privateKey, 32)));
      } catch (error) {
        throw mapFailure(error);
      } finally {
        privateKey.fill(0);
      }
    },
    verifyEd25519: (
      publicKey: Uint8Array,
      message: Uint8Array,
      signature: Uint8Array,
    ) => {
      try {
        const result = binding.verifyEd25519(
          requireBytes(publicKey, 32),
          requireBytes(message),
          requireBytes(signature, 64),
        );
        if (typeof result !== "boolean") {
          throw bindingFailure();
        }
        return result;
      } catch (error) {
        throw mapFailure(error);
      }
    },
    generateX25519: () => {
      try {
        return new X25519Handle(binding.generateX25519());
      } catch (error) {
        throw mapFailure(error);
      }
    },
    importX25519: (privateKey: Uint8Array) => {
      try {
        return new X25519Handle(binding.importX25519(requireBytes(privateKey, 32)));
      } catch (error) {
        throw mapFailure(error);
      } finally {
        privateKey.fill(0);
      }
    },
  });
}
