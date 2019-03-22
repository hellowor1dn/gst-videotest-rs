use failure_derive::Fail;

#[derive(Debug, Fail)]
#[fail(display = "Missing element {}", _0)]
pub struct MissingElement(pub &'static str);
