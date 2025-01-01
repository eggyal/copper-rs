use super::NameableType;

name! {
    bool,
    i8,
    u8,
    i16,
    u16,
    i32,
    u32,
    i64,
    u64,
    i128,
    u128,
    isize,
    usize,
    f32,
    f64,
    char,
    str,
}

impl<T: NameableType, const N: usize> NameableType for [T; N] {
    const NAME: &dyn std::fmt::Display = &concat!["[", T::NAME, "; ", N, "]"];
}
impl<T: NameableType> NameableType for [T] {
    const NAME: &dyn std::fmt::Display = &concat!["[", T::NAME, "]"];
}
impl<T: NameableType> NameableType for *const T {
    const NAME: &dyn std::fmt::Display = T::NAME;
}
impl<T: NameableType> NameableType for *mut T {
    const NAME: &dyn std::fmt::Display = T::NAME;
}
