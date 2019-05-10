pub trait Min {
    fn min(self, other: Self) -> Self;
}

macro_rules! impl_min {
    ($ty:ty) => {
        impl Min for $ty {
            fn min(self, other: Self) -> Self {
                std::cmp::Ord::min(self, other)
            }
        }
    };
}

impl_min!(bool);
impl_min!(char);

impl_min!(i8);
impl_min!(i16);
impl_min!(i32);
impl_min!(i64);
impl_min!(i128);
impl_min!(isize);

impl_min!(u8);
impl_min!(u16);
impl_min!(u32);
impl_min!(u64);
impl_min!(u128);
impl_min!(usize);

impl Min for f32 {
    fn min(self, other: Self) -> Self {
        f32::min(self, other)
    }
}

impl Min for f64 {
    fn min(self, other: Self) -> Self {
        f64::min(self, other)
    }
}
