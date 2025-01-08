use super::{DataFormat, Lowerable, Lowered};

/// Creates [`LowerableElement`]s.
macro_rules! elements {
    ($($(* $($star:lifetime)?)? $e:ident $($t:ident)?),* $(,)?) => {$(
        impl$(<$t: $crate::Lowerable>)? $crate::Lowerable for $(* $($star)?)? $e $($t)? {
            const NAME: &dyn ::core::fmt::Display = &$crate::concat![$("*" $($star)?,)? ::core::stringify!($e) $(, $t::NAME)?];
            type Sigil = $crate::element::Element;
        }
        impl$(<$t: $crate::Lowerable>)? $crate::element::LowerableElement for $(* $($star)?)? $e $($t)? {}
    )*};
}

pub(crate) use elements;

mod private {
    /// [`Lowerable::Sigil`] sigil for [`LowerableElement`]s.
    ///
    /// [`Lowerable::Sigil`]: crate::Lowerable::Sigil
    /// [`LowerableElement`]: super::LowerableElement
    pub enum Element {}
}
pub(crate) use private::Element;
pub trait LowerableElement: Lowerable<Sigil = Element> {}

/// Proxy trait for implementing [`Lowers<T>`] when `T` is an [`LowerableElement`].
///
/// [`DataFormat`] definers will typically want to implement this trait for each
/// [`LowerableElement`], to describe how that format encodes it.
///
/// Consumers will usually want to use [`Lowers<T>`] instead, which is blanket implemented
/// for implementers of this trait (but also for other [`Lowerable`]s).
///
/// [`Lowers<T>`]: crate::Lowers<T>
pub trait LowersElement<T: ?Sized + LowerableElement>: DataFormat {
    type ElementLowered<'a>: Lowered<Self>
    where
        T: 'a;
    fn lower_element(t: &T) -> Self::ElementLowered<'_>;
}
