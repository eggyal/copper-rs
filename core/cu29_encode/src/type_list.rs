use core::fmt;

/// A Cons-list.
#[repr(C)]
pub struct Cons<Head, Tail> {
    pub head: Head,
    pub tail: Tail,
}
/// Termination of a Cons-list.
#[repr(C)]
pub struct Nil;

/// A macro that constructs the type of a [`Cons`]-list from the comma-delimited list of
/// its constituent types (in order).
#[macro_export]
macro_rules! Cons {
    ($t:ty $(, $($ts:ty),* $(,)?)?) => {
        $crate::Cons<$t, $crate::Cons!($($($ts),*)?)>
    };
    () => { $crate::Nil };
}
/// A macro that constructs an instance of a [`Cons`]-list from the comma-delimited list of
/// its constituent values (in order).
#[macro_export]
macro_rules! cons {
    ($e:expr $(, $($es:expr),* $(,)?)?) => {
        $crate::Cons {
            head: $e,
            tail: $crate::cons!($($($es),*)?),
        }
    };
    () => { $crate::Nil };
}

/// An Alt-list.
// rustc appears to be pretty good at optimising the layout of #[repr(Rust)] nested enums,
// packing the discriminants into the same byte(s) and utilising the remaining byte(s) for
// the union of the variants' data.
pub enum Alt<Head, Tail> {
    Head(Head),
    Tail(Tail),
}
/// Termination of an Alt-list.
pub struct End;

/// A macro that constructs the type of a [`Either`]-list from the comma-delimited list of
/// its constituent types (in order).
#[macro_export]
macro_rules! Alt {
    ($t:ty $(, $($ts:ty),* $(,)?)?) => {
        $crate::Alt<$t, $crate::Alt!($($($ts),*)?)>
    };
    () => { $crate::End };
}
#[macro_export]
macro_rules! alt {
    ($e:expr) => { $crate::Alt::Head($e) };
    (@ $($rest:tt)+) => { $crate::Alt::Tail($crate::alt!($($rest)+)) };
}

// pub trait Array<T> {
//     const LENGTH: usize;
//     fn as_slice(&self) -> &[T] {
//         let data = self as *const Self as *const T;
//         unsafe { std::slice::from_raw_parts(data, Self::LENGTH) }
//     }
//     fn as_mut_slice(&mut self) -> &mut [T] {
//         let data = self as *mut Self as *mut T;
//         unsafe { std::slice::from_raw_parts_mut(data, Self::LENGTH) }
//     }
// }
// impl<T> Array<T> for Nil {
//     const LENGTH: usize = 0;
// }
// impl<T, Tail: Array<T>> Array<T> for Cons<T, Tail> {
//     const LENGTH: usize = Tail::LENGTH + 1;
// }

pub struct Concat<T>(pub T);
impl<Head, Tail> fmt::Display for Concat<Cons<Head, Tail>>
where
    Head: fmt::Display,
    Tail: fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This isn't esspecially cache friendly: each element in the concat list could
        // well involve different cache lines.  That said, they will typically all be
        // located within immutable memory, so these reads shouldn't involve any cache
        // invalidation.
        self.0.head.fmt(f)?;
        self.0.tail.fmt(f)
    }
}
impl fmt::Display for Concat<Nil> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
/// A macro that constructs an instance of a [`Cons`]-list from the comma-delimited list of
/// its constituent values (in order).
#[macro_export]
macro_rules! concat {
    ($e:expr $(, $($es:expr),* $(,)?)?) => {
        $crate::Concat($crate::Cons {
            head: $e,
            tail: $crate::concat!($($($es),*)?),
        })
    };
    () => { $crate::Concat($crate::Nil) };
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use static_assertions::assert_type_eq_all;

//     const _: () = {
//         assert_type_eq_all!(Cons!(), Nil);
//         assert_type_eq_all!(Cons!(u32), Cons<u32, Nil>);
//         assert_type_eq_all!(Cons!(u32, f32), Cons<u32, Cons<f32, Nil>>);

//         let _: Cons!() = cons![];
//         let a: Cons!(u32) = cons![123];
//         let b: Cons!(u32, f32) = cons![456, 7.89];

//         assert!(a.head == 123);
//         assert!(b.head == 456 && b.tail.head == 7.89);
//     };
// }
