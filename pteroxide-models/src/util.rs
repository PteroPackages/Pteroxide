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

#[macro_export]
macro_rules! impl_time {
    ($type:ident) => {
        use $crate::util::{self, Time};

        impl $type {
            /// Parses the string created at time string into a [`Time`] object.
            pub fn parse_created_at(&self) -> Time {
                util::parse(self.created_at.clone())
            }

            /// Attempts to parse the created at time string into a [`Time`] object, returning an
            /// option.
            pub fn try_parse_created_at(&self) -> Option<Time> {
                match util::try_parse(self.created_at.clone()) {
                    Ok(t) => Some(t),
                    Err(_) => None,
                }
            }

            /// Parses the updated at time string into a [`Time`] object, returning an option if the field
            /// has a value.
            pub fn parse_updated_at(&self) -> Option<Time> {
                match &self.updated_at {
                    Some(s) => match util::try_parse(s.clone()) {
                        Ok(t) => Some(t),
                        Err(_) => None,
                    },
                    None => None,
                }
            }
        }
    };
}
