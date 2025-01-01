//! Schemaful serialisation of self-describing data types.
//!
//! This crate is format agnostic.  It provides the machinery for types
//! to describe their schema via trait implementations that ensure
//! compile-time coherence with the serialised data, through Rust's type
//! system.  A derive macro is provided by the `cu29_derive` crate to
//! ease implementation in regular cases.
//!
//! Implementations of particular serialisation formats can then take
//! place downstream.  A skeleton of the ROS 2.0 message format is
//! currently implemented in the `cu29-ros2msg` crate, showcasing in
//! broad terms how this library can be extended to support arbitrary
//! serialisation formats.
//!
//! [`Encodes`]s are marshalled through instances of some [`FormatType`]
//! in order to decouple the serialised encoding from its schema type.
//! For example, a data format might encode text as an array of bytes
//! yet require the schema type to reflect that the bytes are UTF-8.`

extern crate self as cu29_encode;

macro_rules! delegate {
    ($(fn$([$($lt:lifetime, )?$t:ident$(: $($bounds:tt)+)?])?($self:ident: $from:ty) -> $to:ty { $def:expr })*) => {$(
        impl<$($($lt, )?)?Format: $crate::Encodes<$to>$(+ $crate::Encodes<$t>, $t: $crate::NameableType $(+ $($bounds)+)?)?> $crate::Encodes<$from> for Format {
            type FormatType<'a> = <Format as $crate::Encodes<$to>>::FormatType<'a> where $from: 'a;
            #[allow(clippy::needless_lifetimes)]
            fn encodable<'a>($self: &'a $from) -> Self::FormatType<'a> {
                let def: &$to = &$def;
                <Self as $crate::Encodes<$to>>::encodable(def)
            }
        }
        impl$(<$($lt, )?$t: $crate::NameableType$(+ $($bounds)+)?>)? $crate::NameableType for $from {
            const NAME: &'static dyn ::core::fmt::Display = <$to as $crate::NameableType>::NAME;
        }
    )*};
}

macro_rules! defer {
    ($(fn$([$($lt:lifetime, )?$t:ident$(: $($bounds:tt)+)?])?($self:ident: $from:ty) -> $to:ty { $def:expr })*) => {$(
        impl<$($($lt, )?)?Format: $crate::Encodes<$to>$(+ $crate::Encodes<$t>, $t: $crate::NameableType $(+ $($bounds)+)?)?> $crate::Encodes<$from> for Format {
            type FormatType<'a> = $crate::Defer<$to, Format> where $from: 'a;
            #[allow(clippy::needless_lifetimes)]
            fn encodable<'a>($self: &'a $from) -> Self::FormatType<'a> {
                $crate::Defer::new($def)
            }
        }
        impl$(<$($lt, )?$t: $crate::NameableType$(+ $($bounds)+)?>)? $crate::NameableType for $from {
            const NAME: &'static dyn ::core::fmt::Display = <$to as $crate::NameableType>::NAME;
        }
    )*};
}

#[macro_use]
mod type_list;
use std::{fmt, marker::PhantomData, mem, ops::Deref};

use bincode::{enc::Encoder, error::EncodeError, Encode};
pub use type_list::*;

#[macro_use]
mod complex;
pub use complex::*;

/// # Safety
/// `Self` must be `#[repr(transparent)]` for `Self::Inner`.  Furthermore, all
/// values of `Self::Inner` must be valid for `Self`.
pub unsafe trait Wrapped {
    type Inner: ?Sized;
}

fn wrap<W: Wrapped<Inner: Sized>>(inner: W::Inner) -> W {
    const {
        assert!(mem::size_of::<W>() == mem::size_of::<W::Inner>());
        assert!(mem::align_of::<W>() == mem::align_of::<W::Inner>());
    }
    let inner = mem::ManuallyDrop::new(inner);
    unsafe { mem::transmute_copy(&*inner) }
}
fn wrap_ref<W: ?Sized + Wrapped>(inner: &W::Inner) -> &W {
    unsafe { mem::transmute_copy(&inner) }
}

/// A data format determines how messages are serialised.
pub trait DataFormat {
    /// The field type of a data format is describes any serialised datum's type
    /// in the data format's serialisation schema.
    type FieldType;
    type Wrapped<Inner>: Wrapped<Inner = Inner>;
}

/// A format type is a serializable datum that knows its type in `Format`'s
/// serialisation schema.
pub trait FormatType<Format: ?Sized + DataFormat>: bincode::Encode {
    const FIELD_TYPE: Format::FieldType;
}
pub trait NameableType {
    // It's a little bit annoying that we need to use the indirection of a trait object here,
    // but unfortunately it's necessary to work around the inability to otherwise promote
    // const-context variables to statics.  Nevertheless the use of these constants is pretty
    // rare (just once per occurrence within a schema), so the indirection is unlikely to be
    // a material expense.
    const NAME: &dyn fmt::Display;
}
/// A value type is a serializable datum that knows the [`FormatType`] through
/// which it should be marshalled.
pub trait Encodes<T: ?Sized + NameableType>: DataFormat {
    type FormatType<'a>: FormatType<Self>
    where
        T: 'a;
    fn encodable(_: &T) -> <Self as Encodes<T>>::FormatType<'_>;
}

pub struct Defer<D, Format>(pub D, PhantomData<Format>);
impl<D, Format> Defer<D, Format> {
    pub fn new(d: D) -> Self {
        Self(d, PhantomData)
    }
}
impl<D: NameableType, Format: Encodes<D>> FormatType<Format> for Defer<D, Format>
where
    Self: Encode,
{
    const FIELD_TYPE: Format::FieldType = Format::FormatType::FIELD_TYPE;
}

impl<D: NameableType, Format: Encodes<D>> Encode for Defer<D, Format> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        Format::encodable(&self.0).encode(encoder)
    }
}

#[derive(Clone, Copy)]
pub struct Ref<R>(pub R);
impl<R> Ref<R> {
    pub fn new(r: R) -> Self {
        Self(r)
    }
}
impl<Format: Encodes<R::Target>, R: Deref<Target: NameableType>> Encodes<Ref<R>> for Format {
    type FormatType<'a>
        = Format::FormatType<'a>
    where
        R: 'a;
    fn encodable(this: &Ref<R>) -> Self::FormatType<'_> {
        Self::encodable(&this.0)
    }
}
impl<R: Deref<Target: NameableType>> NameableType for Ref<R> {
    const NAME: &dyn fmt::Display = R::Target::NAME;
}
