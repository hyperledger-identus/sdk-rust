use identus_derive::Newtype;

#[derive(Newtype)]
pub struct Named {
    inner: String,
}

fn main() {}
