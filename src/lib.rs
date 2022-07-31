#![doc = include_str!("../README.md")]

use std::io::Cursor;

use serde::de::Visitor;
use serde::serde_if_integer128;
use serde_json::de::{IoRead, Read, StrRead};

/// A wrapper over [`serde_json::Deserializer`] `where OwnedJsonDeserializer: serde::Deserializer`.
///
/// Because apparently `serde_json` only deserializes through a reference™.
pub struct OwnedJsonDeserializer<T>(pub serde_json::Deserializer<T>);

impl OwnedJsonDeserializer<StrRead<'static>> {
    /// Creates an owned JSON deserializer from a `&'static str`.
    #[allow(clippy::should_implement_trait)] // matches method on serde_json::Deserializer
    pub fn from_str(s: &'static str) -> Self {
        OwnedJsonDeserializer(serde_json::Deserializer::from_str(s))
    }
}

impl OwnedJsonDeserializer<IoRead<Cursor<Vec<u8>>>> {
    /// Creates an owned JSON deserializer from an owned String.
    pub fn from_string(s: String) -> Self {
        OwnedJsonDeserializer(serde_json::Deserializer::from_reader(Cursor::new(s.into_bytes())))
    }
}

impl<'de, T> serde::Deserializer<'de> for OwnedJsonDeserializer<T>
where
    T: Read<'de>,
{
    type Error = serde_json::Error;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_any(visitor)
    }

    #[inline]
    fn deserialize_bool<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_bool(visitor)
    }

    #[inline]
    fn deserialize_i8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_i8(visitor)
    }

    #[inline]
    fn deserialize_i16<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_i16(visitor)
    }

    #[inline]
    fn deserialize_i32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_i32(visitor)
    }

    #[inline]
    fn deserialize_i64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_i64(visitor)
    }

    serde_if_integer128! {
        #[inline]
        fn deserialize_i128<V>(mut self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
            self.0.deserialize_i128(visitor)
        }
    }

    #[inline]
    fn deserialize_u8<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_u8(visitor)
    }

    #[inline]
    fn deserialize_u16<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_u16(visitor)
    }

    #[inline]
    fn deserialize_u32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_u32(visitor)
    }

    #[inline]
    fn deserialize_u64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_u64(visitor)
    }

    serde_if_integer128! {
        #[inline]
        fn deserialize_u128<V>(mut self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
            self.0.deserialize_u128(visitor)
        }
    }

    #[inline]
    fn deserialize_f32<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_f32(visitor)
    }

    #[inline]
    fn deserialize_f64<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_f64(visitor)
    }

    #[inline]
    fn deserialize_char<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_char(visitor)
    }

    #[inline]
    fn deserialize_str<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_string(visitor)
    }

    #[inline]
    fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_bytes(visitor)
    }

    #[inline]
    fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_byte_buf(visitor)
    }

    #[inline]
    fn deserialize_option<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_option(visitor)
    }

    #[inline]
    fn deserialize_unit<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_unit(visitor)
    }

    #[inline]
    fn deserialize_unit_struct<V>(mut self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_unit_struct(name, visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(mut self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_newtype_struct(name, visitor)
    }

    #[inline]
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_tuple(len, visitor)
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        mut self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_tuple_struct(name, len, visitor)
    }

    #[inline]
    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_map(visitor)
    }

    #[inline]
    fn deserialize_struct<V>(
        mut self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_struct(name, fields, visitor)
    }

    #[inline]
    fn deserialize_enum<V>(
        mut self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_enum(name, variants, visitor)
    }

    #[inline]
    fn deserialize_identifier<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_identifier(visitor)
    }

    #[inline]
    fn deserialize_ignored_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.deserialize_ignored_any(visitor)
    }
}
