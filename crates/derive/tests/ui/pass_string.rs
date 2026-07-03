use identus_derive::Newtype;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Newtype)]
#[newtype(display)]
pub struct Tag(String);

fn main() {
    let t = Tag::new("hi".to_owned());
    let _ = t.to_string();
}
