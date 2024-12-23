//! This module defines format-agnostic types for describing schema.
//!
//! [`Value`]s are marshalled through instances of some [`FormatType`]
//! in order to decouple the serialised encoding from its schema type.
//! For example, a data format might encode text as an array of bytes
//! yet require the schema type to reflect that the bytes are UTF-8.`

#[macro_use]
mod type_list;
use std::ops::Deref;

pub use type_list::*;

#[macro_use]
mod messages;
pub use messages::*;

/// A data format determines how messages are serialised.
pub trait DataFormat {
    /// The field type of a data format is describes any serialised datum's type
    /// in the data format's serialisation schema.
    type FieldType;
}

/// A format type is a serializable datum that knows its type in `Format`'s
/// serialisation schema.
pub trait FormatType<Format: DataFormat>: bincode::Encode {
    const FIELD_TYPE: Format::FieldType;
}
/// A value type is a serializable datum that knows the [`FormatType`] through
/// which it should be marshalled.
pub trait Value<Format: DataFormat> {
    type FormatType<'a>: FormatType<Format>
    where
        Self: 'a;
    fn encodable(&self) -> <Self as Value<Format>>::FormatType<'_>;
}

#[derive(Clone, Copy)]
pub struct Ref<R>(pub R);
impl<R> Ref<R> {
    pub fn new(r: R) -> Self {
        Self(r)
    }
}
impl<Format: DataFormat, R: Deref<Target: Value<Format>>> Value<Format> for Ref<R> {
    type FormatType<'a>
        = <R::Target as Value<Format>>::FormatType<'a>
    where
        Self: 'a;
    fn encodable(&self) -> <Self as Value<Format>>::FormatType<'_> {
        self.0.encodable()
    }
}
