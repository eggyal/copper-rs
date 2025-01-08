use super::{DataFormat, EncodableType, Encodes, FormatType};
use bincode::{enc::Encoder, error::EncodeError, Encode};
use core::marker::PhantomData;

/// Creates [`ForwardingType`]s via borrowed values.
#[macro_export]
macro_rules! delegate {
    ($(fn$([$($lt:lifetime, )?$t:ident$(: $($bounds:tt)+)?])?($self:ident: $from:ty) -> $to:ty { $def:expr })*) => {$(
        impl<$($($lt, )?)?Format: $crate::Encodes<$to>$(, $t: $crate::EncodableType $(+ $($bounds)+)?)?> $crate::forward::EncodesForwarding<$from> for Format {
            type ForwardingFormatType<'a> = <Format as $crate::Encodes<$to>>::FormatType<'a> where $from: 'a;
            #[allow(clippy::needless_lifetimes)]
            fn forward_encodable<'a>($self: &'a $from) -> Self::ForwardingFormatType<'a> {
                let def: &$to = &$def;
                Self::encodable(def)
            }
        }
        impl$(<$($lt, )?$t: $crate::EncodableType$(+ $($bounds)+)?>)? $crate::EncodableType for $from {
            //FIXME: should this generate a name for $from instead ?
            const NAME: &'static dyn ::core::fmt::Display = <$to as $crate::EncodableType>::NAME;
            type Sigil = $crate::forward::Forwarding;
        }
        impl$(<$($lt, )?$t: $crate::EncodableType$(+ $($bounds)+)?>)? $crate::forward::ForwardingType for $from {}
    )*};
}

/// Creates [`ForwardingType`]s via owned values.
#[macro_export]
macro_rules! defer {
    (fn$([$($lt:lifetime, )?$t:ident$(: $($bounds:tt)+)?])?(&$b:lifetime $self:ident: $from:ty) -> $to:ty as $via:ty { $def:expr } $($rest:tt)*) => {
        impl<$($($lt, )?)?Format: $crate::Encodes<$to>$(, $t: $crate::EncodableType $(+ $($bounds)+)?)?> $crate::forward::EncodesForwarding<$from> for Format {
            type ForwardingFormatType<$b> = $crate::forward::Defer<$via, Format> where $from: $b;
            #[allow(clippy::needless_lifetimes)]
            fn forward_encodable<$b>($self: &$b $from) -> Self::ForwardingFormatType<$b> {
                $crate::forward::Defer::new($def)
            }
        }
        impl$(<$($lt, )?$t: $crate::EncodableType$(+ $($bounds)+)?>)? $crate::EncodableType for $from {
            //FIXME: should this generate a name for $from instead ?
            const NAME: &'static dyn ::core::fmt::Display = <$to as $crate::EncodableType>::NAME;
            type Sigil = $crate::forward::Forwarding;
        }
        impl$(<$($lt, )?$t: $crate::EncodableType$(+ $($bounds)+)?>)? $crate::forward::ForwardingType for $from {}
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

/// Creates [`ForwardingType`]s via iterators.
#[macro_export]
macro_rules! iterate {
    ($format:ident::unbounded<$to:ty, $ordered:tt>) => {$format::UnboundedIterator<$to, {$crate::iterate!($ordered)}>};
    ($format:ident::bounded<$to:ty, $ordered:tt, $n:ident>) => {$format::BoundedIterator<$to, {$crate::iterate!($ordered)}, $n>};
    ($format:ident::static<$to:ty, $ordered:tt, $n:ident>) => {$format::StaticLenIterator<$to, {$crate::iterate!($ordered)}, $n>};

    (arbitrary) => {false};
    (defined) => {true};

    ($($iterator:tt $ordered:tt fn$([$($lt:lifetime, )?$($t:ident$(: $($bounds:tt)+)?),*$(; const $n:ident: usize)?])?(&$b:lifetime $self:ident: $($from:tt)+) -> $to:ty as $item:ty { $def:expr })*) => {$(
        impl<$($($lt, )?)?Format: for<$b> $crate::Encodes<$item>$($(, $t: $crate::EncodableType$( + $($bounds)+)?)*$(, const $n: usize)?)?>
            $crate::forward::EncodesForwarding<$($from)+> for Format
        {
            type ForwardingFormatType<$b> = $crate::iterate!(Format::$iterator<$to, $ordered$($(, $n)?)?>) where $($from)+: $b;
            fn forward_encodable<$b>($self: &$b $($from)+) -> Self::ForwardingFormatType<$b> {
                ::core::convert::Into::into($def)
            }
        }
        impl$(<$($lt, )?$($t: $crate::EncodableType$(+ $($bounds)+)?),*$(, const $n: usize)?>)? $crate::EncodableType for $($from)+ {
            const NAME: &dyn ::core::fmt::Display = &$crate::name!($([$($t),*$(;$n)?])?$($from)+);
            type Sigil = $crate::forward::Forwarding;
        }
        impl$(<$($lt, )?$($t: $crate::EncodableType$(+ $($bounds)+)?),*$(, const $n: usize)?>)? $crate::forward::ForwardingType for $($from)+ {}
    )*};
}

macro_rules! tuples {
    (@$($n:tt:$t:ident),*) => {
        impl<'this$(, $t: $crate::EncodableType)*> $crate::compound::CompoundTypeDef<'this> for ($($t,)*) {
            type Intermediate = $crate::Cons![$(&'this $t),*];
            const DESCRIPTOR: $crate::compound::Desc<Self::Intermediate> = $crate::cons![$(
                $crate::compound::FieldDescriptor::no_default(::core::stringify!($n))
            ),*];
            fn intermediate(&'this self) -> Self::Intermediate {
                $crate::cons![$(&self.$n,)*]
            }
        }
        impl<$($t: $crate::EncodableType),*> $crate::EncodableType for ($($t,)*) {
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

/// [`EncodableType::Sigil`] sigil for [`ForwardingType`]s.
pub enum Forwarding {}
pub trait ForwardingType: EncodableType<Sigil = Forwarding> {}

/// Proxy trait for implementing [`Encodes<T>`] when `T` is a [`ForwardingType`].
///
/// Implementations of this trait are generated by the [`defer`], [`delegate`], [`iterate`]
/// and (crate-private) `tuples` macros, which are the recommended ways to implement it.
///
/// Consumers will usually want to use [`Encodes<T>`] instead, which is blanket implemented
/// for implementers of this trait (but also for other [`EncodableType`]s).
pub trait EncodesForwarding<T: ?Sized + EncodableType<Sigil = Forwarding>>: DataFormat {
    type ForwardingFormatType<'a>: FormatType<Self>
    where
        T: 'a;
    fn forward_encodable(t: &T) -> Self::ForwardingFormatType<'_>;
}

pub struct Defer<D, Format>(pub D, PhantomData<Format>);
impl<D, Format> Defer<D, Format> {
    pub fn new(d: D) -> Self {
        Self(d, PhantomData)
    }
}
impl<D: EncodableType, Format: Encodes<D>> FormatType<Format> for Defer<D, Format>
where
    Self: Encode,
{
    const FIELD_TYPE: Format::FieldType = Format::FormatType::FIELD_TYPE;
}

impl<D: EncodableType, Format: Encodes<D>> Encode for Defer<D, Format> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Format::encodable(&self.0).encode(encoder)
    }
}
