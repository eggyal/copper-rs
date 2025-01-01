macro_rules! name {
    ($($t:ty),* $(,)?) => {$(
        impl $crate::NameableType for $t {
            const NAME: &dyn std::fmt::Display = &stringify!($t);
        }
    )*};
}

mod alloc_impls;
mod core_impls;
mod primitive_impls;
mod std_impls;

mod private;
use private::{Bounds, IntoEncodable, Sealed};

use super::*;

pub use cu29_derive::ComplexType;

pub trait Content: Sealed {
    type Descriptor;
    fn calculate_discriminant(&self, _: usize) -> Option<usize> {
        None
    }
}

pub struct FieldDescriptor<FieldType> {
    pub name: &'static str,
    pub default: Option<FieldType>,
}
impl<FieldType> FieldDescriptor<FieldType> {
    pub const fn no_default(name: &'static str) -> Self {
        Self {
            name,
            default: None,
        }
    }
}
trait Fields: Content {}
impl Fields for Nil {}
impl Sealed for Nil {}
impl Content for Nil {
    type Descriptor = Nil;
}
impl<Head, Tail: Fields> Fields for Cons<Head, Tail> {}
impl<Head, Tail: Fields> Sealed for Cons<Head, Tail> {}
impl<Head, Tail: Fields> Content for Cons<Head, Tail> {
    type Descriptor = Cons<FieldDescriptor<Head>, Tail::Descriptor>;
}

pub struct VariantDescriptor<Struct> {
    pub name: &'static str,
    pub fields: Struct,
}
impl<Struct> VariantDescriptor<Struct> {
    pub const fn new(name: &'static str, fields: Struct) -> Self {
        Self { name, fields }
    }
}
trait Variants: Content {}
impl Variants for End {}
impl Sealed for End {}
impl Content for End {
    type Descriptor = Nil;
}
impl<Left: Fields, Right: Variants> Variants for Alt<Left, Right> {}
impl<Left: Fields, Right: Variants> Sealed for Alt<Left, Right> {}
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

pub type Desc<C> = <C as Content>::Descriptor;

/// A complex type (typically comprising multiple fields).
///
/// The [`ComplexType`][cu29_derive::ComplexType] derive macro implements this trait,
/// providing an ergonomic interface for message definers.  Explicit implementations are
/// likely only to be defined by authors of [`DataFormat`]s, for upstream library types.
///
/// Consumers of this trait should typically use its [`ComplexType`] subtrait instead, which
/// avoids any need to name the `'this` lifetime parameter.  The `ImplicitBounds` parameter
/// exists only to facilitate that (and in any event cannot be named owing to its [`Sealed`]
/// constraint: the default of [`Bounds<'this, Self>`] is the only type that the parameter
/// can or should be).  In effect this enables the associated `Descriptor` type to be
/// generic over the `'this` lifetime parameter in a usable way.
///
/// See [The Better Alternative to Lifetime GATs] for more information.
///
/// [The Better Alternative to Lifetime GATs]: https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats
pub trait ComplexTypeDef<'this, ImplicitBounds: Sealed = Bounds<'this, Self>> {
    /// The list of this message's field types.  Due to this trait's `'this` lifetime
    /// parameter, fields can borrow from the message itself.
    ///
    /// Implementors may find it convenient to use the [`Cons`] macro to construct the list.
    type Content: Content;

    /// The list of descriptors for this message's field types.
    ///
    /// Implementors may find it convenient to use the [`cons`] macro to construct the list.
    const DESCRIPTOR: Desc<Self::Content>;

    /// Marshals this message into a list of its fields.
    ///
    /// Implementors may find it convenient to use the [`cons`] macro to construct the list.
    fn content(&'this self) -> Self::Content;
}

/// A complex type (typically comprising multiple fields).
///
/// The [`ComplexType`][cu29_derive::ComplexType] derive macro implements this trait,
/// providing an ergonomic interface for message definers.  Explicit implementations are
/// likely only to be defined by authors of [`DataFormat`]s, for upstream library types.
///
/// When such a complex type is required in a particular [`DataFormat`], consumers should
/// typically use this trait's [`ComplexTypeExt<Format>`] subtrait instead as that provides
/// the schema and encodable fields for that format.
pub trait ComplexType: NameableType + for<'this> ComplexTypeDef<'this> {
    // const NAME: &dyn fmt::Display;
}
impl<T: NameableType + for<'this> ComplexTypeDef<'this>> ComplexType for T {}

pub struct EncodableCons<List, Format>(pub List, PhantomData<Format>);
pub struct EncodableAlt<List, Format>(pub List, PhantomData<Format>);

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
pub trait ComplexTypeExtDef<'this, Format: DataFormat, ImplicitBounds = Bounds<'this, Self>>
where
    ImplicitBounds: Sealed,
    Self: 'this + ComplexTypeDef<'this>,
{
    //     // type FieldSchema;
    //     // const FIELD_SCHEMA: Self::FieldSchema;
    fn as_encodable_content(&'this self) -> impl Encode;
    fn discriminant(&'this self) -> Option<usize>;
}
impl<'this, Format: DataFormat, Msg> ComplexTypeExtDef<'this, Format> for Msg
where
    Msg: ComplexTypeDef<'this>,
    Msg::Content: IntoEncodable<Format>,
    // Msg::Descriptor: IntoEncodable<Format>, // + IntoSchema<Format>,
{
    //     // type FieldSchema = <Self::Fields as IntoSchema<Format>>::FieldSchema;
    //     // const FIELD_SCHEMA: Self::FieldSchema = Self::Fields::FIELD_SCHEMA;
    fn as_encodable_content(&'this self) -> impl Encode {
        self.content().into_encodable()
    }
    fn discriminant(&'this self) -> Option<usize> {
        self.content().calculate_discriminant(0)
    }
}

/// A blanket-implemented extension trait that (through its unnameable [`MessageExtFields`]
/// supertrait) provides convenient access to the schema of a message's fields, and the list
/// of encodable field values.
pub trait ComplexTypeExt<Format: DataFormat>
where
    Self: ComplexType + for<'this> ComplexTypeExtDef<'this, Format>,
{
}

impl<Format: DataFormat, Msg> ComplexTypeExt<Format> for Msg where
    Msg: ComplexType + for<'this> ComplexTypeExtDef<'this, Format>
{
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use bincode::{enc::Encoder, error::EncodeError};
//     use static_assertions::assert_type_eq_all;

//     const _: () = {
//         const fn assert_impl_fields<T: ?Sized + Fields>() {}
//         assert_impl_fields::<Cons!()>();
//         assert_impl_fields::<Cons!(u32)>();
//         assert_impl_fields::<Cons!(u32, f32)>();

//         assert_type_eq_all!(Desc<Cons!()>, Nil);
//         assert_type_eq_all!(Desc<Cons!(u32)>, Cons!(FieldDescriptor<u32>));
//         assert_type_eq_all!(
//             Desc<Cons!(u32, f32)>,
//             Cons!(FieldDescriptor<u32>, FieldDescriptor<f32>)
//         );
//     };

//     #[derive(PartialEq)]
//     enum TestField {
//         String,
//         Slice,
//         Array,
//     }

//     struct TestFormat;
//     impl DataFormat for TestFormat {
//         type FieldType = TestField;
//     }

//     #[derive(Encode)]
//     struct StringField;
//     #[derive(Encode)]
//     struct SliceField;
//     #[derive(Encode)]
//     struct ArrayField;

//     impl FormatType<TestFormat> for StringField {
//         const FIELD_TYPE: TestField = TestField::String;
//     }
//     impl FormatType<TestFormat> for SliceField {
//         const FIELD_TYPE: TestField = TestField::Slice;
//     }
//     impl FormatType<TestFormat> for ArrayField {
//         const FIELD_TYPE: TestField = TestField::Array;
//     }

//     impl Value<TestFormat> for str {
//         type FormatType<'a>
//             = StringField
//         where
//             Self: 'a;
//         fn encodable(&self) -> <Self as Value<TestFormat>>::FormatType<'_> {
//             StringField
//         }
//     }
//     impl Value<TestFormat> for [u32] {
//         type FormatType<'a>
//             = SliceField
//         where
//             Self: 'a;
//         fn encodable(&self) -> <Self as Value<TestFormat>>::FormatType<'_> {
//             SliceField
//         }
//     }
//     impl Value<TestFormat> for [u8; 4] {
//         type FormatType<'a> = ArrayField;
//         fn encodable(&self) -> <Self as Value<TestFormat>>::FormatType<'_> {
//             ArrayField
//         }
//     }

//     impl<List> Encode for EncodableList<List, TestFormat> {
//         fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
//             unimplemented!()
//         }
//     }
//     impl<X> Encode for EncodableAlt<X, TestFormat> {
//         fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), EncodeError> {
//             unimplemented!()
//         }
//     }

//     #[derive(Message)]
//     struct TestMessage<'a> {
//         string: &'a str,
//         slice: &'a [u32],
//         #[message(clone)]
//         array: [u8; 4],
//     }

//     #[allow(unused)]
//     fn obtains_encodable_fields<Msg: MessageExt<TestFormat>>(message: &Msg, test: &TestMessage) {
//         let _ = message.as_encodable_fields();
//         let _ = MessageExtFields::<TestFormat>::as_encodable_fields(test);
//     }

//     // const _: () = {
//     //     if let <TestMessage as MessageExtFields<TestFormat>>::FIELD_SCHEMA =
//     //         altdesc![cons![TestField::String, TestField::Slice, TestField::Array]]
//     //     {
//     //     } else {
//     //         panic!();
//     //     }
//     // };
// }
