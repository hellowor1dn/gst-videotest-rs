use failure_derive::Fail;

#[derive(Debug, Fail)]
#[fail(display = "Failed to add bus callback")]
pub struct WatchError;
