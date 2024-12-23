//! Internal machinery to supports messages, but that need not be exposed.

use super::{
    Alt, AltDesc, Cons, DataFormat, EncodableAlt, EncodableList, FormatType, MessageFields, Nil,
    Term, Value,
};
use bincode::Encode;
use std::marker::PhantomData;

/// Marker trait for constraints that can only be met by private types.
pub trait Sealed {}
pub struct Bounds<'a, T: ?Sized>(PhantomData<&'a T>);
impl<T: ?Sized> Sealed for Bounds<'_, T> {}

/// A list that can be converted into one that is encodable in `Format`.
///
/// The sole two implementations recursively wrap each level of the list in an
/// [`EncodableList`].  This ensures that each `Format` can provide distinct
/// implementations of [`Encode`] for the same initial list.
pub trait IntoEncodable<Format> {
    type Encodable: Encode;
    fn into_encodable(self) -> Self::Encodable;
}

impl<Format, A, B> IntoEncodable<Format> for Alt<A, B>
where
    A: IntoEncodable<Format>,
    B: IntoEncodable<Format>,
    EncodableAlt<Alt<A::Encodable, B::Encodable>, Format>: Encode,
{
    type Encodable = EncodableAlt<Alt<A::Encodable, B::Encodable>, Format>;
    fn into_encodable(self) -> Self::Encodable {
        EncodableAlt(
            match self {
                Self::A(a) => Alt::A(a.into_encodable()),
                Self::B(b) => Alt::B(b.into_encodable()),
            },
            PhantomData,
        )
    }
}
impl<Format> IntoEncodable<Format> for Term
where
    EncodableAlt<Term, Format>: Encode,
{
    type Encodable = EncodableAlt<Term, Format>;
    fn into_encodable(self) -> Self::Encodable {
        EncodableAlt(self, PhantomData)
    }
}

impl<Format, Head, Tail> IntoEncodable<Format> for Cons<Head, Tail>
where
    Tail: IntoEncodable<Format>,
    EncodableList<Cons<Head, Tail::Encodable>, Format>: Encode,
{
    type Encodable = EncodableList<Cons<Head, Tail::Encodable>, Format>;
    fn into_encodable(self) -> Self::Encodable {
        EncodableList(
            Cons {
                head: self.head,
                tail: self.tail.into_encodable(),
            },
            PhantomData,
        )
    }
}
impl<Format> IntoEncodable<Format> for Nil
where
    EncodableList<Nil, Format>: Encode,
{
    type Encodable = EncodableList<Nil, Format>;
    fn into_encodable(self) -> Self::Encodable {
        EncodableList(self, PhantomData)
    }
}

/// A list that can be converted into its schema in `Format`.
///
/// The sole two implementations recursively convert each list item to its
/// [`DataFormat::FieldType`] as defined by its [`Value<Format>`].
pub trait IntoSchema<Format> {
    type FieldSchema;
    const FIELD_SCHEMA: Self::FieldSchema;
}

impl<Format, A, B> IntoSchema<Format> for Alt<A, B>
where
    A: IntoSchema<Format>,
    B: IntoSchema<Format>,
{
    type FieldSchema = AltDesc<A::FieldSchema, B::FieldSchema>;
    const FIELD_SCHEMA: Self::FieldSchema = AltDesc {
        a: A::FIELD_SCHEMA,
        b: B::FIELD_SCHEMA,
    };
}
impl<Format> IntoSchema<Format> for Term {
    type FieldSchema = Term;
    const FIELD_SCHEMA: Term = Term;
}

impl<'this, Format, Head, Tail> IntoSchema<Format> for Cons<Head, Tail>
where
    Format: DataFormat,
    Head: 'this + Value<Format>,
    Tail: IntoSchema<Format>,
{
    type FieldSchema = Cons<Format::FieldType, Tail::FieldSchema>;
    const FIELD_SCHEMA: Self::FieldSchema = Cons {
        head: <Head::FormatType<'this>>::FIELD_TYPE,
        tail: Tail::FIELD_SCHEMA,
    };
}
impl<Format> IntoSchema<Format> for Nil {
    type FieldSchema = Nil;
    const FIELD_SCHEMA: Nil = Nil;
}

/// A blanket-implemented extension trait that provides convenient access to the schema of a
/// message's fields, and the list of encodable field values.
///
/// Consumers of this trait should use its [`MessageExt`] subtrait instead, which is not only
/// nameable (whereas this trait is not) but furthermore avoids any need to name the `'this`
/// lifetime parameter.  The `ImplicitBounds` parameter exists only to facilitate that (and
/// in any event cannot be named owing to its [`Sealed`] constraint: the default of
/// [`Bounds<'this, Self>`] is the only type that the parameter can or should be).  In effect
/// this enables the associated `Fields` type (of the [`MessageFields`] supertrait) to be
/// generic over the `'this` lifetime parameter in a usable way.
///
/// See [The Better Alternative to Lifetime GATs] for more information.
///
/// [`MessageExt`]: super::MessageExt
/// [The Better Alternative to Lifetime GATs]: https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats
pub trait MessageExtFields<'this, Format, ImplicitBounds = Bounds<'this, Self>>
where
    ImplicitBounds: Sealed,
    Self: MessageFields<'this, ImplicitBounds, Fields: IntoEncodable<Format> + IntoSchema<Format>>,
{
    const FIELD_SCHEMA: <Self::Fields as IntoSchema<Format>>::FieldSchema;
    fn as_encodable_fields(&'this self) -> impl Encode {
        self.as_fields().into_encodable()
    }
}
impl<'this, Format, Msg> MessageExtFields<'this, Format> for Msg
where
    Msg: MessageFields<'this>,
    Msg::Fields: IntoEncodable<Format> + IntoSchema<Format>,
{
    const FIELD_SCHEMA: <Self::Fields as IntoSchema<Format>>::FieldSchema =
        Self::Fields::FIELD_SCHEMA;
}
