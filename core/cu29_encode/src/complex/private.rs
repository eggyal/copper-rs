//! Internal machinery to supports messages, but that need not be exposed.

use super::{Alt, Cons, DataFormat, End, Nil, wrap};
// use super::{[]
//     Alt, AltDesc, Cons, DataFormat, Wrapper, EncodableList, FormatType, Nil, Term, Value,
// };
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

impl<Format: DataFormat, A, B> IntoEncodable<Format> for Alt<A, B>
where
    A: IntoEncodable<Format>,
    B: IntoEncodable<Format>,
    Format::Wrapped<Alt<A::Encodable, B::Encodable>>: Encode,
{
    type Encodable = Format::Wrapped<Alt<A::Encodable, B::Encodable>>;
    fn into_encodable(self) -> Self::Encodable {
        wrap(match self {
            Self::Head(a) => Alt::Head(a.into_encodable()),
            Self::Tail(b) => Alt::Tail(b.into_encodable()),
        })
    }
}
impl<Format: DataFormat> IntoEncodable<Format> for End
where
    Format::Wrapped<End>: Encode,
{
    type Encodable = Format::Wrapped<End>;
    fn into_encodable(self) -> Self::Encodable {
        wrap(self)
    }
}

impl<Format: DataFormat, Head, Tail> IntoEncodable<Format> for Cons<Head, Tail>
where
    Tail: IntoEncodable<Format>,
    Format::Wrapped<Cons<Head, Tail::Encodable>>: Encode,
{
    type Encodable = Format::Wrapped<Cons<Head, Tail::Encodable>>;
    fn into_encodable(self) -> Self::Encodable {
        wrap(Cons {
            head: self.head,
            tail: self.tail.into_encodable(),
        })
    }
}
impl<Format: DataFormat> IntoEncodable<Format> for Nil
where
    Format::Wrapped<Nil>: Encode,
{
    type Encodable = Format::Wrapped<Nil>;
    fn into_encodable(self) -> Self::Encodable {
        wrap(self)
    }
}

// /// A list that can be converted into its schema in `Format`.
// ///
// /// The sole two implementations recursively convert each list item to its
// /// [`DataFormat::FieldType`] as defined by its [`Value<Format>`].
// pub trait IntoSchema<Format> {
//     type FieldSchema;
//     const FIELD_SCHEMA: Self::FieldSchema;
// }

// impl<Format, A, B> IntoSchema<Format> for Alt<A, B>
// where
//     A: IntoSchema<Format>,
//     B: IntoSchema<Format>,
// {
//     type FieldSchema = AltDesc<A::FieldSchema, B::FieldSchema>;
//     const FIELD_SCHEMA: Self::FieldSchema = AltDesc {
//         a: A::FIELD_SCHEMA,
//         b: B::FIELD_SCHEMA,
//     };
// }
// impl<Format> IntoSchema<Format> for Term {
//     type FieldSchema = Term;
//     const FIELD_SCHEMA: Term = Term;
// }

// impl<'this, Format, Head, Tail> IntoSchema<Format> for Cons<Head, Tail>
// where
//     Format: DataFormat,
//     Head: 'this + Value<Format>,
//     Tail: IntoSchema<Format>,
// {
//     type FieldSchema = Cons<Format::FieldType, Tail::FieldSchema>;
//     const FIELD_SCHEMA: Self::FieldSchema = Cons {
//         head: <Head::FormatType<'this>>::FIELD_TYPE,
//         tail: Tail::FIELD_SCHEMA,
//     };
// }
// impl<Format> IntoSchema<Format> for Nil {
//     type FieldSchema = Nil;
//     const FIELD_SCHEMA: Nil = Nil;
// }

// // type FieldSchema<L, Format> = <L as IntoSchema<Format>>::FieldSchema;
