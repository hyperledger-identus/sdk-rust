import {
  CryptoError,
  type CardanoV2PrivateKeyHandle,
  type CardanoV2PublicKeyHandle,
  type CryptoErrorCode,
  type HdDerivationObserver,
  type HdDerivationOperation,
  type HdDerivationProfile,
  type HdDerivationProvider,
  type HdDerivationProviderOptions,
  type HdPrivateKeyHandle,
  type HdPrivateKeyHandleFor,
  type HdPublicKeyValue,
} from "../public.js";
import type {
  RawHdDerivationBinding,
  RawHdPrivateHandle,
  RawHdPublicHandle,
  RawPrivateKeyHandle,
} from "./crypto-binding.js";

const BINDING_API_VERSION = 1;
const CARDANO_PROFILE = "ed25519-cardano-v2-v1" as const;
const MAPPING_ID = "crypto.hd-derivation.typescript.v1" as const;
const PROFILES: ReadonlySet<string> = new Set<HdDerivationProfile>([
  "secp256k1-bip32-hardened-v1",
  "ed25519-slip10-hardened-v1",
  CARDANO_PROFILE,
]);
const ERROR_CODES: ReadonlySet<string> = new Set<CryptoErrorCode>([
  "crypto.handle_disposed",
  "crypto.hd.invalid_profile",
  "crypto.hd.invalid_seed",
  "crypto.hd.invalid_path",
  "crypto.hd.forbidden_axis",
  "crypto.hd.depth_overflow",
  "crypto.hd.derivation_failed",
  "crypto.hd.projection_failed",
  "crypto.observer_failure",
  "adapter.binding_contract",
]);

function bindingFailure(): CryptoError {
  return new CryptoError("adapter.binding_contract");
}

function mapFailure(error: unknown): CryptoError {
  if (error instanceof CryptoError) return error;
  if (error instanceof Error && ERROR_CODES.has(error.message)) {
    return new CryptoError(error.message as CryptoErrorCode);
  }
  return bindingFailure();
}

function requireProfile(value: unknown): HdDerivationProfile {
  if (typeof value !== "string" || !PROFILES.has(value)) throw bindingFailure();
  return value as HdDerivationProfile;
}

function requireInputProfile(value: unknown): HdDerivationProfile {
  if (typeof value !== "string" || !PROFILES.has(value)) {
    throw new CryptoError("crypto.hd.invalid_profile");
  }
  return value as HdDerivationProfile;
}

function requireDepth(value: unknown): number {
  if (!Number.isSafeInteger(value) || (value as number) < 0 || (value as number) > 255) {
    throw bindingFailure();
  }
  return value as number;
}

function requireBytes(value: unknown, length: number): Uint8Array {
  if (!(value instanceof Uint8Array) || value.length !== length) throw bindingFailure();
  return new Uint8Array(value);
}

function publicLength(profile: HdDerivationProfile): number {
  if (profile === "secp256k1-bip32-hardened-v1") return 33;
  if (profile === "ed25519-slip10-hardened-v1") return 32;
  return 64;
}

function publicValue<Profile extends HdDerivationProfile>(
  profile: Profile,
  depth: number,
  bytes: Uint8Array,
): HdPublicKeyValue<Profile> {
  const retained = new Uint8Array(bytes);
  const value = {
    profile,
    depth,
    get bytes(): Uint8Array {
      return new Uint8Array(retained);
    },
  };
  return Object.freeze(value);
}

function observe<T>(
  operation: HdDerivationOperation,
  profile: HdDerivationProfile,
  depth: number,
  observer: HdDerivationObserver | undefined,
  action: () => T,
  cleanup?: (value: T | undefined) => void,
): T {
  const started = performance.now();
  let result: "success" | "error" = "success";
  let errorCode: CryptoErrorCode | undefined;
  let value: T | undefined;
  try {
    value = action();
    return value;
  } catch (error) {
    result = "error";
    const failure = mapFailure(error);
    errorCode = failure.code;
    throw failure;
  } finally {
    if (observer !== undefined) {
      try {
        observer(Object.freeze({
          operation,
          profile,
          route: "rust-wasm" as const,
          mappingId: MAPPING_ID,
          result,
          errorCode,
          depth,
          durationMs: Math.max(0, performance.now() - started),
        }));
      } catch {
        cleanup?.(value);
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
    // Generated ownership is terminal even if Rust cleanup reports failure.
  } finally {
    try {
      raw.free();
    } catch {
      // Generated cleanup cannot be retried safely.
    }
  }
}

const finalizer = new FinalizationRegistry<RawPrivateKeyHandle>(destroyRaw);

abstract class HandleBase<Raw extends RawPrivateKeyHandle> {
  protected raw: Raw | undefined;
  protected readonly observer: HdDerivationObserver | undefined;
  private readonly token = Object.freeze({});

  protected constructor(raw: Raw, observer: HdDerivationObserver | undefined) {
    this.raw = raw;
    this.observer = observer;
    finalizer.register(this, raw, this.token);
  }

  protected requireRaw(): Raw {
    if (this.raw === undefined) throw new CryptoError("crypto.handle_disposed");
    return this.raw;
  }

  protected release(): void {
    const raw = this.raw;
    if (raw === undefined) return;
    this.raw = undefined;
    finalizer.unregister(this.token);
    destroyRaw(raw);
  }
}

class PrivateHandle<Profile extends HdDerivationProfile>
  extends HandleBase<RawHdPrivateHandle>
  implements HdPrivateKeyHandle<Profile> {
  readonly profile: Profile;
  readonly depth: number;

  constructor(
    raw: RawHdPrivateHandle,
    profile: Profile,
    observer: HdDerivationObserver | undefined,
  ) {
    const actualProfile = requireProfile(raw.profile());
    const depth = requireDepth(raw.depth());
    if (actualProfile !== profile) throw bindingFailure();
    super(raw, observer);
    this.profile = profile;
    this.depth = depth;
  }

  derive(path: string): HdPrivateKeyHandle<Profile> {
    return observe("hd.private.derive", this.profile, this.depth, this.observer, () => {
      const raw = this.requireRaw().derive(path);
      return wrapPrivate(raw, this.profile, this.observer);
    }, (value) => value?.dispose());
  }

  publicKey(): HdPublicKeyValue<Profile> {
    return observe("hd.private.public-key", this.profile, this.depth, this.observer, () =>
      publicValue(
        this.profile,
        this.depth,
        requireBytes(this.requireRaw().publicKeyBytes(), publicLength(this.profile)),
      ));
  }

  dispose(): void {
    observe("hd.private.dispose", this.profile, this.depth, this.observer, () => this.release());
  }
}

class CardanoPrivateHandle extends PrivateHandle<typeof CARDANO_PROFILE>
  implements CardanoV2PrivateKeyHandle {
  override derive(path: string): CardanoV2PrivateKeyHandle {
    return super.derive(path) as CardanoV2PrivateKeyHandle;
  }

  publicDerivation(): CardanoV2PublicKeyHandle {
    return observe(
      "hd.private.public-derivation",
      this.profile,
      this.depth,
      this.observer,
      () => wrapPublic(this.requireRaw().cardanoPublicHandle(), this.observer),
      (value) => value?.dispose(),
    );
  }
}

class CardanoPublicHandle extends HandleBase<RawHdPublicHandle>
  implements CardanoV2PublicKeyHandle {
  readonly profile = CARDANO_PROFILE;
  readonly depth: number;

  constructor(raw: RawHdPublicHandle, observer: HdDerivationObserver | undefined) {
    const profile = requireProfile(raw.profile());
    const depth = requireDepth(raw.depth());
    if (profile !== CARDANO_PROFILE) throw bindingFailure();
    super(raw, observer);
    this.depth = depth;
  }

  derive(path: string): CardanoV2PublicKeyHandle {
    return observe("hd.public.derive", this.profile, this.depth, this.observer, () =>
      wrapPublic(this.requireRaw().derive(path), this.observer),
    (value) => value?.dispose());
  }

  publicKey(): HdPublicKeyValue<typeof CARDANO_PROFILE> {
    return observe("hd.public.public-key", this.profile, this.depth, this.observer, () =>
      publicValue(this.profile, this.depth, requireBytes(this.requireRaw().publicKeyBytes(), 64)));
  }

  dispose(): void {
    observe("hd.public.dispose", this.profile, this.depth, this.observer, () => this.release());
  }
}

function wrapPrivate<Profile extends HdDerivationProfile>(
  raw: RawHdPrivateHandle,
  profile: Profile,
  observer: HdDerivationObserver | undefined,
): PrivateHandle<Profile> {
  try {
    return profile === CARDANO_PROFILE
      ? new CardanoPrivateHandle(raw, CARDANO_PROFILE, observer) as unknown as PrivateHandle<Profile>
      : new PrivateHandle(raw, profile, observer);
  } catch (error) {
    destroyRaw(raw);
    throw error;
  }
}

function wrapPublic(
  raw: RawHdPublicHandle,
  observer: HdDerivationObserver | undefined,
): CardanoPublicHandle {
  try {
    return new CardanoPublicHandle(raw, observer);
  } catch (error) {
    destroyRaw(raw);
    throw error;
  }
}

export function createRustHdDerivationProvider(
  binding: RawHdDerivationBinding,
  options: HdDerivationProviderOptions = {},
): HdDerivationProvider {
  if (binding.hdDerivationBindingApiVersion() !== BINDING_API_VERSION) {
    throw bindingFailure();
  }
  return Object.freeze({
    createRoot<Profile extends HdDerivationProfile>(
      profile: Profile,
      seed: Uint8Array,
    ): HdPrivateKeyHandleFor<Profile> {
      let copiedSeed: Uint8Array | undefined;
      const suppliedSeed = seed instanceof Uint8Array ? seed : undefined;
      try {
        const checkedProfile = requireInputProfile(profile);
        if (suppliedSeed === undefined) throw new CryptoError("crypto.hd.invalid_seed");
        copiedSeed = new Uint8Array(suppliedSeed);
        const raw = observe(
          "hd.root",
          checkedProfile,
          0,
          options.observer,
          () => binding.createRoot(checkedProfile, copiedSeed as Uint8Array),
          destroyRaw,
        );
        return wrapPrivate(
          raw,
          checkedProfile,
          options.observer,
        ) as unknown as HdPrivateKeyHandleFor<Profile>;
      } finally {
        copiedSeed?.fill(0);
        suppliedSeed?.fill(0);
      }
    },
  });
}
