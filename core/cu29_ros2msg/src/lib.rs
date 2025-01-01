//! A [`DataFormat`][encoding::DataFormat] implementation for [ROS 2] messages.
//!
//! [ROS 2]: http://docs.ros.org/en/jazzy/Concepts/Basic/About-Interfaces.html

mod codec;
mod format;
mod schema;
mod value;

use cu29_encode::{self as encoding, DataFormat, FormatType, NameableType};
use schema::FieldType;

#[repr(transparent)]
pub struct Ros2<Inner>(Inner);

impl<M: NameableType> NameableType for Ros2<M> {
    const NAME: &dyn std::fmt::Display = M::NAME;
}
unsafe impl<Inner> encoding::Wrapped for Ros2<Inner> {
    type Inner = Inner;
}

pub struct Ros2Msg;
impl DataFormat for Ros2Msg {
    type FieldType = FieldType;
    type Wrapped<Inner> = Ros2<Inner>;
}

macro_rules! invoke_macro_with_primitives {
    ($macro:ident) => {
        $macro! {
            Bool(bool),
            Float32(f32),
            Float64(f64),
            Int8(i8),
            UInt8(u8),
            Int16(i16),
            UInt16(u16),
            Int32(i32),
            UInt32(u32),
            Int64(i64),
            UInt64(u64),
        }
    };
}
use invoke_macro_with_primitives;

pub struct Constant<T: FormatType<Ros2Msg>> {
    name: &'static str,
    value: T,
}
