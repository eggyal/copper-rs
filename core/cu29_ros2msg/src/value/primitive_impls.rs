use cu29_encode::NameableType;

use super::super::{encoding::Encodes, format::*, invoke_macro_with_primitives, Ros2Msg};

macro_rules! impl_value {
    ($($format:ident($rust:ty)),* $(,)?) => {$(
        impl Encodes<$rust> for Ros2Msg {
            type FormatType<'a> = $format;
            fn encodable(this: &$rust) -> $format {
                $format(*this)
            }
        }
    )*};
}

invoke_macro_with_primitives!(impl_value);

impl Encodes<str> for Ros2Msg {
    type FormatType<'a> = String<'a>;
    fn encodable(this: &str) -> Self::FormatType<'_> {
        String(this.as_bytes())
    }
}
impl Encodes<char> for Ros2Msg {
    type FormatType<'a> = BoundedString<4>;
    fn encodable(this: &char) -> BoundedString<4> {
        let mut data = [0; 4];
        let result = this.encode_utf8(&mut data);
        BoundedString {
            length: result.len(),
            data,
        }
    }
}
impl<T: NameableType, const N: usize> Encodes<[T; N]> for Ros2Msg where Ros2Msg: Encodes<T> {
    type FormatType<'a>
        = StaticArray<'a, T, N>
    where
        T: 'a;
    fn encodable(this: &[T; N]) -> StaticArray<'_, T, N> {
        StaticArray(this)
    }
}
impl<T: NameableType> Encodes<[T]> for Ros2Msg where Ros2Msg: Encodes<T> {
    type FormatType<'a>
        = UnboundedArray<'a, core::slice::Iter<'a, T>, T>
    where
        T: 'a;
    fn encodable(this: &[T]) -> Self::FormatType<'_> {
        UnboundedArray::new(this.iter())
    }
}
