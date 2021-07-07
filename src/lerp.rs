pub fn lerp<T: en::Float>(a: T, b: T, f: T) -> T {
    (b - a) * f + a
}

pub fn lerp_half<T: en::Num>(a: T, b: T) -> T {
    (a + b).halved()
}
