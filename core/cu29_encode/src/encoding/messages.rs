mod private;
use private::{Bounds, MessageExtFields, Sealed};

use super::*;
use std::marker::PhantomData;

pub use cu29_derive::Message;

pub trait Variants: Sealed {
    /// The descriptor type for this list variant.
    type Descriptor;
}

/// A list of field types.
pub trait Fields: Variants {}
/// A format-agnostic field descriptor.
pub struct FieldDescriptor<T> {
    pub name: &'static str,
    pub default: Option<T>,
}

// The following implementations map a list of field types to a
// list of field descriptors for those types, ensuring that the
// map is coherent at the type level.
impl<Head, Tail: Fields> Sealed for Cons<Head, Tail> {}
impl<Head, Tail: Fields> Fields for Cons<Head, Tail> {}
impl<Head, Tail: Fields> Variants for Cons<Head, Tail> {
    type Descriptor = Cons<FieldDescriptor<Head>, Tail::Descriptor>;
}
impl Sealed for Nil {}
impl Fields for Nil {}
impl Variants for Nil {
    type Descriptor = Nil;
}

impl<A: Fields, B: Variants> Sealed for Alt<A, B> {}
impl<A: Fields, B: Variants> Variants for Alt<A, B> {
    type Descriptor = AltDesc<A::Descriptor, B::Descriptor>;
}
impl Sealed for Term {}
impl Variants for Term {
    type Descriptor = Term;
}

/// A more ergonomic way to access the descriptor for a list of fields.
pub type Desc<L> = <L as Variants>::Descriptor;

/// A message that has fields.
///
/// The [`Message`][] derive macro implements this trait, providing an ergonomic interface
/// for message definers.  Explicit implementations are likely only to be defined by authors
/// of [`DataFormat`]s, for upstream library types.
///
/// Consumers of this trait should typically use its [`Message`] subtrait instead, which
/// avoids any need to name the `'this` lifetime parameter.  The `ImplicitBounds` parameter
/// exists only to facilitate that (and in any event cannot be named owing to its [`Sealed`]
/// constraint: the default of [`Bounds<'this, Self>`] is the only type that the parameter
/// can or should be).  In effect this enables the associated `Fields` type to be generic
/// over the `'this` lifetime parameter in a usable way.
///
/// See [The Better Alternative to Lifetime GATs] for more information.
///
/// [The Better Alternative to Lifetime GATs]: https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats
pub trait MessageFields<'this, ImplicitBounds: Sealed = Bounds<'this, Self>> {
    /// The list of this message's field types.  Due to this trait's `'this` lifetime
    /// parameter, fields can borrow from the message itself.
    ///
    /// Implementors may find it convenient to use the [`Cons`] macro to construct the list.
    type Fields: Variants;

    /// The list of descriptors for this message's field types.
    ///
    /// Implementors may find it convenient to use the [`cons`] macro to construct the list.
    const DESCRIPTOR: Desc<Self::Fields>;

    /// Marshals this message into a list of its fields.
    ///
    /// Implementors may find it convenient to use the [`cons`] macro to construct the list.
    fn as_fields(&'this self) -> Self::Fields;
}

/// A message that has fields.
///
/// The [`Message`][] derive macro implements this trait, providing an ergonomic interface
/// for message definers.  Explicit implementations are likely only to be defined by authors
/// of [`DataFormat`]s, for upstream library types.
///
/// When a message is required in a particular [`DataFormat`], consumers should typically
/// use this trait's [`MessageExt<Format>`] subtrait instead as that provides the schema
/// and encodable fields for that format.
pub trait Message: for<'this> MessageFields<'this> {
    const NAME: &'static str;
}

pub struct EncodableList<List, Format>(pub List, PhantomData<Format>);
pub struct EncodableAlt<Variants, Format>(pub Variants, PhantomData<Format>);

/// A blanket-implemented extension trait that (through its unnameable [`MessageExtFields`]
/// supertrait) provides convenient access to the schema of a message's fields, and the list
/// of encodable field values.
pub trait MessageExt<Format>
where
    Self: Message + for<'this> MessageExtFields<'this, Format>,
{
}

impl<Format, Msg> MessageExt<Format> for Msg where
    Msg: Message + for<'this> MessageExtFields<'this, Format>
{
}
