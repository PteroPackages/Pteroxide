use serde::{Deserialize, Serialize};

/// A value wrapper for environment variables. Environment objects can contain or require values of
/// more than one type, however, Rust does not support unions so instead of coercing types from
/// a string, this enum wraps the value to [`String`][Value::String] (which is a static string
/// slice), [`Number`][Value::Number] (a 32-bit integer), [`Boolean`][Value::Boolean] for bools,
/// and [`Null`][Value::Null] for [`None`].
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
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
