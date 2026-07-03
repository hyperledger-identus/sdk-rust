use identus_derive::Newtype;

#[derive(Newtype)]
#[newtype(validate_fn = validate, validate_err = E)]
pub struct Tag(String);

fn validate(s: &str) -> Result<(), E> {
    let _ = s;
    Ok(())
}

#[derive(Debug)]
struct E;

fn main() {
    // The infallible `new` is not generated for `validate_fn`-configured types.
    let _ = Tag::new("hi".to_owned());
}
