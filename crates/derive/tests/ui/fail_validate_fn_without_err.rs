use identus_derive::Newtype;

#[derive(Newtype)]
#[newtype(validate_fn = validate)]
pub struct Tag(String);

fn validate(s: &str) -> Result<(), E> {
    let _ = s;
    Ok(())
}

fn main() {}
