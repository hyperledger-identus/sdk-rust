use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::{Error, error::DocumentError};

use super::invalid;

/// A non-empty value that preserves whether its JSON form was one value or an
/// array.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OneOrMany<T> {
    values: Vec<T>,
    is_many: bool,
}

impl<T> OneOrMany<T> {
    /// Construct the scalar representation.
    #[must_use]
    pub fn one(value: T) -> Self {
        Self {
            values: vec![value],
            is_many: false,
        }
    }

    /// Construct the array representation, rejecting an empty array.
    ///
    /// This representation helper intentionally does not apply a DID domain
    /// collection ceiling. Validated owners such as [`DidDocument`] and
    /// [`Service`] enforce [`MAX_DOCUMENT_ITEMS`] before they are returned.
    pub fn try_many(values: Vec<T>) -> Result<Self, Error> {
        if values.is_empty() {
            return Err(invalid(DocumentError::EmptyValue));
        }
        Ok(Self {
            values,
            is_many: true,
        })
    }

    /// Borrow the normalized non-empty slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    /// Return `true` when the preserved JSON representation is an array.
    #[must_use]
    pub const fn is_many(&self) -> bool {
        self.is_many
    }

    /// Consume the value and return its normalized items.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.values
    }
}

impl<T> From<T> for OneOrMany<T> {
    fn from(value: T) -> Self {
        Self::one(value)
    }
}

impl<T: Serialize> Serialize for OneOrMany<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.is_many {
            self.values.serialize(serializer)
        } else {
            self.values[0].serialize(serializer)
        }
    }
}

impl<'de, T> Deserialize<'de> for OneOrMany<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Wire<T> {
            One(T),
            Many(Vec<T>),
        }

        match Wire::deserialize(deserializer)? {
            Wire::One(value) => Ok(Self::one(value)),
            Wire::Many(values) => Self::try_many(values).map_err(de::Error::custom),
        }
    }
}
