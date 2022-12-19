use serde::Serialize;

#[derive(Debug)]
pub struct Value {
    inner: String,
    kind: ValueKind,
}

impl Value {
    pub fn null() -> Self {
        Self {
            inner: Default::default(),
            kind: ValueKind::Null,
        }
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self {
            inner: String::from(v),
            kind: ValueKind::String,
        }
    }
}

impl From<String> for Value {
    fn from(inner: String) -> Self {
        Self {
            inner,
            kind: ValueKind::String,
        }
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Self {
            inner: v.to_string(),
            kind: ValueKind::Number,
        }
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self {
            inner: v.to_string(),
            kind: ValueKind::Number,
        }
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self {
            inner: v.to_string(),
            kind: ValueKind::Boolean,
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match self.kind {
            ValueKind::String => serializer.serialize_str(&self.inner),
            ValueKind::Number => serializer.serialize_i64(self.inner.parse::<i64>().unwrap()),
            ValueKind::Boolean => serializer.serialize_bool(self.inner.parse::<bool>().unwrap()),
            ValueKind::Null => serializer.serialize_none(),
        }
    }
}

#[derive(Debug)]
pub enum ValueKind {
    String,
    Number,
    Boolean,
    Null,
}
