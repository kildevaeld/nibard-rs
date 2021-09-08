use super::{Column, DatabaseRow, Row};
use nibard_shared::{Error as SharedError, Value};
use serde::{de, forward_to_deserialize_any};
use std::collections::VecDeque;

struct MapAccess<R> {
    row: R,
    columns: VecDeque<Column<'static>>,
}

impl<R> MapAccess<R>
where
    R: Row,
{
    fn new(table: R) -> Self {
        let columns: Vec<Column<'static>> = unsafe { std::mem::transmute(table.columns()) };
        MapAccess {
            row: table,
            columns: columns.into_iter().collect(),
        }
    }
}

struct StrDeserializer<'a>(&'a str);

impl<'a> StrDeserializer<'a> {
    fn new(key: &'a str) -> Self {
        StrDeserializer(key)
    }
}

impl<'de, 'a> de::Deserializer<'de> for StrDeserializer<'a> {
    type Error = SharedError;

    #[inline]
    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_str(self.0)
    }

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq
        bytes byte_buf map struct unit enum newtype_struct
        identifier ignored_any unit_struct tuple_struct tuple option
    }
}

impl<'de, R> de::MapAccess<'de> for MapAccess<R>
where
    R: Row,
{
    type Error = SharedError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some(&ref key_s) = self.columns.front() {
            let key_de = StrDeserializer::new(key_s.name as &str);
            let key = de::DeserializeSeed::deserialize(seed, key_de)?;

            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key = self.columns.pop_front().unwrap();
        let value: Value = self.row.try_get(&key.name).expect("serialize to value");
        de::DeserializeSeed::deserialize(seed, value) //.map_err(|e| e.prepend_key(key))
    }
}

impl<'de> de::Deserializer<'de> for DatabaseRow {
    type Error = SharedError;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(MapAccess::new(self))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
