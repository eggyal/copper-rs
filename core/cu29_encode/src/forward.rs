use super::{DataFormat, Lowerable, Lowered, Lowers};
use bincode::{enc::Encoder, error::EncodeError, Encode};
use core::marker::PhantomData;

/// Creates [`LowerableVia`]s via borrowed values.
#[macro_export]
macro_rules! delegate {
    ($(fn$([$($lt:lifetime, )?$t:ident$(: $($bounds:tt)+)?])?($self:ident: $from:ty) -> $to:ty { $def:expr })*) => {$(
        impl<$($($lt, )?)?Format: $crate::Lowers<$to>$(, $t: $crate::Lowerable $(+ $($bounds)+)?)?> $crate::forward::LowersVia<$from> for Format {
            type ViaLowered<'a> = <Format as $crate::Lowers<$to>>::Lowered<'a> where $from: 'a;
            #[allow(clippy::needless_lifetimes)]
            fn lower_via<'a>($self: &'a $from) -> Self::ViaLowered<'a> {
                let def: &$to = &$def;
                Self::lower(def)
            }
        }
        impl$(<$($lt, )?$t: $crate::Lowerable$(+ $($bounds)+)?>)? $crate::Lowerable for $from {
            //FIXME: should this generate a name for $from instead ?
            const NAME: &'static dyn ::core::fmt::Display = <$to as $crate::Lowerable>::NAME;
            type Sigil = $crate::forward::Forwarding;
        }
        impl$(<$($lt, )?$t: $crate::Lowerable$(+ $($bounds)+)?>)? $crate::forward::LowerableVia for $from {}
    )*};
}

/// Creates [`LowerableVia`]s via owned values.
#[macro_export]
macro_rules! defer {
    (fn$([$($lt:lifetime, )?$t:ident$(: $($bounds:tt)+)?])?(&$b:lifetime $self:ident: $from:ty) -> $to:ty as $via:ty { $def:expr } $($rest:tt)*) => {
        impl<$($($lt, )?)?Format: $crate::Lowers<$to>$(, $t: $crate::Lowerable $(+ $($bounds)+)?)?> $crate::forward::LowersVia<$from> for Format {
            type ViaLowered<$b> = $crate::forward::Defer<$via, Format> where $from: $b;
            #[allow(clippy::needless_lifetimes)]
            fn lower_via<$b>($self: &$b $from) -> Self::ViaLowered<$b> {
                $crate::forward::Defer::new($def)
            }
        }
        impl$(<$($lt, )?$t: $crate::Lowerable$(+ $($bounds)+)?>)? $crate::Lowerable for $from {
            //FIXME: should this generate a name for $from instead ?
            const NAME: &'static dyn ::core::fmt::Display = <$to as $crate::Lowerable>::NAME;
            type Sigil = $crate::forward::Forwarding;
        }
        impl$(<$($lt, )?$t: $crate::Lowerable$(+ $($bounds)+)?>)? $crate::forward::LowerableVia for $from {}
        defer! { $($rest)* }
    };
    (fn$([$($lt:lifetime, )?$t:ident$(: $($bounds:tt)+)?])?($self:ident: $from:ty) -> $to:ty { $def:expr } $($rest:tt)*) => {
        defer! {
            fn$([$($lt, )?$t$(: $($bounds)+)?])?(&'a $self: $from) -> $to as $to { $def }
            $($rest)*
        }
    };
    () => {};
}

#[macro_export]
macro_rules! name {
    ($([$($t:ident),*$(; $n:ident)?])? $from:ident$($params:tt)*) => {
        $crate::concat![::core::stringify!($from), $("<", $($t::NAME,)* $($n,)? ">")?]
    };
    ([$t:ident$(; $n:ident)?] [$($from:tt)+]) => {
        $crate::concat!["[", $t::NAME, $(";", $n,)? "]"]
    };
}

/// Creates [`LowerableVia`]s via iterators.
#[macro_export]
macro_rules! iterate {
    ($format:ident::unbounded<$to:ty, $ordered:tt>) => {$format::UnboundedIterator<$to, {$crate::iterate!($ordered)}>};
    ($format:ident::bounded<$to:ty, $ordered:tt, $n:ident>) => {$format::BoundedIterator<$to, {$crate::iterate!($ordered)}, $n>};
    ($format:ident::static<$to:ty, $ordered:tt, $n:ident>) => {$format::StaticLenIterator<$to, {$crate::iterate!($ordered)}, $n>};

    (arbitrary) => {false};
    (defined) => {true};

    ($($iterator:tt $ordered:tt fn$([$($lt:lifetime, )?$($t:ident$(: $($bounds:tt)+)?),*$(; const $n:ident: usize)?])?(&$b:lifetime $self:ident: $($from:tt)+) -> $to:ty as $item:ty { $def:expr })*) => {$(
        impl<$($($lt, )?)?Format: for<$b> $crate::Lowers<$item>$($(, $t: $crate::Lowerable$( + $($bounds)+)?)*$(, const $n: usize)?)?>
            $crate::forward::LowersVia<$($from)+> for Format
        {
            type ViaLowered<$b> = $crate::iterate!(Format::$iterator<$to, $ordered$($(, $n)?)?>) where $($from)+: $b;
            fn lower_via<$b>($self: &$b $($from)+) -> Self::ViaLowered<$b> {
                ::core::convert::Into::into($def)
            }
        }
        impl$(<$($lt, )?$($t: $crate::Lowerable$(+ $($bounds)+)?),*$(, const $n: usize)?>)? $crate::Lowerable for $($from)+ {
            const NAME: &dyn ::core::fmt::Display = &$crate::name!($([$($t),*$(;$n)?])?$($from)+);
            type Sigil = $crate::forward::Forwarding;
        }
        impl$(<$($lt, )?$($t: $crate::Lowerable$(+ $($bounds)+)?),*$(, const $n: usize)?>)? $crate::forward::LowerableVia for $($from)+ {}
    )*};
}

macro_rules! tuples {
    (@$($n:tt:$t:ident),*) => {
        impl<'this$(, $t: $crate::Lowerable)*> $crate::compound::LowerableCompoundDef<'this> for ($($t,)*) {
            type Intermediate = $crate::Cons![$(&'this $t),*];
            const DESCRIPTOR: $crate::compound::Desc<Self::Intermediate> = $crate::cons![$(
                $crate::compound::FieldDescriptor::no_default(::core::stringify!($n))
            ),*];
            fn intermediate(&'this self) -> Self::Intermediate {
                $crate::cons![$(&self.$n,)*]
            }
        }
        impl<$($t: $crate::Lowerable),*> $crate::Lowerable for ($($t,)*) {
            const NAME: &dyn ::core::fmt::Display = &$crate::concat!["(", $($t::NAME,)* ")"];
            type Sigil = $crate::compound::Compound;
        }
    };
    ([$($done:tt)*] $n:tt:$t:ident $(,$($todo:tt)*)?) => {
        $crate::forward::tuples!(@ $($done)* $n:$t);
        $crate::forward::tuples!([$($done)* $n:$t,] $($($todo)*)?);
    };
    ([$($done:tt)*]) => {};
    ($($todo:tt)*) => {
        $crate::forward::tuples!(@);
        $crate::forward::tuples!([] $($todo)*);
    };
}

pub(crate) use tuples;

/// [`Lowerable::Sigil`] sigil for [`LowerableVia`]s.
pub enum Forwarding {}
pub trait LowerableVia: Lowerable<Sigil = Forwarding> {}

/// Proxy trait for implementing [`Lowers<T>`] when `T` is a [`LowerableVia`].
///
/// Implementations of this trait are generated by the [`defer`], [`delegate`], [`iterate`]
/// and (crate-private) `tuples` macros, which are the recommended ways to implement it.
///
/// Consumers will usually want to use [`Lowers<T>`] instead, which is blanket implemented
/// for implementers of this trait (but also for other [`Lowerable`]s).
pub trait LowersVia<T: ?Sized + Lowerable<Sigil = Forwarding>>: DataFormat {
    type ViaLowered<'a>: Lowered<Self>
    where
        T: 'a;
    fn lower_via(t: &T) -> Self::ViaLowered<'_>;
}

pub struct Defer<D, Format>(pub D, PhantomData<Format>);
impl<D, Format> Defer<D, Format> {
    pub fn new(d: D) -> Self {
        Self(d, PhantomData)
    }
}
impl<D: Lowerable, Format: Lowers<D>> Lowered<Format> for Defer<D, Format>
where
    Self: Encode,
{
    const FIELD_TYPE: Format::FieldType = Format::Lowered::FIELD_TYPE;
}

impl<D: Lowerable, Format: Lowers<D>> Encode for Defer<D, Format> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Format::lower(&self.0).encode(encoder)
    }
}
