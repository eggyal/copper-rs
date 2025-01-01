use super::{
    ComplexTypeDef, Defer, Desc, FieldDescriptor, NameableType, Ref, Encodes,
    VariantDescriptor,
};
use std::{
    collections::{HashMap, HashSet},
    ffi::{CStr, CString},
    fmt,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard},
};

name! {
    CStr,
    Path,
    Ipv4Addr,
    Ipv6Addr,
}

delegate! {
    fn(this: CString) -> CStr { this }
    fn(this: PathBuf) -> Path { this }
}

impl<Format: Encodes<T>, T: ?Sized + NameableType> Encodes<Mutex<T>> for Format {
    type FormatType<'a>
        = Defer<Ref<MutexGuard<'a, T>>, Format>
    where
        T: 'a;
    fn encodable(this: &Mutex<T>) -> Self::FormatType<'_> {
        Defer::new(Ref::new(this.lock().unwrap()))
    }
}
impl<T: ?Sized + NameableType> NameableType for Mutex<T> {
    const NAME: &dyn std::fmt::Display = T::NAME;
}
impl<Format: Encodes<T>, T: ?Sized + NameableType> Encodes<RwLock<T>> for Format {
    type FormatType<'a>
        = Defer<Ref<RwLockReadGuard<'a, T>>, Format>
    where
        T: 'a;
    fn encodable(this: &RwLock<T>) -> Self::FormatType<'_> {
        Defer::new(Ref::new(this.read().unwrap()))
    }
}
impl<T: ?Sized + NameableType> NameableType for RwLock<T> {
    const NAME: &dyn std::fmt::Display = T::NAME;
}

impl NameableType for IpAddr {
    const NAME: &dyn fmt::Display = &"IpAddr";
}
impl<'a> ComplexTypeDef<'a> for IpAddr {
    type Content = Alt!(Cons!(Ref<&'a Ipv4Addr>), Cons!(Ref<&'a Ipv6Addr>));
    const DESCRIPTOR: Desc<Self::Content> = cons![
        VariantDescriptor::new("V4", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("V6", cons![FieldDescriptor::no_default("0")]),
    ];
    fn content(&'a self) -> Self::Content {
        match self {
            IpAddr::V4(v4) => alt![cons![Ref::new(v4)]],
            IpAddr::V6(v6) => alt![@cons![Ref::new(v6)]],
        }
    }
}

impl NameableType for SocketAddr {
    const NAME: &dyn fmt::Display = &"SocketAddr";
}
impl<'a> ComplexTypeDef<'a> for SocketAddr {
    type Content = Alt!(Cons!(Ref<&'a SocketAddrV4>), Cons!(Ref<&'a SocketAddrV6>),);
    const DESCRIPTOR: Desc<Self::Content> = cons![
        VariantDescriptor::new("V4", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("V6", cons![FieldDescriptor::no_default("0")]),
    ];
    fn content(&'a self) -> Self::Content {
        match self {
            SocketAddr::V4(v4) => alt![cons![Ref::new(v4)]],
            SocketAddr::V6(v6) => alt![@cons![Ref::new(v6)]],
        }
    }
}

impl NameableType for SocketAddrV4 {
    const NAME: &dyn fmt::Display = &"SocketAddrV4";
}
impl<'a> ComplexTypeDef<'a> for SocketAddrV4 {
    type Content = Cons!(&'a Ipv4Addr, u16,);
    const DESCRIPTOR: Desc<Self::Content> = cons![
        FieldDescriptor::no_default("ip"),
        FieldDescriptor::no_default("port"),
    ];
    fn content(&'a self) -> Self::Content {
        cons![self.ip(), self.port(),]
    }
}
impl NameableType for SocketAddrV6 {
    const NAME: &dyn fmt::Display = &"SocketAddrV6";
}
impl<'a> ComplexTypeDef<'a> for SocketAddrV6 {
    type Content = Cons!(&'a Ipv6Addr, u16, u32, u32,);
    const DESCRIPTOR: Desc<Self::Content> = cons![
        FieldDescriptor::no_default("ip"),
        FieldDescriptor::no_default("port"),
        FieldDescriptor::no_default("flowinfo"),
        FieldDescriptor::no_default("scope_id"),
    ];
    fn content(&'a self) -> Self::Content {
        cons![self.ip(), self.port(), self.flowinfo(), self.scope_id(),]
    }
}

impl<K: NameableType, V: NameableType> NameableType for HashMap<K, V> {
    const NAME: &dyn fmt::Display = &concat!["HashMap<", K::NAME, ", ", V::NAME, ">"];
}
impl<T: NameableType> NameableType for HashSet<T> {
    const NAME: &dyn fmt::Display = &concat!["HashSet<", T::NAME, ">"];
}
