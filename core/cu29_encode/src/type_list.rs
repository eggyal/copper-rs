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
        $crate::type_list::Cons<$t, $crate::Cons!($($($ts),*)?)>
    };
    () => { $crate::type_list::Nil };
}
/// A macro that constructs an instance of a [`Cons`]-list from the comma-delimited list of
/// its constituent values (in order).
#[macro_export]
macro_rules! cons {
    ($e:expr $(, $($es:expr),* $(,)?)?) => {
        $crate::type_list::Cons {
            head: $e,
            tail: $crate::cons!($($($es),*)?),
        }
    };
    () => { $crate::type_list::Nil };
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

/// A macro that constructs the type of a [`Alt`]-list from the comma-delimited list of
/// its constituent types (in order).
#[macro_export]
macro_rules! Alt {
    ($t:ty $(, $($ts:ty),* $(,)?)?) => {
        $crate::type_list::Alt<$t, $crate::Alt!($($($ts),*)?)>
    };
    () => { $crate::type_list::End };
}
#[macro_export]
macro_rules! alt {
    ($e:expr) => { $crate::type_list::Alt::Head($e) };
    (@ $($rest:tt)+) => { $crate::type_list::Alt::Tail($crate::alt!($($rest)+)) };
}

pub struct Concat<T>(pub T);
impl<Head, Tail> fmt::Display for Concat<Cons<Head, Tail>>
where
    Head: fmt::Display,
    Tail: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This isn't esspecially cache friendly: each element in the concat list could
        // well involve different cache lines.  That said, they will typically all be
        // located within immutable memory, so these reads shouldn't involve any cache
        // invalidation.
        self.0.head.fmt(f)?;
        self.0.tail.fmt(f)
    }
}
impl fmt::Display for Concat<Nil> {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
/// A macro that constructs an instance of a [`Cons`]-list from the comma-delimited list of
/// its constituent values (in order).
#[macro_export]
macro_rules! concat {
    ($e:expr $(, $($es:expr),* $(,)?)?) => {
        $crate::type_list::Concat($crate::type_list::Cons {
            head: $e,
            tail: $crate::concat!($($($es),*)?),
        })
    };
    () => { $crate::type_list::Concat($crate::type_list::Nil) };
}
