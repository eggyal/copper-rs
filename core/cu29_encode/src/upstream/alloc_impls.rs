extern crate alloc;
use super::{Entry, EntryIter};
use crate::{delegate, iterate};
use alloc::{
    borrow::Cow,
    collections::{btree_map, BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    rc::Rc,
    sync::Arc,
};

delegate! {
    fn[T](this: Vec<T>) -> [T] { this }
    fn(this: String) -> str { this }
    fn[T: ?Sized](this: Box<T>) -> T { this }
    fn['b, T: ?Sized + ToOwned](this: Cow<'b, T>) -> T { this }
    fn[T: ?Sized](this: Rc<T>) -> T { this }
    fn[T: ?Sized](this: Arc<T>) -> T { this }
}

iterate! {
    // `BinaryHeap<T>` does not delegate to `[T]` (eg via `BinaryHeap::as_slice`),
    // because that would result in a defined order iterator whereas the order is
    // arbitrary.
    unbounded arbitrary fn[T](&'a this: BinaryHeap<T>) -> &'a BinaryHeap<T> as T { this }

    // Whilst the following do have defined orders, they cannot delegate to slices
    // because their data is not (necessarily) stored contiguously.

    unbounded defined fn[K, V](&'a this: BTreeMap<K, V>) -> EntryIter<btree_map::Iter<'a, K, V>>
        as Entry<'a, K, V> { EntryIter(this.iter()) }
    unbounded defined fn[T](&'a this: BTreeSet<T>) -> &'a BTreeSet<T> as T { this }
    unbounded defined fn[T](&'a this: VecDeque<T>) -> &'a VecDeque<T> as T { this }
}
