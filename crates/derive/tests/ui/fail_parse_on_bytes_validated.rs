use identus_derive::Newtype;

#[derive(Newtype)]
#[newtype(display, validate_fn = validate, validate_err = E)]
pub struct NonEmpty(Vec<u8>);

fn validate(v: &[u8]) -> Result<(), E> {
    let _ = v;
    Ok(())
}

#[derive(Debug)]
struct E;

fn main() {
    // `parse`/`FromStr` are not generated for the bytes category.
    let _ = NonEmpty::parse("deadbeef");
}
