use cu29_encode::NameableType;

use super::{
    encoding::Encodes,
    format::{Defer, String, UnboundedArray},
    /* Entry, EntryIter, Ros2,*/ Ros2Msg,
};
use std::{
    collections::{/* HashMap, */ HashSet},
    ffi::CStr,
    net::{Ipv4Addr, Ipv6Addr},
    path::Path, // time::SystemTime,  TODO:
};

impl Encodes<CStr> for Ros2Msg {
    type FormatType<'a> = String<'a>;
    fn encodable(this: &CStr) -> String<'_> {
        String(this.to_bytes())
    }
}

impl Encodes<Path> for Ros2Msg {
    type FormatType<'a> = String<'a>;
    fn encodable(this: &Path) -> Self::FormatType<'_> {
        String(this.as_os_str().as_encoded_bytes())
    }
}

impl Encodes<Ipv4Addr> for Ros2Msg {
    type FormatType<'a> = Defer<[u8; 4]>;
    fn encodable(this: &Ipv4Addr) -> Self::FormatType<'_> {
        Defer::new(this.octets())
    }
}
impl Encodes<Ipv6Addr> for Ros2Msg {
    type FormatType<'a> = Defer<[u8; 16]>;
    fn encodable(this: &Ipv6Addr) -> Self::FormatType<'_> {
        Defer::new(this.octets())
    }
}

// impl<K: Value<Ros2Msg>, V: Value<Ros2Msg>> Value<Ros2Msg> for Ros2<HashMap<K, V>> {
//     type FormatType<'a>
//         = UnboundedArray<'a, EntryIter<std::collections::hash_map::Iter<'a, K, V>>, Entry<'a, K, V>>
//     where
//         K: 'a,
//         V: 'a;
//     fn encodable(this: &) -> Self::FormatType<'_> {
//         UnboundedArray::new(EntryIter(this.iter()))
//     }
// }
impl<T: NameableType> Encodes<HashSet<T>> for Ros2Msg where Ros2Msg: Encodes<T> {
    type FormatType<'a>
        = UnboundedArray<'a, std::collections::hash_set::Iter<'a, T>, T>
    where
        T: 'a;
    fn encodable(this: &HashSet<T>) -> Self::FormatType<'_> {
        UnboundedArray::new(this.iter())
    }
}

// type FieldSchema<L> = <L as IntoSchema<TestFormat>>::FieldSchema;
// const _: FieldSchema<<TestMessage as MessageFields>::Fields> = cons![
//     TestField::String,
//     TestField::Slice,
//     TestField::Array,
// ];

// const _: () = {
//     let s4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
//     <SocketAddrV4 as crate::encoding::MessageExt<Ros2Msg>>::FIELD_SCHEMA;
// };
