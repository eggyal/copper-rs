mod alloc_impls;
mod primitive_impls;
mod std_impls;

use super::{
    encoding::{self, ComplexType, ComplexTypeExt, Encodes},
    format::{self, MessageType},
    Ros2,
    Ros2Msg,
};

impl<M: ComplexTypeExt<Ros2Msg>> Encodes<M> for Ros2Msg {
    type FormatType<'a>
        = MessageType<'a, M>
    where
        M: 'a;
    fn encodable(this: &Ros2<M>) -> MessageType<'_, M> {
        MessageType(&this.0)
    }
}

#[derive(ComplexType)]
pub struct Entry<'a, K, V> {
    key: &'a K,
    val: &'a V,
}

#[derive(Clone)]
pub struct EntryIter<I>(I);
impl<'a, I: Iterator<Item = (&'a K, &'a V)>, K: 'a, V: 'a> Iterator for EntryIter<I> {
    type Item = Ros2<Entry<'a, K, V>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(key, val)| Ros2(Entry { key, val }))
    }
}
