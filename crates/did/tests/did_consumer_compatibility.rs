use std::{
    collections::BTreeMap,
    future::Future,
    pin::pin,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

use identus_did::{
    Did, DidDocument, DidDocumentMetadata, DidMethod, DidMethodBinding, DidMethodRegistry,
    DidResolutionError, DidResolutionErrorKind, DidResolutionFuture, DidResolutionMetadata,
    DidResolutionResult, DidResolver, ResolutionOptions,
};
use serde_json::{Value, json};

const NEOPRISM_REVISION: &str = "8becb225132efb1d9302b2c5f6ed4d87b84e8685";
const MIDNIGHT_IDENTITY_REVISION: &str = "427f8571950c42967a18726cbcbefecc19ef8d79";
const LACE_ID_PORTAL_REVISION: &str = "804de0a9e58cf48ece3cc6c24b2245bb70bc80f1";
const OXID_REVISION: &str = "685f9670af4846d52697a4cfeb94779758ae1075";

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn parsed_document(value: Value) -> DidDocument {
    DidDocument::from_json_str(&value.to_string())
        .expect("independent compatibility shape is valid")
}

fn success(document: DidDocument, consumer: &'static str) -> DidResolutionResult {
    DidResolutionResult::success(
        DidResolutionMetadata::new(
            None,
            None,
            BTreeMap::from([
                ("consumer".to_owned(), json!(consumer)),
                ("syntheticFixture".to_owned(), json!(true)),
            ]),
        )
        .unwrap(),
        document,
        DidDocumentMetadata::empty(),
    )
    .unwrap()
}

struct StaticResolver {
    method: &'static str,
    consumer: &'static str,
    document: Option<DidDocument>,
}

impl DidResolver for StaticResolver {
    fn resolve<'a>(
        &'a self,
        did: &'a Did,
        _options: &'a ResolutionOptions,
    ) -> DidResolutionFuture<'a> {
        Box::pin(async move {
            assert_eq!(did.method(), self.method);
            let document = self
                .document
                .clone()
                .unwrap_or_else(|| DidDocument::builder(did.clone()).build().unwrap());
            assert_eq!(document.id(), did);
            success(document, self.consumer)
        })
    }
}

#[test]
fn neoprism_projection_preserves_relationships_and_migrates_errors_explicitly() {
    let value = json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": "did:prism:fixture123",
        "verificationMethod": [{
            "id": "did:prism:fixture123#key-1",
            "type": "JsonWebKey2020",
            "controller": "did:prism:fixture123",
            "publicKeyJwk": {
                "kty": "OKP",
                "crv": "Ed25519",
                "x": "11qYAYdk9JTuG6urO8YUNELcD9q4G2O_TW9LtUo9G2M"
            },
            "prism:usage": "master"
        }],
        "authentication": [
            "did:prism:fixture123#key-1",
            {
                "id": "did:prism:fixture123#embedded",
                "type": "Multikey",
                "controller": "did:prism:fixture123",
                "publicKeyMultibase": "z6MkrJVnaZkeFzdQy"
            }
        ],
        "prism:version": 3
    });

    let document = parsed_document(value.clone());
    assert_eq!(document.id().method(), "prism");
    assert_eq!(document.authentication().unwrap().len(), 2);
    assert_eq!(document.extensions()["prism:version"], 3);
    assert_eq!(serde_json::to_value(document).unwrap(), value);

    let migrated = DidResolutionError::from_legacy_keyword("notFound").unwrap();
    assert_eq!(migrated.kind(), Some(DidResolutionErrorKind::NotFound));
    assert!(serde_json::from_str::<DidResolutionError>(r#""notFound""#).is_err());

    let rejected = "did:prism:private fixture";
    let error = Did::parse(rejected).unwrap_err();
    assert!(!error.to_string().contains(rejected));
    assert_eq!(NEOPRISM_REVISION.len(), 40);
}

#[test]
fn midnight_identity_projection_preserves_rich_public_document() {
    let subject = format!("did:midnight:undeployed:{}", "a".repeat(64));
    let value = json!({
        "@context": [
            "https://www.w3.org/ns/did/v1",
            "https://w3id.org/security/jwk/v1"
        ],
        "id": subject,
        "controller": subject,
        "alsoKnownAs": ["urn:example:midnight-holder"],
        "verificationMethod": [
            {
                "id": format!("{subject}#signing"),
                "type": "JsonWebKey2020",
                "controller": subject,
                "publicKeyJwk": {
                    "kty": "OKP",
                    "crv": "Ed25519",
                    "x": "11qYAYdk9JTuG6urO8YUNELcD9q4G2O_TW9LtUo9G2M"
                }
            },
            {
                "id": format!("{subject}#agreement"),
                "type": "JsonWebKey2020",
                "controller": subject,
                "publicKeyJwk": {
                    "kty": "OKP",
                    "crv": "X25519",
                    "x": "hSDwCYkwp1R0i33ctD73Wg2_Og0mOBr066SpjqqbTmo"
                }
            }
        ],
        "authentication": [format!("{subject}#signing")],
        "assertionMethod": [format!("{subject}#signing")],
        "keyAgreement": [format!("{subject}#agreement")],
        "service": [{
            "id": format!("{subject}#messages"),
            "type": ["DIDCommMessaging", "MidnightMessaging"],
            "serviceEndpoint": [
                "https://wallet.example/messages",
                {"uri": "https://backup.example/messages", "accept": ["didcomm/v2"]}
            ]
        }],
        "midnight:network": "undeployed"
    });

    let document = parsed_document(value.clone());
    assert_eq!(document.id().method(), "midnight");
    assert_eq!(document.verification_methods().unwrap().len(), 2);
    assert_eq!(document.services().unwrap().len(), 1);
    assert_eq!(document.extensions()["midnight:network"], "undeployed");
    assert_eq!(serde_json::to_value(document).unwrap(), value);
    assert_eq!(MIDNIGHT_IDENTITY_REVISION.len(), 40);
}

#[test]
fn lace_projection_uses_object_safe_service_resolver_without_http_policy() {
    let did = Did::parse("did:midnight:testnet:lace-fixture").unwrap();
    let value = json!({
        "@context": "https://www.w3.org/ns/did/v1",
        "id": did.as_str(),
        "verificationMethod": [{
            "id": format!("{did}#portal"),
            "type": "Multikey",
            "controller": did.as_str(),
            "publicKeyMultibase": "z6MkrJVnaZkeFzdQy",
            "lace:purpose": "portal-authentication"
        }],
        "authentication": [format!("{did}#portal")],
        "service": [{
            "id": format!("{did}#credential-inbox"),
            "type": "CredentialInbox",
            "serviceEndpoint": {
                "uri": "https://identity.example/credentials",
                "accept": ["application/vp+ld+json"]
            },
            "lace:priority": 1
        }],
        "lace:portal": {"enabled": true, "synthetic": true}
    });
    let document = parsed_document(value.clone());
    let resolver: Arc<dyn DidResolver> = Arc::new(StaticResolver {
        method: "midnight",
        consumer: "lace-id-portal",
        document: Some(document),
    });

    let result = block_on(resolver.resolve(&did, &ResolutionOptions::empty()));
    result.validate_for(&did).unwrap();
    assert_eq!(result.metadata().extensions()["consumer"], "lace-id-portal");
    assert_eq!(result.metadata().extensions()["syntheticFixture"], true);
    assert!(result.metadata().content_type().is_none());
    let resolved_document = result.document().expect("resolver returns a DID document");
    assert_eq!(resolved_document.verification_methods().unwrap().len(), 1);
    assert_eq!(resolved_document.authentication().unwrap().len(), 1);
    assert_eq!(resolved_document.services().unwrap().len(), 1);
    assert_eq!(
        resolved_document.extensions()["lace:portal"]["enabled"],
        true
    );
    assert_eq!(serde_json::to_value(resolved_document).unwrap(), value);
    assert_eq!(LACE_ID_PORTAL_REVISION.len(), 40);
}

#[test]
fn oxid_projection_layers_wallet_policy_and_composes_methods() {
    let subject = format!("did:midnight:testnet:{}", "b".repeat(64));
    let value = json!({
        "@context": [
            "https://www.w3.org/ns/did/v1",
            {"oxid": "https://oxid.example/ns#"}
        ],
        "id": subject,
        "verificationMethod": [{
            "id": format!("{subject}#holder"),
            "type": "JsonWebKey2020",
            "controller": subject,
            "publicKeyJwk": {
                "kty": "OKP",
                "crv": "Ed25519",
                "x": "11qYAYdk9JTuG6urO8YUNELcD9q4G2O_TW9LtUo9G2M"
            }
        }],
        "authentication": [format!("{subject}#holder")],
        "oxid:profile": {"kind": "wallet", "synthetic": true}
    });

    let document = parsed_document(value.clone());
    assert_eq!(document.id().method(), "midnight");
    assert!(
        document
            .verification_methods()
            .unwrap()
            .iter()
            .all(|method| {
                method.controller() == document.id()
                    && method.public_key_jwk().is_some_and(|jwk| {
                        jwk.get("kty") == Some(&json!("OKP"))
                            && jwk.get("crv") == Some(&json!("Ed25519"))
                    })
            })
    );
    assert_eq!(serde_json::to_value(&document).unwrap(), value);

    let mut private = value;
    private["verificationMethod"][0]["publicKeyJwk"]["d"] = json!("private-fixture");
    assert!(DidDocument::from_json_str(&private.to_string()).is_err());

    let registry = DidMethodRegistry::builder()
        .register(DidMethodBinding::new(
            DidMethod::parse("midnight").unwrap(),
            Arc::new(StaticResolver {
                method: "midnight",
                consumer: "oxid",
                document: None,
            }),
        ))
        .unwrap()
        .register(DidMethodBinding::new(
            DidMethod::parse("prism").unwrap(),
            Arc::new(StaticResolver {
                method: "prism",
                consumer: "neoprism",
                document: None,
            }),
        ))
        .unwrap()
        .build();
    let resolver: Arc<dyn DidResolver> = Arc::new(registry);

    for (value, consumer) in [
        (subject.as_str(), "oxid"),
        ("did:prism:wallet-fixture", "neoprism"),
    ] {
        let did = Did::parse(value).unwrap();
        let result = block_on(resolver.resolve(&did, &ResolutionOptions::empty()));
        result.validate_for(&did).unwrap();
        assert_eq!(result.metadata().extensions()["consumer"], consumer);
    }
    assert_eq!(OXID_REVISION.len(), 40);
}
