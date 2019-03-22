use failure_derive::Fail;

#[derive(Debug, Fail)]
#[fail(display = "Usage: {} <directory>", _0)]
pub struct UsageError(pub String);
