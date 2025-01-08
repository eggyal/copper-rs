/// Marker trait for constraints that can only be met by private types.
pub trait Sealed {}
pub struct Bounds<'a, T: ?Sized>(core::marker::PhantomData<&'a T>);
impl<T: ?Sized> Sealed for Bounds<'_, T> {}

use super::{LowerableCompoundDef, FieldDescriptor, VariantDescriptor};
use crate::{
    type_list::{Alt, Cons, End, Nil},
    DataFormat,
};
use bincode::Encode;

pub trait Content {
    type Descriptor;
    fn calculate_discriminant(&self, _: usize) -> Option<usize> {
        None
    }
}

trait Fields: Content {}
impl Fields for Nil {}
impl Content for Nil {
    type Descriptor = Nil;
}
impl<Head, Tail: Fields> Fields for Cons<Head, Tail> {}
impl<Head, Tail: Fields> Content for Cons<Head, Tail> {
    type Descriptor = Cons<FieldDescriptor<Head>, Tail::Descriptor>;
}

trait Variants: Content {}
impl Variants for End {}
impl Content for End {
    type Descriptor = Nil;
}
impl<Left: Fields, Right: Variants> Variants for Alt<Left, Right> {}
impl<Left: Fields, Right: Variants> Content for Alt<Left, Right> {
    type Descriptor = Cons<VariantDescriptor<Left::Descriptor>, Right::Descriptor>;

    // Determining the active variant is (superficially) costly: one must recursively traverse
    // `Alt`-lists until a `Head` variant is encountered noting the depth at which it was reached
    // (by incrementing a depth counter as one traverses).
    //
    // In some cases (enums without fields or with primitive reprs) we could utilise more direct
    // access to their Rust discriminant: however we should first determine how well the recursion
    // below is optimised and how costly it is in practice.
    fn calculate_discriminant(&self, depth: usize) -> Option<usize> {
        match self {
            Self::Head(_) => Some(depth),
            Self::Tail(right) => right.calculate_discriminant(depth + 1),
        }
    }
}

/// A list that can be converted into one that is encodable in `Format`.
///
/// The sole two implementations recursively wrap each level of the list in an
/// [`EncodableList`].  This ensures that each `Format` can provide distinct
/// implementations of [`Encode`] for the same initial list.
pub trait IntoEncodable<Format> {
    type Encodable: Encode;
    fn into_encodable(self) -> Self::Encodable;
}

impl<Format: DataFormat, A, B> IntoEncodable<Format> for Alt<A, B>
where
    A: IntoEncodable<Format>,
    B: IntoEncodable<Format>,
    Format::EncodableAlt<Alt<A::Encodable, B::Encodable>>: Encode,
{
    type Encodable = Format::EncodableAlt<Alt<A::Encodable, B::Encodable>>;
    fn into_encodable(self) -> Self::Encodable {
        match self {
            Self::Head(a) => Alt::Head(a.into_encodable()),
            Self::Tail(b) => Alt::Tail(b.into_encodable()),
        }
        .into()
    }
}
impl<Format: DataFormat> IntoEncodable<Format> for End
where
    Format::EncodableAlt<End>: Encode,
{
    type Encodable = Format::EncodableAlt<End>;
    fn into_encodable(self) -> Self::Encodable {
        self.into()
    }
}

impl<Format: DataFormat, Head, Tail> IntoEncodable<Format> for Cons<Head, Tail>
where
    Tail: IntoEncodable<Format>,
    Format::EncodableCons<Cons<Head, Tail::Encodable>>: Encode,
{
    type Encodable = Format::EncodableCons<Cons<Head, Tail::Encodable>>;
    fn into_encodable(self) -> Self::Encodable {
        Cons {
            head: self.head,
            tail: self.tail.into_encodable(),
        }
        .into()
    }
}
impl<Format: DataFormat> IntoEncodable<Format> for Nil
where
    Format::EncodableCons<Nil>: Encode,
{
    type Encodable = Format::EncodableCons<Nil>;
    fn into_encodable(self) -> Self::Encodable {
        self.into()
    }
}

/// A blanket-implemented extension trait that provides convenient access to the schema of a
/// compound type's fields, and the list of encodable field values.
///
/// Consumers of this trait should use its [`LowerableCompoundExt`] subtrait instead, which is not
/// only nameable (whereas this trait is not) but furthermore avoids any need to name the
/// `'this` lifetime parameter.  The `ImplicitBounds` parameter exists only to facilitate that
/// (and in any event cannot be named owing to its `Sealed` constraint: the default of
/// [`Bounds<'this, Self>`] is the only type that the parameter can or should be).  In effect
/// this enables the associated [`Intermediate`] type (of the [`LowerableCompoundDef`] supertrait)
/// to be generic over the `'this` lifetime parameter in a usable way.
///
/// See [The Better Alternative to Lifetime GATs] for more information.
///
/// [`LowerableCompoundExt`]: super::LowerableCompoundExt
/// [`Intermediate`]: LowerableCompoundDef::Intermediate
/// [The Better Alternative to Lifetime GATs]: https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats
pub trait LowerableCompoundExtDef<'this, Format: DataFormat, ImplicitBounds = Bounds<'this, Self>>
where
    ImplicitBounds: Sealed,
    Self: 'this + LowerableCompoundDef<'this>,
{
    //     // type FieldSchema;
    //     // const FIELD_SCHEMA: Self::FieldSchema;
    fn as_encodable_content(&'this self) -> impl Encode;
    fn discriminant(&'this self) -> Option<usize>;
}
impl<'this, Format: DataFormat, T> LowerableCompoundExtDef<'this, Format> for T
where
    T: LowerableCompoundDef<'this>,
    T::Intermediate: IntoEncodable<Format>,
    // T::Descriptor: IntoEncodable<Format>, // + IntoSchema<Format>,
{
    //     // type FieldSchema = <Self::Fields as IntoSchema<Format>>::FieldSchema;
    //     // const FIELD_SCHEMA: Self::FieldSchema = Self::Fields::FIELD_SCHEMA;
    fn as_encodable_content(&'this self) -> impl Encode {
        self.intermediate().into_encodable()
    }
    fn discriminant(&'this self) -> Option<usize> {
        self.intermediate().calculate_discriminant(0)
    }
}
