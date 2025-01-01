use super::super::{
    encoding::element::EncodesElement, format::*, invoke_macro_with_primitives, Ros2Msg,
};

macro_rules! impl_value {
    ($($format:ident($rust:ty)),* $(,)?) => {$(
        impl EncodesElement<$rust> for Ros2Msg {
            type ElementFormatType<'a> = $format;
            fn element_encodable(this: &$rust) -> $format {
                $format(*this)
            }
        }
    )*};
}

invoke_macro_with_primitives!(impl_value);

impl EncodesElement<str> for Ros2Msg {
    type ElementFormatType<'a> = String<'a>;
    fn element_encodable(this: &str) -> Self::ElementFormatType<'_> {
        String(this.as_bytes())
    }
}
impl EncodesElement<char> for Ros2Msg {
    type ElementFormatType<'a> = BoundedString<4>;
    fn element_encodable(this: &char) -> BoundedString<4> {
        let mut data = [0; 4];
        let result = this.encode_utf8(&mut data);
        BoundedString {
            length: result.len(),
            data,
        }
    }
}
