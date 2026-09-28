use identus_derive::Newtype;

#[derive(Newtype)]
#[newtype(serde)]
struct PlainString(String);

#[derive(Newtype)]
#[newtype(serde, validate_fn = validate_string, validate_err = ValidationError)]
struct ValidatedString(String);

#[derive(Newtype)]
#[newtype(serde)]
struct PlainNumber(u16);

#[derive(Newtype)]
#[newtype(serde, validate_fn = validate_number, validate_err = ValidationError)]
struct ValidatedNumber(u16);

#[derive(Debug)]
struct ValidationError;

impl core::fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("invalid value")
    }
}

fn validate_string(value: &str) -> Result<(), ValidationError> {
    (!value.is_empty()).then_some(()).ok_or(ValidationError)
}

fn validate_number(value: &u16) -> Result<(), ValidationError> {
    (*value > 0).then_some(()).ok_or(ValidationError)
}

fn main() {
    let _: PlainString = serde_json::from_str(r#""plain""#).unwrap();
    let _: ValidatedString = serde_json::from_str(r#""validated""#).unwrap();
    let _: PlainNumber = serde_json::from_str("1").unwrap();
    let _: ValidatedNumber = serde_json::from_str("1").unwrap();
}
