use super::super::{encoding::Value, format::*, invoke_macro_with_primitives, Ros2Msg};

macro_rules! impl_value {
    ($($format:ident($rust:ty)),* $(,)?) => {$(
        impl Value<Ros2Msg> for $rust {
            type FormatType<'a> = $format;
            fn encodable(&self) -> $format {
                $format(*self)
            }
        }
    )*};
}

invoke_macro_with_primitives!(impl_value);

impl Value<Ros2Msg> for str {
    type FormatType<'a> = String<'a>;
    fn encodable(&self) -> Self::FormatType<'_> {
        String(self.as_bytes())
    }
}
impl Value<Ros2Msg> for char {
    type FormatType<'a> = BoundedString<4>;
    fn encodable(&self) -> BoundedString<4> {
        let mut data = [0; 4];
        let result = self.encode_utf8(&mut data);
        BoundedString {
            length: result.len(),
            data,
        }
    }
}
impl<T: Value<Ros2Msg>, const N: usize> Value<Ros2Msg> for [T; N] {
    type FormatType<'a>
        = StaticArray<'a, T, N>
    where
        T: 'a;
    fn encodable(&self) -> StaticArray<'_, T, N> {
        StaticArray(self)
    }
}
impl<T: Value<Ros2Msg>> Value<Ros2Msg> for [T] {
    type FormatType<'a>
        = UnboundedArray<'a, core::slice::Iter<'a, T>, T>
    where
        T: 'a;
    fn encodable(&self) -> Self::FormatType<'_> {
        UnboundedArray::new(self.iter())
    }
}
