#[cfg(feature = "time")]
use chrono::{NaiveDate, NaiveDateTime};
#[cfg(feature = "json")]
use serde_json::Value as JsonValue;
use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
pub enum ValueRef<'a> {
    Text(#[cfg_attr(feature = "serde", serde(borrow))] Cow<'a, str>),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    Float(f64),
    Real(f32),
    Bool(bool),
    Binary(#[cfg_attr(feature = "serde", serde(borrow))] &'a [u8]),
    #[cfg(feature = "time")]
    Date(NaiveDate),
    #[cfg(feature = "time")]
    DateTime(NaiveDateTime),
    #[cfg(feature = "json")]
    Json(#[cfg_attr(feature = "serde", serde(borrow))] &'a JsonValue),
    Null,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "value"))]
pub enum Value {
    Text(String),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    Float(f64),
    Real(f32),
    Bool(bool),
    Binary(Vec<u8>),
    #[cfg(feature = "time")]
    Date(NaiveDate),
    #[cfg(feature = "time")]
    DateTime(NaiveDateTime),
    #[cfg(feature = "json")]
    Json(JsonValue),
    Null,
}

impl Value {
    pub fn as_ref<'a>(&'a self) -> ValueRef<'a> {
        match self {
            Value::SmallInt(i) => ValueRef::SmallInt(*i),
            Value::Int(i) => ValueRef::Int(*i),
            Value::BigInt(i) => ValueRef::BigInt(*i),
            Value::Float(f) => ValueRef::Float(*f),
            Value::Real(d) => ValueRef::Real(*d),
            Value::Bool(b) => ValueRef::Bool(*b),
            Value::Binary(b) => ValueRef::Binary(b.as_ref()),
            Value::Text(s) => ValueRef::Text(Cow::Borrowed(s.as_str())),
            #[cfg(feature = "time")]
            Value::Date(data) => ValueRef::Date(*data),
            #[cfg(feature = "time")]
            Value::DateTime(data) => ValueRef::DateTime(*data),
            #[cfg(feature = "json")]
            Value::Json(json) => ValueRef::Json(json),
            Value::Null => ValueRef::Null,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Text(s) => serializer.serialize_str(&s),
            Value::BigInt(i) => serializer.serialize_i64(*i),
            Value::Int(i) => serializer.serialize_i32(*i),
            Value::SmallInt(i) => serializer.serialize_i16(*i),
            #[cfg(feature = "time")]
            Value::Date(date) => date.serialize(serializer),
            #[cfg(feature = "time")]
            Value::DateTime(date) => date.serialize(serializer),
            Value::Null => serializer.serialize_unit(),
            Value::Binary(b) => serializer.serialize_bytes(&b),
            _ => {
                unimplemented!("not implemented {:?}", self)
            }
        }
    }
}

impl<'a> From<ValueRef<'a>> for Value {
    fn from(value: ValueRef<'a>) -> Value {
        match value {
            ValueRef::SmallInt(i) => Value::SmallInt(i),
            ValueRef::Int(i) => Value::Int(i),
            ValueRef::BigInt(i) => Value::BigInt(i),
            ValueRef::Float(f) => Value::Float(f),
            ValueRef::Real(d) => Value::Real(d),
            ValueRef::Bool(b) => Value::Bool(b),
            ValueRef::Binary(b) => Value::Binary(b.to_vec()),
            ValueRef::Text(s) => Value::Text(s.to_string()),
            #[cfg(feature = "time")]
            ValueRef::Date(data) => Value::Date(data),
            #[cfg(feature = "time")]
            ValueRef::DateTime(data) => Value::DateTime(data),
            #[cfg(feature = "json")]
            ValueRef::Json(json) => Value::Json(json.clone()),
            ValueRef::Null => Value::Null,
        }
    }
}

impl<'a> From<&'a str> for ValueRef<'a> {
    fn from(s: &'a str) -> ValueRef<'a> {
        ValueRef::Text(s.into())
    }
}

impl<'a> From<i32> for ValueRef<'a> {
    fn from(value: i32) -> Self {
        ValueRef::Int(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::Text(value)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Value {
        Value::Text(value.to_owned())
    }
}

impl<'a> From<&'a [u8]> for Value {
    fn from(value: &'a [u8]) -> Value {
        Value::Binary(value.to_vec())
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Value {
        Value::Binary(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Value {
        Value::SmallInt(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Value {
        Value::Int(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Value {
        Value::BigInt(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Value {
        Value::Real(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value::Float(value)
    }
}
