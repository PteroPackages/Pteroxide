pub use time::{format_description::well_known::Iso8601, Error, PrimitiveDateTime, Time};

/// A helper method for parsing an [ISO 8601] time string into a [`Time`] object.
/// 
/// ## Errors
/// 
/// Returns an [`Error`] if the time string fails to be parsed.
///
/// [ISO 8601]: https://www.iso.org/iso-8601-date-and-time-format.html
pub fn try_parse(input: String) -> Result<Time, Error> {
    match PrimitiveDateTime::parse(&input, &Iso8601::DEFAULT) {
        Ok(t) => Ok(t.time()),
        Err(e) => Err(e.into()),
    }
}

/// Same as [`try_parse`] but unwraps the result value. You should only use this if you are certain
/// that the time string is valid.
///
/// [ISO 8601]: https://www.iso.org/iso-8601-date-and-time-format.html
pub fn parse(input: String) -> Time {
    try_parse(input).unwrap()
}
