use super::{Error, Value};
use serde::{de, forward_to_deserialize_any};

impl<'de> de::Deserializer<'de> for Value {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        // Deserialize based on the underlying type
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Int(i) => visitor.visit_i32(i),
            Value::SmallInt(i) => visitor.visit_i16(i),
            Value::Real(r) => visitor.visit_f32(r),
            Value::BigInt(i) => visitor.visit_i64(i),
            Value::Bool(b) => visitor.visit_bool(b),
            Value::Float(f) => visitor.visit_f64(f),
            Value::Text(s) => visitor.visit_string(s),
            _ => {
                unimplemented!("Type {:?}", self)
            }
        }
    }

    #[inline]
    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_bool(self.into_bool()?)
    }

    #[inline]
    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i8(self.into_i16()? as i8)
    }

    #[inline]
    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i16(self.into_i16()? as i16)
    }

    #[inline]
    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_i32(self.into_i32()? as i32)
    }

    #[inline]
    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_i64(self.into_i64()?)
    }

    #[inline]
    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u8(self.into_i16()? as u8)
    }

    #[inline]
    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u16(self.into_i16()? as u16)
    }

    #[inline]
    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u32(self.into_i32()? as u32)
    }

    #[inline]
    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        // FIXME: This should *fail* if the value does not fit in the requets integer type
        visitor.visit_u64(self.into_i64()? as u64)
    }

    #[inline]
    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_f32(self.into_f32()? as f32)
    }

    #[inline]
    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_f64(self.into_f64()?)
    }

    #[inline]
    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_string(self.into_str()?)
    }

    #[inline]
    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_string(self.into_str()?)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        // Match an explicit nil as None and everything else as Some
        match self {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // fn deserialize_enum<V>(
    //     self,
    //     name: &'static str,
    //     variants: &'static [&'static str],
    //     visitor: V,
    // ) -> Result<V::Value, Error>
    // where
    //     V: de::Visitor<'de>,
    // {
    //     visitor.visit_enum(EnumAccess {
    //         value: self,
    //         name,
    //         variants,
    //     })
    // }

    forward_to_deserialize_any! {
        char seq
        bytes byte_buf map struct unit enum
        identifier ignored_any unit_struct tuple_struct tuple
    }
}
