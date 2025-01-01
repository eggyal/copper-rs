use super::{
    encoding::{compound::CompoundTypeExt, EncodableType, Encodes, FormatType},
    format::*,
    invoke_macro_with_primitives, Ros2Msg,
};
use core::fmt;

pub enum FieldType {
    Bool,
    Byte,
    Char,
    Float32,
    Float64,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    String,
    WString,

    StaticArray {
        elements: &'static Self,
        length: usize,
    },
    UnboundedArray {
        elements: &'static Self,
    },
    BoundedArray {
        elements: &'static Self,
        bound: usize,
    },
    BoundedString {
        bound: usize,
    },

    Message {
        name: &'static dyn fmt::Display,
    },
}

macro_rules! impl_formattype {
    ($($format:ident($rust:ty)),* $(,)?) => {$(
        impl FormatType<Ros2Msg> for $format {
            const FIELD_TYPE: FieldType = FieldType::$format;
        }
    )*};
}

invoke_macro_with_primitives!(impl_formattype);
impl_formattype! {
    Byte(_),
    Char(_),
}

impl<'a, I: Clone + IntoIterator<Item: 'a + EncodableType>, const LEN: usize> FormatType<Ros2Msg>
    for StaticArray<I, LEN>
where
    Ros2Msg: Encodes<I::Item>,
{
    const FIELD_TYPE: FieldType = FieldType::StaticArray {
        elements: &<Ros2Msg as Encodes<I::Item>>::FormatType::<'a>::FIELD_TYPE,
        length: LEN,
    };
}
impl<'a, I: Clone + IntoIterator<Item: 'a + EncodableType>, const MAX: usize> FormatType<Ros2Msg>
    for BoundedArray<I, MAX>
where
    Ros2Msg: Encodes<I::Item>,
{
    const FIELD_TYPE: FieldType = FieldType::BoundedArray {
        elements: &<Ros2Msg as Encodes<I::Item>>::FormatType::<'a>::FIELD_TYPE,
        bound: MAX,
    };
}
impl<'a, I: Clone + IntoIterator<Item: 'a + EncodableType>> FormatType<Ros2Msg>
    for UnboundedArray<I>
where
    Ros2Msg: Encodes<I::Item>,
{
    const FIELD_TYPE: FieldType = FieldType::UnboundedArray {
        elements: &<Ros2Msg as Encodes<I::Item>>::FormatType::<'a>::FIELD_TYPE,
    };
}

impl FormatType<Ros2Msg> for String<'_> {
    const FIELD_TYPE: FieldType = FieldType::String;
}
impl FormatType<Ros2Msg> for WString<'_> {
    const FIELD_TYPE: FieldType = FieldType::WString;
}
impl<const N: usize> FormatType<Ros2Msg> for BoundedString<N> {
    const FIELD_TYPE: FieldType = FieldType::BoundedString { bound: N };
}

impl<M: CompoundTypeExt<Ros2Msg>> FormatType<Ros2Msg> for MessageType<'_, M> {
    const FIELD_TYPE: FieldType = FieldType::Message { name: M::NAME };
}
