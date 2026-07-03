use identus_derive::Newtype;

#[derive(Newtype)]
#[newtype(validate_err = E)]
pub struct Tag(String);

#[derive(Debug)]
struct E;

fn main() {}
