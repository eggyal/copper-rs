use crate::{
    alt,
    compound::{Compound, CompoundTypeDef, Desc, FieldDescriptor, VariantDescriptor},
    concat, cons, defer, delegate,
    forward::{Defer, EncodesForwarding, Forwarding, ForwardingType},
    Alt, Cons, EncodableType, Encodes, NameableType,
};
use core::{
    cell::{Cell, Ref, RefCell, RefMut},
    fmt,
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64,
        NonZeroU8,
    },
    ops::{Bound, Range, RangeBounds, RangeInclusive},
    sync::atomic::{
        AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicPtr, AtomicU16,
        AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering::SeqCst,
    },
    time::Duration,
};

defer! {
    fn(this: NonZeroI8) -> i8 { this.get() }
    fn(this: NonZeroU8) -> u8 { this.get() }
    fn(this: NonZeroI16) -> i16 { this.get() }
    fn(this: NonZeroU16) -> u16 { this.get() }
    fn(this: NonZeroI32) -> i32 { this.get() }
    fn(this: NonZeroU32) -> u32 { this.get() }
    fn(this: NonZeroI64) -> i64 { this.get() }
    fn(this: NonZeroU64) -> u64 { this.get() }

    fn(this: AtomicBool) -> bool { this.load(SeqCst) }
    fn(this: AtomicI8) -> i8 { this.load(SeqCst) }
    fn(this: AtomicU8) -> u8 { this.load(SeqCst) }
    fn(this: AtomicI16) -> i16 { this.load(SeqCst) }
    fn(this: AtomicU16) -> u16 { this.load(SeqCst) }
    fn(this: AtomicI32) -> i32 { this.load(SeqCst) }
    fn(this: AtomicU32) -> u32 { this.load(SeqCst) }
    fn(this: AtomicI64) -> i64 { this.load(SeqCst) }
    fn(this: AtomicU64) -> u64 { this.load(SeqCst) }
    fn(this: AtomicIsize) -> isize { this.load(SeqCst) }
    fn(this: AtomicUsize) -> usize { this.load(SeqCst) }
    fn[T](this: AtomicPtr<T>) -> *mut T { this.load(SeqCst) }

    fn[T: Copy](this: Cell<T>) -> T { this.get() }
}

delegate! {
    fn['b, T](this: Ref<'b, T>) -> T { this }
    fn['b, T](this: RefMut<'b, T>) -> T { this }
}

impl<Format: Encodes<T>, T: EncodableType> EncodesForwarding<RefCell<T>> for Format {
    type ForwardingFormatType<'a>
        = Defer<Ref<'a, T>, Format>
    where
        T: 'a;
    fn forward_encodable(this: &RefCell<T>) -> Self::ForwardingFormatType<'_> {
        Defer::new(this.borrow())
    }
}
impl<T: EncodableType> NameableType for RefCell<T> {
    const NAME: &dyn fmt::Display = T::NAME;
}
impl<T: EncodableType> EncodableType for RefCell<T> {
    type Sigil = Forwarding;
}
impl<T: EncodableType> ForwardingType for RefCell<T> {}

impl NameableType for Duration {
    const NAME: &dyn fmt::Display = &"Duration";
}
impl EncodableType for Duration {
    type Sigil = Compound;
}
impl CompoundTypeDef<'_> for Duration {
    type Intermediate = Cons!(u64, u32);
    const DESCRIPTOR: Desc<Self::Intermediate> = cons![
        FieldDescriptor::no_default("secs"),
        FieldDescriptor::no_default("nanoseconds"),
    ];
    fn intermediate(&self) -> Self::Intermediate {
        cons![self.as_secs(), self.subsec_nanos(),]
    }
}

impl<T: EncodableType> NameableType for Option<T> {
    const NAME: &dyn fmt::Display = &concat!["Option<", T::NAME, ">"];
}
impl<T: EncodableType> EncodableType for Option<T> {
    type Sigil = Compound;
}
impl<'this, T> CompoundTypeDef<'this> for Option<T> {
    type Intermediate = Alt![Cons![&'this T], Cons![]];
    const DESCRIPTOR: Desc<Self::Intermediate> = cons![
        VariantDescriptor::new("Some", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("None", cons![]),
    ];
    fn intermediate(&'this self) -> Self::Intermediate {
        match self {
            Some(value) => alt![cons![value]],
            None => alt![@cons![]],
        }
    }
}

impl<T: EncodableType, E: EncodableType> NameableType for Result<T, E> {
    const NAME: &dyn fmt::Display = &concat!["Result<", T::NAME, ",", E::NAME, ">"];
}
impl<T: EncodableType, E: EncodableType> EncodableType for Result<T, E> {
    type Sigil = Compound;
}
impl<'this, T, E> CompoundTypeDef<'this> for Result<T, E> {
    type Intermediate = Alt![Cons![&'this T], Cons![&'this E]];
    const DESCRIPTOR: Desc<Self::Intermediate> = cons![
        VariantDescriptor::new("Ok", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("Err", cons![FieldDescriptor::no_default("0")]),
    ];
    fn intermediate(&'this self) -> Self::Intermediate {
        match self {
            Ok(value) => alt![cons![value]],
            Err(err) => alt![@cons![err]],
        }
    }
}

impl<T: EncodableType> NameableType for Range<T> {
    const NAME: &dyn fmt::Display = &concat!["Range<", T::NAME, ">"];
}
impl<T: EncodableType> EncodableType for Range<T> {
    type Sigil = Compound;
}
impl<'this, T> CompoundTypeDef<'this> for Range<T> {
    type Intermediate = Cons![&'this T, &'this T];
    const DESCRIPTOR: Desc<Self::Intermediate> = cons![
        FieldDescriptor::no_default("start"),
        FieldDescriptor::no_default("end"),
    ];
    fn intermediate(&'this self) -> Self::Intermediate {
        cons![&self.start, &self.end]
    }
}

impl<T: EncodableType> NameableType for Bound<T> {
    const NAME: &dyn fmt::Display = &concat!["Bound<", T::NAME, ">"];
}
impl<T: EncodableType> EncodableType for Bound<T> {
    type Sigil = Compound;
}
impl<'this, T> CompoundTypeDef<'this> for Bound<T> {
    type Intermediate = Alt![Cons![&'this T], Cons![&'this T], Cons![]];
    const DESCRIPTOR: Desc<Self::Intermediate> = cons![
        VariantDescriptor::new("Included", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("Excluded", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("Unbounded", cons![]),
    ];
    fn intermediate(&'this self) -> Self::Intermediate {
        match self {
            Bound::Included(t) => alt![cons![t]],
            Bound::Excluded(t) => alt![@cons![t]],
            Bound::Unbounded => alt![@@cons![]],
        }
    }
}

impl<T: EncodableType> NameableType for RangeInclusive<T> {
    const NAME: &dyn fmt::Display = &concat!["RangeInclusive<", T::NAME, ">"];
}
impl<T: EncodableType> EncodableType for RangeInclusive<T> {
    type Sigil = Compound;
}
impl<'this, T> CompoundTypeDef<'this> for RangeInclusive<T> {
    type Intermediate = Cons![&'this T, &'this T, bool];
    const DESCRIPTOR: Desc<Self::Intermediate> = cons![
        FieldDescriptor::no_default("start"),
        FieldDescriptor::no_default("end"),
        FieldDescriptor::no_default("exhausted"),
    ];
    fn intermediate(&'this self) -> Self::Intermediate {
        cons![
            self.start(),
            self.end(),
            matches!(self.end_bound(), Bound::Excluded(_))
        ]
    }
}
