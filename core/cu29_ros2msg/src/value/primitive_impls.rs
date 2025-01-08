use super::super::{
    encoding::element::LowersElement, format::*, invoke_macro_with_primitives, Ros2Msg,
};

macro_rules! impl_value {
    ($($format:ident($rust:ty)),* $(,)?) => {$(
        impl LowersElement<$rust> for Ros2Msg {
            type ElementLowered<'a> = $format;
            fn lower_element(this: &$rust) -> $format {
                $format(*this)
            }
        }
    )*};
}

invoke_macro_with_primitives!(impl_value);

impl LowersElement<str> for Ros2Msg {
    type ElementLowered<'a> = String<'a>;
    fn lower_element(this: &str) -> Self::ElementLowered<'_> {
        String(this.as_bytes())
    }
}
impl LowersElement<char> for Ros2Msg {
    type ElementLowered<'a> = BoundedString<4>;
    fn lower_element(this: &char) -> BoundedString<4> {
        let mut data = [0; 4];
        let result = this.encode_utf8(&mut data);
        BoundedString {
            length: result.len(),
            data,
        }
    }
}
