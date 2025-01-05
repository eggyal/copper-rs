use crate::{defer, delegate};
use core::{
    cell::{Cell, Ref, RefCell, RefMut},
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
use cu29_derive::fake_derive;

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
    fn[T](&'a this: RefCell<T>) -> T as Ref<'a, T> { this.borrow() }
}

delegate! {
    fn['b, T](this: Ref<'b, T>) -> T { this }
    fn['b, T](this: RefMut<'b, T>) -> T { this }
}

#[fake_derive(CompoundType)]
struct Duration {
    #[compound_type(owned, via = self.as_secs())]
    secs: u64,
    #[compound_type(owned, via = self.subsec_nanos())]
    nanoseconds: u32,
}

#[fake_derive(CompoundType)]
enum Option<T> {
    Some(T),
    None,
}

#[fake_derive(CompoundType)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[fake_derive(CompoundType)]
struct Range<T> {
    start: T,
    end: T,
}

#[fake_derive(CompoundType)]
enum Bound<T> {
    Included(T),
    Excluded(T),
    Unbounded,
}

#[fake_derive(CompoundType)]
struct RangeInclusive<T> {
    #[compound_type(via = self.start())]
    start: T,
    #[compound_type(via = self.end())]
    end: T,
    #[compound_type(owned, via = matches!(self.end_bound(), Bound::Excluded(_)))]
    exhausted: bool,
}
