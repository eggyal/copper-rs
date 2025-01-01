mod alloc_impls;
mod core_impls;
mod primitive_impls;
mod std_impls;

use crate::compound::CompoundType;

#[derive(CompoundType)]
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
