use super::{
    compound::{Compound, EncodesCompound},
    element::{Element, ElementType, EncodesElement},
    forward::{EncodesForwarding, Forwarding},
    DataFormat, EncodableType, Encodes, FormatType,
};

/// Sealed trait to restrict [`EncodableType::Sigil`] to only the supported sigils.
pub trait Sigil {}

impl Sigil for Element {}
impl Sigil for Compound {}
impl Sigil for Forwarding {}

/// Sealed helper trait that exists to unify the [`EncodesElement`], [`EncodesCompound`] and
/// [`EncodesForwarding`] traits so that each can provide blanket implementations of [`Encodes`].
pub trait UnifiedEncodes<T: ?Sized + EncodableType, Sigil = <T as EncodableType>::Sigil>:
    DataFormat
{
    type UnifiedFormatType<'a>: FormatType<Self>
    where
        T: 'a;
    fn unified_encodable(t: &T) -> Self::UnifiedFormatType<'_>;
}

/// The unified blanket implementation.
impl<Format: UnifiedEncodes<T, T::Sigil>, T: ?Sized + EncodableType> Encodes<T> for Format {
    type FormatType<'a>
        = Self::UnifiedFormatType<'a>
    where
        T: 'a;
    fn encodable(t: &T) -> Self::FormatType<'_> {
        Format::unified_encodable(t)
    }
}

impl<T: ?Sized + ElementType, Format: EncodesElement<T>> UnifiedEncodes<T, Element> for Format {
    type UnifiedFormatType<'a>
        = Format::ElementFormatType<'a>
    where
        T: 'a;
    fn unified_encodable(t: &T) -> Self::UnifiedFormatType<'_> {
        Format::element_encodable(t)
    }
}

impl<T: ?Sized + EncodableType<Sigil = Compound>, Format: EncodesCompound<T>>
    UnifiedEncodes<T, Compound> for Format
{
    type UnifiedFormatType<'a>
        = Format::CompoundFormatType<'a>
    where
        T: 'a;
    fn unified_encodable(t: &T) -> Self::UnifiedFormatType<'_> {
        Format::complex_encodable(t)
    }
}

impl<T: ?Sized + EncodableType<Sigil = Forwarding>, Format: EncodesForwarding<T>>
    UnifiedEncodes<T, Forwarding> for Format
{
    type UnifiedFormatType<'a>
        = Format::ForwardingFormatType<'a>
    where
        T: 'a;
    fn unified_encodable(t: &T) -> Self::UnifiedFormatType<'_> {
        Format::forward_encodable(t)
    }
}
