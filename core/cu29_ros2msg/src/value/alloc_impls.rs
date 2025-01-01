extern crate alloc;

use crate::Ros2;

use super::{encoding::Encodes, format::UnboundedArray, Entry, EntryIter, Ros2Msg};
use alloc::collections::{btree_map, btree_set, vec_deque, BTreeMap, BTreeSet, VecDeque};
use cu29_encode::NameableType;

impl<K: NameableType, V: NameableType> Encodes<BTreeMap<K, V>> for Ros2Msg where Ros2Msg: Encodes<K> + Encodes<V> {
    type FormatType<'a>
        = UnboundedArray<'a, EntryIter<btree_map::Iter<'a, K, V>>, Ros2<Entry<'a, K, V>>>
    where
        K: 'a,
        V: 'a;
    fn encodable(this: &BTreeMap<K, V>) -> Self::FormatType<'_> {
        UnboundedArray::new(EntryIter(this.iter()))
    }
}
impl<T: NameableType> Encodes<BTreeSet<T>> for Ros2Msg where Ros2Msg: Encodes<T> {
    type FormatType<'a>
        = UnboundedArray<'a, btree_set::Iter<'a, T>, T>
    where
        T: 'a;
    fn encodable(this: &BTreeSet<T>) -> Self::FormatType<'_> {
        UnboundedArray::new(this.iter())
    }
}
impl<T: NameableType> Encodes<VecDeque<T>> for Ros2Msg where Ros2Msg: Encodes<T>{
    type FormatType<'a>
        = UnboundedArray<'a, vec_deque::Iter<'a, T>, T>
    where
        T: 'a;
    fn encodable(this: &VecDeque<T>) -> Self::FormatType<'_> {
        UnboundedArray::new(this.iter())
    }
}
