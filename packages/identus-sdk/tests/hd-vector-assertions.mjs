const vectors = Object.freeze([
  Object.freeze({
    profile: "secp256k1-bip32-hardened-v1",
    seedHex: "000102030405060708090a0b0c0d0e0f",
    rootPublicHex: "0339a36013301597daef41fbe593a02cc513d0b55527ec2df1050e2e8ff49c85c2",
    childPath: "m/0'",
    childPublicHex: "035a784662a4a20a65bf6aab9ae98a6c068a81c52e4b032c0fb5400c706cfccc56",
  }),
  Object.freeze({
    profile: "ed25519-slip10-hardened-v1",
    seedHex: "000102030405060708090a0b0c0d0e0f",
    rootPublicHex: "a4b2856bfec510abab89753fac1ac0e1112364e7d250545963f135f2a33188ed",
    childPath: "m/0'",
    childPublicHex: "8c8a13df77a28f3445213a0f432fde644acaa215fc72dcdf300d5efaa85d350c",
  }),
]);

function bytes(hex) {
  return Uint8Array.from(hex.match(/../gu) ?? [], (value) => Number.parseInt(value, 16));
}

function hex(value) {
  return Array.from(value, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

function requireEqual(actual, expected, label) {
  if (actual !== expected) throw new Error(`${label} mismatch`);
}

export function assertHdDerivationVectors(provider) {
  for (const vector of vectors) {
    const seed = bytes(vector.seedHex);
    const root = provider.createRoot(vector.profile, seed);
    requireEqual(seed.every((byte) => byte === 0), true, `${vector.profile} seed erasure`);
    requireEqual(root.profile, vector.profile, `${vector.profile} identity`);
    requireEqual(root.depth, 0, `${vector.profile} root depth`);
    requireEqual(hex(root.publicKey().bytes), vector.rootPublicHex, `${vector.profile} root`);
    const child = root.derive(vector.childPath);
    requireEqual(child.depth, 1, `${vector.profile} child depth`);
    requireEqual(hex(child.publicKey().bytes), vector.childPublicHex, `${vector.profile} child`);
    const mutable = child.publicKey().bytes;
    mutable.fill(0);
    requireEqual(hex(child.publicKey().bytes), vector.childPublicHex, `${vector.profile} immutable projection`);
    try {
      root.derive("m/0");
      throw new Error(`${vector.profile} accepted a soft child`);
    } catch (error) {
      requireEqual(error?.code, "crypto.hd.forbidden_axis", `${vector.profile} soft rejection`);
    }
    child.dispose();
    root.dispose();
    root.dispose();
  }

  const cardanoSeed = new Uint8Array(64).fill(7);
  const cardano = provider.createRoot("ed25519-cardano-v2-v1", cardanoSeed);
  requireEqual(cardanoSeed.every((byte) => byte === 0), true, "Cardano seed erasure");
  requireEqual(
    hex(cardano.publicKey().bytes),
    "ae73682c5092217b9b3b1815c3c15f9dff1eaf3b36ab11d9b418fc7d85b37f740707070707070707070707070707070707070707070707070707070707070707",
    "Cardano root",
  );
  const privateChild = cardano.derive("m/0");
  const publicRoot = cardano.publicDerivation();
  const publicChild = publicRoot.derive("m/0");
  requireEqual(hex(privateChild.publicKey().bytes), hex(publicChild.publicKey().bytes), "Cardano public/private child");
  try {
    publicRoot.derive("m/0'");
    throw new Error("Cardano public handle accepted a hardened child");
  } catch (error) {
    requireEqual(error?.code, "crypto.hd.forbidden_axis", "Cardano hardened public rejection");
  }
  publicChild.dispose();
  publicRoot.dispose();
  privateChild.dispose();
  cardano.dispose();
  return 3;
}
