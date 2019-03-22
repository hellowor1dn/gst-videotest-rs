use gstreamer as gst;
use gst::prelude::*;

use failure::Error;

mod errors;
pub use self::errors::*;

pub fn make_element<'a, P: Into<Option<&'a str>>>(
    factory_name: &'static str,
    element_name: P,
) -> Result<gst::Element, Error> {
    match gst::ElementFactory::make(factory_name, element_name.into()) {
        Some(elem) => Ok(elem),
        None => Err(Error::from(MissingElement(factory_name))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_element() {
        // TODO test make element
    }
}
