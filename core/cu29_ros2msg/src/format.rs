use super::{invoke_macro_with_primitives, Ros2Msg};
use bincode::Encode;

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

pub struct StaticArray<I, const LEN: usize>(pub(super) I);
impl<I: IntoIterator, const LEN: usize> From<I> for StaticArray<I, LEN> {
    fn from(iter: I) -> Self {
        Self(iter)
    }
}
pub struct UnboundedArray<I>(pub(super) I);
impl<I: IntoIterator> From<I> for UnboundedArray<I> {
    fn from(iter: I) -> Self {
        Self(iter)
    }
}
pub struct BoundedArray<I, const MAX: usize> {
    pub(super) iter: I,
    // pub(super) length: usize,
}
impl<I: IntoIterator, const MAX: usize> From<I> for BoundedArray<I, MAX> {
    fn from(iter: I) -> Self {
        Self { iter }
    }
}

pub type Defer<D> = super::encoding::forward::Defer<D, Ros2Msg>;

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
