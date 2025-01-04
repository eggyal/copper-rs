use super::{Entry, EntryIter};
use crate::{
    alt, concat, cons, defer, delegate,
    element::elements,
    forward::{EncodesForwarding, Forwarding, ForwardingType},
    Alt, Cons, EncodableType, Encodes, NameableType,
};
use cu29_derive::fake_derive;
use std::{
    collections::{hash_map, HashMap, HashSet},
    ffi::{CStr, CString},
    fmt,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

elements! {
    CStr,
    Path,
    Ipv4Addr,
    Ipv6Addr,
}

delegate! {
    fn(this: CString) -> CStr { this }
    fn(this: PathBuf) -> Path { this }

    fn['b, T: ?Sized](this: MutexGuard<'b, T>) -> T { this }
    fn['b, T: ?Sized](this: RwLockReadGuard<'b, T>) -> T { this }
    fn['b, T: ?Sized](this: RwLockWriteGuard<'b, T>) -> T { this }
}

defer! {
    fn[T: ?Sized](&'a this: Mutex<T>) -> T as MutexGuard<'a, T> { this.lock().unwrap() }
    fn[T: ?Sized](&'a this: RwLock<T>) -> T as RwLockReadGuard<'a, T> { this.read().unwrap() }
}

#[fake_derive(CompoundType)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[fake_derive(CompoundType)]
enum SocketAddr {
    V4(SocketAddrV4),
    V6(SocketAddrV6),
}

#[fake_derive(CompoundType)]
struct SocketAddrV4 {
    #[compound_type(via = self.ip())]
    ip: Ipv4Addr,
    #[compound_type(owned, via = self.port())]
    port: u16,
}

#[fake_derive(CompoundType)]
struct SocketAddrV6 {
    #[compound_type(via = self.ip())]
    ip: Ipv6Addr,
    #[compound_type(owned, via = self.port())]
    port: u16,
    #[compound_type(owned, via = self.flowinfo())]
    flowinfo: u32,
    #[compound_type(owned, via = self.scope_id())]
    scope_id: u32,
}

impl<Format: for<'a> Encodes<Entry<'a, K, V>>, K: EncodableType, V: EncodableType>
    EncodesForwarding<HashMap<K, V>> for Format
{
    type ForwardingFormatType<'a>
        = Format::UnboundedIterator<EntryIter<hash_map::Iter<'a, K, V>>, false>
    where
        K: 'a,
        V: 'a;
    fn forward_encodable(t: &HashMap<K, V>) -> Self::ForwardingFormatType<'_> {
        EntryIter(t.iter()).into()
    }
}
impl<K: EncodableType, V: EncodableType> NameableType for HashMap<K, V> {
    const NAME: &dyn fmt::Display = &concat!["HashMap<", K::NAME, ", ", V::NAME, ">"];
}
impl<K: EncodableType, V: EncodableType> EncodableType for HashMap<K, V> {
    type Sigil = Forwarding;
}
impl<K: EncodableType, V: EncodableType> ForwardingType for HashMap<K, V> {}

impl<Format: Encodes<T>, T: EncodableType> EncodesForwarding<HashSet<T>> for Format {
    type ForwardingFormatType<'a>
        = Format::UnboundedIterator<&'a HashSet<T>, false>
    where
        T: 'a;
    fn forward_encodable(t: &HashSet<T>) -> Self::ForwardingFormatType<'_> {
        t.into()
    }
}
impl<T: EncodableType> NameableType for HashSet<T> {
    const NAME: &dyn fmt::Display = &concat!["HashSet<", T::NAME, ">"];
}
impl<T: EncodableType> EncodableType for HashSet<T> {
    type Sigil = Forwarding;
}
impl<T: EncodableType> ForwardingType for HashSet<T> {}
