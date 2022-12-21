use serde::Serialize;

#[derive(Debug)]
pub enum Value {
    String(&'static str),
    Number(i32),
    Boolean(bool),
    Null,
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::String(v) => serializer.serialize_str(v),
            Self::Number(v) => serializer.serialize_i32(v),
            Self::Boolean(v) => serializer.serialize_bool(v),
            Self::Null => serializer.serialize_none(),
        }
    }
}
