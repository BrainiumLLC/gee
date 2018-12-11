use std::ops::{Add, Mul, Sub};

fn lerp<T, U, V, F>(a: T, b: T, f: F) -> T
where
    T: Sub<T, Output = U> + Add<V, Output = T> + Clone,
    U: Mul<F, Output = V>,
{
    b.clone() + (a - b) * f
}
