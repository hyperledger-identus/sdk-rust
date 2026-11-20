export interface RawPrivateKeyHandle {
  publicKeyBytes(): Uint8Array;
  dispose(): void;
  free(): void;
}

export interface RawEd25519PrivateHandle extends RawPrivateKeyHandle {
  sign(message: Uint8Array): Uint8Array;
}

export interface RawX25519PrivateHandle extends RawPrivateKeyHandle {
  deriveSharedSecret(peerPublicKey: Uint8Array): Uint8Array;
}

export interface RawKeyOperationsBinding {
  cryptoBindingApiVersion(): unknown;
  generateEd25519(): RawEd25519PrivateHandle;
  importEd25519(privateKey: Uint8Array): RawEd25519PrivateHandle;
  verifyEd25519(
    publicKey: Uint8Array,
    message: Uint8Array,
    signature: Uint8Array,
  ): unknown;
  generateX25519(): RawX25519PrivateHandle;
  importX25519(privateKey: Uint8Array): RawX25519PrivateHandle;
}
