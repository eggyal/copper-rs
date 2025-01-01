extern crate alloc;
use super::NameableType;
use alloc::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    rc::Rc,
    sync::Arc,
};

delegate! {
    fn[T](this: BinaryHeap<T>) -> [T] { this.as_slice() }
    fn[T](this: Vec<T>) -> [T] { this }
    fn(this: String) -> str { this }
    // fn[T](this: Box<T>) -> T { this }  //FIXME:
    fn['b, T: ?Sized + ToOwned](this: Cow<'b, T>) -> T { this }
    fn[T: ?Sized](this: Rc<T>) -> T { this }
    fn[T: ?Sized](this: Arc<T>) -> T { this }
}

impl<K: NameableType, V: NameableType> NameableType for BTreeMap<K, V> {
    const NAME: &dyn std::fmt::Display = &concat!["BTreeMap<", K::NAME, ", ", V::NAME, ">"];
}
impl<T: NameableType> NameableType for BTreeSet<T> {
    const NAME: &dyn std::fmt::Display = &concat!["BTreeSet<", T::NAME, ">"];
}
impl<T: NameableType> NameableType for VecDeque<T> {
    const NAME: &dyn std::fmt::Display = &concat!["VecDeque<", T::NAME, ">"];
}
