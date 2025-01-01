use crate::{
    concat, delegate,
    element::{elements, Element, ElementType},
    forward::{EncodesForwarding, Forwarding, ForwardingType},
    EncodableType, Encodes, NameableType,
};
use core::fmt;

elements! {
    bool,
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    i128,
    u128,
    isize,
    usize,
    f32,
    f64,
    char,
    str,
}

delegate! {
    fn['b, T](this: &'b T) -> T { this }
    fn['b, T](this: &'b mut T) -> T { this }
}

impl<Format: Encodes<T>, T: EncodableType, const N: usize> EncodesForwarding<[T; N]> for Format {
    type ForwardingFormatType<'a>
        = Format::StaticLenIterator<&'a [T; N], true, N>
    where
        T: 'a;
    fn forward_encodable(t: &[T; N]) -> Self::ForwardingFormatType<'_> {
        t.into()
    }
}
impl<T: EncodableType, const N: usize> NameableType for [T; N] {
    const NAME: &dyn fmt::Display = &concat!["[", T::NAME, "; ", N, "]"];
}
impl<T: EncodableType, const N: usize> EncodableType for [T; N] {
    type Sigil = Forwarding;
}
impl<T: EncodableType, const N: usize> ForwardingType for [T; N] {}
impl<Format: Encodes<T>, T: EncodableType> EncodesForwarding<[T]> for Format {
    type ForwardingFormatType<'a>
        = Format::UnboundedIterator<&'a [T], true>
    where
        T: 'a;
    fn forward_encodable(t: &[T]) -> Self::ForwardingFormatType<'_> {
        t.into()
    }
}
impl<T: EncodableType> NameableType for [T] {
    const NAME: &dyn fmt::Display = &concat!["[", T::NAME, "]"];
}
impl<T: EncodableType> EncodableType for [T] {
    type Sigil = Forwarding;
}
impl<T: EncodableType> ForwardingType for [T] {}
impl<T: EncodableType> NameableType for *const T {
    const NAME: &dyn fmt::Display = &concat!["*const ", T::NAME];
}
impl<T: EncodableType> EncodableType for *const T {
    type Sigil = Element;
}
impl<T: EncodableType> ElementType for *const T {}
impl<T: EncodableType> NameableType for *mut T {
    const NAME: &dyn fmt::Display = &concat!["*mut ", T::NAME];
}
impl<T: EncodableType> EncodableType for *mut T {
    type Sigil = Element;
}
impl<T: EncodableType> ElementType for *mut T {}
