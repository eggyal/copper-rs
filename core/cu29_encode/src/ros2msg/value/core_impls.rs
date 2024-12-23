use super::{
    delegate,
    encoding::{self, Desc, FieldDescriptor, Message, MessageFields, Value},
    format::Defer,
    Ros2Msg,
};
use core::{
    cell::{Cell, Ref, RefCell},
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64,
        NonZeroU8,
    },
    // ops::{Bound, Range, RangeInclusive}, //  TODO: generic message types
    time::Duration,
};

delegate! {
    fn(self: NonZeroI8) -> i8 { self.get() }
    fn(self: NonZeroU8) -> u8 { self.get() }
    fn(self: NonZeroI16) -> i16 { self.get() }
    fn(self: NonZeroU16) -> u16 { self.get() }
    fn(self: NonZeroI32) -> i32 { self.get() }
    fn(self: NonZeroU32) -> u32 { self.get() }
    fn(self: NonZeroI64) -> i64 { self.get() }
    fn(self: NonZeroU64) -> u64 { self.get() }
}

impl<T: Copy + Value<Ros2Msg>> Value<Ros2Msg> for Cell<T> {
    type FormatType<'a>
        = Defer<T>
    where
        T: 'a;
    fn encodable(&self) -> Defer<T> {
        Defer(self.get())
    }
}
impl<T: Value<Ros2Msg>> Value<Ros2Msg> for RefCell<T> {
    type FormatType<'a>
        = Defer<encoding::Ref<Ref<'a, T>>>
    where
        T: 'a;
    fn encodable(&self) -> Self::FormatType<'_> {
        Defer(encoding::Ref::new(self.borrow()))
    }
}

impl Message for Duration {
    const NAME: &'static str = "Duration";
}
impl MessageFields<'_> for Duration {
    type Fields = Cons!(u64, u32);
    const DESCRIPTOR: Desc<Self::Fields> = cons![
        FieldDescriptor {
            name: "secs",
            default: None
        },
        FieldDescriptor {
            name: "nanoseconds",
            default: None
        },
    ];
    fn as_fields(&self) -> Self::Fields {
        cons![self.as_secs(), self.subsec_nanos(),]
    }
}
