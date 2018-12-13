use std::ops::{Add, Mul, Sub};

pub fn lerp<T, U, V, F>(a: T, b: T, f: F) -> T
where
    for<'a> T: Sub<&'a T, Output = U>,
    U: Mul<F, Output = V>,
    V: Add<T, Output = T>,
{
    (b - &a) * f + a
}
