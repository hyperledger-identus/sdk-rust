function assertEqual(actual, expected, label) {
  if (!Object.is(actual, expected)) {
    throw new Error(`Peer DID vector assertion failed: ${label}`);
  }
}

function assertFrozen(value, label) {
  assertEqual(Object.isFrozen(value), true, `${label}:frozen`);
}

const errorMessages = Object.freeze({
  "did.peer.wrong_profile": "Peer DID profile is invalid for this operation",
  "did.peer.unsupported_algorithm": "Peer DID algorithm is unsupported",
  "did.peer.malformed_encoding": "Peer DID encoding is malformed",
  "did.peer.unsupported_codec": "Peer DID codec is unsupported",
  "did.peer.invalid_input_document": "Peer DID input document is invalid",
  "did.peer.hash_mismatch": "Peer DID long and short forms do not match",
  "did.peer.long_form_required": "Peer DID long form is required",
  "did.peer.resource_limit": "Peer DID resource limit was exceeded",
});

function assertDidError(run, vector) {
  try {
    run();
  } catch (error) {
    assertEqual(error?.name, "DidError", `${vector.id}:error-name`);
    assertEqual(error?.code, vector.expected.errorCode, `${vector.id}:error-code`);
    assertEqual(error?.message, errorMessages[error.code], `${vector.id}:message`);
    assertEqual(error?.stack, undefined, `${vector.id}:stack`);
    const serialized = JSON.stringify(error);
    assertEqual(
      serialized.includes("private-scalar") || serialized.includes("did:example:123"),
      false,
      `${vector.id}:redacted`,
    );
    return;
  }
  throw new Error(`Peer DID vector assertion failed: ${vector.id}:expected-error`);
}

function generatedDocument({ prefix, repeat, count, suffix }) {
  return JSON.parse(`${prefix}${repeat.repeat(count)}${suffix}`);
}

export function assertAllPeerVectors(provider, vectors) {
  const current = vectors.find((vector) => vector.operation === "did-peer.numalgo4.create");
  const compatibility = vectors.find(
    (vector) => vector.operation === "did-peer.numalgo2.parse-resolve",
  );
  const pairwiseDonor = vectors.find(
    (vector) => vector.operation === "did-peer.numalgo2.create-pairwise",
  );
  if (current === undefined || compatibility === undefined || pairwiseDonor === undefined) {
    throw new Error("Peer DID vector packet is incomplete");
  }

  const inputDocument = JSON.parse(current.input.literal);
  const createdCurrent = provider.createPeerDidNumalgo4(inputDocument);
  assertEqual(
    createdCurrent.longForm.serialized,
    current.expected.fields.did,
    `${current.id}:long-form`,
  );
  assertEqual(createdCurrent.profile, "numalgo4", `${current.id}:profile`);
  assertFrozen(createdCurrent, current.id);
  assertFrozen(createdCurrent.inputDocument, `${current.id}:input-document`);
  assertFrozen(createdCurrent.document, `${current.id}:document`);

  const parsedCurrent = provider.parsePeerDidNumalgo4Long(
    createdCurrent.longForm.serialized,
  );
  assertEqual(
    parsedCurrent.shortForm.serialized,
    createdCurrent.shortForm.serialized,
    `${current.id}:parsed-short-form`,
  );
  const parsedShort = provider.parsePeerDidNumalgo4Short(
    createdCurrent.shortForm.serialized,
  );
  assertEqual(
    parsedShort.shortForm.serialized,
    createdCurrent.shortForm.serialized,
    `${current.id}:validated-short-form`,
  );
  const resolvedShort = provider.resolvePeerDidNumalgo4Short(
    createdCurrent.shortForm.serialized,
    createdCurrent.longForm.serialized,
  );
  assertEqual(
    resolvedShort.document.id.serialized,
    createdCurrent.shortForm.serialized,
    `${current.id}:short-subject`,
  );

  const parsedCompatibility = provider.parsePeerDidNumalgo2Compatibility(
    compatibility.input.literal,
  );
  assertEqual(
    parsedCompatibility.did.serialized,
    compatibility.expected.fields.did,
    `${compatibility.id}:parsed`,
  );
  assertEqual(
    parsedCompatibility.document.authentication?.[0]?.reference,
    `${compatibility.input.literal}#key-1`,
    `${compatibility.id}:authentication`,
  );
  assertEqual(
    parsedCompatibility.document.keyAgreement?.[0]?.reference,
    `${compatibility.input.literal}#key-2`,
    `${compatibility.id}:agreement`,
  );
  assertEqual(
    parsedCompatibility.document.service?.[0]?.type,
    "DIDCommMessaging",
    `${compatibility.id}:service`,
  );

  const createdPairwise = provider.createPeerDidNumalgo2Compatibility(
    JSON.parse(pairwiseDonor.input.literal),
  );
  assertEqual(
    createdPairwise.did.serialized,
    pairwiseDonor.expected.fields.did,
    `${pairwiseDonor.id}:created`,
  );
  assertEqual(
    createdPairwise.document.authentication?.[0]?.reference,
    `${createdPairwise.did.serialized}${pairwiseDonor.expected.fields.authentication}`,
    `${pairwiseDonor.id}:authentication`,
  );
  assertEqual(
    createdPairwise.document.keyAgreement?.[0]?.reference,
    `${createdPairwise.did.serialized}${pairwiseDonor.expected.fields.keyAgreement}`,
    `${pairwiseDonor.id}:key-agreement`,
  );
  const donorService = createdPairwise.document.service?.[0];
  assertEqual(
    donorService?.type,
    pairwiseDonor.expected.fields.serviceType,
    `${pairwiseDonor.id}:service-type`,
  );
  assertEqual(
    donorService?.serviceEndpoint?.uri,
    pairwiseDonor.expected.fields.serviceUri,
    `${pairwiseDonor.id}:service-uri`,
  );
  assertEqual(
    donorService?.serviceEndpoint?.routingKeys?.[0],
    pairwiseDonor.expected.fields.routingKey,
    `${pairwiseDonor.id}:routing-key`,
  );
  assertEqual(
    donorService?.serviceEndpoint?.accept?.length,
    pairwiseDonor.expected.fields.acceptCount,
    `${pairwiseDonor.id}:accept`,
  );
  assertFrozen(createdPairwise, pairwiseDonor.id);
  assertFrozen(createdPairwise.document, `${pairwiseDonor.id}:document`);

  const pairwise = Object.freeze({
    from: createdPairwise.did.serialized,
    to: createdCurrent.shortForm.serialized,
  });
  assertFrozen(pairwise, "pairwise");
  assertEqual(pairwise.from, pairwiseDonor.expected.fields.did, "pairwise:from");
  assertEqual(pairwise.to, createdCurrent.shortForm.serialized, "pairwise:to");

  for (const vector of vectors) {
    if (vector.expected.kind === "success") {
      if (vector.input.generated !== undefined) {
        const value = provider.createPeerDidNumalgo4(
          generatedDocument(vector.input.generated),
        );
        assertFrozen(value, vector.id);
      }
      continue;
    }
    const invoke = (() => {
      switch (vector.operation) {
        case "did-peer.numalgo4.parse-long":
          return () => provider.parsePeerDidNumalgo4Long(vector.input.literal);
        case "did-peer.numalgo2.parse-resolve":
          return () => provider.parsePeerDidNumalgo2Compatibility(vector.input.literal);
        case "did-peer.numalgo4.resolve-short":
          return () => provider.resolvePeerDidNumalgo4Short(vector.input.literal, undefined);
        case "did-peer.numalgo4.create":
          return () => provider.createPeerDidNumalgo4(
            vector.input.generated === undefined
              ? JSON.parse(vector.input.literal)
              : generatedDocument(vector.input.generated),
          );
        case "did-peer.numalgo2.create":
          return () => provider.createPeerDidNumalgo2Compatibility(
            JSON.parse(vector.input.literal),
          );
        default:
          throw new Error(`Unsupported peer DID vector operation: ${vector.operation}`);
      }
    })();
    assertDidError(invoke, vector);
  }
}
