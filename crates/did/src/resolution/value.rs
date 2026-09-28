use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::{Error, error::ResolutionError};

/// Maximum byte length of a media type.
pub const MAX_MEDIA_TYPE_BYTES: usize = 1_024;
/// Maximum byte length of a DID Resolution datetime.
pub const MAX_DID_RESOLUTION_DATETIME_BYTES: usize = 128;
/// Maximum byte length of a version identifier.
pub const MAX_VERSION_ID_BYTES: usize = 1_024;
/// A bounded, syntactically valid media type value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MediaType(String);

impl MediaType {
    /// Parse a single media type and preserve its exact valid spelling.
    pub fn parse(value: &str) -> Result<Self, Error> {
        validate_media_type(value)?;
        Ok(Self(value.to_owned()))
    }

    /// Validate an owned media type while retaining its allocation.
    pub fn try_new(value: String) -> Result<Self, Error> {
        validate_media_type(&value)?;
        Ok(Self(value))
    }

    /// Borrow the exact validated value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume this value.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

/// A bounded XML Schema 1.1 whole-second UTC datetime used by DID Resolution.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DidResolutionDateTime(String);

impl DidResolutionDateTime {
    /// Parse the XML Schema 1.1 profile adjusted to UTC whole seconds.
    pub fn parse(value: &str) -> Result<Self, Error> {
        validate_datetime(value)?;
        Ok(Self(value.to_owned()))
    }

    /// Validate an owned datetime while retaining its allocation.
    pub fn try_new(value: String) -> Result<Self, Error> {
        validate_datetime(&value)?;
        Ok(Self(value))
    }

    /// Borrow the exact validated value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume this value.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

/// A bounded opaque DID document version identifier.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VersionId(String);

impl VersionId {
    /// Parse a non-empty, trimmed printable ASCII version id.
    pub fn parse(value: &str) -> Result<Self, Error> {
        validate_version_id(value)?;
        Ok(Self(value.to_owned()))
    }

    /// Validate an owned version id while retaining its allocation.
    pub fn try_new(value: String) -> Result<Self, Error> {
        validate_version_id(&value)?;
        Ok(Self(value))
    }

    /// Borrow the exact validated value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume this value.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

macro_rules! impl_string_value {
    ($name:ty) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }

        impl TryFrom<String> for $name {
            type Error = Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::try_new(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::try_new(value).map_err(de::Error::custom)
            }
        }
    };
}

impl_string_value!(MediaType);
impl_string_value!(DidResolutionDateTime);
impl_string_value!(VersionId);
fn validate_version_id(value: &str) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > MAX_VERSION_ID_BYTES
        || !value.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
        || value.trim() != value
    {
        return Err(Error::InvalidResolution(ResolutionError::InvalidString));
    }
    Ok(())
}

fn validate_datetime(value: &str) -> Result<(), Error> {
    let bytes = value.as_bytes();
    if bytes.len() < 20
        || bytes.len() > MAX_DID_RESOLUTION_DATETIME_BYTES
        || !bytes.is_ascii()
        || bytes.last() != Some(&b'Z')
    {
        return Err(Error::InvalidResolution(ResolutionError::InvalidDateTime));
    }

    let year_start = usize::from(bytes.first() == Some(&b'-'));
    let Some(year_end) = bytes[year_start..]
        .iter()
        .position(|byte| *byte == b'-')
        .map(|offset| year_start + offset)
    else {
        return Err(Error::InvalidResolution(ResolutionError::InvalidDateTime));
    };
    let year = &bytes[year_start..year_end];
    if year.len() < 4
        || !year.iter().all(u8::is_ascii_digit)
        || (year.len() > 4 && year.first() == Some(&b'0'))
    {
        return Err(Error::InvalidResolution(ResolutionError::InvalidDateTime));
    }

    let date_time_tail = &bytes[year_end..];
    if date_time_tail.len() != 16
        || date_time_tail[0] != b'-'
        || date_time_tail[3] != b'-'
        || date_time_tail[6] != b'T'
        || date_time_tail[9] != b':'
        || date_time_tail[12] != b':'
        || date_time_tail[15] != b'Z'
        || date_time_tail
            .iter()
            .enumerate()
            .filter(|(index, _)| ![0, 3, 6, 9, 12, 15].contains(index))
            .any(|(_, byte)| !byte.is_ascii_digit())
    {
        return Err(Error::InvalidResolution(ResolutionError::InvalidDateTime));
    }

    let number = |start: usize, end: usize| -> u32 {
        date_time_tail[start..end]
            .iter()
            .fold(0, |value, byte| value * 10 + u32::from(byte - b'0'))
    };
    let month = number(1, 3);
    let day = number(4, 6);
    let hour = number(7, 9);
    let minute = number(10, 12);
    let second = number(13, 15);
    let year_mod_400 = year.iter().fold(0_u16, |value, byte| {
        (value * 10 + u16::from(byte - b'0')) % 400
    });
    let leap = year_mod_400 % 4 == 0 && (year_mod_400 % 100 != 0 || year_mod_400 == 0);
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => 0,
    };
    let valid_time =
        hour <= 23 && minute <= 59 && second <= 59 || hour == 24 && minute == 0 && second == 0;
    if day == 0 || day > max_day || !valid_time {
        return Err(Error::InvalidResolution(ResolutionError::InvalidDateTime));
    }
    Ok(())
}
fn validate_media_type(value: &str) -> Result<(), Error> {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes.len() > MAX_MEDIA_TYPE_BYTES || !bytes.is_ascii() {
        return Err(Error::InvalidResolution(ResolutionError::InvalidMediaType));
    }

    let mut index = consume_token(bytes, 0)?;
    if bytes.get(index) != Some(&b'/') {
        return Err(Error::InvalidResolution(ResolutionError::InvalidMediaType));
    }
    index = consume_token(bytes, index + 1)?;
    while index < bytes.len() {
        index = consume_ows(bytes, index);
        if bytes.get(index) != Some(&b';') {
            return Err(Error::InvalidResolution(ResolutionError::InvalidMediaType));
        }
        index = consume_ows(bytes, index + 1);
        index = consume_token(bytes, index)?;
        index = consume_ows(bytes, index);
        if bytes.get(index) != Some(&b'=') {
            return Err(Error::InvalidResolution(ResolutionError::InvalidMediaType));
        }
        index = consume_ows(bytes, index + 1);
        if bytes.get(index) == Some(&b'"') {
            index = consume_quoted(bytes, index + 1)?;
        } else {
            index = consume_token(bytes, index)?;
        }
    }
    Ok(())
}

fn consume_token(bytes: &[u8], start: usize) -> Result<usize, Error> {
    let mut index = start;
    while bytes.get(index).is_some_and(|byte| is_token(*byte)) {
        index += 1;
    }
    if index == start {
        return Err(Error::InvalidResolution(ResolutionError::InvalidMediaType));
    }
    Ok(index)
}

fn consume_ows(bytes: &[u8], mut index: usize) -> usize {
    while bytes
        .get(index)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t'))
    {
        index += 1;
    }
    index
}

fn consume_quoted(bytes: &[u8], mut index: usize) -> Result<usize, Error> {
    while let Some(byte) = bytes.get(index) {
        match byte {
            b'"' => return Ok(index + 1),
            b'\\' => {
                index += 1;
                if !bytes
                    .get(index)
                    .is_some_and(|escaped| matches!(escaped, b'\t' | 0x20..=0x7e))
                {
                    return Err(Error::InvalidResolution(ResolutionError::InvalidMediaType));
                }
            }
            b'\t' | 0x20..=0x21 | 0x23..=0x5b | 0x5d..=0x7e => {}
            _ => return Err(Error::InvalidResolution(ResolutionError::InvalidMediaType)),
        }
        index += 1;
    }
    Err(Error::InvalidResolution(ResolutionError::InvalidMediaType))
}

const fn is_token(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || matches!(
            byte,
            b'!' | b'#'
                | b'$'
                | b'%'
                | b'&'
                | b'\''
                | b'*'
                | b'+'
                | b'-'
                | b'.'
                | b'^'
                | b'_'
                | b'`'
                | b'|'
                | b'~'
        )
}
