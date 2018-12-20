use num_traits::One;
use std::ops::{Add, Div, Mul, Sub};

pub fn lerp<T, U, V, F>(a: T, b: T, f: F) -> T
where
    for<'a> T: Sub<&'a T, Output = U>,
    U: Mul<F, Output = V>,
    V: Add<T, Output = T>,
{
    (b - &a) * f + a
}

pub fn lerp_half<T, U>(a: T, b: T) -> U::Output
where
    T: Add<Output = U> + One,
    U: Div,
{
    (a + b) / (T::one() + T::one())
}
