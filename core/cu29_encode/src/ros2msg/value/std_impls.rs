use super::{
    delegate,
    encoding::{Desc, FieldDescriptor, Message, MessageFields, Ref, Value},
    format::{Defer, String, UnboundedArray},
    Entry, EntryIter, Ros2Msg,
};
use std::{
    collections::{HashMap, HashSet},
    ffi::{CStr, CString},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    path::{Path, PathBuf},
    sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard},
    // time::SystemTime,  TODO:
};

impl Value<Ros2Msg> for CStr {
    type FormatType<'a> = String<'a>;
    fn encodable(&self) -> String<'_> {
        String(self.to_bytes())
    }
}

delegate! {
    fn(self: CString) -> CStr { self }
    fn(self: PathBuf) -> Path { self }
}

impl<T: ?Sized + Value<Ros2Msg>> Value<Ros2Msg> for Mutex<T> {
    type FormatType<'a>
        = Defer<Ref<MutexGuard<'a, T>>>
    where
        T: 'a;
    fn encodable(&self) -> Self::FormatType<'_> {
        Defer(Ref::new(self.lock().unwrap()))
    }
}
impl<T: ?Sized + Value<Ros2Msg>> Value<Ros2Msg> for RwLock<T> {
    type FormatType<'a>
        = Defer<Ref<RwLockReadGuard<'a, T>>>
    where
        T: 'a;
    fn encodable(&self) -> Self::FormatType<'_> {
        Defer(Ref::new(self.read().unwrap()))
    }
}

impl Value<Ros2Msg> for Path {
    type FormatType<'a> = String<'a>;
    fn encodable(&self) -> Self::FormatType<'_> {
        String(self.as_os_str().as_encoded_bytes())
    }
}

impl Message for IpAddr {
    const NAME: &str = "IpAddr";
}
impl<'a> MessageFields<'a> for IpAddr {
    type Fields = Alt!(Cons!(Ref<&'a Ipv4Addr>), Cons!(Ref<&'a Ipv6Addr>));
    const DESCRIPTOR: Desc<Self::Fields> = altdesc![
        cons![FieldDescriptor {
            name: "0",
            default: None
        }],
        cons![FieldDescriptor {
            name: "0",
            default: None
        }],
    ];
    fn as_fields(&'a self) -> Self::Fields {
        match self {
            IpAddr::V4(v4) => alt![cons![Ref::new(v4)]],
            IpAddr::V6(v6) => alt![@cons![Ref::new(v6)]],
        }
    }
}

impl Value<Ros2Msg> for Ipv4Addr {
    type FormatType<'a> = Defer<[u8; 4]>;
    fn encodable(&self) -> Self::FormatType<'_> {
        Defer(self.octets())
    }
}
impl Value<Ros2Msg> for Ipv6Addr {
    type FormatType<'a> = Defer<[u8; 16]>;
    fn encodable(&self) -> Self::FormatType<'_> {
        Defer(self.octets())
    }
}

impl Message for SocketAddr {
    const NAME: &str = "SocketAddr";
}
impl<'a> MessageFields<'a> for SocketAddr {
    type Fields = Alt!(Cons!(Ref<&'a SocketAddrV4>), Cons!(Ref<&'a SocketAddrV6>));
    const DESCRIPTOR: Desc<Self::Fields> = altdesc![
        cons![FieldDescriptor {
            name: "0",
            default: None
        }],
        cons![FieldDescriptor {
            name: "0",
            default: None
        }],
    ];
    fn as_fields(&'a self) -> Self::Fields {
        match self {
            SocketAddr::V4(v4) => alt![cons![Ref::new(v4)]],
            SocketAddr::V6(v6) => alt![@cons![Ref::new(v6)]],
        }
    }
}

impl Message for SocketAddrV4 {
    const NAME: &'static str = "SocketAddrV4";
}
impl<'a> MessageFields<'a> for SocketAddrV4 {
    type Fields = Cons!(&'a Ipv4Addr, u16);
    const DESCRIPTOR: Desc<Self::Fields> = cons![
        FieldDescriptor {
            name: "ip",
            default: None
        },
        FieldDescriptor {
            name: "port",
            default: None
        },
    ];
    fn as_fields(&'a self) -> Self::Fields {
        cons![self.ip(), self.port(),]
    }
}
impl Message for SocketAddrV6 {
    const NAME: &'static str = "SocketAddrV6";
}
impl<'a> MessageFields<'a> for SocketAddrV6 {
    type Fields = Cons!(&'a Ipv6Addr, u16, u32, u32);
    const DESCRIPTOR: Desc<Self::Fields> = cons![
        FieldDescriptor {
            name: "ip",
            default: None
        },
        FieldDescriptor {
            name: "port",
            default: None
        },
        FieldDescriptor {
            name: "flowinfo",
            default: None
        },
        FieldDescriptor {
            name: "scope_id",
            default: None
        },
    ];
    fn as_fields(&'a self) -> Self::Fields {
        cons![self.ip(), self.port(), self.flowinfo(), self.scope_id(),]
    }
}
impl<K: Value<Ros2Msg>, V: Value<Ros2Msg>> Value<Ros2Msg> for HashMap<K, V> {
    type FormatType<'a>
        = UnboundedArray<'a, EntryIter<std::collections::hash_map::Iter<'a, K, V>>, Entry<'a, K, V>>
    where
        K: 'a,
        V: 'a;
    fn encodable(&self) -> Self::FormatType<'_> {
        UnboundedArray::new(EntryIter(self.iter()))
    }
}
impl<T: Value<Ros2Msg>> Value<Ros2Msg> for HashSet<T> {
    type FormatType<'a>
        = UnboundedArray<'a, std::collections::hash_set::Iter<'a, T>, T>
    where
        T: 'a;
    fn encodable(&self) -> Self::FormatType<'_> {
        UnboundedArray::new(self.iter())
    }
}
