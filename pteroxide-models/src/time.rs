use time::{format_description::well_known::Iso8601, Error, PrimitiveDateTime, Time};

pub fn try_parse(input: String) -> Result<Time, Error> {
    match PrimitiveDateTime::parse(&input, &Iso8601::DEFAULT) {
        Ok(t) => Ok(t.time()),
        Err(e) => Err(e.into()),
    }
}

pub fn parse(input: String) -> Time {
    try_parse(input).unwrap()
}
