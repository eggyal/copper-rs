mod alloc_impls;
mod core_impls;
mod primitive_impls;
mod std_impls;

use super::{
    encoding::{self, Message, MessageExt, Value},
    format::{self, MessageType},
    Ros2Msg,
};

impl<M: MessageExt<Ros2Msg>> Value<Ros2Msg> for M {
    type FormatType<'a>
        = MessageType<'a, M>
    where
        M: 'a;
    fn encodable(&self) -> MessageType<'_, M> {
        MessageType(self)
    }
}

#[derive(Message)]
pub struct Entry<'a, K, V> {
    key: &'a K,
    val: &'a V,
}

#[derive(Clone)]
pub struct EntryIter<I>(I);
impl<'a, I: Iterator<Item = (&'a K, &'a V)>, K: 'a, V: 'a> Iterator for EntryIter<I> {
    type Item = Entry<'a, K, V>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(key, val)| Entry { key, val })
    }
}

macro_rules! delegate {
    ($(fn$([$t:ident$(: $($bounds:tt)+)?])?($self:ident: $from:ty) -> $to:ty { $def:expr })*) => {$(
        impl$(<$t: $crate::encoding::Value<$crate::ros2msg::Ros2Msg> $(+ $($bounds)+)?>)? $crate::encoding::Value<$crate::ros2msg::Ros2Msg> for $from {
            type FormatType<'a> = <$to as $crate::encoding::Value<$crate::ros2msg::Ros2Msg>>::FormatType<'a> where Self: 'a;
            fn encodable(&$self) -> Self::FormatType<'_> {
                let def: &$to = &$def;
                $crate::encoding::Value::<$crate::ros2msg::Ros2Msg>::encodable(def)
            }
        }
    )*};
}
use delegate;
