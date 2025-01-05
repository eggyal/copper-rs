use crate::{delegate, element::elements, forward::tuples, iterate};

elements! {
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

    *const T,
    *mut T,
}

delegate! {
    fn['b, T](this: &'b T) -> T { this }
    fn['b, T](this: &'b mut T) -> T { this }
}

iterate! {
    static defined fn[T; const N: usize](&'a this: [T; N]) -> &'a [T; N] as T { this }
    unbounded defined fn[T](&'a this: [T]) -> &'a [T] as T { this }
}

tuples![
    0:A,
    1:B,
    2:C,
    3:D,
    4:E,
    5:F,
    6:G,
    7:H,
    8:I,
    9:J,
    10:K,
    11:L,
    12:M,
    13:N,
    14:O,
    15:P,
];
