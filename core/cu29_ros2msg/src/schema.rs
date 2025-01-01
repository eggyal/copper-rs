use cu29_encode::NameableType;

use crate::encoding::ComplexTypeExt;

use super::{
    encoding::{FormatType, Encodes},
    format::*,
    invoke_macro_with_primitives, Ros2Msg,
};
use core::fmt;
use std::borrow::Borrow;

pub enum FieldType {
    Bool,
    Byte,
    Char,
    Float32,
    Float64,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    String,
    WString,

    StaticArray {
        elements: &'static Self,
        length: usize,
    },
    UnboundedArray {
        elements: &'static Self,
    },
    BoundedArray {
        elements: &'static Self,
        bound: usize,
    },
    BoundedString {
        bound: usize,
    },

    Message {
        name: &'static dyn fmt::Display,
    },
}

macro_rules! impl_formattype {
    ($($format:ident($rust:ty)),* $(,)?) => {$(
        impl FormatType<Ros2Msg> for $format {
            const FIELD_TYPE: FieldType = FieldType::$format;
        }
    )*};
}

invoke_macro_with_primitives!(impl_formattype);
impl_formattype! {
    Byte(_),
    Char(_),
}

impl<'a, T: 'a + NameableType, const N: usize> FormatType<Ros2Msg> for StaticArray<'_, T, N> where Ros2Msg: Encodes<T> {
    const FIELD_TYPE: FieldType = FieldType::StaticArray {
        elements: &<Ros2Msg as Encodes<T>>::FormatType::<'a>::FIELD_TYPE,
        length: N,
    };
}
impl<'a, T: 'a + NameableType, I: Clone + Iterator<Item: Borrow<T>>> FormatType<Ros2Msg>
    for UnboundedArray<'_, I, T>
where
    Ros2Msg: Encodes<T>,
{
    const FIELD_TYPE: FieldType = FieldType::UnboundedArray {
        elements: &<Ros2Msg as Encodes<T>>::FormatType::<'a>::FIELD_TYPE,
    };
}
impl<'a, T: 'a + NameableType, const N: usize> FormatType<Ros2Msg> for BoundedArray<'_, T, N> where Ros2Msg: Encodes<T> {
    const FIELD_TYPE: FieldType = FieldType::BoundedArray {
        elements: &<Ros2Msg as Encodes<T>>::FormatType::<'a>::FIELD_TYPE,
        bound: N,
    };
}

impl FormatType<Ros2Msg> for String<'_> {
    const FIELD_TYPE: FieldType = FieldType::String;
}
impl FormatType<Ros2Msg> for WString<'_> {
    const FIELD_TYPE: FieldType = FieldType::WString;
}
impl<const N: usize> FormatType<Ros2Msg> for BoundedString<N> {
    const FIELD_TYPE: FieldType = FieldType::BoundedString { bound: N };
}

impl<M: ComplexTypeExt<Ros2Msg>> FormatType<Ros2Msg> for MessageType<'_, M> {
    const FIELD_TYPE: FieldType = FieldType::Message { name: M::NAME };
}

// impl<T: Value<Ros2Msg>> Value<Ros2Msg> for Option<T> {
//     type FormatType<'a>
//         = BoundedArray<'a, T, 1>
//     where
//         T: 'a;
//     fn encodable(&self) -> <Self as Value<Ros2Msg>>::FormatType<'_> {
//         BoundedArray {
//             data: self.as_ref().map(std::slice::from_ref).unwrap_or_default(),
//             length: if self.is_some() { 1 } else { 0 },
//         }
//     }
// }

// use typenum::Unsigned;

// /// A [unicode scalar value] is any [unicode code point] other than a [surrogate code point].
// ///
// /// [unicode scalar value]: https://www.unicode.org/glossary/#unicode_scalar_value
// /// [unicode code point]: https://www.unicode.org/glossary/#code_point
// /// [surrogate code point]: https://www.unicode.org/glossary/#surrogate_code_point
// trait UnicodeScalarValue: Unsigned {
//     /// The number of bytes in this unicode scalar value's UTF-8 encoding
//     type Utf8Length: ArrayLength<ArrayType<u8>: ConstDefault>
//         + typenum::IsGreaterOrEqual<typenum::U1, Output = typenum::True>
//         + typenum::IsLessOrEqual<typenum::U4, Output = typenum::True>;
// }

// /// Implements [`UnicodeScalarValue`] for the specified typenum integers.
// ///
// /// A space-separated list of implementees is grouped within braces `{` and `}` by their length
// /// when encoded in UTF-8; such length precedes each group.
// ///
// /// Implementees are specified within square brackets `[` and `]` by a space-separated list of
// /// their bits, each of which may be either:
// ///   * a literal: `1` or `0`; or
// ///   * a variable: an ident.
// ///
// /// Note that bits are specified from most to least significant, and literals can only form a
// /// (most-significant) prefix.  Moreover, the most-significant specified bit must always be a
// /// literal `1`.
// macro_rules! USV {
//     // This matcher is the macro's entry point.  It repeatedly delegates to the subsequent matcher.
//     ($( $len:literal { $([$($spec:tt)*])+ })*) => {$($( USV! { $len [$($spec)*] [$($spec)*] } )+)*};

//     // This matcher handles a single implementee, but requires its list of bit specifiers twice:
//     //
//     //   * once to distinguish between literal (`$lit`) and variable (`$var`) bits, so that the
//     //     the latter can be expanded to the impl generics; and
//     //
//     //   * once as `tt` fragments (`$spec`) so that literals can be matched against their tokens
//     //     (note that, as mentioned in The Rust Reference under  [Forwarding a matched fragment],
//     //     only `ident`, `lifetime` and `tt` fragments can be matched by literal tokens).
//     //
//     // [Forwarding a matched fragment]: https://doc.rust-lang.org/reference/macros-by-example.html#forwarding-a-matched-fragment
//     ($len:literal [$($lit:literal)* $($($var:ident)+)?] [$($spec:tt)*]) => {
//         impl$(<$($var: ::typenum::Bit),*>)? $crate::ros2msg::schema::UnicodeScalarValue for USV!([]$($spec)*) {
//             type Utf8Length = ::typenum::U<$len>;
//         }
//     };

//     // The innermost bit of a `UInt` is the most significant, so we need to reverse the bit
//     // specifiers prior to expansion, by recursively prepending `$this` onto`$rev` until
//     // `$spec` is empty.
//     ([$($rev:tt)*] $this:tt $($spec:tt)*) => { USV!([$this $($rev)*] $($spec)*) };

//     // Literal bit specifiers expand to a `UInt` containing the relevant bit type.
//     ([0 $($rev:tt)*]) => { ::typenum::UInt<USV!([$($rev)*]), ::typenum::B0> };
//     ([1 $($rev:tt)*]) => { ::typenum::UInt<USV!([$($rev)*]), ::typenum::B1> };

//     // Variable bit specifiers expand to a `UInt` containing the generic.
//     ([$var:ident $($rev:tt)*]) => { ::typenum::UInt<USV!([$($rev)*]), $var> };

//     // An empty list of bit specifiers expands to `UTerm`.
//     ([]) => { ::typenum::UTerm };
// }

// USV! {
//     // Code points comprising up to 7 bits (inclusive) always encode to 1 byte.
//     1 {
//         [                                                                                  ] // 0x____00 ..= 0x____00
//         [                                                                                 1] // 0x____01 ..= 0x____01
//         [                                                                              1 B0] // 0x____02 ..= 0x____03
//         [                                                                           1 B1 B0] // 0x____04 ..= 0x____07
//         [                                                                        1 B2 B1 B0] // 0x____08 ..= 0x____0f
//         [                                                                   1   B3 B2 B1 B0] // 0x____10 ..= 0x____1f
//         [                                                                1 B4   B3 B2 B1 B0] // 0x____20 ..= 0x____3f
//         [                                                             1 B5 B4   B3 B2 B1 B0] // 0x____40 ..= 0x____7f
//     }

//     // Code points comprising between 8 and 11 bits (inclusive) always encode to 2 bytes.
//     2 {
//         [                                                          1 B6 B5 B4   B3 B2 B1 B0] // 0x___080 ..= 0x___0ff
//         [                                                     1   B7 B6 B5 B4   B3 B2 B1 B0] // 0x___100 ..= 0x___1ff
//         [                                                  1 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x___200 ..= 0x___3ff
//         [                                               1 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x___400 ..= 0x___7ff
//     }

//     // Code points comprising between 12 and 16 bits (inclusive) always encode to 3 bytes.
//     3 {
//         [                                           1 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x___800 ..= 0x___fff
//         [                                     1   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x__1000 ..= 0x__1fff
//         [                                 1 B12   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x__2000 ..= 0x__3fff
//         [                             1 B13 B12   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x__4000 ..= 0x__7fff

//     // However 0xd800 through to 0xdfff (inclusive) are surrogates
//         [                         1   0 B13 B12   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x__8000 ..= 0x__bfff
//         [                         1   1   0   0   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x__c000 ..= 0x__cfff
//         [                         1   1   0   1     0 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x__d000 ..= 0x__d7ff
//     //  [                         1   1   0   1     1 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x__d800 ..= 0x__dfff **SURROGATES**
//         [                         1   1   1 B12   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x__e000 ..= 0x__ffff
//     }
//     // Code points comprising between 17 and 21 bits (inclusive) always encode to 4 bytes.
//     4 {
//         [                   1   B15 B14 B13 B12   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x_10000 ..= 0x_1ffff
//         [               1 B16   B15 B14 B13 B12   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x_20000 ..= 0x_3ffff
//         [           1 B17 B16   B15 B14 B13 B12   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x_40000 ..= 0x_7ffff
//         [       1 B18 B17 B16   B15 B14 B13 B12   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x_80000 ..= 0x_fffff

//     // However 0x10ffff is the maximum:
//         [  1    0   0   0   0   B15 B14 B13 B12   B11 B10 B9 B8   B7 B6 B5 B4   B3 B2 B1 B0] // 0x100000 ..= 0x10ffff
//     }
// }

// trait ToChar {
//     const CHAR: char;
// }
// impl<C: UnicodeScalarValue> ToChar for C {
//     const CHAR: char = char::from_u32(C::U32).unwrap();
// }

// #[repr(transparent)]
// struct Str<N: ArrayLength>(GenericArray<u8, N>);
// impl<N: ArrayLength> Clone for Str<N> {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }
// impl<N: ArrayLength<ArrayType<u8>: Copy>> Copy for Str<N> {}
// impl<N: ArrayLength> Str<N> {
//     const fn as_str(&self) -> &str {
//         unsafe { std::str::from_utf8_unchecked(self.0.as_slice()) }
//     }
//     const fn as_str_mut(&mut self) -> &mut str {
//         unsafe { std::str::from_utf8_unchecked_mut(self.0.as_mut_slice()) }
//     }
// }

// impl<N: ArrayLength> Deref for Str<N> {
//     type Target = str;
//     fn deref(&self) -> &str {
//         self.as_str()
//     }
// }
// impl<N: ArrayLength> DerefMut for Str<N> {
//     fn deref_mut(&mut self) -> &mut str {
//         self.as_str_mut()
//     }
// }

// trait ToStr {
//     type Len: ArrayLength<ArrayType<u8>: Copy>;
//     const STR: Str<Self::Len>;
//     // static S: Str<Self::Len> = Self::STR;
//     // const STR2: &str = {
//     //     S.as_str()
//     // };
// }

// impl<C: UnicodeScalarValue> ToStr for C {
//     type Len = C::Utf8Length;
//     const STR: Str<Self::Len> = {
//         assert!(Self::Len::USIZE == Self::CHAR.len_utf8());
//         let mut buf = GenericArray::DEFAULT;
//         C::CHAR.encode_utf8(buf.as_mut_slice());
//         Str(buf)
//     };
// }

// impl ToStr for Nil {
//     type Len = typenum::U0;
//     const STR: Str<typenum::U0> = Str(GenericArray::from_array([]));
// }

// impl<Head: ToStr, Tail: ToStr> ToStr for Cons<Head, Tail>
// where
//     Head::Len: Add<Tail::Len>,
//     typenum::Sum<Head::Len, Tail::Len>: ArrayLength,
// {
//     type Len = typenum::Sum<Head::Len, Tail::Len>;
//     const STR: Str<Self::Len> = {
//         const fn write_into<T>(slice: &mut [MaybeUninit<u8>], src: T) {
//             assert!(std::mem::size_of::<T>() == slice.len());
//             assert!(std::mem::align_of::<T>() == 1);

//             let dst = slice.as_mut_ptr().cast();
//             unsafe { std::ptr::write(dst, src) }
//         }

//         let mut buffer = GenericArray::uninit();
//         let (head, tail) = buffer.as_mut_slice().split_at_mut(Head::Len::USIZE);

//         write_into(head, Head::STR);
//         write_into(tail, Tail::STR);

//         Str(unsafe { GenericArray::assume_init(buffer) })
//     };
// }

// fn test() {
//     const {
//         let mut left = <Str!("hello")>::STR.as_str().as_bytes();
//         let mut right = "hello".as_bytes();

//         loop {

//             match (left.split_first(), right.split_first()) {
//                 (Some(l), Some(r)) => {
//                     assert!(*l.0 == *r.0);
//                     (left, right) = (l.1, r.1);
//                 }
//                 (None, None) => break,
//                 _ => unreachable!(),
//             }
//         }
//     }
// }

// impl TypeStr for Nil {
//     type Buffer = [u8; 0];
// }
// impl<Tail: TypeStr, const C: char> TypeStr for TypeChar<Tail, C> {
//     type Buffer = [u8; C.len_utf8()];
// }
// impl TypeStr for Nil {}
// impl<Tail: TypeStr, const C: char> TypeStr for TypeChar<Tail, C> {}

// impl fmt::Display for Nil {
//     fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
//         Ok(())
//     }
// }
// impl<Tail: TypeStr, const C: char> fmt::Display for TypeChar<Tail, C> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // let mut buffer = MaybeUninit::<[u8; {C.len_utf8() + Tail::}]
//         f.write_char(C)?;
//         self.0.fmt(f)
//     }
// }

// const fn stringify<Tail: Stringify, const C: char>(type_str: TypeChar<Tail, C>) -> &'static str {
//     const A: usize = char::len_utf8(C);
// }

// trait TypeStr {
//     const STRING: &str;
// }
// impl TypeStr for Nil {
//     const STRING: &str = "";
// }
// impl<Tail: TypeStr, const C: char> TypeStr for TypeChar<Tail, C> {
//     const STRING: &str = {
//         #[repr(C)]
//         struct Prepend<const A: usize, const B: usize>([u8; A], [u8; B]);

//         let a = C.len_utf8();
//         const B: usize = Tail::STRING.len();

//         let br = Tail::STRING.as_bytes().as_ptr().cast();
//         let bs: [u8; B] = unsafe { *br };

//         // let p = Prepend(C.ut)

//         C + TypeStr::STRING
//     };
// }

// impl<'a, M: 'a + MessageFields<'a>> VariantNames for M {
//     const NAMES: &'static [&'static str] = {
//         const NAME: &str = <M as Message>::NAME;
//         // const VARIANT: &str = M::
//         M::DESCRIPTOR
//         &[]
//     };
// }

// trait ToOptions<'a> {
//     type Options;
// }
// impl<'a, A: 'a + Value<Ros2Msg>, B: ToOptions<'a>> ToOptions<'a> for AltDesc<A, B> {
//     type Options = Cons<<Option<A> as Value<Ros2Msg>>::FormatType<'a>, B::Options>;
// }
// impl ToOptions<'_> for Term {
//     type Options = Nil;
// }

// struct Enum<M> {
//     discriminant: usize,
//     variants: M,//FIXME: needs to be a projection form M to its Converted form
// }
// impl<M: Message> Message for Enum<M> {
//     const NAME: &str = "Enum";
// }
// impl<'a, M: MessageFields<'a>> MessageFields<'a> for Enum<M> {
//     type Fields = Cons!(usize, <M::Fields as Variants>::Descriptor);
//     const DESCRIPTOR: crate::encoding::Desc<Self::Fields> = ();
//     fn as_fields(&'_ self) -> Self::Fields {

//     }
// }

// trait ToBoundedArrays<'a> {
//     type VariantsToBoundedArrays;
// }
// impl<'a, A, B: ToBoundedArrays<'a>> ToBoundedArrays<'a> for Alt<A, B> {
//     type VariantsToBoundedArrays = Cons<BoundedArray<'a, T, 1>, B::VariantsToBoundedArrays>;
// }
// impl ToBoundedArrays<'_> for Term {
//     type VariantsToBoundedArrays = Term;
// }

// impl<M: MessageExt<Ros2Msg>> Value<Ros2Msg> for Enum<M> {}

// use super::encoding::{Desc, Message, MessageFields};

// struct EnumVariants<Variants>(Variants);
// impl<Variants> Message for EnumVariants<Variants> {
//     const NAME: &'static str = "EnumVariants";
// }
// impl<Variants> MessageFields<'_> for EnumVariants<Variants> {
//     type Fields = ();
//     const DESCRIPTOR: Desc<Self::Fields> = ();
//     fn as_fields(&'_ self) -> Self::Fields {

//     }
// }

// trait ConvertInnerSchema<'a> {
//     type Converted;
// }

// impl<'a, A: 'a, B: ConvertInnerSchema<'a>> ConvertInnerSchema<'a> for AltDesc<A, B> {
//     // Self::Converted lacks a discriminant
//     // Self::Converted is not (currently) a FormatType<Ros2Msg> because A is not a Value<Ros2Msg> - it's a FormatType<Ros2Msg> !
//     type Converted = Cons!(BoundedArray<'a, A, 1>, B::Converted);
// }

// trait InnerSchema<'this> {
//     type Fields;
//     const FIELDS: Self::Fields;
// }
// impl<'this, M: 'this + MessageExtFields<'this, Ros2Msg, FieldSchema: ConvertInnerSchema<'this>>> InnerSchema<'this> for M {
//     type Fields = <M::FieldSchema as ConvertInnerSchema<'this>>::Converted;
//     const FIELDS: Self::Fields = M::FIELD_SCHEMA;
// }
