const vectors = Object.freeze({
  ed25519: Object.freeze({
    privateKeyHex: "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
    publicKeyHex: "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
    signatureHex: "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b",
  }),
  x25519: Object.freeze({
    privateKeyHex: "77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a",
    publicKeyHex: "8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a",
    peerPublicKeyHex: "de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f",
    sharedSecretHex: "4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742",
  }),
});

function bytes(hex) {
  return Uint8Array.from(hex.match(/../gu) ?? [], (value) => Number.parseInt(value, 16));
}

function hex(value) {
  return Array.from(value, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

function requireEqual(actual, expected, label) {
  if (actual !== expected) throw new Error(`${label} mismatch`);
}

export function assertCryptoVectors(provider) {
  const edSeed = bytes(vectors.ed25519.privateKeyHex);
  const ed = provider.importEd25519(edSeed);
  const signature = ed.sign(new Uint8Array());
  requireEqual(hex(ed.publicKey().bytes), vectors.ed25519.publicKeyHex, "Ed25519 public key");
  const mutableEdPublic = ed.publicKey().bytes;
  mutableEdPublic.fill(0);
  requireEqual(hex(ed.publicKey().bytes), vectors.ed25519.publicKeyHex, "Ed25519 retained public key");
  requireEqual(hex(signature), vectors.ed25519.signatureHex, "Ed25519 signature");
  requireEqual(provider.verifyEd25519(ed.publicKey().bytes, new Uint8Array(), signature), true, "Ed25519 verification");
  const tampered = new Uint8Array(signature);
  tampered[0] ^= 1;
  requireEqual(provider.verifyEd25519(ed.publicKey().bytes, new Uint8Array(), tampered), false, "Ed25519 tamper rejection");
  requireEqual(edSeed.every((byte) => byte === 0), true, "Ed25519 caller buffer erasure");
  ed.dispose();
  ed.dispose();
  try {
    ed.sign(new Uint8Array());
    throw new Error("disposed Ed25519 handle accepted an operation");
  } catch (error) {
    requireEqual(error?.code, "crypto.handle_disposed", "Ed25519 disposed error");
  }

  const xSeed = bytes(vectors.x25519.privateKeyHex);
  const x = provider.importX25519(xSeed);
  requireEqual(hex(x.publicKey().bytes), vectors.x25519.publicKeyHex, "X25519 public key");
  const mutableXPublic = x.publicKey().bytes;
  mutableXPublic.fill(0);
  requireEqual(hex(x.publicKey().bytes), vectors.x25519.publicKeyHex, "X25519 retained public key");
  requireEqual(hex(x.deriveSharedSecret(bytes(vectors.x25519.peerPublicKeyHex))), vectors.x25519.sharedSecretHex, "X25519 shared secret");
  try {
    x.deriveSharedSecret(new Uint8Array(32));
    throw new Error("non-contributory X25519 peer was accepted");
  } catch (error) {
    requireEqual(error?.code, "crypto.non_contributory_peer", "X25519 contributory error");
  }
  requireEqual(xSeed.every((byte) => byte === 0), true, "X25519 caller buffer erasure");
  x.dispose();
  return 2;
}
