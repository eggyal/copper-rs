use super::{
    compound::{Compound, LowersCompound},
    element::{Element, LowerableElement, LowersElement},
    forward::{Forwarding, LowersVia},
    DataFormat, Lowerable, Lowered, Lowers,
};

/// Sealed trait to restrict [`Lowerable::Sigil`] to only the supported sigils.
pub trait Sigil {}

impl Sigil for Element {}
impl Sigil for Compound {}
impl Sigil for Forwarding {}

/// Sealed helper trait that exists to unify the [`LowersElement`], [`LowersCompound`] and
/// [`LowersVia`] traits so that each can provide blanket implementations of [`Lowers`].
pub trait LowersUnified<T: ?Sized + Lowerable, Sigil = <T as Lowerable>::Sigil>:
    DataFormat
{
    type UnifiedLowered<'a>: Lowered<Self>
    where
        T: 'a;
    fn lower_unified(t: &T) -> Self::UnifiedLowered<'_>;
}

/// The unified blanket implementation.
impl<Format: LowersUnified<T, T::Sigil>, T: ?Sized + Lowerable> Lowers<T> for Format {
    type Lowered<'a>
        = Self::UnifiedLowered<'a>
    where
        T: 'a;
    fn lower(t: &T) -> Self::Lowered<'_> {
        Format::lower_unified(t)
    }
}

impl<T: ?Sized + LowerableElement, Format: LowersElement<T>> LowersUnified<T, Element> for Format {
    type UnifiedLowered<'a>
        = Format::ElementLowered<'a>
    where
        T: 'a;
    fn lower_unified(t: &T) -> Self::UnifiedLowered<'_> {
        Format::lower_element(t)
    }
}

impl<T: ?Sized + Lowerable<Sigil = Compound>, Format: LowersCompound<T>> LowersUnified<T, Compound>
    for Format
{
    type UnifiedLowered<'a>
        = Format::LoweredCompound<'a>
    where
        T: 'a;
    fn lower_unified(t: &T) -> Self::UnifiedLowered<'_> {
        Format::lower_compound(t)
    }
}

impl<T: ?Sized + Lowerable<Sigil = Forwarding>, Format: LowersVia<T>> LowersUnified<T, Forwarding>
    for Format
{
    type UnifiedLowered<'a>
        = Format::ViaLowered<'a>
    where
        T: 'a;
    fn lower_unified(t: &T) -> Self::UnifiedLowered<'_> {
        Format::lower_via(t)
    }
}
