use super::{Entry, EntryIter};
use crate::{defer, delegate, element::elements, iterate};
use cu29_derive::fake_derive;
use std::{
    collections::{hash_map, HashMap, HashSet},
    ffi::{CStr, CString},
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

iterate! {
    unbounded arbitrary fn[K, V](&'a this: HashMap<K, V>) -> EntryIter<hash_map::Iter<'a, K, V>> as Entry<'a, K, V> {
        EntryIter(this.iter())
    }
    unbounded arbitrary fn[T](&'a this: HashSet<T>) -> &'a HashSet<T> as T { this }
}
