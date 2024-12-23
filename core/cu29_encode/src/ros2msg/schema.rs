use super::{
    encoding::{FormatType, MessageExt, Value},
    format::*,
    invoke_macro_with_primitives, Ros2Msg,
};
use std::borrow::Borrow;

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
        name: &'static str,
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

impl<D: Value<Ros2Msg>> FormatType<Ros2Msg> for Defer<D> {
    const FIELD_TYPE: FieldType = D::FormatType::FIELD_TYPE;
}

impl<'a, T: 'a + Value<Ros2Msg>, const N: usize> FormatType<Ros2Msg> for StaticArray<'_, T, N> {
    const FIELD_TYPE: FieldType = FieldType::StaticArray {
        elements: &T::FormatType::<'a>::FIELD_TYPE,
        length: N,
    };
}
impl<'a, T: 'a + Value<Ros2Msg>, I: Clone + Iterator<Item: Borrow<T>>> FormatType<Ros2Msg>
    for UnboundedArray<'_, I, T>
{
    const FIELD_TYPE: FieldType = FieldType::UnboundedArray {
        elements: &T::FormatType::<'a>::FIELD_TYPE,
    };
}
impl<'a, T: 'a + Value<Ros2Msg>, const N: usize> FormatType<Ros2Msg> for BoundedArray<'_, T, N> {
    const FIELD_TYPE: FieldType = FieldType::BoundedArray {
        elements: &T::FormatType::<'a>::FIELD_TYPE,
        bound: N,
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

impl<M: MessageExt<Ros2Msg>> FormatType<Ros2Msg> for MessageType<'_, M> {
    const FIELD_TYPE: FieldType = FieldType::Message { name: M::NAME };
}
