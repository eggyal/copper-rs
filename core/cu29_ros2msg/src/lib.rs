//! A [`DataFormat`][encoding::DataFormat] implementation for [ROS 2] messages.
//!
//! [ROS 2]: http://docs.ros.org/en/jazzy/Concepts/Basic/About-Interfaces.html

mod codec;
mod format;
mod schema;
mod value;

use cu29_encode::{self as encoding, DataFormat, Lowerable, Lowered, Lowers};
use schema::FieldType;

pub enum Ros2Msg {}
pub struct Ros2List<List>(List);
impl<List> From<List> for Ros2List<List> {
    fn from(list: List) -> Self {
        Self(list)
    }
}
impl DataFormat for Ros2Msg {
    type FieldType = FieldType;
    type EncodableAlt<List> = Ros2List<List>;
    type EncodableCons<List> = Ros2List<List>;
    type UnboundedIterator<Iter: Clone + IntoIterator<Item: Lowerable>, const ORDER_DEFINED: bool>
        = format::UnboundedArray<Iter>
    where
        Self: Lowers<Iter::Item>;
    type BoundedIterator<
        Iter: Clone + IntoIterator<Item: Lowerable>,
        const ORDER_DEFINED: bool,
        const MAX: usize,
    >
        = format::BoundedArray<Iter, MAX>
    where
        Self: Lowers<Iter::Item>;
    type StaticLenIterator<
        Iter: Clone + IntoIterator<Item: Lowerable>,
        const ORDER_DEFINED: bool,
        const LEN: usize,
    >
        = format::StaticArray<Iter, LEN>
    where
        Self: Lowers<Iter::Item>;
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

pub struct Constant<T: Lowered<Ros2Msg>> {
    pub name: &'static str,
    pub value: T,
}
