use super::{Entry, EntryIter};
use crate::{
    alt,
    compound::{Compound, CompoundTypeDef, Desc, FieldDescriptor, VariantDescriptor},
    concat, cons, delegate,
    element::elements,
    forward::{Defer, EncodesForwarding, Forwarding, ForwardingType},
    Alt, Cons, EncodableType, Encodes, NameableType,
};
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

impl<Format: Encodes<T>, T: ?Sized + EncodableType> EncodesForwarding<Mutex<T>> for Format {
    type ForwardingFormatType<'a>
        = Defer<MutexGuard<'a, T>, Format>
    where
        T: 'a;
    fn forward_encodable(this: &Mutex<T>) -> Self::ForwardingFormatType<'_> {
        Defer::new(this.lock().unwrap())
    }
}
impl<T: ?Sized + EncodableType> NameableType for Mutex<T> {
    const NAME: &dyn fmt::Display = T::NAME;
}
impl<T: ?Sized + EncodableType> EncodableType for Mutex<T> {
    type Sigil = Forwarding;
}
impl<T: ?Sized + EncodableType> ForwardingType for Mutex<T> {}

impl<Format: Encodes<T>, T: ?Sized + EncodableType> EncodesForwarding<RwLock<T>> for Format {
    type ForwardingFormatType<'a>
        = Defer<RwLockReadGuard<'a, T>, Format>
    where
        T: 'a;
    fn forward_encodable(this: &RwLock<T>) -> Self::ForwardingFormatType<'_> {
        Defer::new(this.read().unwrap())
    }
}
impl<T: ?Sized + EncodableType> NameableType for RwLock<T> {
    const NAME: &dyn fmt::Display = T::NAME;
}
impl<T: ?Sized + EncodableType> EncodableType for RwLock<T> {
    type Sigil = Forwarding;
}
impl<T: ?Sized + EncodableType> ForwardingType for RwLock<T> {}

impl NameableType for IpAddr {
    const NAME: &dyn fmt::Display = &"IpAddr";
}
impl EncodableType for IpAddr {
    type Sigil = Compound;
}
impl<'a> CompoundTypeDef<'a> for IpAddr {
    type Intermediate = Alt!(Cons!(&'a Ipv4Addr), Cons!(&'a Ipv6Addr));
    const DESCRIPTOR: Desc<Self::Intermediate> = cons![
        VariantDescriptor::new("V4", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("V6", cons![FieldDescriptor::no_default("0")]),
    ];
    fn intermediate(&'a self) -> Self::Intermediate {
        match self {
            IpAddr::V4(v4) => alt![cons![v4]],
            IpAddr::V6(v6) => alt![@cons![v6]],
        }
    }
}

impl NameableType for SocketAddr {
    const NAME: &dyn fmt::Display = &"SocketAddr";
}
impl EncodableType for SocketAddr {
    type Sigil = Compound;
}
impl<'a> CompoundTypeDef<'a> for SocketAddr {
    type Intermediate = Alt!(Cons!(&'a SocketAddrV4), Cons!(&'a SocketAddrV6),);
    const DESCRIPTOR: Desc<Self::Intermediate> = cons![
        VariantDescriptor::new("V4", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("V6", cons![FieldDescriptor::no_default("0")]),
    ];
    fn intermediate(&'a self) -> Self::Intermediate {
        match self {
            SocketAddr::V4(v4) => alt![cons![v4]],
            SocketAddr::V6(v6) => alt![@cons![v6]],
        }
    }
}

impl NameableType for SocketAddrV4 {
    const NAME: &dyn fmt::Display = &"SocketAddrV4";
}
impl EncodableType for SocketAddrV4 {
    type Sigil = Compound;
}
impl<'a> CompoundTypeDef<'a> for SocketAddrV4 {
    type Intermediate = Cons!(&'a Ipv4Addr, u16,);
    const DESCRIPTOR: Desc<Self::Intermediate> = cons![
        FieldDescriptor::no_default("ip"),
        FieldDescriptor::no_default("port"),
    ];
    fn intermediate(&'a self) -> Self::Intermediate {
        cons![self.ip(), self.port(),]
    }
}
impl NameableType for SocketAddrV6 {
    const NAME: &dyn fmt::Display = &"SocketAddrV6";
}
impl EncodableType for SocketAddrV6 {
    type Sigil = Compound;
}
impl<'a> CompoundTypeDef<'a> for SocketAddrV6 {
    type Intermediate = Cons!(&'a Ipv6Addr, u16, u32, u32,);
    const DESCRIPTOR: Desc<Self::Intermediate> = cons![
        FieldDescriptor::no_default("ip"),
        FieldDescriptor::no_default("port"),
        FieldDescriptor::no_default("flowinfo"),
        FieldDescriptor::no_default("scope_id"),
    ];
    fn intermediate(&'a self) -> Self::Intermediate {
        cons![self.ip(), self.port(), self.flowinfo(), self.scope_id(),]
    }
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
