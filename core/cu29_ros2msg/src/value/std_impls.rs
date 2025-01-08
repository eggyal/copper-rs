use super::{
    encoding::element::LowersElement,
    format::{Defer, String},
    Ros2Msg,
};
use std::{
    ffi::CStr,
    net::{Ipv4Addr, Ipv6Addr},
    path::Path, // time::SystemTime,  TODO:
};

impl LowersElement<CStr> for Ros2Msg {
    type ElementLowered<'a> = String<'a>;
    fn lower_element(this: &CStr) -> String<'_> {
        String(this.to_bytes())
    }
}

impl LowersElement<Path> for Ros2Msg {
    type ElementLowered<'a> = String<'a>;
    fn lower_element(this: &Path) -> Self::ElementLowered<'_> {
        String(this.as_os_str().as_encoded_bytes())
    }
}

impl LowersElement<Ipv4Addr> for Ros2Msg {
    type ElementLowered<'a> = Defer<[u8; 4]>;
    fn lower_element(this: &Ipv4Addr) -> Self::ElementLowered<'_> {
        Defer::new(this.octets())
    }
}
impl LowersElement<Ipv6Addr> for Ros2Msg {
    type ElementLowered<'a> = Defer<[u8; 16]>;
    fn lower_element(this: &Ipv6Addr) -> Self::ElementLowered<'_> {
        Defer::new(this.octets())
    }
}
