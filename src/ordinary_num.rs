use crate::{cast, Max, Min};
use num_traits::{Num, NumCast};
use std::fmt::Debug;

pub trait OrdinaryNum: Copy + Debug + Min + Max + Num + NumCast + PartialOrd {
    fn two() -> Self {
        cast::num(2)
    }

    fn halved(self) -> Self {
        self / Self::two()
    }
}

impl<T: Copy + Debug + Min + Max + Num + NumCast + PartialOrd> OrdinaryNum for T {}
