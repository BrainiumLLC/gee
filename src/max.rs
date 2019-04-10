pub trait Max {
    fn max(self, other: Self) -> Self;
}

impl Max for bool {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for i8 {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for i16 {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for i32 {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for i64 {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for i128 {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for isize {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}

impl Max for u8 {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for u16 {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for u32 {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for u64 {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for u128 {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
impl Max for usize {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}

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

impl Max for char {
    fn max(self, other: Self) -> Self {
        std::cmp::Ord::max(self, other)
    }
}
