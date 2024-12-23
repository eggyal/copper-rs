/// A Cons-list.
#[derive(PartialEq)]
pub struct Cons<Head, Tail> {
    pub head: Head,
    pub tail: Tail,
}
/// Termination of a Cons-list.
#[derive(PartialEq)]
pub struct Nil;

/// A macro that constructs the type of a [`Cons`]-list from the comma-delimited list of
/// its constituent types (in order).
#[macro_export]
macro_rules! Cons {
    ($t:ty $(, $($ts:ty),* $(,)?)?) => {
        $crate::encoding::Cons<$t, $crate::Cons!($($($ts),*)?)>
    };
    () => { $crate::encoding::Nil };
}

/// A macro that constructs an instance of a [`Cons`]-list from the comma-delimited list of
/// its constituent values (in order).
#[macro_export]
macro_rules! cons {
    ($e:expr $(, $($es:expr),* $(,)?)?) => {
        $crate::encoding::Cons {
            head: $e,
            tail: $crate::cons!($($($es),*)?),
        }
    };
    () => { $crate::encoding::Nil };
}

pub enum Alt<A, B> {
    A(A),
    B(B),
}
/// Termination of a Alt-list.
#[derive(PartialEq)]
pub struct Term;

#[derive(PartialEq)]
pub struct AltDesc<A, B> {
    pub a: A,
    pub b: B,
}

#[macro_export]
macro_rules! Alt {
    ($t:ty $(, $($ts:ty),* $(,)?)?) => {
        $crate::encoding::Alt<$t, $crate::Alt!($($($ts),*)?)>
    };
    () => { $crate::encoding::Term };
}

#[macro_export]
macro_rules! alt {
    (@ $($rest:tt)+) => {
        $crate::encoding::Alt::B($crate::alt!($($rest)+))
    };
    ($e:expr) => { $crate::encoding::Alt::A($e) };
}

#[macro_export]
macro_rules! altdesc {
    ($e:expr $(, $($es:expr),* $(,)?)?) => {
        $crate::encoding::AltDesc {
            a: $e,
            b: $crate::altdesc!($($($es),*)?),
        }
    };
    () => { $crate::encoding::Term };
}
