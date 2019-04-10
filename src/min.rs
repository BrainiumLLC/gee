pub trait Min {
    fn min(self, other: Self) -> Self;
}

impl Min for bool {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for i8 {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for i16 {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for i32 {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for i64 {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for i128 {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for isize {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}

impl Min for u8 {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for u16 {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for u32 {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for u64 {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for u128 {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
impl Min for usize {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}

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

impl Min for char {
    fn min(self, other: Self) -> Self {
        std::cmp::Ord::min(self, other)
    }
}
