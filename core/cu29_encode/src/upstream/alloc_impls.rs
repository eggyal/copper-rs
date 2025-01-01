extern crate alloc;
use super::{Entry, EntryIter};
use crate::{
    concat, delegate,
    forward::{EncodesForwarding, Forwarding, ForwardingType},
    EncodableType, Encodes, NameableType,
};
use alloc::{
    borrow::Cow,
    collections::{btree_map, BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    rc::Rc,
    sync::Arc,
};
use core::fmt;

delegate! {
    fn[T](this: Vec<T>) -> [T] { this }
    fn(this: String) -> str { this }
    fn[T: ?Sized](this: Box<T>) -> T { this }
    fn['b, T: ?Sized + ToOwned](this: Cow<'b, T>) -> T { this }
    fn[T: ?Sized](this: Rc<T>) -> T { this }
    fn[T: ?Sized](this: Arc<T>) -> T { this }
}

impl<Format: Encodes<T>, T: EncodableType> EncodesForwarding<BinaryHeap<T>> for Format {
    type ForwardingFormatType<'a>
        = Format::UnboundedIterator<&'a BinaryHeap<T>, false>
    where
        T: 'a;
    fn forward_encodable(t: &BinaryHeap<T>) -> Self::ForwardingFormatType<'_> {
        t.into()
    }
}
impl<T: EncodableType> NameableType for BinaryHeap<T> {
    const NAME: &dyn fmt::Display = &concat!["BinaryHeap<", T::NAME, ">"];
}
impl<T: EncodableType> EncodableType for BinaryHeap<T> {
    type Sigil = Forwarding;
}
impl<T: EncodableType> ForwardingType for BinaryHeap<T> {}

impl<Format: for<'a> Encodes<Entry<'a, K, V>>, K: EncodableType, V: EncodableType>
    EncodesForwarding<BTreeMap<K, V>> for Format
{
    type ForwardingFormatType<'a>
        = Format::UnboundedIterator<EntryIter<btree_map::Iter<'a, K, V>>, true>
    where
        K: 'a,
        V: 'a;
    fn forward_encodable(t: &BTreeMap<K, V>) -> Self::ForwardingFormatType<'_> {
        EntryIter(t.iter()).into()
    }
}
impl<K: EncodableType, V: EncodableType> NameableType for BTreeMap<K, V> {
    const NAME: &dyn fmt::Display = &concat!["BTreeMap<", K::NAME, ", ", V::NAME, ">"];
}
impl<K: EncodableType, V: EncodableType> EncodableType for BTreeMap<K, V> {
    type Sigil = Forwarding;
}
impl<K: EncodableType, V: EncodableType> ForwardingType for BTreeMap<K, V> {}

impl<Format: Encodes<T>, T: EncodableType> EncodesForwarding<BTreeSet<T>> for Format {
    type ForwardingFormatType<'a>
        = Format::UnboundedIterator<&'a BTreeSet<T>, true>
    where
        T: 'a;
    fn forward_encodable(t: &BTreeSet<T>) -> Self::ForwardingFormatType<'_> {
        t.into()
    }
}
impl<T: EncodableType> NameableType for BTreeSet<T> {
    const NAME: &dyn fmt::Display = &concat!["BTreeSet<", T::NAME, ">"];
}
impl<T: EncodableType> EncodableType for BTreeSet<T> {
    type Sigil = Forwarding;
}
impl<T: EncodableType> ForwardingType for BTreeSet<T> {}

impl<Format: Encodes<T>, T: EncodableType> EncodesForwarding<VecDeque<T>> for Format {
    type ForwardingFormatType<'a>
        = Format::UnboundedIterator<&'a VecDeque<T>, true>
    where
        T: 'a;
    fn forward_encodable(t: &VecDeque<T>) -> Self::ForwardingFormatType<'_> {
        t.into()
    }
}
impl<T: EncodableType> NameableType for VecDeque<T> {
    const NAME: &dyn fmt::Display = &concat!["VecDeque<", T::NAME, ">"];
}
impl<T: EncodableType> EncodableType for VecDeque<T> {
    type Sigil = Forwarding;
}
impl<T: EncodableType> ForwardingType for VecDeque<T> {}
