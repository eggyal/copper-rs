use super::{
    encoding::element::EncodesElement,
    format::{Defer, String},
    Ros2Msg,
};
use std::{
    ffi::CStr,
    net::{Ipv4Addr, Ipv6Addr},
    path::Path, // time::SystemTime,  TODO:
};

impl EncodesElement<CStr> for Ros2Msg {
    type ElementFormatType<'a> = String<'a>;
    fn element_encodable(this: &CStr) -> String<'_> {
        String(this.to_bytes())
    }
}

impl EncodesElement<Path> for Ros2Msg {
    type ElementFormatType<'a> = String<'a>;
    fn element_encodable(this: &Path) -> Self::ElementFormatType<'_> {
        String(this.as_os_str().as_encoded_bytes())
    }
}

impl EncodesElement<Ipv4Addr> for Ros2Msg {
    type ElementFormatType<'a> = Defer<[u8; 4]>;
    fn element_encodable(this: &Ipv4Addr) -> Self::ElementFormatType<'_> {
        Defer::new(this.octets())
    }
}
impl EncodesElement<Ipv6Addr> for Ros2Msg {
    type ElementFormatType<'a> = Defer<[u8; 16]>;
    fn element_encodable(this: &Ipv6Addr) -> Self::ElementFormatType<'_> {
        Defer::new(this.octets())
    }
}
