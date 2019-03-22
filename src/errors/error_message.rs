use failure_derive::Fail;

#[derive(Debug, Fail)]
#[fail(display = "Received error message from {}: {} (debug: {:?})", src, error, debug)]
pub struct ErrorMessage {
    pub src: String,
    pub error: String,
    pub debug: Option<String>,
    #[cause]
    pub cause: glib::Error,
}
