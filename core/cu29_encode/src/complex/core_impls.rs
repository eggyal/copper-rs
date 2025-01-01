use super::{
    ComplexTypeDef, Defer, Desc, FieldDescriptor, NameableType, Ref, Encodes,
    VariantDescriptor,
};
use core::{
    cell::{self, Cell, RefCell},
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
    fn[T](this: AtomicPtr<T>) -> *const T { this.load(SeqCst) }
}

impl<Format: Encodes<T>, T: Copy + NameableType> Encodes<Cell<T>> for Format {
    type FormatType<'a>
        = Defer<T, Format>
    where
        T: 'a;
    fn encodable(this: &Cell<T>) -> Defer<T, Format> {
        Defer::new(this.get())
    }
}
impl<T: NameableType> NameableType for Cell<T> {
    const NAME: &dyn std::fmt::Display = T::NAME;
}

impl<Format: Encodes<T>, T: NameableType> Encodes<RefCell<T>> for Format {
    type FormatType<'a>
        = Defer<Ref<cell::Ref<'a, T>>, Format>
    where
        T: 'a;
    fn encodable(this: &RefCell<T>) -> Self::FormatType<'_> {
        Defer::new(Ref::new(this.borrow()))
    }
}
impl<T: NameableType> NameableType for RefCell<T> {
    const NAME: &dyn std::fmt::Display = T::NAME;
}

impl NameableType for Duration {
    const NAME: &dyn fmt::Display = &"Duration";
}
impl ComplexTypeDef<'_> for Duration {
    type Content = Cons!(u64, u32);
    const DESCRIPTOR: Desc<Self::Content> = cons![
        FieldDescriptor::no_default("secs"),
        FieldDescriptor::no_default("nanoseconds"),
    ];
    fn content(&self) -> Self::Content {
        cons![self.as_secs(), self.subsec_nanos(),]
    }
}

impl<T: NameableType> NameableType for Option<T> {
    const NAME: &dyn fmt::Display = &concat!["Option<", T::NAME, ">"];
}
impl<'this, T> ComplexTypeDef<'this> for Option<T> {
    type Content = Alt![Cons![Ref<&'this T>], Cons![]];
    const DESCRIPTOR: Desc<Self::Content> = cons![
        VariantDescriptor::new("Some", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("None", cons![]),
    ];
    fn content(&'this self) -> Self::Content {
        match self {
            Some(value) => alt![cons![Ref::new(value)]],
            None => alt![@cons![]],
        }
    }
}

impl<T: NameableType, E: NameableType> NameableType for Result<T, E> {
    const NAME: &dyn fmt::Display = &concat!["Result<", T::NAME, ",", E::NAME, ">"];
}
impl<'this, T, E> ComplexTypeDef<'this> for Result<T, E> {
    type Content = Alt![Cons![Ref<&'this T>], Cons![Ref<&'this E>]];
    const DESCRIPTOR: Desc<Self::Content> = cons![
        VariantDescriptor::new("Ok", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("Err", cons![FieldDescriptor::no_default("0")]),
    ];
    fn content(&'this self) -> Self::Content {
        match self {
            Ok(value) => alt![cons![Ref::new(value)]],
            Err(err) => alt![@cons![Ref::new(err)]],
        }
    }
}

impl<T: NameableType> NameableType for Range<T> {
    const NAME: &dyn fmt::Display = &concat!["Range<", T::NAME, ">"];
}
impl<'this, T> ComplexTypeDef<'this> for Range<T> {
    type Content = Cons![Ref<&'this T>, Ref<&'this T>];
    const DESCRIPTOR: Desc<Self::Content> = cons![
        FieldDescriptor::no_default("start"),
        FieldDescriptor::no_default("end"),
    ];
    fn content(&'this self) -> Self::Content {
        cons![Ref::new(&self.start), Ref::new(&self.end)]
    }
}

impl<T: NameableType> NameableType for Bound<T> {
    const NAME: &dyn fmt::Display = &concat!["Bound<", T::NAME, ">"];
}
impl<'this, T> ComplexTypeDef<'this> for Bound<T> {
    type Content = Alt![Cons![Ref<&'this T>], Cons![Ref<&'this T>], Cons![]];
    const DESCRIPTOR: Desc<Self::Content> = cons![
        VariantDescriptor::new("Included", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("Excluded", cons![FieldDescriptor::no_default("0")]),
        VariantDescriptor::new("Unbounded", cons![]),
    ];
    fn content(&'this self) -> Self::Content {
        match self {
            Bound::Included(t) => alt![cons![Ref::new(t)]],
            Bound::Excluded(t) => alt![@cons![Ref::new(t)]],
            Bound::Unbounded => alt![@@cons![]],
        }
    }
}

impl<T: NameableType> NameableType for RangeInclusive<T> {
    const NAME: &dyn fmt::Display = &concat!["RangeInclusive<", T::NAME, ">"];
}
impl<'this, T> ComplexTypeDef<'this> for RangeInclusive<T> {
    type Content = Cons![Ref<&'this T>, Ref<&'this T>, bool];
    const DESCRIPTOR: Desc<Self::Content> = cons![
        FieldDescriptor::no_default("start"),
        FieldDescriptor::no_default("end"),
        FieldDescriptor::no_default("exhausted"),
    ];
    fn content(&'this self) -> Self::Content {
        cons![
            Ref::new(self.start()),
            Ref::new(self.end()),
            matches!(self.end_bound(), Bound::Excluded(_))
        ]
    }
}
