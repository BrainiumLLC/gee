pub trait Max: PartialOrd {
    fn max(self, other: Self) -> Self;
}

macro_rules! impl_max {
    ($ty:ty) => {
        impl Max for $ty {
            fn max(self, other: Self) -> Self {
                std::cmp::Ord::max(self, other)
            }
        }
    };
}

impl_max!(bool);
impl_max!(char);

impl_max!(i8);
impl_max!(i16);
impl_max!(i32);
impl_max!(i64);
impl_max!(i128);
impl_max!(isize);

impl_max!(u8);
impl_max!(u16);
impl_max!(u32);
impl_max!(u64);
impl_max!(u128);
impl_max!(usize);

impl Max for f32 {
    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }
}

impl Max for f64 {
    fn max(self, other: Self) -> Self {
        f64::max(self, other)
    }
}
