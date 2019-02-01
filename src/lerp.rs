use std::ops::{Add, Div, Mul, Sub};

pub fn lerp<T, U, V, F>(a: T, b: T, f: F) -> T
where
    for<'a> T: Sub<&'a T, Output = U>,
    U: Mul<F, Output = V>,
    V: Add<T, Output = T>,
{
    (b - &a) * f + a
}

pub fn lerp_half<T, U, V>(a: T, b: T) -> U::Output
where
    T: Add<Output = U>,
    V: From<u8>,
    U: Div<V>,
{
    (a + b) / 2.into()
}
