use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::Value;

use crate::{
    Did, Error, Uri,
    error::DocumentError,
    json_cleanup::{RejectionGuard, drop_json_values_iteratively},
    multibase::is_canonical_public_key_carrier,
};

use super::{
    JsonBudget, MAX_OPEN_TYPE_BYTES, MAX_PUBLIC_KEY_MULTIBASE_BYTES, PRIVATE_JWK_MEMBERS,
    VERIFICATION_RESERVED, invalid, validate_json_map, validate_open_string,
};

/// A verification method used directly inside a relationship or by reference.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerificationRelationship {
    /// An RFC 3986 reference to a verification method resource.
    Reference(Uri),
    /// A verification method scoped to this relationship.
    Embedded(VerificationMethod),
}
/// An extensible W3C verification method with public material only.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VerificationMethod {
    id: Uri,
    #[serde(rename = "type")]
    type_: String,
    controller: Did,
    #[serde(flatten)]
    properties: BTreeMap<String, Value>,
}

impl VerificationMethod {
    /// Construct and validate a verification method.
    pub fn new(
        id: Uri,
        type_: String,
        controller: Did,
        properties: BTreeMap<String, Value>,
    ) -> Result<Self, Error> {
        let method = RejectionGuard::new(
            Self {
                id,
                type_,
                controller,
                properties,
            },
            drop_verification_method_json,
        );
        method.owner().validate(&mut JsonBudget::default())?;
        Ok(method.into_owner())
    }

    /// Borrow the verification method identifier.
    #[must_use]
    pub const fn id(&self) -> &Uri {
        &self.id
    }

    /// Borrow the open verification method type name.
    #[must_use]
    pub fn method_type(&self) -> &str {
        &self.type_
    }

    /// Borrow the controlling DID.
    #[must_use]
    pub const fn controller(&self) -> &Did {
        &self.controller
    }

    /// Borrow all suite-defined verification properties.
    #[must_use]
    pub const fn properties(&self) -> &BTreeMap<String, Value> {
        &self.properties
    }

    /// Borrow the recognized public JWK map, when present.
    #[must_use]
    pub fn public_key_jwk(&self) -> Option<&serde_json::Map<String, Value>> {
        self.properties
            .get("publicKeyJwk")
            .and_then(Value::as_object)
    }

    /// Borrow the recognized multibase public key string, when present.
    ///
    /// Construction accepts canonical `z` base58-btc and `u`
    /// base64url-no-pad carriers. This accessor does not imply that the
    /// decoded bytes contain a supported multicodec or cryptographic key.
    #[must_use]
    pub fn public_key_multibase(&self) -> Option<&str> {
        self.properties
            .get("publicKeyMultibase")
            .and_then(Value::as_str)
    }

    pub(super) fn validate(&self, budget: &mut JsonBudget) -> Result<(), Error> {
        validate_open_string(&self.type_, MAX_OPEN_TYPE_BYTES)?;
        if self.properties.is_empty() {
            return Err(invalid(DocumentError::EmptyValue));
        }
        validate_json_map(&self.properties, VERIFICATION_RESERVED, budget)?;

        let jwk = self.properties.get("publicKeyJwk");
        let multibase = self.properties.get("publicKeyMultibase");
        if jwk.is_some() && multibase.is_some() {
            return Err(invalid(DocumentError::MultipleVerificationMaterial));
        }
        if let Some(jwk) = jwk {
            validate_public_jwk(jwk)?;
        }
        if let Some(multibase) = multibase {
            let Some(multibase) = multibase.as_str() else {
                return Err(invalid(DocumentError::InvalidPropertyShape));
            };
            if multibase.is_empty()
                || multibase.len() > MAX_PUBLIC_KEY_MULTIBASE_BYTES
                || !is_canonical_public_key_carrier(multibase)
            {
                return Err(invalid(DocumentError::InvalidString));
            }
        }
        Ok(())
    }
}

fn drop_verification_method_json(method: VerificationMethod) {
    let mut roots = Vec::new();
    collect_verification_method_json(method, &mut roots);
    drop_json_values_iteratively(roots);
}

pub(super) fn collect_verification_method_json(method: VerificationMethod, roots: &mut Vec<Value>) {
    let VerificationMethod {
        id: _,
        type_: _,
        controller: _,
        properties,
    } = method;
    roots.extend(properties.into_values());
}

impl<'de> Deserialize<'de> for VerificationMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            id: Uri,
            #[serde(rename = "type")]
            type_: String,
            controller: Did,
            #[serde(flatten)]
            properties: BTreeMap<String, Value>,
        }

        let wire = Wire::deserialize(deserializer)?;
        Self::new(wire.id, wire.type_, wire.controller, wire.properties).map_err(de::Error::custom)
    }
}
fn validate_public_jwk(value: &Value) -> Result<(), Error> {
    let Some(jwk) = value.as_object() else {
        return Err(invalid(DocumentError::InvalidPropertyShape));
    };
    if PRIVATE_JWK_MEMBERS
        .iter()
        .any(|member| jwk.contains_key(*member))
    {
        return Err(invalid(DocumentError::PrivateKeyMaterial));
    }
    let Some(key_type) = jwk.get("kty").and_then(Value::as_str) else {
        return Err(invalid(DocumentError::InvalidPropertyShape));
    };
    validate_open_string(key_type, MAX_OPEN_TYPE_BYTES)
}
