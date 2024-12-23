use super::invoke_macro_with_primitives;
use bincode::Encode;
use std::marker::PhantomData;

macro_rules! define {
    ($($format:ident($rust:ty)),* $(,)?) => {$(
        #[derive(Encode)]
        pub struct $format(pub(super) $rust);
    )*};
}

invoke_macro_with_primitives!(define);

#[derive(Encode)]
pub struct Byte(pub(super) u8);
#[derive(Encode)]
pub struct Char(pub(super) u8);

pub struct StaticArray<'a, T, const N: usize>(pub(super) &'a [T; N]);
pub struct UnboundedArray<'a, I, T>(pub(super) I, PhantomData<&'a T>);
impl<I, T> UnboundedArray<'_, I, T> {
    pub(super) fn new(iter: I) -> Self {
        Self(iter, PhantomData)
    }
}
pub struct BoundedArray<'a, T, const N: usize> {
    pub(super) data: &'a [T; N],
    pub(super) length: usize,
}

pub struct Defer<D>(pub(super) D);

#[derive(Encode)]
pub struct String<'a>(pub(super) &'a [u8]);
#[derive(Encode)]
pub struct WString<'a>(pub(super) &'a [u16]);
#[derive(Encode)]
pub struct BoundedString<const N: usize> {
    pub(super) data: [u8; N],
    pub(super) length: usize,
}

pub struct MessageType<'a, M>(pub(super) &'a M);
