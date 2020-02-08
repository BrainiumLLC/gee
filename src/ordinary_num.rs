use crate::{Max, Min};
use num_traits::{Num, NumCast};
use std::fmt::Debug;

pub trait OrdinaryNum: Copy + Debug + Min + Max + Num + NumCast + PartialOrd {
    fn two() -> Self {
        let one = Self::one();
        one + one
    }

    fn half(self) -> Self {
        self / Self::two()
    }
}
impl<T: Copy + Debug + Min + Max + Num + NumCast + PartialOrd> OrdinaryNum for T {}
