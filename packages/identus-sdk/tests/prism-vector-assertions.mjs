const canonicalDid = "did:prism:44fb82ad3544a8c8acb763c01a6fc7d92d81c38104096319345a4618d1a4275c";
const longFormDid = `${canonicalDid}:Cj8KPRI7CgdtYXN0ZXIwEAFKLgoJc2VjcDI1NmsxEiEC5FmY4YEXETg1uXK0N4T7a2RwtyLLLcFxxfugAmW8gO4`;
const signedCreate = "CgdtYXN0ZXIwEkgwRgIhALjle4EjdeqePDrnlFnIaVoUlBm_aMLp4YoYXd3OSwivAiEAql-m3H3ufA0Lkc4FwgRSfoS7qe-H4nhwhRLQyRrCQDwaQQo_Cj0SOwoHbWFzdGVyMBABSi4KCXNlY3AyNTZrMRIhAuRZmOGBFxE4NblytDeE-2tkcLciyy3BccX7oAJlvIDu";

function equal(actual, expected, label) {
  if (!Object.is(actual, expected)) {
    throw new Error(`PRISM DID assertion failed: ${label}`);
  }
}

function frozen(value, label) {
  equal(Object.isFrozen(value), true, `${label}:frozen`);
}

export function assertPrismStaticVectors(provider) {
  const created = provider.createPrismDid({
    keys: [{
      id: "master0",
      usage: "master",
      curve: "secp256k1",
      publicKey: "AuRZmOGBFxE4NblytDeE-2tkcLciyy3BccX7oAJlvIDu",
    }],
  });
  equal(created.form, "long-form", "create:form");
  equal(created.did.serialized, longFormDid, "create:long-form");
  equal(created.canonicalDid.serialized, canonicalDid, "create:canonical");
  equal(created.document?.id.serialized, longFormDid, "create:document-subject");
  frozen(created, "create");
  frozen(created.did, "create:did");
  frozen(created.canonicalDid, "create:canonical-did");
  frozen(created.document, "create:document");

  const parsed = provider.parsePrismDid(canonicalDid);
  equal(parsed.form, "canonical", "parse:form");
  equal(parsed.did.serialized, canonicalDid, "parse:did");
  equal(parsed.document, undefined, "parse:no-document");
  frozen(parsed, "parse");

  const resolved = provider.resolvePrismDidLongForm(longFormDid);
  equal(resolved.canonicalDid.serialized, canonicalDid, "resolve:canonical");
  equal(resolved.document?.id.serialized, longFormDid, "resolve:subject");
  frozen(resolved, "resolve");
}

export async function assertPrismHistoryVector(createDidQueryProvider) {
  let request;
  const provider = await createDidQueryProvider({
    history: {
      async history(value) {
        request = value;
        frozen(value, "history:request");
        frozen(value.did, "history:did");
        frozen(value.canonicalDid, "history:canonical-did");
        return [signedCreate];
      },
    },
  });
  const state = await provider.resolvePrismDid(canonicalDid);
  equal(request?.did.serialized, canonicalDid, "history:requested-did");
  equal(request?.canonicalDid.serialized, canonicalDid, "history:canonical-did-value");
  equal(state.status, "active", "history:status");
  equal(state.did.serialized, canonicalDid, "history:state-did");
  equal(state.canonicalDid.serialized, canonicalDid, "history:state-canonical");
  equal(state.document?.id.serialized, canonicalDid, "history:document-subject");
  frozen(state, "history:state");
  frozen(state.document, "history:document");
}
