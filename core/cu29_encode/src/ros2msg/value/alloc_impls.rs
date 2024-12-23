extern crate alloc;
use alloc::{
    borrow::Cow,
    collections::{btree_map, btree_set, vec_deque, BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    rc::Rc,
    sync::Arc,
};

use super::{delegate, encoding::Value, format::UnboundedArray, Entry, EntryIter, Ros2Msg};

delegate! {
    fn[T](self: BinaryHeap<T>) -> [T] { self.as_slice() }
    fn[T](self: Vec<T>) -> [T] { self }
    fn(self: String) -> str { self }
    // fn[T](self: Box<T>) -> T { self }  //FIXME:
    fn[T: ?Sized + ToOwned](self: Cow<'_, T>) -> T { self }
    fn[T: ?Sized](self: Rc<T>) -> T { self }
    fn[T: ?Sized](self: Arc<T>) -> T { self }
}

impl<K: Value<Ros2Msg>, V: Value<Ros2Msg>> Value<Ros2Msg> for BTreeMap<K, V> {
    type FormatType<'a>
        = UnboundedArray<'a, EntryIter<btree_map::Iter<'a, K, V>>, Entry<'a, K, V>>
    where
        K: 'a,
        V: 'a;
    fn encodable(&self) -> Self::FormatType<'_> {
        UnboundedArray::new(EntryIter(self.iter()))
    }
}
impl<T: Value<Ros2Msg>> Value<Ros2Msg> for BTreeSet<T> {
    type FormatType<'a>
        = UnboundedArray<'a, btree_set::Iter<'a, T>, T>
    where
        T: 'a;
    fn encodable(&self) -> Self::FormatType<'_> {
        UnboundedArray::new(self.iter())
    }
}
impl<T: Value<Ros2Msg>> Value<Ros2Msg> for VecDeque<T> {
    type FormatType<'a>
        = UnboundedArray<'a, vec_deque::Iter<'a, T>, T>
    where
        T: 'a;
    fn encodable(&self) -> Self::FormatType<'_> {
        UnboundedArray::new(self.iter())
    }
}
