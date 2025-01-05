use super::{DataFormat, EncodableType, FormatType};

/// Creates [`ElementType`]s.
macro_rules! elements {
    ($($(* $($star:lifetime)?)? $e:ident $($t:ident)?),* $(,)?) => {$(
        impl$(<$t: $crate::EncodableType>)? $crate::NameableType for $(* $($star)?)? $e $($t)? {
            const NAME: &dyn ::core::fmt::Display = &$crate::concat![$("*" $($star)?,)? ::core::stringify!($e) $(, $t::NAME)?];
        }
        impl$(<$t: $crate::EncodableType>)? $crate::EncodableType for $(* $($star)?)? $e $($t)? {
            type Sigil = $crate::element::Element;
        }
        impl$(<$t: $crate::EncodableType>)? $crate::element::ElementType for $(* $($star)?)? $e $($t)? {}
    )*};
}

pub(crate) use elements;

mod private {
    /// [`EncodableType::Sigil`] sigil for [`ElementType`]s.
    ///
    /// [`EncodableType::Sigil`]: crate::EncodableType::Sigil
    /// [`ElementType`]: super::ElementType
    pub enum Element {}
}
pub(crate) use private::Element;
pub trait ElementType: EncodableType<Sigil = Element> {}

/// Proxy trait for implementing [`Encodes<T>`] when `T` is an [`ElementType`].
///
/// [`DataFormat`] definers will typically want to implement this trait for each
/// [`ElementType`], to describe how that format encodes it.
///
/// Consumers will usually want to use [`Encodes<T>`] instead, which is blanket implemented
/// for implementers of this trait (but also for other [`EncodableType`]s).
///
/// [`Encodes<T>`]: crate::Encodes<T>
pub trait EncodesElement<T: ?Sized + ElementType>: DataFormat {
    type ElementFormatType<'a>: FormatType<Self>
    where
        T: 'a;
    fn element_encodable(t: &T) -> Self::ElementFormatType<'_>;
}
